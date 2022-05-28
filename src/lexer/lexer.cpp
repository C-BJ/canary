#include "lexer.h"
#include "tokens.h"
#include <iostream>
#include <vector>

void advance() {
    Lexer_T l;
    l.current++;
}

char get_char() {
    Lexer_T l;
    return l.source[l.current];
}

namespace Lexer {
    void init(std::string s) {
        Lexer_T l;
        l.current = 0;
        l.source = s;
    }

    void lexer() {
        Lexer_T l;
        std::vector<Tokens_T> tokens;

        std::cout << l.source;

        while(l.current < l.source.size()) {
            advance();
        }
    }
}