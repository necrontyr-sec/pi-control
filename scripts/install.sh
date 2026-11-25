#!/bin/bash

set -e

echo "Installing Reboot..."

sudo install -Dm755 \
    target/release/pi-control \
    /usr/local/bin/pi-control

echo "Installation completed!"