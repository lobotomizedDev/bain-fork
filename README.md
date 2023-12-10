# rain - the battery indicator written in rust

This program is highly inspired by
[bain](https://github.com/amishbni/bain/tree/master)

## How to use?

- Make sure you have these programs installed on your machine:
  [git](https://git-scm.com/),
  [rust](https://rust-lang.github.io/rustup/installation/index.html)

- Supported environments:

* DWM (requires feh)
* i3 (requires feh)
* GNOME
* KDE
* Cinnamon
* Unity
* Budgie
* XFCE
* LXDE
* MATE
* Deepin
* Most Wayland compositors (requires swaybg)

- Clone repository

```bash
git clone https://github.com/lobotomizedDev/rain
```

- Run Makefile script

```bash
make install
```

- Run the script from your startup file (e.g. `.xinitrc`). Make sure to use `&`
  at the end of the command as it is a blocking script.

```bash
rain &
```

- If you want to specify which image you want to use as your battery indicator
  use name of image as argument

```bash
rain arch &
```

- Restart your X session (log out and log back in).

```bash
pkill X
```

## How to add your own battery indicator?

- Create png image with #8FBCBB background color, to convert your image to that
  color use this ImageMagick command:

```bash
convert input_image.png -fill "#8fbcbb" -colorize 100% output_image.png
```

- Copy your image to images directory

```bash
cp ~/Pictures/output_image.png ~/.rain/images
```

- Run rain with the name of your image as arg

```bash
rain output_image &
```

## Create custom color scheme

- Open color_schemes.rs file

```bash
vim rain/src/color_schemes.rs
```

- Inside of match statement add (colors should be in rgba)

```bash
"example" => Colors {
    charging: [r, g, b, a],
    default: [r, g, b, a],
    low_battery: [r, g, b, a],
    background: [r, g, b, a],
},
```

- Run Makefile script

```bash
make clean install
```
