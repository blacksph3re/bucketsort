all:
	make build
	
build:
	cargo build --release

input:
	${CXX} makeinput.cpp -o makeinput -DNUM_LINES=100000000 -O3 # Creates roughly 770MB input
	./makeinput
	rm makeinput
run:
	target/release/bucketsort input -n 2 -o output
	
scalingtest:
	sh run.sh
