use analyzer_core::player::Player;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Error;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use crate::app::app_error::AppError;
use crate::ui::PlayerLoadCtx;

#[allow(clippy::allow_attributes, clippy::missing_errors_doc)]
impl PlayerLoadCtx {
    pub fn load_player(&mut self) -> Result<Player, AppError> {
        let api_key = Self::get_api_key();
        let raw_username = format!("{}#{:?}", self.username, self.region);
        let profile_path: PathBuf = self
            .root_dir
            .join(format!("assets/profiles/{raw_username}.json"));

        let mut loaded_player = Player::default();
        if self.indexed_players.contains(&raw_username) {
            if let Ok(p_string) = fs::read_to_string(profile_path.clone()) {
                loaded_player.set_api(api_key);
                loaded_player.load_indexed_player(p_string)?;
            } else {
                Self::remove_player_file(&self.root_dir, &raw_username)?;
                return Err(AppError::new("Player file could not be found"));
            }
            //loaded_player.set_api(api_key);
            //let player_as_str = fs::read_to_string(profile_path.clone())?;
            //loaded_player.load_indexed_player(player_as_str)?;
        } else {
            loaded_player = Player::new(raw_username.as_str(), api_key)?;
            Self::update_index_file(&self.root_dir, raw_username.clone())?;
            File::create(profile_path.clone())?;
            loaded_player.load_new_player()?;
        }

        let mut player_file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(profile_path.clone())?;
        player_file.write_all(serde_json::to_string(&loaded_player)?.as_bytes())?;
        Ok(loaded_player)
    }

    fn get_api_key() -> String {
        dotenv().ok();
        std::env::var("API_TOKEN").expect("Could not find environment variables")
    }

    pub fn read_indexed_players(root: &Path) -> Result<Vec<String>, Error> {
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

    pub fn remove_player_file(root: &Path, player: &str) -> Result<(), Error> {
        let indexed_profile_path = root.join("assets/profile_index.json");
        #[derive(Serialize, Deserialize)]
        struct Temp {
            profiles: Vec<String>,
        }

        let mut indexed = Self::read_indexed_players(root)?;
        if let Some(index) = indexed.iter().position(|x| x == &String::from(player)) {
            indexed.remove(index);
        }

        let serialized = serde_json::to_string(&Temp { profiles: indexed })?;
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(indexed_profile_path)?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }
}
