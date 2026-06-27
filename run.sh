#!/usr/bin/bash

for year in $(find ./build/ -maxdepth 1 -mindepth 1 -type d); do
    year=${year##*/}
    if [ "$year" != "year_X" ]; then
        cd "build/$year/release" || continue
        echo "╔═ $year ═"
        for day in $(find . -maxdepth 1 -type f -regextype posix-extended -regex "^\.\/day[0-9]{1,2}[a-zA-Z_]*$"); do
            input=${day#*/}
            input=${input%%_*}".txt"
            #echo "$input"
            #echo "$day"
            echo "║ ${day#*/}: "
            $day "../../../solutions/$year/src/input/$input" | sed "s/^/║ /"
            echo "║"
        done
        echo "╚═ $year ═"
    fi
done
