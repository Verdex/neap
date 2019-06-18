
#ifndef __data_h__
#define __data_h__

#include<stdint.h>
#include<stdlib.h>

struct Tokens {
    size_t length;
    size_t allocated_length;
    struct Token* tokens;
};

struct SString {
    size_t length;
    size_t allocated_length;
    char* chars;
};

enum TT { TMod
        , TSig
        , TUse
        , TString
        , TNumber
        , TLCurl
        , TRCurl
        , TLSquare
        , TRSquare
        , TLParen
        , TRParen
        , TLAngle
        , TRAngle
        , TExt
        , TFun
        , };

struct Token {
    enum TT type;
    union {
        struct SString string;
    };
};

#endif 

