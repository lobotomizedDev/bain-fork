mod battery;

use battery::{find_battery_path, Battery, BatteryStatus};
use image::{
    self, imageops, io::Reader, DynamicImage, GenericImageView, ImageBuffer, Pixel, Rgb, RgbImage,
    Rgba,
};
use reqwest::get;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    env,
    error::Error,
    fs::{self, File},
    io::{BufRead, BufReader, Cursor},
    path::PathBuf,
    thread,
    time::Duration,
};

static DELAY: Duration = Duration::from_secs(1);

#[derive(Debug, Serialize, Deserialize)]
struct Colors {
    charging: [u8; 3],
    default: [u8; 3],
    low_battery: [u8; 3],
    background: [u8; 3],
}

impl Default for Colors {
    fn default() -> Self {
        Self {
            charging: [255, 255, 0],
            default: [91, 194, 54],
            low_battery: [191, 19, 28],
            background: [40, 40, 40],
        }
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let name = get_name(args).unwrap_or("linux".into());
    let ruin_dir = {
        let home_dir = env::var("HOME").expect("Could not find home dir");
        PathBuf::from(format!("{}/.config/ruin", home_dir))
    };

    let img_path = ruin_dir.join(format!("images/{name}.png"));
    let image = match image::open(&img_path) {
        Ok(image) => image,
        Err(_) => get_image(&name, &img_path)
            .await
            .expect("Failed to fetch image from server"),
    };

    let mut previous = Battery {
        capacity: 0,
        status: BatteryStatus::NotCharging,
    };

    let color_scheme = get_colorscheme(&ruin_dir, &name).unwrap_or_default();
    let battery_path = find_battery_path().expect("Battery not found");

    loop {
        let battery = Battery::new(&battery_path);
        if battery != previous {
            let image = create(&battery, &color_scheme, &image);
            let _ = wlrs::set_from_memory(image);
            previous = battery;
        }
        thread::sleep(DELAY);
    }
}

fn get_name(args: Vec<String>) -> Result<String, Box<dyn Error>> {
    if let Some(name) = args.get(1) {
        return Ok(name.into());
    }

    let file = File::open("/etc/os-release")?;
    let buf_reader = BufReader::new(file);
    let line = buf_reader
        .lines()
        .filter_map(|line| line.ok())
        .find(|line| line.split_once("=").unwrap_or_default().0 == "ID");

    Ok(line
        .ok_or("")?
        .split_once("=")
        .ok_or("")?
        .1
        .trim()
        .to_owned())
}

async fn get_image(name: &String, img_path: &PathBuf) -> Result<DynamicImage, Box<dyn Error>> {
    let image = get(format!("https://ruin.shuttleapp.rs/{name}"))
        .await?
        .bytes()
        .await?;
    let image = Reader::new(Cursor::new(image))
        .with_guessed_format()?
        .decode()?;
    let _ = fs::create_dir_all(img_path.parent().unwrap());
    image.save(img_path)?;
    Ok(image)
}

fn get_colorscheme(path: &PathBuf, name: &String) -> Result<Colors, Box<dyn Error>> {
    let file = fs::read_to_string(path.join(format!("colorschemes.yaml")))?;
    let mut colorschemes: HashMap<String, Colors> = serde_yaml::from_str(&file)?;
    Ok(colorschemes.remove(name).ok_or("")?)
}

fn create(battery: &Battery, color_scheme: &Colors, image: &DynamicImage) -> RgbImage {
    let (status, capacity) = (&battery.status, battery.capacity);
    let (width, height) = (image.width(), image.height());

    let color = match status {
        BatteryStatus::Charging => color_scheme.charging,
        _ if capacity >= 30_u8 => color_scheme.default,
        _ => color_scheme.low_battery,
    };

    let mut output = RgbImage::new(width, height);
    image.pixels().for_each(|(x, y, pixel)| {
        let capacity = 1.0 - capacity as f32 / 100.0;
        match pixel {
            Rgba([143, 188, 187, 255]) if y as f32 > height as f32 * capacity => {
                output.put_pixel(x, y, Rgb(color))
            }
            Rgba([_, _, _, alpha]) if alpha < 255 => {
                output.put_pixel(x, y, Rgb(color_scheme.background))
            }
            _ => output.put_pixel(x, y, pixel.to_rgb()),
        }
    });
    let mut background = ImageBuffer::new(3840, 2160);
    background
        .pixels_mut()
        .collect::<Vec<_>>()
        .iter_mut()
        .for_each(|pixel| **pixel = Rgb(color_scheme.background));

    let x = (3840 - width) / 2;
    let y = (2160 - height) / 2;
    imageops::overlay(&mut background, &output, x as i64, y as i64);

    background
}
