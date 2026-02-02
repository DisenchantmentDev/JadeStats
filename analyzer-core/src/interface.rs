use serde::{Deserialize, Serialize};

use crate::api_error::ApiError;
use crate::interface::match_data::MatchData;
use crate::interface::timeline::Timeline;
use crate::player::PlayerIdent;

pub mod game_interface;
pub mod match_data;
pub mod ranked_data;
pub mod timeline;
pub mod user_interface;

#[derive(Deserialize, Serialize, Clone, Default)]
pub struct Interface {
    api_key: String,
    server: String,
}

impl Interface {
    pub fn new(api_key: &str) -> Self {
        Self {
            api_key: api_key.to_owned(),
            server: String::default(),
        }
    }

    pub fn new_with_server(api_key: &str, s: String) -> Self {
        Self {
            api_key: api_key.to_owned(),
            server: s,
        }
    }

    pub fn gen_player_ident_from_string(
        &mut self,
        raw_username: &str,
    ) -> Result<PlayerIdent, ApiError> {
        let p: Vec<&str> = raw_username.split("#").collect();
        if p.len() > 3 {
            return Err(ApiError::new("Incorrect format for inputted player"));
        }
        println!("{:?}", p);
        match p.get(2) {
            Some(server) => {
                self.server = String::from(*server);
                self.request_player_data(
                    String::from(*p.first().expect("Could not index player")).replace(" ", "%20"),
                    String::from(*p.get(1).expect("Could not index player")),
                )
            }
            None => Err(ApiError::new("Incorrect format for inputted player")),
        }
        //self.server = String::from(p[2]);
        //self.request_player_data(String::from(p[0]).replace(" ", "%20"), String::from(p[1]))
    }

    pub fn get_server(server: &str) -> &str {
        match server {
            "NA" => "americas",
            "BR" => "americas",
            "LAS" => "americas",
            "LAN" => "americas",
            "KR" => "asia",
            "JP" => "asia",
            "EUW" => "europe",
            "EUNE" => "europe",
            "ME" => "europe",
            "TR" => "europe",
            "RU" => "europe",
            "OCE" => "sea",
            "VN" => "sea",
            "TW" => "sea",
            &_ => "americas",
        }
    }
}
