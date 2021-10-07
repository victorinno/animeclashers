use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::Storage;
use web_sys::Window;
use serde_json::to_string;

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Player {
    name: String,
}

#[wasm_bindgen]
impl Player {
    pub fn new(name: &str) -> Player {
        if name.is_empty() {
            panic!("The player name must be provided.")
        }
        let p = Player {
            name: String::from(name),
        };
        p
    }

    pub fn convert(player: &Player) -> String {
        to_string(player).expect("Player")
    }

    pub fn from_name(name: String) -> Player {
        Player { name: name.clone() }
    }

    pub fn from(json: String) -> Player {
        let p: Player = serde_json::from_str(&json)
            .expect(format!("String is not a valied Player json. {}", json.as_str()).as_str());
        return p;
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }
    #[wasm_bindgen(setter)]
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}
