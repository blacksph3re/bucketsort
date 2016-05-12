#!/bin/sh

cargo build --release
cp target/release/bucketsort .

for i in 1 2 16
do
	echo "-----"
	echo "Number of threads = $i"
	time ./bucketsort input -n $i
done
