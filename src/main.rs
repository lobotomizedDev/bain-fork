use dirs::home_dir;
use std::{fs, path::PathBuf, thread, time::Duration};

const BATTERY_STATUS_COLOR_CHARGING: &str = "#C0C0C0";
const BATTERY_STATUS_COLOR_GOOD: &str = "#C45508";
const BATTERY_STATUS_COLOR_LOW: &str = "#800020";

fn main() {
    let tmp_directory: PathBuf = PathBuf::from("/tmp/bain");
    fs::create_dir_all(tmp_directory).expect("Failed to create dir");

    match find_battery_path() {
        Some(path) => loop {
            let battery_percentage = &path.join("capacity");
            let battery_percentage = fs::read_to_string(battery_percentage)
                .expect("Failed to read battery percentage")
                .trim()
                .parse::<u32>()
                .expect("Failed to parse battery percentage");
            let battery_status = &path.join("status");
            let original_image = home_dir()
                .expect("Failed to get home directory")
                .join(".bain/images/rust.png");
            create_and_set(
                original_image,
                battery_percentage,
                fs::read_to_string(battery_status).unwrap().trim(),
            );
            thread::sleep(Duration::from_secs(5));
            println!("yo");
        },
        None => {
            eprintln!("Couldn't find battery path")
        }
    }
}

fn find_battery_path() -> Option<PathBuf> {
    let power_dir =
        fs::read_dir("/sys/class/power_supply").expect("Failed to read power supply directory");

    for dir in power_dir {
        let path = dir.expect("Failed to read directory").path().join("type");
        let content = fs::read_to_string(&path).expect("Failed to read file content");
        if content.trim() == "Battery" {
            return Some(
                path.parent()
                    .expect("Failed to get parent directory")
                    .to_path_buf(),
            );
        }
    }

    None
}

fn create_and_set(original_image: PathBuf, battery_percentage: u32, battery_status: &str) {}
