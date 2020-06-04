use crate::class::class::Class;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Clasher {
    name: String,
    pub power: usize,
    pub dexterity: usize,
    pub agility: usize,
    pub resistence: usize,
    pub max_health_points: usize,
    pub health_points: usize,
    pub max_energy_points: usize,
    pub energy_points: usize,
    clazz: Class,
}

#[wasm_bindgen]
impl Clasher {
    pub fn new(
        name: String,
        power: usize,
        dexterity: usize,
        agility: usize,
        resistence: usize,
        max_health_points: usize,
        health_points: usize,
        max_energy_points: usize,
        energy_points: usize,
        clazz: Class,
    ) -> Self {
        Self {
            name,
            power,
            dexterity,
            agility,
            resistence,
            max_health_points,
            health_points,
            max_energy_points,
            energy_points,
            clazz,
        }
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
    pub fn clazz(&self) -> Class {
        self.clazz.clone()
    }
    #[wasm_bindgen(setter)]
    pub fn set_clazz(&mut self, clazz: Class) {
        self.clazz = clazz;
    }
}
