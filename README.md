# rain - the battery indicator written in rust

This program is highly inspired by [bain](https://github.com/amishbni/bain/tree/master)

## How to use?

* Make sure you have these programs installed on your machine: [git](https://git-scm.com/), [imagemagick](https://imagemagick.org), and [feh](https://feh.finalrewind.org).

* Clone the repository into `~/.rain`

```bash
git clone https://github.com/lobotomizedDev/rain ~/.rain
```

* Copy it to `/usr/local/bin`, so that the command is recognized by your terminal. Make sure to run it with `sudo`.
```bash
sudo cp ~/.rain/bin/rain /usr/local/bin
```

* Run the script from your startup file (e.g. `.bashrc`). Make sure to use `&` at the end of the command as it is a blocking script.
```bash
rain rust &
```

* Execute your bashrc file
```bash
source ~/.bashrc
```

## How to add your own battery indicator?

* Create png image with #8FBCBB background color

* Copy your image to images directory

```bash
cp ~/Pictures/example.png ~/.rain/images
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

* Compile rain

```bash
cargo build --release
```

* Continue from "How to use?"
