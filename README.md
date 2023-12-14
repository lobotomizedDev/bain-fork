# Rain - Highly performant battery indicator

### Prerequisites

Make sure you have the following programs installed:

- [git](https://git-scm.com/)
- [rust](https://rust-lang.github.io/rustup/installation/index.html)

### Supported Environments

Rain supports the following environments:

- DWM (requires feh)
- i3 (requires feh)
- GNOME
- KDE
- Cinnamon
- Unity
- Budgie
- XFCE
- LXDE
- MATE
- Deepin
- Most Wayland compositors (requires swaybg)

### Installation

1. Clone the repository:

    ```bash
    git clone https://github.com/lobotomizedDev/rain
    ```

2. Run the Makefile script:

    ```bash
    make install
    ```

3. Run the script from your startup file (e.g., `.xinitrc`). Add `&` at the end of the command to run it in the background:

    ```bash
    rain &
    ```

4. To specify a custom image for your battery indicator, use the image name as an argument:

    ```bash
    rain arch &
    ```

5. Restart your X session (log out and log back in):

    ```bash
    pkill X
    ```

### Adding Custom Battery Indicator

1. Create a PNG image with a `#8FBCBB` background color. Use the following ImageMagick command to convert your image:

    ```bash
    convert input_image.png -fill "#8fbcbb" -colorize 100% output_image.png
    ```

2. Copy your image to the `images` directory:

    ```bash
    cp ~/Pictures/output_image.png ~/.rain/images
    ```

3. Run Rain with the name of your image as an argument:

    ```bash
    rain output_image &
    ```

### Custom Color Scheme

1. Open the `color_schemes.rs` file:

    ```bash
    vim rain/src/color_schemes.rs
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
