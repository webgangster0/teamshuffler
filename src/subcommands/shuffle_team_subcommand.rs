use crate::declarations::Team;

pub struct ShuffleTeamSubcommand {
    work_dir_path: String,
    team_name: String,
}

impl ShuffleTeamSubcommand {
    pub fn new(work_dir_path: &String, team_name: &String) -> Self {
        ShuffleTeamSubcommand {
            work_dir_path: work_dir_path.clone(),
            team_name: team_name.clone(),
        }
    }

    pub fn execute(&self) -> Result<(), String> {
        let team = Team::new(&self.work_dir_path, &self.team_name);
        let shuffled_names = match team.get_shuffled_members() {
            Err(_) => return Err("Invalid team!".to_string()),
            Ok(shuffled_names) => shuffled_names,
        };

        let mut i = 1;
        for name in shuffled_names {
            println!("[{}] {}", i, name);
            i += 1;
        }

        Ok(())
    }
}
