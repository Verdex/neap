
#include<stdio.h>
#include<stdlib.h>
#include "s_alloc.h"
#include "data.h"
#include "lexer.h"


struct Tokens lex(struct Alloc* alloc, FILE* input) {
    int c = fgetc(input);
    size_t word_index = 0;
    size_t word_allocated_length = 8;
    while ( c != EOF ) {
        if ( c == '\n' || c == '\r' || c == '\t' || c == ' ' ) {
            c = fgetc(input); 
        } 
        else {
             
        }
    }
}

