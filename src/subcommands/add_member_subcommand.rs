use crate::declarations::Team;

pub struct AddMemberSubcommand {
    work_dir_path: String,
    team_name: String,
    member_name: String,
}

impl AddMemberSubcommand {
    pub fn new(work_dir_path: &String, team_name: &String, member_name: &String) -> Self {
        AddMemberSubcommand {
            work_dir_path: work_dir_path.clone(),
            team_name: team_name.clone(),
            member_name: member_name.clone(),
        }
    }

    pub fn execute(&self) -> Result<(), String> {
        let team = Team::new(&self.work_dir_path, &self.team_name);
        team.add_member(&self.member_name)
    }
}
