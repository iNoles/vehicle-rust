pub mod vehicle_manager;

use std::sync::{Arc, Mutex};
use crate::vehicle_manager::VehicleManager;

// Define a global static instance of VehicleManager using lazy initialization
lazy_static::lazy_static! {
    pub static ref MANAGER: Arc<Mutex<VehicleManager>> = Arc::new(Mutex::new(VehicleManager::new()));
}

fn main() {
    // Access the singleton instance of VehicleManager
    let mut manager = MANAGER.lock().unwrap();

    // Add vehicles to the manager
    let vehicles = vec!["Car", "Motorcycle", "Boat", "Trucks", "Bike"];
    for vehicle in &vehicles {
        manager.add_vehicle(vehicle);
    }

    // Print the count of vehicles managed
    println!("Total number of vehicles: {}", manager.get_vehicle_count());
}
