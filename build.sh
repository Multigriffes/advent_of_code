#!/usr/bin/bash

for folder in $(find ./solutions/ -maxdepth 1 -mindepth 1 -type d); do
    folder=${folder##*/}
    if [ "$folder" != "year_X" ]; then
        echo "╔═ $folder ═"
        export CARGO_TARGET_DIR=./build/"$folder"
        #echo "$CARGO_TARGET_DIR"
        cargo build -r -p "$folder"
        echo "╚═ $folder ═"
    fi
done
