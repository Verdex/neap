
all: lexer.o main.o
	clang -o sliver lexer.o main.o

main.o: 
	clang -c main.c

lexer.o:
	clang -c lexer.c

clean:
	rm -rf *.o
	rm -rf sliver 

