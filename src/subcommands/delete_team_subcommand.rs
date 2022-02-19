use crate::declarations::Team;

pub struct DeleteTeamSubcommand {
    work_dir_path: String,
    team_name: String,
}

impl DeleteTeamSubcommand {
    pub fn new(work_dir_path: &String, team_name: &String) -> Self {
        DeleteTeamSubcommand {
            work_dir_path: work_dir_path.clone(),
            team_name: team_name.clone(),
        }
    }

    pub fn execute(&self) -> Result<(), String> {
        let team = Team::new(&self.work_dir_path, &self.team_name);
        team.delete()
    }
}
