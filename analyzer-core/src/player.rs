use crate::data_processor::Games;
use crate::interface::Interface;
use crate::{StartData, player};
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Player {
    pub ident: PlayerIdent,
    pub start_data: StartData,
    pub games: Games,
    pub interface: Interface,
    pub max_games: usize,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq)]
pub struct PlayerIdent {
    pub summoner: Summoner,
    pub game_name: String,
    pub tagline: String,
    pub server: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Summoner {
    pub puuid: String,
    pub profile_icon_id: i64,
    pub revision_date: i64,
    pub summoner_level: i64,
}

impl Player {
    pub fn new(raw_username: &str, api_key: String) -> Player {
        let mut inter = Interface::new(&api_key);
        println!("Creating new player {}", raw_username);
        let start_of_day = Utc::now()
            .date_naive()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_local_timezone(Utc)
            .unwrap()
            .timestamp();
        let ident = inter.gen_player_ident_from_string(raw_username);
        Player {
            ident: ident.clone(),
            start_data: StartData {
                api_key: api_key,
                puuid: ident.summoner.puuid.clone(),
                start_date: start_of_day,
                region: ident.server.clone(),
            },
            games: Games::default(),
            interface: inter,
            max_games: 30,
        }
    }

    pub fn load_new_player(&mut self) {
        let mut time = self.start_data.start_date.clone();
        let mut game_ids = self
            .interface
            .get_game_ids(&time.to_string(), &self.start_data.puuid)
            .unwrap();
        //TODO: check if we have fewer than 15 games, then check again with backed up timestamp
        let mut count = 0;
        while game_ids.len() < 15 {
            time -= 86400;
            if count > 30 {
                break;
            }
            count += 1;
            game_ids = self
                .interface
                .get_game_ids(&time.to_string(), &self.start_data.puuid)
                .unwrap();
        }
        self.games = Games::new(
            self.interface
                .get_match_data_collection(game_ids, &self.start_data.puuid)
                .unwrap(),
        );
    }

    pub fn load_indexed_player(api_key: String, player_as_string: String) -> Player {
        let save: Player = serde_json::from_str(&player_as_string).unwrap();
        let mut inter = Interface::new(&api_key);
        Player {
            ident: save.ident,
            start_data: save.start_data,
            games: save.games,
            interface: inter,
            max_games: 30,
        }
    }

    pub fn load_new_games(&mut self) -> bool {
        let new_games = self
            .interface
            .get_game_ids(
                &self.games.last_game_end().to_string(),
                &self.start_data.puuid,
            )
            .unwrap();
        if !new_games.is_empty() {
            self.games.append_games(
                self.interface
                    .get_match_data_collection(new_games, &self.start_data.puuid)
                    .unwrap(),
            );
            self.trim_games();
            self.sort_games();
            return true;
        }
        return false;
    }

    fn trim_games(&mut self) {
        if self.games.length() > self.max_games {
            self.games.trim_to_length(self.max_games);
        }
    }

    pub fn sort_games(&mut self) {
        self.games.sort_games();
    }

    pub fn set_api(&mut self, api_key: String) {
        self.start_data.api_key = api_key;
    }

    pub fn get_player(&self) -> Player {
        self.clone()
    }

    pub fn is_empty_games(&self) -> bool {
        self.clone().games.is_empty()
    }
}
