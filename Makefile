
all: data.cmi lexer.cmx main.cmx
	ocamlopt -o sliver lexer.cmx main.cmx

main.cmx:
	ocamlopt -c main.ml

lexer.cmx:
	ocamlc -c lexer.mli
	ocamlopt -c lexer.ml

data.cmi: 
	ocamlopt -c data.mli

clean:
	rm -rf *.cmi
	rm -rf *.cmx
	rm -rf *.o
	rm -rf sliver 


