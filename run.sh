#!/bin/sh



for i in 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16
do
	echo "-----"
	echo "Number of threads = $i"
	rm output
	time target/release/bucketsort input -n $i -o output
done

