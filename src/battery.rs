use std::{fs, path::Path};

#[derive(PartialEq, Eq)]
pub enum BatteryStatus {
    Charging,
    NotCharging,
}

impl BatteryStatus {
    pub fn new(status: &str) -> BatteryStatus {
        match status {
            "Charging" => Self::Charging,
            _ => Self::NotCharging,
        }
    }
}

pub struct Battery {
    pub status: BatteryStatus,
    pub capacity: u8,
}

impl Battery {
    fn get_status(battery_path: &Path) -> BatteryStatus {
        let status = fs::read_to_string(battery_path.join("status")).unwrap();
        BatteryStatus::new(status.trim())
    }

    fn get_capacity(battery_path: &Path) -> u8 {
        fs::read_to_string(battery_path.join("capacity"))
            .unwrap()
            .trim()
            .parse::<u8>()
            .unwrap_or(0)
    }

    pub fn new(battery_path: &Path) -> Self {
        let status = Self::get_status(battery_path);
        let capacity = Self::get_capacity(battery_path);
        Self { status, capacity }
    }
}
