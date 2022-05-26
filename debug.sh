#!/bin/sh

rm /home/nevin/.config/Code/User/globalStorage/pcsoftware.readable/bin/resync
cargo build
cp target/debug/resync ~/.config/Code/User/globalStorage/pcsoftware.readable/bin