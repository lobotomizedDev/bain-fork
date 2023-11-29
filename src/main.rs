mod color_schemes;

use color_schemes::color_schemes;
use home;
use image;
use lazy_static::lazy_static;
use os_release::OsRelease;
use std::{
    env, fs,
    path::{Path, PathBuf},
    process::{self, Command},
    thread,
    time::Duration,
};
use wallpaper;

#[derive(PartialEq, Eq)]
enum BatteryStatus {
    Charging,
    NotCharging,
}

struct Battery {
    status: BatteryStatus,
    capacity: u8,
}

struct Previous {
    status: BatteryStatus,
    capacity: u8,
}

pub struct Colors {
    charging: String,
    default: String,
    low_battery: String,
}

lazy_static! {
    static ref PATH: PathBuf = PathBuf::from(home::home_dir().unwrap().join(".rain"));
}

fn main() {
    match wallpaper::set_from_path(&format!("{}/background.png", PATH.display())) {
        Ok(wallpaper) => wallpaper,
        _ => {}
    };

    let args: Vec<String> = env::args().collect();
    let name = match args.get(1) {
        Some(arg) => arg.to_string(),
        _ => match OsRelease::new() {
            Ok(os_release) => os_release.id,
            _ => "rust".to_string(),
        },
    };
    let img_path = {
        let home_dir = home::home_dir().unwrap();
        home_dir.join(format!(".rain/images/{}.png", name))
    };

    let battery_path = match find_battery_path() {
        Some(path) => path,
        None => {
            eprintln!("Could not find battery");
            return;
        }
    };

    let mut previous = Previous {
        capacity: 0,
        status: BatteryStatus::NotCharging,
    };

    loop {
        let battery = create_battery_struct(&battery_path);
        if battery.capacity != previous.capacity || battery.status != previous.status {
            create_and_set(&img_path, battery.capacity, &battery.status, &name);
            previous.capacity = battery.capacity;
            previous.status = battery.status;
        }
        thread::sleep(Duration::from_secs(5));
    }
}

fn create_and_set(img_path: &PathBuf, capacity: u8, status: &BatteryStatus, name: &String) {
    let image = match image::open(img_path) {
        Ok(image) => image,
        Err(_) => {
            eprintln!("Image {}.png not found", name);
            process::exit(1);
        }
    };
    let image_size = (image.width(), image.height());
    let color_scheme = color_schemes(name);
    let color = match status {
        BatteryStatus::Charging => color_scheme.charging,
        _ if capacity >= 30_u8 => color_scheme.default,
        _ => color_scheme.low_battery,
    };

    fs::create_dir_all(&*PATH).unwrap();
    let background = format!("{}/background.png", PATH.display());

    // I hate this with my whole heart but I'm too lazy to play with image crate
    Command::new("convert")
        .arg(img_path)
        .arg("(")
        .arg("+clone")
        .arg("-gravity")
        .arg("South")
        .arg("-crop")
        .arg(format!("x{}%", capacity))
        .arg("-fuzz")
        .arg("50%")
        .arg("-fill")
        .arg(color)
        .arg("-opaque")
        .arg("#8FBCBB")
        .arg("-background")
        .arg("transparent")
        .arg("-extent")
        .arg(format!("{}x{}", image_size.0, image_size.1))
        .arg(")")
        .arg("-gravity")
        .arg("Center")
        .arg("-composite")
        .arg("-background")
        .arg("#282828")
        .arg("-extent")
        .arg("3840x2160")
        .arg(&background)
        .status()
        .expect("Failed to run command convert, check if ImageMagick is installed");

    wallpaper::set_from_path(&background).expect("Error while setting wallpaper");
}

fn find_battery_path() -> Option<PathBuf> {
    fs::read_dir("/sys/class/power_supply")
        .expect("Could not find power_supply dir")
        .map(|entry| {
            let path = entry.unwrap().path();
            let handle = thread::spawn(move || {
                let file_content = fs::read_to_string(path.join("type")).unwrap_or_default();
                if file_content.trim() == "Battery" {
                    Some(path)
                } else {
                    None
                }
            });
            Some(handle)
        })
        .find_map(|handle| handle?.join().unwrap())
}

fn create_battery_struct(battery_path: &Path) -> Battery {
    let capacity = fs::read_to_string(battery_path.join("capacity"))
        .unwrap()
        .trim()
        .to_string()
        .parse::<u8>()
        .unwrap_or(0);
    let status = match fs::read_to_string(battery_path.join("status"))
        .unwrap()
        .trim()
    {
        "Charging" => BatteryStatus::Charging,
        _ => BatteryStatus::NotCharging,
    };

    Battery { status, capacity }
}
