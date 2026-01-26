use project_root;
use serde_derive::{Deserialize, Serialize};
use serde_json;
use std::fs;

pub mod filter;
pub mod items;

use items::{Item, Items};

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
    pub purchase_history: [(PurchaseHistory, PurchaseHistory); 5],
    pub game_end: i64,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Me {
    pub side: Side,
    pub champ: String,
    pub pos: Position,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct PurchaseHistory {
    pub purchases: Vec<PurchaseEvent>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct PurchaseEvent {
    pub event_type: String,
    pub item_id: Option<i64>,
    pub timestamp: i64,
    pub pid: Option<i64>,
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

#[allow(clippy::allow_attributes, clippy::redundant_field_names)]
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
            game_start: data.game_end,
            game_end: data.game_end,
            position: data.me.pos.clone(),
            champion: data.me.champ.clone(),
            gd15: Self::calc_gd(data, &side, &p_index),
            csm: Self::find_csm(data, &side, &p_index),
            dpm: Self::find_dpm(data, &side, &p_index),
            kp: Self::find_kp(data, &side, &p_index),
            wl: Self::find_wl(data, &side, &p_index),
        }
        //todo!();
    }

    fn calc_gd(game: &RawData, side: &Side, p_index: &usize) -> i32 {
        match side {
            Side::BLUE => {
                (game.g15[*p_index].0
                    + Self::inventory_item_value(&game.purchase_history[*p_index].0) as i32)
                    - (game.g15[*p_index].1
                        + Self::inventory_item_value(&game.purchase_history[*p_index].1) as i32)
            }
            Side::RED => {
                (game.g15[*p_index].1
                    + Self::inventory_item_value(&game.purchase_history[*p_index].0) as i32)
                    - (game.g15[*p_index].0
                        + Self::inventory_item_value(&game.purchase_history[*p_index].1) as i32)
            }
        }
    }

    fn inventory_item_value(history: &PurchaseHistory) -> i64 {
        let items_list = Items::new();
        let mut player_inv: Vec<Item> = Vec::new();
        let mut out: i64 = 0;
        let mut action_stack = Vec::<PurchaseEvent>::new();

        for p in &history.purchases {
            if (p.timestamp / 60000) > 15 {
                continue;
            }
            if let Some(top_iid) = p.item_id {
                let temp = top_iid.to_string();
                let item = match items_list.get(temp) {
                    Some(i) => i.clone(),
                    None => {
                        println!("Item accessed does not exist, skipping item...");
                        continue;
                    }
                };

                let _: () = match p.event_type.as_str() {
                    "ITEM_PURCHASED" => {
                        //println!("Added item '{}' to player inventory", item.get_item_name());
                        player_inv.push(item.clone());
                        action_stack.push(p.clone());
                    }
                    "ITEM_DESTROYED" => {
                        if let Some(iid) = p.item_id
                            && iid.to_string().len() < 5
                            && items_list.get_unchecked(iid.to_string()).transforms()
                        {
                            //println!("Item {} has (probably) upgraded!", iid);
                        } else {
                            action_stack.push(p.clone());
                        }
                    }
                    "ITEM_SOLD" => {
                        if let Some(iid) = p.item_id
                            && let Some(index) = player_inv
                                .iter()
                                .position(|x| x == items_list.get_unchecked(iid.to_string()))
                        {
                            player_inv.remove(index);
                            action_stack.push(p.clone());
                        }
                    }
                    "ITEM_UNDO" => {
                        if let Some(last_action) = action_stack.pop()
                            && let Some(iid) = last_action.item_id
                        {
                            if last_action.event_type == "ITEM_DESTROYED" {
                                println!("Undo Item Purchase");
                                player_inv.push(items_list.get_unchecked(iid.to_string()).clone());
                                //remove item from inv, pop actions until we are not destroying something relevant
                                while let Some(check) = action_stack.pop()
                                    && check.event_type != "ITEM_PURCHASED"
                                    && let Some(check_iid) = check.item_id
                                    && let Some(index) = player_inv.iter().position(|x| {
                                        x == items_list.get_unchecked(check_iid.to_string())
                                    })
                                {
                                    player_inv.remove(index);
                                    println!("added {} as part of undo", iid);
                                }
                            } else if last_action.event_type == "ITEM_SOLD"
                                && let Some(iid) = last_action.item_id
                            {
                                player_inv.push(items_list.get_unchecked(iid.to_string()).clone());
                            }
                        }
                    }
                    &_ => {}
                };
            }
        }

        for i in player_inv {
            out += i.get_item_value();
        }

        out
    }

    fn find_csm(game: &RawData, side: &Side, p_index: &usize) -> f32 {
        match side {
            Side::BLUE => game.csm[*p_index].0,
            Side::RED => game.csm[*p_index].1,
        }
    }

    fn find_dpm(game: &RawData, side: &Side, p_index: &usize) -> f32 {
        match side {
            Side::BLUE => game.dpm[*p_index].0,
            Side::RED => game.dpm[*p_index].1,
        }
    }

    fn find_kp(game: &RawData, side: &Side, p_index: &usize) -> f32 {
        match side {
            Side::BLUE => game.kp[*p_index].0,
            Side::RED => game.kp[*p_index].1,
        }
    }

    fn find_wl(game: &RawData, side: &Side, p_index: &usize) -> bool {
        match side {
            Side::BLUE => game.win_loss.0,
            Side::RED => game.win_loss.1,
        }
    }

    fn get_index_from_pos(pos: &Position) -> usize {
        match pos {
            Position::TOP => 0,
            Position::JUNGLE => 1,
            Position::MIDDLE => 2,
            Position::BOTTOM => 3,
            Position::SUPPORT => 4,
        }
    }
}
