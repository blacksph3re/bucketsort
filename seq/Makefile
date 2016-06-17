CC    = gcc
FLAGS = -O3 -Wall -Wno-unused-result

SRC   = bucketsort.c main.c
EXEC  = bin/bucketsort

all: make_dirs $(EXEC)

clean:
	rm -f -r bin

make_dirs:
	mkdir -p bin

$(EXEC): $(SRC)
	$(CC) $(FLAGS) $^ -o $@

run:
	./$(EXEC) input/medium.in input/medium.out

