use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Class {
    name: String,
    pub power: usize,
    pub dexterity: usize,
    pub agility: usize,
    pub resistence: usize,
    pub health_points: usize,
    pub energy_points: usize,
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
