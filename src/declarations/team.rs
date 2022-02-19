use crate::declarations::WeightMember;
use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::path::Path;

pub struct Team {
    full_path: String,
}

impl Team {
    pub fn new(path: &String, name: &String) -> Self {
        Team {
            full_path: format!("{}/{}.json", path, name),
        }
    }

    pub fn create(&self) -> Result<(), String> {
        if self.exists() {
            return Err("Team exists".to_string());
        }

        match File::create(&self.full_path) {
            Err(_) => Err("Team creation error!".to_string()),
            Ok(_) => Ok(()),
        }
    }

    pub fn delete(&self) -> Result<(), String> {
        if !self.exists() {
            return Err("Team exists".to_string());
        }

        match fs::remove_file(&self.full_path) {
            Err(_) => Err("Team deleting error!".to_string()),
            Ok(_) => Ok(()),
        }
    }

    pub fn add_member(&self, name: &String) -> Result<(), String> {
        let team_members = match self.read_team_members_from_file() {
            Err(_) => return Err("Wrong team".to_string()),
            Ok(team_members) => team_members,
        };

        let mut members_set = team_members.into_iter().collect::<HashSet<String>>();
        members_set.insert(name.clone());

        let updated_team_members = members_set.into_iter().collect::<Vec<String>>();

        self.write_team_members_to_file(updated_team_members)
    }

    pub fn delete_member(&self, name: &String) -> Result<(), String> {
        let team_members = match self.read_team_members_from_file() {
            Err(_) => return Err("Wrong team".to_string()),
            Ok(team_members) => team_members,
        };

        let mut members_set = team_members.into_iter().collect::<HashSet<String>>();
        members_set.remove(name);

        let updated_team_members = members_set.into_iter().collect::<Vec<String>>();

        self.write_team_members_to_file(updated_team_members)
    }

    pub fn get_shuffled_members(&self) -> Result<Vec<String>, String> {
        let team_members = match self.read_team_members_from_file() {
            Err(message) => return Err(message),
            Ok(team_members) => team_members,
        };

        let mut weight_team_members = WeightMember::get_weight_list_from_vec(&team_members);
        weight_team_members.sort_by(|a, b| a.weight.partial_cmp(&b.weight).unwrap());

        Ok(WeightMember::get_vec_from_weight_list(&weight_team_members))
    }

    fn read_team_members_from_file(&self) -> Result<Vec<String>, String> {
        let file_string = match fs::read_to_string(&self.full_path) {
            Err(_) => return Err("Invalid file!".to_string()),
            Ok(file_string) => file_string,
        };

        match serde_json::from_str::<Vec<String>>(&file_string) {
            Err(_) => Ok(vec![]),
            Ok(team_members) => Ok(team_members),
        }
    }

    fn write_team_members_to_file(&self, team_members: Vec<String>) -> Result<(), String> {
        let updated_file_string = match serde_json::to_string(&team_members) {
            Err(_) => return Err("Inserting error!".to_string()),
            Ok(updated_file_string) => updated_file_string,
        };

        match fs::write(&self.full_path, updated_file_string) {
            Err(_) => return Err("Inserting error!".to_string()),
            Ok(()) => Ok(()),
        }
    }

    fn exists(&self) -> bool {
        Path::new(&self.full_path).exists()
    }
}
