mod color_schemes;

use color_schemes::color_schemes;
use image::{self, io::Reader, DynamicImage, GenericImageView, ImageBuffer, Rgba};
use os_release::OsRelease;
use std::{
    env, fs,
    io::Cursor,
    path::{Path, PathBuf},
    thread,
    time::Duration,
};

#[derive(PartialEq, Eq)]
enum BatteryStatus {
    Charging,
    NotCharging,
}

impl BatteryStatus {
    fn new(status: &str) -> BatteryStatus {
        match status {
            "Charging" => Self::Charging,
            _ => Self::NotCharging,
        }
    }
}

struct Battery {
    status: BatteryStatus,
    capacity: u8,
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

    fn new(battery_path: &Path) -> Self {
        let status = Self::get_status(battery_path);
        let capacity = Self::get_capacity(battery_path);
        Self { status, capacity }
    }
}

pub struct Colors {
    charging: [u8; 4],
    default: [u8; 4],
    low_battery: [u8; 4],
    background: [u8; 4],
}

#[tokio::main]
async fn main() {
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
        home_dir.join(format!(".rain/{}.png", name))
    };

    let battery_path = match find_battery_path() {
        Some(path) => path,
        None => panic!("Could not find battery"),
    };

    let image = match image::open(&img_path) {
        Ok(image) => image,
        Err(_) => get_image(&name, &img_path).await,
    };

    let mut previous = Battery {
        capacity: 0,
        status: BatteryStatus::NotCharging,
    };

    loop {
        let battery = Battery::new(&battery_path);
        if battery.capacity != previous.capacity || battery.status != previous.status {
            create_and_set(battery.capacity, &battery.status, &name, &image);
            previous = battery;
        }
        thread::sleep(Duration::from_secs(5));
    }
}

fn create_and_set(capacity: u8, status: &BatteryStatus, name: &String, image: &DynamicImage) {
    let tmp = PathBuf::from("/tmp/rain");
    fs::create_dir_all(&tmp).unwrap();

    let (width, height) = (image.width(), image.height());
    let color_scheme = color_schemes(name);
    let color = match status {
        BatteryStatus::Charging => color_scheme.charging,
        _ if capacity >= 30_u8 => color_scheme.default,
        _ => color_scheme.low_battery,
    };

    let mut output: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(width, height);
    image.pixels().for_each(|(x, y, pixel)| {
        let capacity = 1.0 - capacity as f32 / 100.0;
        if y as f32 > height as f32 * capacity && pixel == Rgba([143, 188, 187, 255]) {
            output.put_pixel(x, y, Rgba(color));
        } else {
            output.put_pixel(x, y, pixel);
        }
    });

    let mut background = ImageBuffer::new(3840, 2160);
    background
        .pixels_mut()
        .collect::<Vec<_>>()
        .iter_mut()
        .for_each(|pixel| **pixel = Rgba(color_scheme.background));

    let x = (3840 - width) / 2;
    let y = (2160 - height) / 2;
    image::imageops::overlay(&mut background, &output, x as i64, y as i64);

    let background_path = tmp.join("background.png");
    background.save(tmp.join(&background_path)).unwrap();
    wallpaper::set_from_path(tmp.join(background_path).to_str().unwrap())
        .expect("Error while setting wallpaper");
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

async fn get_image(name: &String, img_path: &PathBuf) -> DynamicImage {
    let bytes = match reqwest::get(format!("http://127.0.0.1:7878/{name}")).await {
        Ok(image) => image.bytes().await.unwrap(),
        Err(_) => panic!("Image {name}.png not found!"),
    };
    let image = Reader::new(Cursor::new(bytes))
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap();
    fs::create_dir_all(img_path.parent().unwrap()).unwrap();
    image.save(img_path).unwrap();
    image
}
