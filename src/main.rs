use std::{fs, path::PathBuf};

fn main() {
    let battery_path = match find_battery_path() {
        Some(path) => path,
        None => {
            eprintln!("Could not find battery");
            return;
        }
    };
    println!("{:?}", battery_path);
}

fn find_battery_path() -> Option<PathBuf> {
    let mut power_dir_entries = fs::read_dir("/sys/class/power_supply").unwrap();
    power_dir_entries.find_map(|entry| {
        let mut path = entry.unwrap().path();
        path.push("type");
        let file_content = fs::read_to_string(&path).unwrap_or_default();
        if file_content.trim() == "Battery" {
            path.pop();
            Some(path)
        } else {
            None
        }
    })
}
