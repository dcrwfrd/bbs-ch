use std::collections::HashMap;
use std::path::Path;

use anyhow::{Context, Result};
use rusqlite::{params, Connection};

use crate::model::{
    conference::{Conference, Region},
    league::League,
    prestige::{ConferenceTier, ProgramPrestige},
    school::{HomeCourt, School},
    tournament::{BracketFormat, Tournament, TournamentKind},
};

// ---------------------------------------------------------------------------
// Schema
// ---------------------------------------------------------------------------

const SCHEMA: &str = "
PRAGMA journal_mode = WAL;

CREATE TABLE IF NOT EXISTS conferences (
    abbreviation  TEXT PRIMARY KEY,
    name          TEXT NOT NULL,
    region        TEXT NOT NULL,
    tier          TEXT NOT NULL,
    logo          TEXT
);

CREATE TABLE IF NOT EXISTS schools (
    abbreviation                TEXT PRIMARY KEY,
    name                        TEXT NOT NULL,
    nickname                    TEXT NOT NULL,
    prefer_abbreviation         INTEGER NOT NULL DEFAULT 0,
    zip_code                    TEXT    NOT NULL DEFAULT '',
    conference                  TEXT    NOT NULL DEFAULT '',
    primary_color               TEXT    NOT NULL DEFAULT '#333366',
    secondary_color             TEXT    NOT NULL DEFAULT '#ffffff',
    home_court_name             TEXT    NOT NULL DEFAULT '',
    home_court_capacity         INTEGER NOT NULL DEFAULT 0,
    home_court_atmosphere       INTEGER NOT NULL DEFAULT 50,
    nil_budget                  INTEGER NOT NULL DEFAULT 0,
    primary_rival               TEXT,
    prestige_cumulative_success INTEGER NOT NULL DEFAULT 50,
    prestige_recent_success     INTEGER NOT NULL DEFAULT 50,
    prestige_facilities         INTEGER NOT NULL DEFAULT 50,
    prestige_resources          INTEGER NOT NULL DEFAULT 50,
    prestige_atmosphere         INTEGER NOT NULL DEFAULT 50,
    prestige_coaching_staff     INTEGER NOT NULL DEFAULT 50,
    prestige_market             INTEGER NOT NULL DEFAULT 50,
    prestige_media_perception   INTEGER NOT NULL DEFAULT 50,
    prestige_academics          INTEGER NOT NULL DEFAULT 50
);

CREATE TABLE IF NOT EXISTS school_rivals (
    school_abbreviation  TEXT NOT NULL,
    rival_abbreviation   TEXT NOT NULL,
    PRIMARY KEY (school_abbreviation, rival_abbreviation)
);

CREATE TABLE IF NOT EXISTS tournaments (
    abbreviation    TEXT PRIMARY KEY,
    name            TEXT NOT NULL,
    logo            TEXT,
    kind            TEXT NOT NULL,
    bracket_format  TEXT NOT NULL,
    field_size      INTEGER NOT NULL DEFAULT 0,
    host_site       TEXT    NOT NULL DEFAULT '',
    conference      TEXT,
    automatic_bids  INTEGER NOT NULL DEFAULT 0,
    at_large_bids   INTEGER NOT NULL DEFAULT 0
);
";

// ---------------------------------------------------------------------------
// LeagueDb
// ---------------------------------------------------------------------------

/// SQLite-backed store for the static league world: schools, conferences, and
/// tournaments.
///
/// This is *configuration*, not save state — it lives in `data/league.db` and
/// is edited through the League Builder. Dynamic save state (rosters, players,
/// game progress) remains in `JsonStorage` for now.
///
/// The database uses `abbreviation` as the stable primary key for all entities.
/// Runtime integer IDs (used throughout the in-memory model) are assigned by
/// `load_league()` after reading and are not persisted.
pub struct LeagueDb {
    conn: Connection,
}

