#aarch64-linux-gnu-gcc ./src/main.c -o ./target/main.o 
all:
	aarch64-linux-gnu-gcc ./src/main.c -o ./target/main.o
	aarch64-linux-gnu-objdump -d -Mintel target/main.o | less