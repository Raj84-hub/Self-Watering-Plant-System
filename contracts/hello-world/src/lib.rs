#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, Env, Symbol, symbol_short, log};

const PLANT_STATUS: Symbol = symbol_short!("PLANT");

// A struct to hold the plant's watering information
#[contracttype]
#[derive(Clone)]
pub struct PlantData {
    pub last_watered: u64,
    pub moisture_level: u32,
    pub auto_watering_enabled: bool,
}

#[contract]
pub struct SelfWateringPlant;

#[contractimpl]
impl SelfWateringPlant {
    // Function to initialize or update plant data
    pub fn update_moisture(env: Env, moisture_level: u32) {
        let mut plant = Self::view_plant(env.clone());

        plant.moisture_level = moisture_level;
        if plant.auto_watering_enabled && moisture_level < 30 {
            let current_time = env.ledger().timestamp();
            plant.last_watered = current_time;
            log!(&env, "Plant watered at timestamp: {}", current_time);
        }

        env.storage().instance().set(&PLANT_STATUS, &plant);
    }

    // Function to toggle auto-watering
    pub fn toggle_auto_watering(env: Env, enabled: bool) {
        let mut plant = Self::view_plant(env.clone());
        plant.auto_watering_enabled = enabled;
        env.storage().instance().set(&PLANT_STATUS, &plant);
    }

    // View current plant data
    pub fn view_plant(env: Env) -> PlantData {
        env.storage().instance().get(&PLANT_STATUS).unwrap_or(PlantData {
            last_watered: 0,
            moisture_level: 100,
            auto_watering_enabled: false,
        })
    }
}
