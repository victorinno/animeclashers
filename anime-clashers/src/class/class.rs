use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone, Eq)]
pub struct Class {
    name: String,
    pub power: usize,
    pub dexterity: usize,
    pub agility: usize,
    pub resistence: usize,
    pub health_points: usize,
    pub energy_points: usize,
}

impl Ord for Class {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl PartialOrd for Class {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Class {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

#[wasm_bindgen]
impl Class {
    pub fn new(
        name: String,
        power: usize,
        dexterity: usize,
        agility: usize,
        resistence: usize,
        health_points: usize,
        energy_points: usize,
    ) -> Self {
        Self {
            name,
            power,
            dexterity,
            agility,
            resistence,
            health_points,
            energy_points,
        }
    }

    pub fn get_by_name(name: String) -> Class {
        let mut list: Vec<Class> =
            serde_json::from_str(&Class::list_all_classes()).expect("List of classes are static");
        list.sort();
        match list.binary_search_by(|c| c.name.cmp(&name)) {
            Ok(c) => list.get(c).expect("Already found in binary search").clone(),
            Err(c) => Class {
                name: "null".to_string(),
                power: 0,
                dexterity: 0,
                agility: 0,
                resistence: 0,
                health_points: 0,
                energy_points: 0,
            },
        }
    }

    pub fn list_all_classes() -> String {
        serde_json::to_string(&[Class {
            name: "Estudante".to_string(),
            power: 4,
            dexterity: 4,
            agility: 4,
            resistence: 4,
            health_points: 4,
            energy_points: 4,
        }])
        .expect_throw("The classes are static and must work.")
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
