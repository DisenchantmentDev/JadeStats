use serde_derive::{Deserialize, Serialize};
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
    pub purchase_history: [(ItemHistory, ItemHistory); 5],
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

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct ItemHistory {
    pub events: Vec<ItemEvent>,
}

impl ItemHistory {
    pub fn sort(&mut self) {
        self.events.sort_by_key(|a| a.get_timestamp());
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub enum ItemEvent {
    PURCHASE {
        item_id: i64,
        timestamp: i64,
        pid: i64,
    },
    SELL {
        item_id: i64,
        timestamp: i64,
        pid: i64,
    },
    DESTROY {
        item_id: i64,
        timestamp: i64,
        pid: i64,
    },
    UNDO {
        after_id: i64,
        before_id: i64,
        gold_gain: i64,
        timestamp: i64,
        pid: i64,
    },
    #[default]
    UNKNOWN,
}

impl ItemEvent {
    pub fn get_timestamp(&self) -> i64 {
        match &self {
            ItemEvent::PURCHASE {
                item_id: _item_id,
                timestamp,
                pid: _pid,
            }
            | ItemEvent::SELL {
                item_id: _item_id,
                timestamp,
                pid: _pid,
            }
            | ItemEvent::DESTROY {
                item_id: _item_id,
                timestamp,
                pid: _pid,
            } => *timestamp,
            ItemEvent::UNDO {
                after_id: _after_id,
                before_id: _before_id,
                gold_gain: _gold_gain,
                timestamp,
                pid: _pid,
            } => *timestamp,
            ItemEvent::UNKNOWN => 0,
        }
    }
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
            wl: Self::find_wl(data, &side),
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

    fn inventory_item_value(history: &ItemHistory) -> i64 {
        let items_list = Items::new();
        let mut player_inv: Vec<Item> = Vec::new();
        let mut out: i64 = 0;
        let mut sorted_history: ItemHistory = history.clone();
        sorted_history.sort();

        for p in &sorted_history.events {
            if !Self::check_valid_event(p, &items_list) {
                continue;
            }

            println!("Action type: {:?}", p);

            match p {
                ItemEvent::PURCHASE {
                    item_id,
                    timestamp: _timestamp,
                    pid: _pid,
                } => player_inv.push(items_list.get_unchecked(item_id.to_string()).clone()),
                ItemEvent::SELL {
                    item_id,
                    timestamp: _timestamp,
                    pid: _pid,
                } => {
                    if let Some(index) = player_inv
                        .iter()
                        .position(|x| x.eq(items_list.get_unchecked(item_id.to_string())))
                    {
                        player_inv.remove(index);
                    }
                }
                ItemEvent::DESTROY {
                    item_id,
                    timestamp: _timestamp,
                    pid: _pid,
                } => {
                    if items_list.get_unchecked(item_id.to_string()).is_component()
                        && let Some(index) = player_inv
                            .iter()
                            .position(|x| x.eq(items_list.get_unchecked(item_id.to_string())))
                    {
                        player_inv.remove(index);
                    }
                }
                ItemEvent::UNDO {
                    after_id,
                    before_id,
                    gold_gain,
                    timestamp: _timestamp,
                    pid: _pid,
                } => {
                    // if before was something and after is 0, we undid a buy
                    // iff after is something and before was 0, we undid a sell
                    if *after_id == 0 {
                        if let Some(index) = player_inv
                            .iter()
                            .position(|x| x.eq(items_list.get_unchecked(before_id.to_string())))
                        {
                            player_inv.remove(index);
                        }
                    } else if *before_id == 0 {
                        //sell
                        player_inv.push(items_list.get_unchecked(after_id.to_string()).clone());
                    }
                    out += gold_gain; //gold offset for unaccounted for or lost components
                }
                ItemEvent::UNKNOWN => {}
            };
            //print!("Player inventory after event: ");
            //for i in player_inv.clone() {
            //    print!("{}", i.get_item_name());
            //}
            //println!();
        }
        let mut debug_string = Vec::<String>::new();

        for i in player_inv {
            debug_string.push(i.get_item_name());
            out += i.get_item_value();
        }

        println!(
            "Player inventory: {:?}\nTotal gold value of player items: {}",
            debug_string, out
        );

        out
    }

    /* Checks if the event in the list is within the first 15 minutes*/
    fn check_valid_event(event: &ItemEvent, items: &Items) -> bool {
        match event {
            ItemEvent::PURCHASE {
                item_id,
                timestamp,
                pid: _pid,
            }
            | ItemEvent::SELL {
                item_id,
                timestamp,
                pid: _pid,
            }
            | ItemEvent::DESTROY {
                item_id,
                timestamp,
                pid: _pid,
            } => {
                if items.get(item_id.to_string()).is_some() {
                    return timestamp <= &(900000);
                }
                false
            }
            ItemEvent::UNDO {
                after_id,
                before_id,
                gold_gain: _gold_gain,
                timestamp,
                pid: _pid,
            } => {
                if *after_id == 0 && items.get(before_id.to_string()).is_some()
                    || *before_id == 0 && items.get(after_id.to_string()).is_some()
                {
                    return timestamp <= &(900000);
                }
                false
            }
            ItemEvent::UNKNOWN => false,
        }
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

    fn find_wl(game: &RawData, side: &Side) -> bool {
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
