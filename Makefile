all:
	make build
	
build:
	cargo build --release

makeinput:
	clang++ makeinput.cpp -o makeinput -DNUM_LINES=100000000 # Creates roughly 770MB input
	./makeinput
	rm makeinput
run:
	target/release/bucketsort input -o output
	
runpar:
	sh run.sh