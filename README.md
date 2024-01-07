# Ruin - The battery indicator (written in rust)

### Prerequisites

Make sure you have the following programs installed:

- [git](https://git-scm.com/)
- [rust](https://rust-lang.github.io/rustup/installation/index.html)

### Supported Environments

- Most Wayland compositors (requires [swww](https://github.com/Horus645/swww))
- dwm (requires [feh](https://github.com/derf/feh.git))
- i3 (requires [feh](https://github.com/derf/feh.git))
- GNOME
- KDE
- Cinnamon
- Unity
- Budgie
- XFCE
- LXDE
- MATE
- Deepin

### Installation

1. Clone the repository:

    ```bash
    git clone https://github.com/lobotomizedDev/ruin
    ```

2. Run the Makefile script:

    ```bash
    make install
    ```

3. Run the script from your startup file (e.g., `.xinitrc`). Add `&` at the end of the command to run it in the background:

    ```bash
    ruin &
    ```

4. If on wayland start swww daemon from your startup file:

    ```bash
    swww init 
    ```

6. Restart your session (log out and log back in):

### Adding Custom Battery Indicator

1. Create a PNG image with a `#8FBCBB` background color. Use the following ImageMagick command to convert your image:

    ```bash
    convert input_image.png -fill "#8fbcbb" -colorize 100% output_image.png
    ```

2. Copy your image to the `images` directory:

    ```bash
    cp ~/Pictures/output_image.png ~/.ruin/images
    ```

3. Run Ruin with the name of your image as an argument:

    ```bash
    ruin output_image &
    ```

### Custom Color Scheme

1. Open the `color_schemes.rs` file:

    ```bash
    vim ruin/src/color_schemes.rs
    ```

2. Inside the `match` statement, add your custom color scheme (colors should be in rgba):

    ```rust
    "example" => Colors {
        charging: [r, g, b, a],
        default: [r, g, b, a],
        low_battery: [r, g, b, a],
        background: [r, g, b, a],
    },
    ```

3. Run the Makefile script:

    ```bash
    make clean install
    ```

Highly inspired by [bain](https://github.com/amishbni/bain/tree/master).
