use crate::api_error::ApiError;
use crate::interface::Interface;
use crate::player::{PlayerIdent, Summoner};

use serde::Deserialize;

impl Interface {
    pub fn request_player_data(
        &self,
        game_name: String,
        tagline: String,
    ) -> Result<PlayerIdent, ApiError> {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Account {
            pub puuid: String,
            pub game_name: String,
            pub tag_line: String,
        }
        let account = reqwest::blocking::get(format!(
            "https://{}.api.riotgames.com/riot/account/v1/accounts/by-riot-id/{}/{}?api_key={}",
            Self::get_server(&self.server),
            game_name,
            tagline,
            self.api_key
        ))?
        .json::<Account>()?;

        let summ = reqwest::blocking::get(format!(
            "https://{}.api.riotgames.com/lol/summoner/v4/summoners/by-puuid/{}?api_key={}",
            Self::get_legacy_server(self.server.as_str()),
            account.puuid,
            self.api_key
        ))? //figure out the server for the url. Maybe local match?
        .json::<Summoner>()?;

        Ok(PlayerIdent {
            summoner: summ,
            game_name: account.game_name,
            tagline: account.tag_line,
            server: self.server.clone(),
        })
    }

    pub fn get_legacy_server(server: &str) -> &str {
        match server {
            "NA" => "na1",
            "BR" => "br1",
            "EUW" => "euw1",
            "EUNE" => "eun1",
            "OCE" => "oc1",
            &_ => "na1",
        }
    }
}
