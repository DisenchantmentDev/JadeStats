use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::interface::match_data::MatchData;
use crate::interface::timeline::Timeline;

pub mod filter;

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GraphData {
    puuid: String,
    pub game_start: i64,
    pub game_end: i64,
    pub position: Position,
    pub champion: String,
    pub gd15: i32,
    pub csm: f32,
    pub dpm: f32,
    pub kp: f32,
    pub wl: bool,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct RawData {
    pub pids: [(String, String); 5],
    pub me: Me,
    pub g15: [(i32, i32); 5], // (blue, red) g@15
    pub csm: [(f32, f32); 5],
    pub dpm: [(f32, f32); 5],
    pub kp: [(f32, f32); 5],
    pub champs: [(String, String); 5],
    pub win_loss: (bool, bool),
    pub game_end: i64,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Me {
    pub side: Side,
    pub champ: String,
    pub pos: Position,
}
#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Side {
    #[default]
    BLUE,
    RED,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Position {
    #[default]
    TOP,
    JUNGLE,
    MIDDLE,
    BOTTOM,
    SUPPORT,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    pub graph_data: GraphData,
    pub raw_data: RawData,
}

impl Game {
    pub fn new(graph: GraphData, raw: RawData) -> Game {
        Game {
            graph_data: graph,
            raw_data: raw,
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Games {
    pub games: Vec<Game>,
}

impl Games {
    pub fn new(raw_data: Vec<RawData>) -> Games {
        //for each item in raw_data, generate a relevant graph data vec, then insert into game vec
        let mut games = Vec::<Game>::new();
        for data in raw_data {
            let game_graph_data = Games::pull_graph_data(&data);
            games.push(Game::new(game_graph_data, data));
        }
        Games { games: games }
    }

    pub fn is_empty(self) -> bool {
        self.games.is_empty()
    }

    pub fn append_games(&mut self, raw_data: Vec<RawData>) {
        for data in raw_data {
            self.games
                .push(Game::new(Games::pull_graph_data(&data), data));
        }
    }

    pub fn last_game_end(&self) -> i64 {
        self.games.last().unwrap().graph_data.game_end
    }

    pub fn length(&self) -> usize {
        self.games.len()
    }

    pub fn trim_to_length(&mut self, length: usize) {
        self.games
            .drain(..(self.games.len() - (self.games.len() - length) - 1));
    }

    pub fn sort_games(&mut self) {
        self.games
            .sort_by(|v1, v2| v1.graph_data.game_start.cmp(&v2.graph_data.game_start));
    }

    pub fn pull_graph_data(data: &RawData) -> GraphData {
        let side = data.me.side.clone();
        let p_index = Self::get_index_from_pos(&data.me.pos);
        let id = match side {
            Side::BLUE => &data.pids[p_index].0,
            Side::RED => &data.pids[p_index].1,
        };

        GraphData {
            puuid: id.to_string(),
            game_start: data.game_end.clone(),
            game_end: data.game_end.clone(),
            position: data.me.pos.clone(),
            champion: data.me.champ.clone(),
            gd15: Self::calc_gd(&data, &side, &p_index),
            csm: Self::find_csm(&data, &side, &p_index),
            dpm: Self::find_dpm(&data, &side, &p_index),
            kp: Self::find_kp(&data, &side, &p_index),
            wl: Self::find_wl(&data, &side, &p_index),
        }
        //todo!();
    }

    fn calc_gd(game: &RawData, side: &Side, p_index: &usize) -> i32 {
        let gd = match side {
            Side::BLUE => game.g15[*p_index].0 - game.g15[*p_index].1,
            Side::RED => game.g15[*p_index].1 - game.g15[*p_index].0,
        };
        return gd;
    }

    fn find_csm(game: &RawData, side: &Side, p_index: &usize) -> f32 {
        let csm = match side {
            Side::BLUE => game.csm[*p_index].0,
            Side::RED => game.csm[*p_index].1,
        };

        return csm;
    }

    fn find_dpm(game: &RawData, side: &Side, p_index: &usize) -> f32 {
        let dpm = match side {
            Side::BLUE => game.dpm[*p_index].0,
            Side::RED => game.dpm[*p_index].1,
        };

        return dpm;
    }

    fn find_kp(game: &RawData, side: &Side, p_index: &usize) -> f32 {
        let kp = match side {
            Side::BLUE => game.kp[*p_index].0,
            Side::RED => game.kp[*p_index].1,
        };

        return kp;
    }

    fn find_wl(game: &RawData, side: &Side, p_index: &usize) -> bool {
        let wl = match side {
            Side::BLUE => game.win_loss.0,
            Side::RED => game.win_loss.1,
        };

        return wl;
    }

    fn get_index_from_pos(pos: &Position) -> usize {
        return match pos {
            Position::TOP => 0,
            Position::JUNGLE => 1,
            Position::MIDDLE => 2,
            Position::BOTTOM => 3,
            Position::SUPPORT => 4,
        };
    }
}

