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
    pub fn new(api_key: &String) -> Interface {
        Interface {
            api_key: api_key.clone(),
            server: String::default(),
        }
    }

    pub fn gen_player_ident_from_string(
        &mut self,
        raw_username: &str,
    ) -> Result<PlayerIdent, ApiError> {
        let p: Vec<&str> = raw_username.split("#").collect();
        println!("{:?}", p);
        self.server = String::from(p[2]);
        self.request_player_data(String::from(p[0]).replace(" ", "%20"), String::from(p[1]))
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