impl LeagueDb {
    /// Opens (or creates) the league database at `path` and initialises the
    /// schema. Safe to call on an existing database — all `CREATE TABLE`
    /// statements use `IF NOT EXISTS`.
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let conn = Connection::open(path)
            .with_context(|| format!("Failed to open league DB: {}", path.display()))?;
        conn.execute_batch(SCHEMA)
            .context("Failed to initialise league DB schema")?;
        Ok(Self { conn })
    }

    /// Returns `true` if the `schools` table contains no rows.
    ///
    /// Used at startup to detect a freshly created database before deciding
    /// whether to attempt JSON migration.
    pub fn is_empty(&self) -> Result<bool> {
        let n: i64 = self.conn
            .query_row("SELECT COUNT(*) FROM schools", [], |row| row.get(0))
            .context("Failed to count schools")?;
        Ok(n == 0)
    }

    // ── Load ────────────────────────────────────────────────────────────────

    /// Loads the full league from SQLite, assigns deterministic runtime IDs,
    /// rebuilds conference membership from school records, and validates
    /// primary-rival symmetry.
    ///
    /// This mirrors the cross-reference resolution that the old JSON
    /// `DataLoader` performed, now with SQLite as the backing store.
    pub fn load_league(&self) -> Result<League> {
        let mut schools     = self.load_schools()?;
        let mut conferences = self.load_conferences()?;
        let mut tournaments = self.load_tournaments()?;

        // Sort alphabetically, then assign sequential IDs.
        // IDs are stable as long as the data files are unchanged — the same
        // caveat that existed with JSON loading.
        schools.sort_by(|a, b| a.name.cmp(&b.name));
        for (i, s) in schools.iter_mut().enumerate() {
            s.id = (i + 1) as u32;
        }

        conferences.sort_by(|a, b| a.name.cmp(&b.name));
        for (i, c) in conferences.iter_mut().enumerate() {
            c.id = (i + 1) as u32;
        }

        tournaments.sort_by(|a, b| a.name.cmp(&b.name));
        for (i, t) in tournaments.iter_mut().enumerate() {
            t.id = (i + 1) as u32;
        }

        // Build lookup maps for cross-reference resolution.
        let abbr_to_school_id: HashMap<String, u32> = schools.iter()
            .map(|s| (s.abbreviation.clone(), s.id))
            .collect();

        let conf_abbr_to_id: HashMap<String, u32> = conferences.iter()
            .map(|c| (c.abbreviation.clone(), c.id))
            .collect();

        // Resolve school.conference (abbreviation) → school.conference_id.
        for school in &mut schools {
            school.conference_id = conf_abbr_to_id
                .get(&school.conference)
                .copied()
                .unwrap_or(0);
        }

        // Rebuild conference.members and member_ids from the school records.
        // The DB does not store membership separately — it is derived from
        // school.conference, exactly as the old JSON loader did.
        for conf in &mut conferences {
            conf.members = schools.iter()
                .filter(|s| s.conference == conf.abbreviation)
                .map(|s| s.abbreviation.clone())
                .collect();
            conf.member_ids = conf.members.iter()
                .filter_map(|abbr| abbr_to_school_id.get(abbr).copied())
                .collect();
        }

        // Validate primary-rival symmetry. Hard error — the game logic relies
        // on this invariant holding universally.
        let abbr_to_primary: HashMap<String, Option<String>> = schools.iter()
            .map(|s| (s.abbreviation.clone(), s.primary_rival.clone()))
            .collect();

        for school in &schools {
            if let Some(ref rival_abbr) = school.primary_rival {
                let rival_primary = abbr_to_primary
                    .get(rival_abbr.as_str())
                    .with_context(|| format!(
                        "School '{}' has primary_rival '{}' which is not a known school.",
                        school.abbreviation, rival_abbr
                    ))?;
                match rival_primary {
                    Some(ref back) if back == &school.abbreviation => {}
                    Some(ref back) => {
                        anyhow::bail!(
                            "Primary rival mismatch: '{}' → '{}', but '{}' → '{}'.",
                            school.abbreviation, rival_abbr, rival_abbr, back
                        );
                    }
                    None => {
                        anyhow::bail!(
                            "Primary rival mismatch: '{}' → '{}', but '{}' has no primary rival set.",
                            school.abbreviation, rival_abbr, rival_abbr
                        );
                    }
                }
            }
        }

        Ok(League { schools, conferences, tournaments, logo: None })
    }

    fn load_schools(&self) -> Result<Vec<School>> {
        let mut stmt = self.conn.prepare(
            "SELECT abbreviation, name, nickname, prefer_abbreviation, zip_code,
                    conference, primary_color, secondary_color,
                    home_court_name, home_court_capacity, home_court_atmosphere,
                    nil_budget, primary_rival,
                    prestige_cumulative_success, prestige_recent_success,
                    prestige_facilities, prestige_resources, prestige_atmosphere,
                    prestige_coaching_staff, prestige_market, prestige_media_perception,
                    prestige_academics
             FROM schools",
        ).context("Failed to prepare school SELECT")?;

        let mut schools: Vec<School> = stmt
            .query_map([], |row| {
                Ok(School {
                    id:                   0,
                    abbreviation:         row.get(0)?,
                    name:                 row.get(1)?,
                    nickname:             row.get(2)?,
                    prefer_abbreviation:  row.get::<_, i64>(3)? != 0,
                    zip_code:             row.get(4)?,
                    conference:           row.get(5)?,
                    conference_id:        0,
                    primary_color:        row.get(6)?,
                    secondary_color:      row.get(7)?,
                    home_court: HomeCourt {
                        name:       row.get(8)?,
                        capacity:   row.get::<_, i64>(9)? as u32,
                        atmosphere: row.get::<_, i64>(10)? as u8,
                    },
                    nil_budget:    row.get::<_, i64>(11)? as u32,
                    enrollment:    0,
                    primary_rival: row.get(12)?,
                    prestige: ProgramPrestige {
                        cumulative_success: row.get::<_, i64>(13)? as u8,
                        recent_success:     row.get::<_, i64>(14)? as u8,
                        facilities:         row.get::<_, i64>(15)? as u8,
                        resources:          row.get::<_, i64>(16)? as u8,
                        atmosphere:         row.get::<_, i64>(17)? as u8,
                        coaching_staff:     row.get::<_, i64>(18)? as u8,
                        market:             row.get::<_, i64>(19)? as u8,
                        media_perception:   row.get::<_, i64>(20)? as u8,
                        academics:          row.get::<_, i64>(21)? as u8,
                    },
                    rivals: Vec::new(), // populated below from school_rivals
                })
            })?
            .collect::<Result<_, _>>()
            .context("Failed to load schools from DB")?;

        // Load all rival pairs in a single query and distribute them.
        let mut rival_stmt = self.conn.prepare(
            "SELECT school_abbreviation, rival_abbreviation FROM school_rivals",
        ).context("Failed to prepare school_rivals SELECT")?;

        let pairs: Vec<(String, String)> = rival_stmt
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))?
            .collect::<Result<_, _>>()
            .context("Failed to load school rivals from DB")?;

        // Build an abbreviation index so we don't scan the whole Vec per pair.
        let abbr_index: HashMap<String, usize> = schools.iter()
            .enumerate()
            .map(|(i, s)| (s.abbreviation.clone(), i))
            .collect();

        for (school_abbr, rival_abbr) in pairs {
            if let Some(&idx) = abbr_index.get(&school_abbr) {
                schools[idx].rivals.push(rival_abbr);
            }
        }

        // abbr_index borrow ends here — drop it before we sort the vec.
        drop(abbr_index);

        Ok(schools)
    }

    fn load_conferences(&self) -> Result<Vec<Conference>> {
        let mut stmt = self.conn.prepare(
            "SELECT abbreviation, name, region, tier, logo FROM conferences",
        ).context("Failed to prepare conference SELECT")?;

        let result: Vec<Conference> = stmt
            .query_map([], |row| {
                let region_str: String = row.get(2)?;
                let tier_str:   String = row.get(3)?;
                Ok(Conference {
                    id:           0,
                    abbreviation: row.get(0)?,
                    name:         row.get(1)?,
                    region:       str_to_region(&region_str),
                    tier:         str_to_tier(&tier_str),
                    logo:         row.get(4)?,
                    members:      Vec::new(), // rebuilt by load_league
                    member_ids:   Vec::new(),
                })
            })?
            .collect::<Result<_, _>>()
            .context("Failed to load conferences from DB")?;
        Ok(result)
    }

    fn load_tournaments(&self) -> Result<Vec<Tournament>> {
        let mut stmt = self.conn.prepare(
            "SELECT abbreviation, name, logo, kind, bracket_format,
                    field_size, host_site, conference, automatic_bids, at_large_bids
             FROM tournaments",
        ).context("Failed to prepare tournament SELECT")?;

        let result: Vec<Tournament> = stmt
            .query_map([], |row| {
                let kind_str: String = row.get(3)?;
                let fmt_str:  String = row.get(4)?;
                Ok(Tournament {
                    id:             0,
                    abbreviation:   row.get(0)?,
                    name:           row.get(1)?,
                    logo:           row.get(2)?,
                    kind:           str_to_kind(&kind_str),
                    bracket_format: str_to_format(&fmt_str),
                    field_size:     row.get::<_, i64>(5)? as u32,
                    host_site:      row.get(6)?,
                    conference:     row.get(7)?,
                    automatic_bids: row.get::<_, i64>(8)? as u32,
                    at_large_bids:  row.get::<_, i64>(9)? as u32,
                })
            })?
            .collect::<Result<_, _>>()
            .context("Failed to load tournaments from DB")?;
        Ok(result)
    }

    // ── Save ────────────────────────────────────────────────────────────────

    /// Persists the entire league to the database inside a single transaction.
    ///
    /// The strategy is full replace: all existing rows are deleted and the
    /// current in-memory state is re-inserted. This keeps the logic simple and
    /// correct — the league is small enough that partial updates offer no
    /// meaningful benefit.
    pub fn save_league(&mut self, league: &League) -> Result<()> {
        let tx = self.conn.transaction()
            .context("Failed to begin save transaction")?;

        // Delete in an order that respects the implicit FK relationships:
        // rivals → schools → conferences, and tournaments stand alone.
        tx.execute_batch(
            "DELETE FROM school_rivals;
             DELETE FROM schools;
             DELETE FROM conferences;
             DELETE FROM tournaments;",
        ).context("Failed to clear league tables")?;

        for c in &league.conferences {
            tx.execute(
                "INSERT INTO conferences (abbreviation, name, region, tier, logo)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                params![
                    c.abbreviation, c.name,
                    region_to_str(&c.region),
                    tier_to_str(&c.tier),
                    c.logo,
                ],
            ).with_context(|| format!("Failed to insert conference '{}'", c.abbreviation))?;
        }

        for s in &league.schools {
            tx.execute(
                "INSERT INTO schools (
                    abbreviation, name, nickname, prefer_abbreviation, zip_code,
                    conference, primary_color, secondary_color,
                    home_court_name, home_court_capacity, home_court_atmosphere,
                    nil_budget, primary_rival,
                    prestige_cumulative_success, prestige_recent_success,
                    prestige_facilities, prestige_resources, prestige_atmosphere,
                    prestige_coaching_staff, prestige_market, prestige_media_perception,
                    prestige_academics
                 ) VALUES (
                    ?1,  ?2,  ?3,  ?4,  ?5,  ?6,  ?7,  ?8,  ?9,  ?10, ?11,
                    ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22
                 )",
                params![
                    s.abbreviation, s.name, s.nickname,
                    s.prefer_abbreviation as i64,
                    s.zip_code, s.conference, s.primary_color, s.secondary_color,
                    s.home_court.name,
                    s.home_court.capacity as i64,
                    s.home_court.atmosphere as i64,
                    s.nil_budget as i64,
                    s.primary_rival,
                    s.prestige.cumulative_success as i64,
                    s.prestige.recent_success     as i64,
                    s.prestige.facilities         as i64,
                    s.prestige.resources          as i64,
                    s.prestige.atmosphere         as i64,
                    s.prestige.coaching_staff     as i64,
                    s.prestige.market             as i64,
                    s.prestige.media_perception   as i64,
                    s.prestige.academics          as i64,
                ],
            ).with_context(|| format!("Failed to insert school '{}'", s.abbreviation))?;

            for rival in &s.rivals {
                tx.execute(
                    "INSERT INTO school_rivals (school_abbreviation, rival_abbreviation)
                     VALUES (?1, ?2)",
                    params![s.abbreviation, rival],
                ).with_context(|| format!(
                    "Failed to insert rival '{}' for school '{}'",
                    rival, s.abbreviation
                ))?;
            }
        }

        for t in &league.tournaments {
            tx.execute(
                "INSERT INTO tournaments (
                    abbreviation, name, logo, kind, bracket_format,
                    field_size, host_site, conference, automatic_bids, at_large_bids
                 ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
                params![
                    t.abbreviation, t.name, t.logo,
                    kind_to_str(&t.kind),
                    format_to_str(&t.bracket_format),
                    t.field_size as i64,
                    t.host_site,
                    t.conference,
                    t.automatic_bids as i64,
                    t.at_large_bids  as i64,
                ],
            ).with_context(|| format!("Failed to insert tournament '{}'", t.abbreviation))?;
        }

        tx.commit().context("Failed to commit save transaction")
    }
}

