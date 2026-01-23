use analyzer_core::player::Player;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Error;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use crate::app::app_error::AppError;
use crate::ui::App;

#[allow(clippy::allow_attributes, clippy::missing_errors_doc)]
impl App {
    fn get_api_key() -> String {
        dotenv().ok();
        std::env::var("API_TOKEN").expect("Could not find environment variables")
    }

    pub fn load_player(&mut self) -> Result<(), AppError> {
        let api_key = Self::get_api_key();
        let raw_username = format!("{}#{:?}", self.username, self.region);
        self.loaded_player = Player::new(raw_username.as_str(), api_key)?;

        let root_dir = project_root::get_project_root()?;

        let profile_path: PathBuf = root_dir.join(format!("assets/profiles/{raw_username}.json"));

        if Self::indexed_players_contains(&root_dir, &raw_username)? {
            let player_as_str = fs::read_to_string(profile_path.clone())?;
            self.loaded_player.load_indexed_player(player_as_str)?;
            //self.loaded_player.load_new_games()?;
        } else {
            Self::update_index_file(&root_dir, raw_username.clone())?;
            File::create(profile_path.clone())?;
            self.loaded_player.load_new_player()?;
        }

        let mut player_file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(profile_path.clone())?;
        player_file.write_all(serde_json::to_string(&self.loaded_player)?.as_bytes())?;
        Ok(())
    }

    fn indexed_players_contains(root: &Path, player: &String) -> Result<bool, Error> {
        let index = Self::read_indexed_players(root)?;
        Ok(index.contains(player))
    }

    fn read_indexed_players(root: &Path) -> Result<Vec<String>, Error> {
        let indexed_profile_path = root.join("assets/profile_index.json");
        //println!("Read indexed players path: {:?}", indexed_profile_path);
        #[derive(Serialize, Deserialize, Debug)]
        struct Temp {
            profiles: Vec<String>,
        }
        let mut buf = String::new();
        //println!("Path: {:?}", indexed_profile_path);
        OpenOptions::new()
            .read(true)
            .open(indexed_profile_path)?
            .read_to_string(&mut buf)?;
        let indexes: Temp = match serde_json::from_str::<Temp>(&buf) {
            Ok(n) => n,
            Err(_) => Temp {
                profiles: Vec::new(),
            },
        };
        Ok(indexes.profiles)
    }

    fn update_index_file(root: &Path, player: String) -> Result<(), Error> {
        let indexed_profile_path = root.join("assets/profile_index.json");
        //println!("Update indexed players path: {:?}", indexed_profile_path);
        #[derive(Serialize, Deserialize)]
        struct Temp {
            profiles: Vec<String>,
        }

        let mut indexed = Self::read_indexed_players(root)?;
        indexed.push(player);
        //let updated = Temp { profiles: indexed };
        let serialized = serde_json::to_string(&Temp { profiles: indexed })?;
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(indexed_profile_path)?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }
}
