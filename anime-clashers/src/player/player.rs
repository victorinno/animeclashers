use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::Storage;
use web_sys::Window;
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
        Player::save(p)
    }

    pub fn from_name(name: String) -> Player {
        Player { name: name.clone() }
    }

    pub fn from(json: String) -> Player {
        let p: Player = serde_json::from_str(&json)
            .expect(format!("String is not a valied Player json. {}", json.as_str()).as_str());
        return p;
    }

    pub fn save(player: Player) -> Player {
        let window: Window = web_sys::window().expect("no global `window` exists");
        let storage: Storage = window
            .local_storage()
            .expect("no global `local sotrage` exists")
            .expect("no global `local sotrage` exists");
        let json = serde_json::to_string(&player).expect("Player must be valid");
        storage.set_item("player", &json).expect("Player must be saved");
        Player::load().expect("Player must have been saved.")
    }


    pub fn get_player() -> Player {
        Player::load().expect("That must be a Player.")
    }

    fn load() -> Option<Player> {
        let window: Window = web_sys::window().expect("no global `window` exists");
        let storage: Storage = window
            .local_storage()
            .expect("no global `local sotrage` exists")
            .expect("no global `local sotrage` exists");
        let json: Option<String> = match storage.get_item("player") {
            Ok(j) => j,
            _ => None,
        };

        match json {
            Some(j) => Option::from(Player::from(j)),
            _ => None,
        }
    }

    pub fn has_player() -> bool {
        Player::load().is_some()
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
