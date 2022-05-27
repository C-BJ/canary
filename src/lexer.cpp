#include "lexer.h"
#include "tokens.h"
#include <iostream>
#include <vector>

void lexer(std::string source) {
    std::vector<Token> tokens;
    unsigned int current = 0;

    while(current < source.size()-1) {
        if((const char*)source[current] == "=") 
            tokens.push_back({TokenKind::Assign, "="});
        current++;
    }

}