
all: main.o
	clang -o neap main.o

main.o: 
	clang -c main.c
	

clean:
	rm -rf *.o
	rm -rf neap