// ---------------------------------------------------------------------------
// Enum ↔ TEXT helpers
// ---------------------------------------------------------------------------

fn region_to_str(r: &Region) -> &'static str {
    match r {
        Region::Northeast   => "Northeast",
        Region::MidAtlantic => "MidAtlantic",
        Region::Southeast   => "Southeast",
        Region::Midwest     => "Midwest",
        Region::Plains      => "Plains",
        Region::Southwest   => "Southwest",
        Region::West        => "West",
    }
}

fn str_to_region(s: &str) -> Region {
    match s {
        "Northeast"   => Region::Northeast,
        "MidAtlantic" => Region::MidAtlantic,
        "Southeast"   => Region::Southeast,
        "Plains"      => Region::Plains,
        "Southwest"   => Region::Southwest,
        "West"        => Region::West,
        _             => Region::Midwest,
    }
}

fn tier_to_str(t: &ConferenceTier) -> &'static str {
    match t {
        ConferenceTier::PowerConference => "PowerConference",
        ConferenceTier::MidMajor        => "MidMajor",
        ConferenceTier::LowMajor        => "LowMajor",
        ConferenceTier::Independent     => "Independent",
    }
}

fn str_to_tier(s: &str) -> ConferenceTier {
    match s {
        "PowerConference" => ConferenceTier::PowerConference,
        "MidMajor"        => ConferenceTier::MidMajor,
        "Independent"     => ConferenceTier::Independent,
        _                 => ConferenceTier::LowMajor,
    }
}

fn kind_to_str(k: &TournamentKind) -> &'static str {
    match k {
        TournamentKind::InSeason             => "InSeason",
        TournamentKind::ConferenceTourney    => "ConferenceTourney",
        TournamentKind::NationalChampionship => "NationalChampionship",
        TournamentKind::SecondaryPostseason  => "SecondaryPostseason",
    }
}

fn str_to_kind(s: &str) -> TournamentKind {
    match s {
        "ConferenceTourney"    => TournamentKind::ConferenceTourney,
        "NationalChampionship" => TournamentKind::NationalChampionship,
        "SecondaryPostseason"  => TournamentKind::SecondaryPostseason,
        _                      => TournamentKind::InSeason,
    }
}

fn format_to_str(f: &BracketFormat) -> &'static str {
    match f {
        BracketFormat::SingleElimination => "SingleElimination",
        BracketFormat::DoubleElimination => "DoubleElimination",
        BracketFormat::RoundRobin        => "RoundRobin",
    }
}

fn str_to_format(s: &str) -> BracketFormat {
    match s {
        "DoubleElimination" => BracketFormat::DoubleElimination,
        "RoundRobin"        => BracketFormat::RoundRobin,
        _                   => BracketFormat::SingleElimination,
    }
}
