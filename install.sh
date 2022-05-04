#!/bin/bash
# Make sure to install Rust
cargo build --release
# Make sure to run this command with sudo.
sudo mkdir /usr/share/eunomia
sudo cp -r target/release/eunomia /usr/share/eunomia
sudo cp -r Audio/ /usr/share/eunomia

chmod +x eunomia
sudo cp eunomia /usr/bin/
