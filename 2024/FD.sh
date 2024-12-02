#!/bin/bash

day_of_month=$(date +"%e" | sed 's/ //g')

url="https://adventofcode.com/2024/day/$day_of_month/input"
curl "$url"
