pub enum VehicleStatus {
    Parked,
    Driving { speed: u32 },
    BrokenDown(String),
}

impl VehicleStatus {
    pub fn is_operational(&self) -> bool {
        match self {
            VehicleStatus::Parked => true,
            VehicleStatus::Driving { .. } => true,
            VehicleStatus::BrokenDown(_) => false,
        }
    }

    pub fn description(&self) -> String {
        match self {
            VehicleStatus::Parked => "The vehicle is parked.".to_string(),
            VehicleStatus::Driving { speed } => {
                format!("The vehicle is driving at {} km/h.", speed)
            }
            VehicleStatus::BrokenDown(reason) => format!("The vehicle is broken down: {}.", reason),
        }
    }
}
