use crate::data_processor::Game;
use crate::data_processor::RawData;
use crate::interface::Interface;
use crate::interface::MatchData;
use crate::interface::Timeline;

use serde::Deserialize;
use serde_json::Deserializer;
use std::error::Error;

impl Interface {
    pub fn get_game_ids(
        &self,
        start_timestamp: &String,
        puuid: &String,
    ) -> Result<Vec<String>, Box<dyn Error>> {
        //todo!();
        let mut ids = Vec::new();
        let url_server = Self::get_server(&self.server);
        println!(
            "https://{}.api.riotgames.com/lol/match/v5/matches/by-puuid/{}/ids?start={}&api_key={}",
            url_server, puuid, start_timestamp, self.api_key
        );
        let resp = reqwest::blocking::get(
            format!(
                "https://{}.api.riotgames.com/lol/match/v5/matches/by-puuid/{}/ids?startTime={}&api_key={}",
                url_server, puuid, start_timestamp, self.api_key)
            ).unwrap().text().unwrap();
        println!("{}", resp);
        let mut deserializer = Deserializer::from_str(&resp);
        Deserialize::deserialize_in_place(&mut deserializer, &mut ids).unwrap();
        Ok(ids)
    }

    pub fn get_match_data_collection(
        &self,
        ids: Vec<String>,
        puuid: &String,
    ) -> Result<Vec<RawData>, Box<dyn Error>> {
        let mut out: Vec<RawData> = Vec::new();
        let check_valid_game = |game: &MatchData| -> bool {
            if game.info.end_of_game_result != "GameComplete"
                || game.info.game_mode != "CLASSIC"
                || game.info.game_type != "MATCHED_GAME"
            {
                return false;
            }
            true
        };

        for id in &ids {
            let game_data = self.request_game_data(id).unwrap();
            if check_valid_game(&game_data) {
                let game_tl = self.request_match_timeline(id).unwrap();
                let mut raw = RawData::new(&game_data, &game_tl);
                raw.find_me(puuid);
                out.push(raw);
            }
        }
        Ok(out)
    }

    fn request_game_data(&self, id: &String) -> Result<MatchData, Box<dyn Error>> {
        let resp = reqwest::blocking::get(format!(
            "https://{}.api.riotgames.com/lol/match/v5/matches/{}?api_key={}",
            Self::get_server(&self.server),
            id,
            self.api_key
        ))
        .unwrap()
        .text()
        .unwrap();
        let game: MatchData = serde_json::from_str(&resp)?;
        Ok(game)
    }

    fn request_match_timeline(&self, id: &String) -> Result<Timeline, Box<dyn Error>> {
        let resp = reqwest::blocking::get(format!(
            "https://{}.api.riotgames.com/lol/match/v5/matches/{}/timeline?api_key={}",
            Self::get_server(&self.server),
            id,
            self.api_key
        ))
        .unwrap()
        .text()
        .unwrap();
        let out: Timeline = serde_json::from_str(&resp)?;
        Ok(out)
    }
}
