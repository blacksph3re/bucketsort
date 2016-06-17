#!/bin/sh



for i in 1 2 4 8 16 32
do
	echo "-----"
	echo "Number of threads = $i"
	time target/release/bucketsort input -n $i -o output
done

