#!/bin/bash
# Make sure to install Rust
cargo build --release
# Make sure to run this command with sudo.
sudo mkdir /usr/share/eunomia
sudo cp -r target/release/eunomia /usr/share/eunomia

cd /usr/share/eunomia
sudo chmod +x eunomia
sudo cp eunomia /usr/bin/
