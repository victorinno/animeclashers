use crate::{class::class::Class};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use uuid::Uuid;
use wasm_bindgen::prelude::*;
use web_sys::Storage;
use web_sys::Window;

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone, Eq)]
pub struct Clasher {
    id: String,
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

impl Ord for Clasher {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl PartialOrd for Clasher {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Clasher {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
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
            id: Uuid::new_v4().to_hyphenated().to_string(),
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

    pub fn create_clasher(name: &str, clas: Class) -> Clasher {
        if name.is_empty() {
            panic!("Name must be filled.");
        }

        let new_clasher = Clasher {
            id: Uuid::new_v4().to_hyphenated().to_string(),
            name: name.to_string(),
            power: clas.power,
            dexterity: clas.dexterity,
            agility: clas.agility,
            resistence: clas.resistence,
            max_health_points: clas.health_points * 10,
            health_points: clas.health_points * 10,
            max_energy_points: clas.energy_points * 10,
            energy_points: clas.energy_points * 10,
            clazz: clas,
        };
        new_clasher
    }

    pub fn convert(clasher: &Clasher) -> String{serde_json::to_string(clasher).expect("Clasher should be parsable to json")}

    pub fn generate_random_enemy() -> Clasher {
        unimplemented!();
    }

    #[wasm_bindgen(getter)]
    pub fn id(&self) -> String {
        self.id.clone()
    }
    #[wasm_bindgen(setter)]
    pub fn set_id(&mut self, id: String) {
        self.id = id;
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
