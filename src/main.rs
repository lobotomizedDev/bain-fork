mod battery;

use battery::{find_battery_path, Battery, BatteryStatus};
use clap::Parser;
use core::panic;
use image::{imageops, DynamicImage, GenericImageView, ImageBuffer, Pixel, Rgb, RgbImage, Rgba};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    error::Error,
    fs,
    path::{Path, PathBuf},
    thread,
    time::Duration,
};
use wlrs::CropMode;

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

#[derive(Parser, Debug)]
struct Args {
    name: String,
    #[arg(short, long, num_args(0..))]
    screens: Option<Vec<usize>>,
    #[arg(short, long, num_args(0..))]
    time: Option<u64>,
}

fn main() {
    let args = Args::parse();

    let ruin_dir = {
        let home_dir = dirs::config_dir().expect("XDG_HOME_CONFIG not found");
        PathBuf::from(format!("{}/ruin", home_dir.display()))
    };

    let img_path = ruin_dir.join(format!("images/{}.png", args.name));
    let image =
        image::open(img_path).unwrap_or_else(|_| panic!("Image {}.png not found", args.name));

    let mut previous = Battery {
        capacity: 0,
        status: BatteryStatus::NotCharging,
    };

    let color_scheme = get_colorscheme(&ruin_dir, &args.name).unwrap_or_default();
    let battery_path = find_battery_path().expect("Battery not found");

    loop {
        let battery = Battery::new(&battery_path);
        if battery != previous {
            let image = create(&battery, &color_scheme, &image);
            let screens = args.screens.clone().unwrap_or(Vec::new());
            wlrs::set_from_memory(image, screens, CropMode::Fit(None))
                .expect("Failed to set wallpaper");
            previous = battery;
        }
        thread::sleep(Duration::from_secs(args.time.unwrap_or(5)));
    }
}

fn get_colorscheme(path: &Path, name: &String) -> Result<Colors, Box<dyn Error>> {
    let file = fs::read_to_string(path.join("colorschemes.yaml"))?;
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
    let capacity = 1.0 - capacity as f32 / 100.0;
    image.pixels().for_each(|(x, y, pixel)| match pixel {
        Rgba([143, 188, 187, 255]) if y as f32 > height as f32 * capacity => {
            output.put_pixel(x, y, Rgb(color))
        }
        Rgba([_, _, _, alpha]) if alpha < 255 => {
            output.put_pixel(x, y, Rgb(color_scheme.background))
        }
        _ => output.put_pixel(x, y, pixel.to_rgb()),
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
