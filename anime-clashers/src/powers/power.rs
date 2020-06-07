use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use wasm_bindgen::prelude::*;
use web_sys::Storage;
use web_sys::Window;

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone, Eq)]
pub enum PowerType {
    Normal,
    Precise,
    Brute,
}

impl PartialEq for PowerType {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone, Eq)]
pub enum PowerAction {
    Defense,
    Attack,
}

impl PartialEq for PowerAction {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone, Eq)]
pub struct Power {
    name: String,
    pub level: usize,
    pub potency: usize,
    pub precision: usize,
    power_type: PowerType,
    power_action: PowerAction,
}

impl Ord for Power {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl PartialOrd for Power {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Power {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

#[wasm_bindgen]
impl Power {
    pub fn load_db(clasher_id: &str) -> String {
        serde_json::to_string(&Power::load(clasher_id).expect("There must be a Power db"))
            .expect("DB must be parsed.")
    }

    pub fn list_types_power() -> String {
        serde_json::to_string(&Vec::from([
            PowerType::Normal,
            PowerType::Precise,
            PowerType::Brute,
        ]))
        .expect("The list of type action is static")
    }

    pub fn list_types_action() -> String {
        serde_json::to_string(&Vec::from([PowerAction::Attack, PowerAction::Defense]))
            .expect("The list of type power is static")
    }

    fn define_potency(power_type: &PowerType) -> usize {
        match power_type {
            PowerType::Normal => 2,
            PowerType::Precise => 1,
            PowerType::Brute => 3,
        }
    }

    fn define_precision(power_type: &PowerType) -> usize {
        match power_type {
            PowerType::Normal => 2,
            PowerType::Precise => 3,
            PowerType::Brute => 1,
        }
    }

    pub fn create(
        name: String,
        level: usize,
        power_type: PowerType,
        power_action: PowerAction,
        clasher_id: String,
    ) -> Power {
        let power: Power = Power {
            name,
            level,
            potency: Power::define_potency(&power_type),
            precision: Power::define_precision(&power_type),
            power_type,
            power_action,
        };

        let mut db = Power::load(&clasher_id).expect("There must be a power db.");
        db.sort();
        let index = match db.binary_search(&power) {
            Ok(i) => i,
            Err(i) => i,
        };
        db.insert(index, power.clone());
        Power::save_db(db, &clasher_id);
        power.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }
    #[wasm_bindgen(setter)]
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    #[wasm_bindgen(getter)]
    pub fn power_type(&self) -> PowerType {
        self.power_type.clone()
    }
    #[wasm_bindgen(setter)]
    pub fn set_power_type(&mut self, power_type: PowerType) {
        self.power_type = power_type;
    }

    #[wasm_bindgen(getter)]
    pub fn power_action(&self) -> PowerAction {
        self.power_action.clone()
    }
    #[wasm_bindgen(setter)]
    pub fn set_power_action(&mut self, power_action: PowerAction) {
        self.power_action = power_action;
    }
}

impl Power {
    fn load(clasher_id: &str) -> Option<Vec<Power>> {
        let window: Window = web_sys::window().expect("no global `window` exists");
        let storage: Storage = window
            .local_storage()
            .expect("no global `local sotrage` exists")
            .expect("no global `local sotrage` exists");
        let json: Option<String> =
            match storage.get_item(&("powers_".to_owned() + &clasher_id.to_owned())) {
                Ok(j) => j,

                Err(_) => None,
            };

        match json {
            Some(j) => Option::from(Power::parse_db(j)),
            _ => Some(Vec::new()),
        }
    }

    pub fn parse_db(json: String) -> Vec<Power> {
        let p: Vec<Power> = serde_json::from_str(&json)
            .expect(format!("String is not a valied Power json. {}", json.as_str()).as_str());
        return p;
    }

    #[warn(unused_must_use)]
    fn save_db(d: Vec<Power>, clasher_id: &str) {
        let window: Window = web_sys::window().expect("no global `window` exists");
        let storage: Storage = window
            .local_storage()
            .expect("no global `local sotrage` exists")
            .expect("no global `local sotrage` exists");
        let json = serde_json::to_string(&d).expect("Power db must exists");
        storage
            .set_item(
                &("powers_".to_owned() + &clasher_id.to_owned()),
                json.as_str(),
            )
            .expect("the Power failed to persist.");
    }
}
