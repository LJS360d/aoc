#!/usr/bin/env bash
set -euo pipefail

source .env
if [ -z "${session:-}" ]; then
    echo "Error: 'session' variable not defined"
    exit 1
fi

CURRENT_YEAR=$(date +%Y)

for year in $(seq 2015 $CURRENT_YEAR); do
    for day in $(seq 1 25); do
        if [ "$year" -eq "$CURRENT_YEAR" ]; then
                break
        fi

        target_dir="inputs/$year"
        input_file="$target_dir/$day.txt"

        # Create target directory if it doesn't exist
        mkdir -p "$target_dir"

        # Skip if input is already downloaded
        if [ -f "$input_file" ]; then
            echo "Skipping $year Day $day (already exists)"
            continue
        fi

        echo "Fetching input for Year $year, Day $day..."
        
        # Download input file
        curl -s -o "$input_file" --cookie "session=$session" "https://adventofcode.com/$year/day/$day/input"

        # Advent of Code rate-limiting polite buffer (1s)
        sleep 1
    done
done

echo "Finished downloading inputs!"
