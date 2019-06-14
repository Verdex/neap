
#ifndef __data_h__
#define __data_h__

struct SString {
    size_t length;
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
        SString string;
    };
};

#endif 

