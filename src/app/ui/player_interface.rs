use analyzer_core::player::Player;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::path::Path;

use crate::ui::App;

impl App {
    //TODO: write function for serializing environment variables for API key
    //TODO: read indexed players and load if that exists, else load new player
    fn get_api_key(&self) -> String {
        dotenv().ok();
        std::env::var("API_TOKEN").expect("API_TOKEN not set in .env")
    }

    pub fn load_player(&mut self) {
        let api_key = self.get_api_key();
        let raw_username = format!("{}#{:?}", self.username, self.region);
        self.loaded_player = Player::new(raw_username.as_str(), api_key);
        let index_file = Self::read_indexed_players();
        let mut player_as_str = String::new();

        if Self::indexed_players_contains(&raw_username) {
            player_as_str =
                fs::read_to_string(format!("../../../assets/profiles/{raw_username}.json"))
                    .expect("Error reading file");
            self.loaded_player.load_indexed_player(player_as_str);
            self.loaded_player.load_new_games();
        } else {
            Self::update_index_file(raw_username.clone());
            File::create(format!("../../../assets/profiles/{raw_username}.json"))
                .expect("Unable to create file");
            self.loaded_player.load_new_player();
        }

        let mut player_file = OpenOptions::new()
            .write(true)
            .open(Path::new(
                format!("../../../assets/profiles/{raw_username}.json").as_str(),
            ))
            .expect("Unable to open file");
        player_file
            .write_all(
                serde_json::to_string(&self.loaded_player)
                    .expect("Unable to deserialize player")
                    .as_bytes(),
            )
            .expect("Error writing player to file");
    }

    fn indexed_players_contains(player: &String) -> bool {
        let index = Self::read_indexed_players();
        index.contains(player)
    }

    fn read_indexed_players() -> Vec<String> {
        #[derive(Serialize, Deserialize, Debug)]
        struct Temp {
            profiles: Vec<String>,
        }
        let mut buf = String::new();
        OpenOptions::new()
            .read(true)
            .open("../../../assets/profile_index.json")
            .expect("Error reading profile_index.json")
            .read_to_string(&mut buf)
            .expect("Error reading file to string");
        let indexes: Temp = match serde_json::from_str::<Temp>(&buf) {
            Ok(n) => n,
            Err(_) => Temp {
                profiles: Vec::new(),
            },
        };
        indexes.profiles
    }

    fn update_index_file(player: String) {
        #[derive(Serialize, Deserialize)]
        struct Temp {
            profiles: Vec<String>,
        }

        let mut indexed = Self::read_indexed_players();
        indexed.push(player);
        //let updated = Temp { profiles: indexed };
        let serialized =
            serde_json::to_string(&Temp { profiles: indexed }).expect("Failure to deserialize");
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open("../../../assets/profile_index.json")
            .expect("Failed to open profile indexes");
        file.write_all(serialized.as_bytes())
            .expect("Failed to write file");
    }
}
