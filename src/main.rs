mod color_schemes;

use color_schemes::color_schemes;
use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
    thread,
    time::Duration,
};

#[derive(Debug)]
struct Battery {
    status: String,
    capacity: u8,
}

struct Previous {
    status: String,
    capacity: u8,
}

pub struct Colors {
    charging: String,
    default: String,
    low_battery: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let img_path = {
        #[allow(deprecated)]
        let home_dir = env::home_dir().unwrap();
        home_dir.join(format!(".rain/images/{}.png", &args[1]))
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
        status: String::new(),
    };
    loop {
        let battery = create_battery_struct(battery_path.clone());
        if battery.capacity != previous.capacity || battery.status != previous.status {
            create_and_set(&img_path, battery.capacity, &battery.status, &args[1]);
            previous.capacity = battery.capacity.clone();
            previous.status = battery.status.clone();
        }
        thread::sleep(Duration::from_secs(5));
    }
}

fn create_and_set(img_path: &PathBuf, capacity: u8, status: &str, arg: &String) {
    let image = image::open(img_path).expect("Failed to open the original image");
    let image_size = (image.width(), image.height());

    let color_scheme = color_schemes(arg);
    let color = if status == "Charging" {
        color_scheme.charging
    } else if capacity >= 30_u8 {
        color_scheme.default
    } else {
        color_scheme.low_battery
    };

    let tmp = Path::new("/tmp/rain");
    fs::create_dir_all(&tmp).unwrap();
    let background = format!("{}/background.png", tmp.display());

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
        .arg("/tmp/rain/background.png")
        .status()
        .expect("Failed to run convert command");

    let _ = Command::new("feh")
        .arg("--no-fehbg")
        .arg("--bg-scale")
        .arg(&background)
        .status();
}

fn find_battery_path() -> Option<PathBuf> {
    let mut power_dir_entries = fs::read_dir("/sys/class/power_supply").unwrap();
    power_dir_entries.find_map(|entry| {
        let path = entry.unwrap().path();
        let file_content = fs::read_to_string(&path.join("type")).unwrap_or_default();
        if file_content.trim() == "Battery" {
            Some(path)
        } else {
            None
        }
    })
}

fn create_battery_struct(battery_path: PathBuf) -> Battery {
    let capacity = fs::read_to_string(&battery_path.join("capacity")).unwrap();
    let status = fs::read_to_string(&battery_path.join("status")).unwrap();
    Battery {
        status: status.trim().to_string(),
        capacity: capacity.trim().to_string().parse::<u8>().unwrap(),
    }
}
