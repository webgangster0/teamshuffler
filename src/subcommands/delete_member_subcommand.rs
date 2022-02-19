use crate::declarations::Team;

pub struct DeleteMemberSubcommand {
    work_dir_path: String,
    team_name: String,
    member_name: String,
}

impl DeleteMemberSubcommand {
    pub fn new(work_dir_path: &String, team_name: &String, member_name: &String) -> Self {
        DeleteMemberSubcommand {
            work_dir_path: work_dir_path.clone(),
            team_name: team_name.clone(),
            member_name: member_name.clone(),
        }
    }

    pub fn execute(&self) -> Result<(), String> {
        let team = Team::new(&self.work_dir_path, &self.team_name);
        team.delete_member(&self.member_name)
    }
}
