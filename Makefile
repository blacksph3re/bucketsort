all:
	make build
	
build:
	cargo build --release

makeinput:
	${CXX} makeinput.cpp -o makeinput -DNUM_LINES=1000 -O3 # Creates roughly 770MB input
	./makeinput
	rm makeinput
run:
	target/release/bucketsort input -n 2 -o output
	
scalingtest:
	sh run.sh
