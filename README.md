# rain - the battery indicator written in rust

This program is highly inspired by [bain](https://github.com/amishbni/bain/tree/master)

## How to use?

* Make sure you have these programs installed on your machine: [git](https://git-scm.com/), [imagemagick](https://imagemagick.org), [feh](https://feh.finalrewind.org) and [rust](https://rust-lang.github.io/rustup/installation/index.html).

* Clone repository

```bash
git clone https://github.com/lobotomizedDev/rain
```

* Run Makefile script

```bash
make clean install
```

* Run the script from your startup file (e.g. `.xinitrc`). Make sure to use `&` at the end of the command as it is a blocking script.
```bash
rain &
```

* If you want to specify which image you want to use as your battery indicator use name of image as
argument

```bash
rain arch &
```

* Restart your X session (log out and log back in).
```bash
pkill X
```

## How to add your own battery indicator?

* Create png image with #8FBCBB background color

* Copy your image to images directory

```bash
cp ~/Pictures/example.png rain/images
```

* Run Makefile script

```bash
make clean install
```

* Run rain with the name of your image as arg

```bash
rain example &
```

## Create custom color scheme

* Open color\_schemes.rs file

```bash
vim .rain/src/color_schemes.rs
```

* Inside of match statement add (colors should be in hex)

```bash
"example" => Colors {
    charging: "color1".to_string(),
    default: "color2".to_string(),
    low_battery: "color3".to_string(),
},
```

* Run Makefile script

```bash
make clean install
```
