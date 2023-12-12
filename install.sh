#!/bin/sh
cargo build -r
sudo cp -f target/release/brwm /usr/local/bin/brwm
sudo chmod 755 /usr/local/bin/brwm
mkdir -p ~/.config/brwm
cp -n config.json ~/.config/brwm/config.json
echo "exec brwm" > ~/.xinitrc