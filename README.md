# Ruin - The battery indicator

### Prerequisites

- [rust](https://www.rust-lang.org/tools/install)

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

1. Install Ruin:

    ```bash
    cargo install ruin
    ```

2. Run the script from your startup file (e.g., `.xinitrc`). Add `&` at the end of the command to run it in the background:

    ```bash
    ruin &
    ```

3. If on wayland start swww daemon from your startup file:

    ```bash
    swww init 
    ```

4. Restart your session (log out and log back in):

### Adding Custom Battery Indicator

1. Create a PNG image with a `#8FBCBB` color. Use the following ImageMagick command to convert your image:

    ```bash
    convert input_image.png -fill "#8fbcbb" -colorize 100% output_image.png
    ```

2. Copy your image to the `images` directory:

    ```bash
    cp ~/Pictures/output_image.png ~/.config/ruin/images
    ```

3. Run Ruin with the name of your image as an argument:

    ```bash
    ruin output_image &
    ```

### Custom Color Scheme

1. Open the `colorschemes.yaml` file:

    ```bash
    vim ~/.config/ruin/colorschemes.yaml
    ```

2. Add your custom color scheme (colors should be in rgba):

    ```rust
    example:
      charging: [r, g, b, a]
      default: [r, g, b, a]
      low_battery: [r, g, b, a]
      background: [r, g, b, a]
    ```

3. Run the Makefile script:

    ```bash
    make clean install
    ```

Highly inspired by [bain](https://github.com/amishbni/bain/tree/master).
