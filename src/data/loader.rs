use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::model::league::League;
use crate::storage::db::LeagueDb;

/// Loads static league configuration from the `data/` directory.
///
/// The authoritative store is `data/league.db` (SQLite). On first run, if the
/// database does not exist or is empty but legacy JSON files (`schools.json`,
/// `conferences.json`, `tournaments.json`) are present, they are automatically
/// migrated into the database and renamed to `.json.bak` so the migration
/// does not trigger again.
pub struct DataLoader {
    root: PathBuf,
}

impl DataLoader {
    pub fn new(root: impl AsRef<Path>) -> Self {
        Self { root: root.as_ref().to_path_buf() }
    }

    /// Opens (or creates) `data/league.db`, migrates from JSON if needed,
    /// and returns the fully resolved `League`.
    pub fn load_league(&self) -> Result<League> {
        let db_path = self.root.join("league.db");
        let mut db = LeagueDb::open(&db_path)?;

        if db.is_empty()? {
            if let Some(league) = self.try_load_json()? {
                // JSON files exist — migrate them into SQLite.
                db.save_league(&league)
                    .context("Failed to migrate JSON league data to SQLite")?;
                self.archive_json_files();
            }
            // If neither DB nor JSON has data, we just return an empty league
            // (the League Builder is the intended way to populate the world).
        }

        db.load_league()
    }

    /// Attempts to load the league from the legacy JSON files.
    ///
    /// Returns `None` if none of the JSON files exist (no migration needed).
    /// Returns the loaded `League` if at least `schools.json` is present.
    fn try_load_json(&self) -> Result<Option<League>> {
        use std::collections::HashMap;
        use crate::model::{conference::Conference, school::School, tournament::Tournament};

        let schools_path = self.root.join("schools.json");
        if !schools_path.exists() {
            return Ok(None);
        }

        let mut schools: Vec<School> = self.read_json(&schools_path)?;
        let confs_path = self.root.join("conferences.json");
        let mut conferences: Vec<Conference> = if confs_path.exists() {
            self.read_json(&confs_path)?
        } else {
            Vec::new()
        };

        // Sort and assign IDs so cross-reference resolution works.
        schools.sort_by(|a, b| a.name.cmp(&b.name));
        for (i, s) in schools.iter_mut().enumerate() { s.id = (i + 1) as u32; }

        conferences.sort_by(|a, b| a.name.cmp(&b.name));
        for (i, c) in conferences.iter_mut().enumerate() { c.id = (i + 1) as u32; }

        let abbr_to_school_id: HashMap<String, u32> = schools.iter()
            .map(|s| (s.abbreviation.clone(), s.id))
            .collect();
        let conf_abbr_to_id: HashMap<String, u32> = conferences.iter()
            .map(|c| (c.abbreviation.clone(), c.id))
            .collect();

        for school in &mut schools {
            school.conference_id = conf_abbr_to_id
                .get(school.conference.as_str())
                .copied()
                .unwrap_or(0);
        }
        for conf in &mut conferences {
            conf.member_ids = conf.members.iter()
                .filter_map(|abbr| abbr_to_school_id.get(abbr.as_str()).copied())
                .collect();
        }

        let tourneys_path = self.root.join("tournaments.json");
        let mut tournaments: Vec<Tournament> = if tourneys_path.exists() {
            self.read_json(&tourneys_path)?
        } else {
            Vec::new()
        };
        tournaments.sort_by(|a, b| a.name.cmp(&b.name));
        for (i, t) in tournaments.iter_mut().enumerate() { t.id = (i + 1) as u32; }

        Ok(Some(League { schools, conferences, tournaments, logo: None }))
    }

    fn read_json<T: serde::de::DeserializeOwned>(&self, path: &Path) -> Result<Vec<T>> {
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read {}", path.display()))?;
        serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse {}", path.display()))
    }

    /// Renames the legacy JSON files to `.json.bak` so migration does not
    /// trigger again on subsequent runs. Failures are silently ignored — a
    /// duplicate migration would simply overwrite the same data.
    fn archive_json_files(&self) {
        for name in &["schools.json", "conferences.json", "tournaments.json"] {
            let src = self.root.join(name);
            if src.exists() {
                let dst = self.root.join(format!("{name}.bak"));
                let _ = std::fs::rename(&src, &dst);
            }
        }
    }
}
