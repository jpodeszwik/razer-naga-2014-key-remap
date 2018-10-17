# Razer Naga 2014 key remap
This project remaps razer naga 2014 keys into F1..F12 keys in linux operating system.

## Install with cargo
`cargo install razer-naga-2014-key-remap`

run as root `cargo run razer-naga-2014-key-remap`

## Build from source
build with cargo `cargo build --release`

run as root `target/release/razer-naga-2014-key-remap`

## AUR package
Install [razer-naga-2014-key-remap-bin](https://aur.archlinux.org/packages/razer-naga-2014-key-remap-bin) package

Start with systemd `systemctl start razer-naga-2014-key-remap.service`

## Todo
* making keys configurable
