
all: main.o
	clang -o sliver main.o

main.o: 
	clang -c main.c
	

clean:
	rm -rf *.o
	rm -rf sliver 

