pub struct Colors {
    pub charging: [u8; 4],
    pub default: [u8; 4],
    pub low_battery: [u8; 4],
    pub background: [u8; 4],
}

pub fn color_schemes(arg: &String) -> Colors {
    match arg.to_lowercase().as_str() {
        "arch" => Colors {
            charging: [192, 192, 192, 255],
            default: [23, 147, 208, 255],
            low_battery: [128, 0, 32, 255],
            background: [40, 40, 40, 255],
        },
        "gentoo" => Colors {
            default: [48, 48, 48, 255],
            charging: [110, 86, 175, 255],
            low_battery: [242, 222, 222, 255],
            background: [40, 40, 40, 255],
        },
        "artix" => Colors {
            charging: [192, 192, 192, 255],
            default: [23, 147, 208, 255],
            low_battery: [128, 0, 32, 255],
            background: [40, 40, 40, 255],
        },
        "debian" => Colors {
            charging: [192, 192, 192, 255],
            default: [206, 0, 86, 255],
            low_battery: [128, 0, 32, 255],
            background: [40, 40, 40, 255],
        },
        "manjaro" => Colors {
            charging: [192, 192, 192, 255],
            default: [52, 190, 91, 255],
            low_battery: [128, 0, 32, 255],
            background: [40, 40, 40, 255],
        },
        "ubuntu" => Colors {
            charging: [192, 192, 192, 255],
            default: [233, 84, 32, 255],
            low_battery: [128, 0, 32, 255],
            background: [40, 40, 40, 255],
        },
        _ => Colors {
            charging: [255, 255, 0, 255],
            default: [91, 194, 54, 255],
            low_battery: [191, 19, 28, 255],
            background: [40, 40, 40, 255],
        },
    }
}
