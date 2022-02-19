use clap::{Parser, Subcommand};
use std::fs;

use crate::subcommands::{
    AddMemberSubcommand, AddTeamSubcommand, DeleteMemberSubcommand, DeleteTeamSubcommand,
    ShuffleTeamSubcommand,
};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    AddTeam {
        name: String,
    },
    DeleteTeam {
        name: String,
    },
    ShuffleTeam {
        name: String,
    },
    AddMember {
        #[clap(short, long)]
        team_name: String,

        #[clap(short, long)]
        member_name: String,
    },
    DeleteMember {
        #[clap(short, long)]
        team_name: String,

        #[clap(short, long)]
        member_name: String,
    },
}

pub struct App {
    pub work_dir_path: String,
}

impl App {
    pub fn new() -> Result<Self, String> {
        let home_dir = match home::home_dir() {
            Some(path) => path.to_str().unwrap_or("").to_string(),
            None => return Err("Invalid home dir!".to_string()),
        };

        if home_dir == "" {
            return Err("Invalid home dir!".to_string());
        }

        let work_dir_path = format!("{}/.team-shuffler", home_dir);

        let dir_creation_result = fs::create_dir_all(&work_dir_path);

        match dir_creation_result {
            Ok(_) => Ok(App { work_dir_path }),
            Err(_) => Err("Dir creation failed!".to_string()),
        }
    }

    pub fn process_args(&self) -> Result<(), String> {
        let cli = Cli::parse();

        match &cli.command {
            Commands::AddTeam { name } => {
                AddTeamSubcommand::new(&self.work_dir_path, name).execute()
            }

            Commands::DeleteTeam { name } => {
                DeleteTeamSubcommand::new(&self.work_dir_path, name).execute()
            }

            Commands::ShuffleTeam { name } => {
                ShuffleTeamSubcommand::new(&self.work_dir_path, name).execute()
            }

            Commands::AddMember {
                team_name,
                member_name,
            } => AddMemberSubcommand::new(&self.work_dir_path, team_name, member_name).execute(),

            Commands::DeleteMember {
                team_name,
                member_name,
            } => DeleteMemberSubcommand::new(&self.work_dir_path, team_name, member_name).execute(),
        }
    }
}
