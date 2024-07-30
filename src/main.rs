mod battery;

use battery::{find_battery_path, Battery, BatteryStatus};
use clap::Parser;
use image::{imageops, DynamicImage, GenericImageView, ImageBuffer, Pixel, Rgba, RgbaImage};
use inotify::{Inotify, WatchMask};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    error::Error,
    fs,
    num::NonZeroU32,
    path::{Path, PathBuf},
    sync::{mpsc, Arc, RwLock},
    thread,
    time::Duration,
};
use wlrs::{CropMode, Image, SetType, Wlrs};

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
    outputs: Vec<String>,
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

    let color_scheme = Arc::new(RwLock::new(
        get_colorscheme(&ruin_dir, &args.name).unwrap_or_default(),
    ));
    let battery_path = find_battery_path().expect("Battery not found");

    let (tx, rx) = mpsc::channel();

    {
        let color_scheme = Arc::clone(&color_scheme);
        thread::spawn(move || {
            let mut inotify = Inotify::init().unwrap();
            _ = inotify.watches().add(
                &ruin_dir,
                WatchMask::MODIFY | WatchMask::MOVED_TO | WatchMask::CREATE,
            );
            let mut buffer = [0; 1024];
            loop {
                if let Ok(events) = inotify.read_events_blocking(&mut buffer) {
                    events.for_each(|_| {
                        *color_scheme.write().unwrap() =
                            get_colorscheme(&ruin_dir, &args.name).unwrap_or_default();
                        _ = tx.send(());
                    });
                }
            }
        });
    }

    let wlrs = Wlrs::new().unwrap();
    loop {
        let battery = Battery::new(&battery_path);
        if battery != previous || rx.try_recv().is_ok() {
            let image = create(&battery, &color_scheme.read().unwrap(), &image);
            let image_data = Image::new(
                &image,
                NonZeroU32::new(image.width()).unwrap(),
                NonZeroU32::new(image.height()).unwrap(),
            )
            .unwrap();
            _ = wlrs.set(SetType::Img(image_data), &args.outputs, CropMode::Fit(None));
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

fn create(battery: &Battery, color_scheme: &Colors, image: &DynamicImage) -> RgbaImage {
    let (status, capacity) = (&battery.status, battery.capacity);
    let (width, height) = (image.width(), image.height());

    let color = match status {
        BatteryStatus::Charging => color_scheme.charging,
        _ if capacity >= 30_u8 => color_scheme.default,
        _ => color_scheme.low_battery,
    };

    let color = [color[0], color[1], color[2], 255];
    let bg = [
        color_scheme.background[0],
        color_scheme.background[1],
        color_scheme.background[2],
        255,
    ];

    let mut output = RgbaImage::new(width, height);
    let capacity = 1.0 - capacity as f32 / 100.0;
    image.pixels().for_each(|(x, y, pixel)| match pixel {
        Rgba([143, 188, 187, 255]) if y as f32 > height as f32 * capacity => {
            output.put_pixel(x, y, Rgba(color))
        }
        Rgba([_, _, _, alpha]) if alpha < 255 => output.put_pixel(x, y, Rgba(bg)),
        _ => output.put_pixel(x, y, pixel.to_rgba()),
    });
    let mut background = ImageBuffer::new(3840, 2160);
    background
        .pixels_mut()
        .collect::<Vec<_>>()
        .iter_mut()
        .for_each(|pixel| **pixel = Rgba(bg));

    let x = (3840 - width) / 2;
    let y = (2160 - height) / 2;
    imageops::overlay(&mut background, &output, x as i64, y as i64);

    background
}
