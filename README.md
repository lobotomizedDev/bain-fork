# Ruin - The battery indicator

Create a battery indicator from any image

### Prerequisites

- [rust](https://www.rust-lang.org/tools/install)

[Supported Environments](https://github.com/unixpariah/wlrs?tab=readme-ov-file#supported-environments)

### Usage

1. Install Ruin:

    ```bash
    cargo install ruin
    ```

2. Run the script:

    ```bash
    ruin
    ```

3. If you want to set the wallpaper for specific screens (the default behavior is to set the wallpaper on all screens), use the following command:

    ```bash
    ruin -s 0 1
    ```

4. If you want to modify the refresh interval (the default is every 5 seconds), use this command:

    ```bash
    ruin -t 1
    ```

### Adding Custom Battery Indicator

1. Create an image with a `#8FBCBB` color. Use the following ImageMagick command to convert your image:

    ```bash
    convert input_image.png -fill "#8fbcbb" -colorize 100% output_image.png
    ```

2. Copy your image to the config directory:

    ```bash
    cp ./output_image.png ~/.config/ruin/images
    ```

3. Run Ruin with the name of your image as an argument:

    ```bash
    ruin output_image
    ```

### Custom Color Scheme

1. Open the `colorschemes.yaml` file:

    ```bash
    vim ~/.config/ruin/colorschemes.yaml
    ```

2. Add your custom color scheme (colors should be in rgb):

    ```rust
    example:
      charging: [r, g, b]
      default: [r, g, b]
      low_battery: [r, g, b]
      background: [r, g, b]
    ```

3. Run the script

    ```bash
    ruin example
    ```

Inspired by [bain](https://github.com/amishbni/bain/tree/master).
