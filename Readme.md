# Razer Naga2014 key remap
This project remaps razer naga 2014 keys into F1..F12 keys in linux operating system.

## Usage
copy `disable-razer-naga2014-keyboard.conf` file to `/etc/X11/xorg.conf.d/`

restart

build with cargo `cargo build --release`

run as root `target/release/razer-naga2014-key-remap`

## Todo
* making keys configurable
* aur package for achlinux
* instalation without restart
