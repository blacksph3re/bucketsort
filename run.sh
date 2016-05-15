#!/bin/sh

cargo build --release
cp target/release/bucketsort .

for i in 1 2 4 8 16
do
	echo "-----"
	echo "Number of threads = $i"
	time ./bucketsort input -n $i
done
