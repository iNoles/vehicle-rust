// Define a VehicleManager struct to manage vehicles
pub struct VehicleManager {
    vehicles: Vec<String>,
}

impl VehicleManager {
    pub fn new() -> Self {
        VehicleManager {
            vehicles: Vec::new(),
        }
    }

    // Method to add a vehicle to the manager
    pub fn add_vehicle(&mut self, vehicle_name: &&str) {
        self.vehicles.push(vehicle_name.to_string());
        println!("Added vehicle: {}",  vehicle_name);
    }

    // Method to get the count of vehicles managed
    pub fn get_vehicle_count(&self) -> usize {
        self.vehicles.len()
    }
}