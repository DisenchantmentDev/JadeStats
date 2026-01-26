//item id, name, total price, image url, special cases for Archangels/Mejais and whatnot
// Possible hashmap?
// Could develop a way of filtering through the datadragon json file and generating
// the json for this structure specifically

use crate::api_error::ApiError;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Default, Clone)]
pub struct Items {
    items: HashMap<String, Item>,
}

#[derive(Default, PartialEq, Eq, Clone)]
pub struct Item {
    name: String,
    into: Option<Vec<String>>,
    sprite_url: String,
    total_price: i64,
    special_recipe: Option<String>,
    transforms: bool,
}

impl Item {
    pub fn new(data: &Value) -> Self {
        Self {
            name: data
                .get("name")
                .expect("Item does not have a name")
                .to_string(),
            into: data.get("into").and_then(|val| val.as_array()).map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            }),
            sprite_url: data
                .get("image")
                .and_then(|val| val.get("sprite"))
                .expect("Item does not have sprite")
                .to_string(),
            total_price: data
                .get("gold")
                .and_then(|val| val.get("total"))
                .expect("Item does not have price")
                .as_i64()
                .unwrap_or_default(),
            special_recipe: data
                .get("specialRecipe")
                .and_then(|val| val.as_str())
                .map(String::from),
            transforms: data
                .get("description")
                .expect("Could not get item description")
                .to_string()
                .contains("<br>Transforms into "),
        }
    }

    pub fn get_item_value(&self) -> i64 {
        self.total_price
    }

    pub fn get_item_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_into(&self) -> Option<Vec<String>> {
        self.into.clone()
    }

    pub fn get_sr(&self) -> Option<String> {
        self.special_recipe.clone()
    }

    pub fn transforms(&self) -> bool {
        self.transforms
    }
}

impl Items {
    pub fn new() -> Self {
        let temp = Self::filter_sr_items().unwrap_or_default();
        Self { items: temp }
    }

    pub fn get(&self, key: String) -> Option<&Item> {
        self.items.get(&key)
    }

    pub fn get_unchecked(&self, key: String) -> &Item {
        //println!("Getting unchecked id {}", key);
        self.items.get(&key).expect("Failed to get item id, loser")
    }

    fn filter_sr_items() -> Result<HashMap<String, Item>, ApiError> {
        let mut out: HashMap<String, Item> = HashMap::new();
        let path = project_root::get_project_root()?.join("assets/items.json");
        let raw: serde_json::Value = serde_json::from_str(std::fs::read_to_string(path)?.as_str())?;
        let items = raw
            .get("data")
            .and_then(serde_json::Value::as_object)
            .expect("Data must be an object");

        for (id, value) in items {
            if id.len() > 4 {
                continue;
            }
            out.insert(id.clone(), Item::new(value));
        }

        Ok(out)
    }
}
