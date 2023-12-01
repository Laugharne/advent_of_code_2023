#!/bin/bash

for i in {1..25}
do
	echo $i
	name_dir_1=$(printf "day_%02d" "$i")

	cargo new $name_dir_1
	cd $name_dir_1
	touch "README.md"
	echo "[Day $i, part ONE - Advent of Code 2023](https://adventofcode.com/2023/day/$i)" > "README.md"
	echo "" >> "README.md"
	echo "**--- Day $i : XXXXXX ---**" >> "README.md"
	echo "" >> "README.md"
	cd ..

done
