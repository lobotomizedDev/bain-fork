# rain - the battery indicator written in rust

This program is highly inspired by [bain](https://github.com/amishbni/bain/tree/master)

## How to use?

* Make sure you have these programs installed on your machine: [git](https://git-scm.com/), [imagemagick](https://imagemagick.org), and [feh](https://feh.finalrewind.org).

* Clone the repository into `~/.bain`

```bash
git clone https://github.com/lobotomizedDev/rain ~/.rain
```

* Copy it to `/usr/local/bin`, so that the command is recognized by your terminal. Make sure to run it with `sudo`.
```bash
sudo cp ~/.rain/targer/release /usr/local/bin
```

* Run the script from your startup file (e.g. `.xinitrc`). Make sure to use `&` at the end of the command as it is a blocking script.
```bash
rain rust &
```

* Restart your X session (log out and log back in).
```bash
pkill X
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
