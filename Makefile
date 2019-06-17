
all: s_alloc.o lexer.o main.o
	clang -o sliver s_alloc.o lexer.o main.o

main.o: 
	clang -c main.c

lexer.o:
	clang -c lexer.c

s_alloc.o:
	clang -c s_alloc.c

clean:
	rm -rf *.o
	rm -rf sliver 

