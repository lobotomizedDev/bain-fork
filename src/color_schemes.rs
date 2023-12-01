use crate::Colors;

pub fn color_schemes(arg: &String) -> Colors {
    match arg.to_lowercase().as_str() {
        "rust" => Colors {
            charging: [192, 192, 192, 255],
            default: [196, 85, 5, 255],
            low_battery: [128, 0, 32, 255],
        },
        "arch" => Colors {
            charging: [192, 192, 192, 255],
            default: [23, 147, 208, 255],
            low_battery: [128, 0, 32, 255],
        },
        "manjaro" => Colors {
            charging: [192, 192, 192, 255],
            default: [52, 190, 91, 255],
            low_battery: [128, 0, 32, 255],
        },
        "debian" => Colors {
            charging: [192, 192, 192, 255],
            default: [206, 0, 86, 255],
            low_battery: [128, 0, 32, 255],
        },
        "ubuntu" => Colors {
            charging: [192, 192, 192, 255],
            default: [233, 84, 32, 255],
            low_battery: [128, 0, 32, 255],
        },
        "artix" => Colors {
            charging: [192, 192, 192, 255],
            default: [23, 147, 208, 255],
            low_battery: [128, 0, 32, 255],
        },
        _ => Colors {
            charging: [255, 255, 0, 255],
            default: [91, 194, 54, 255],
            low_battery: [191, 19, 28, 255],
        },
    }
}
