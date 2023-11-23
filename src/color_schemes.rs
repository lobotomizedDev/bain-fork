use crate::Colors;

pub fn color_schemes(arg: &String) -> Colors {
    match arg.to_lowercase().as_str() {
        "rust" => Colors {
            charging: "#C0C0C0".to_string(),
            default: "#C45505".to_string(),
            low_battery: "#800020".to_string(),
        },
        "arch" => Colors {
            charging: "#C0C0C0".to_string(),
            default: "#1793D0".to_string(),
            low_battery: "#800020".to_string(),
        },
        _ => Colors {
            charging: "#FFFF00".to_string(),
            default: "#5BC236".to_string(),
            low_battery: "#BF131C".to_string(),
        },
    }
}
