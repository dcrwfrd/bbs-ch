use crate::model::{conference::Conference, school::School, tournament::Tournament};

/// The loaded static game world — schools, conferences, and tournaments that
/// define the league structure. Read from `data/` at startup and held in
/// memory for the lifetime of a session.
///
/// This is config, not save state — it lives in `DataLoader`, not `Storage`.
#[derive(Debug, Clone)]
pub struct League {
    pub schools: Vec<School>,
    pub conferences: Vec<Conference>,
    pub tournaments: Vec<Tournament>,
    /// Path to the governing association's logo asset, relative to `data/`.
    /// e.g., `"logos/league.png"`. `None` if not configured.
    pub logo: Option<String>,
}

impl League {
    pub fn school_by_id(&self, id: u32) -> Option<&School> {
        self.schools.iter().find(|s| s.id == id)
    }

    pub fn school_by_id_mut(&mut self, id: u32) -> Option<&mut School> {
        self.schools.iter_mut().find(|s| s.id == id)
    }

    pub fn conference_by_id(&self, id: u32) -> Option<&Conference> {
        self.conferences.iter().find(|c| c.id == id)
    }

    pub fn conference_by_id_mut(&mut self, id: u32) -> Option<&mut Conference> {
        self.conferences.iter_mut().find(|c| c.id == id)
    }

    pub fn tournament_by_id(&self, id: u32) -> Option<&Tournament> {
        self.tournaments.iter().find(|t| t.id == id)
    }

    pub fn tournament_by_id_mut(&mut self, id: u32) -> Option<&mut Tournament> {
        self.tournaments.iter_mut().find(|t| t.id == id)
    }

    pub fn schools_in_conference(&self, conference_id: u32) -> Vec<&School> {
        self.schools.iter().filter(|s| s.conference_id == conference_id).collect()
    }
}
