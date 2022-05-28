#pragma once
#include <string>

typedef struct {
    std::string source;
    unsigned int current;
} Lexer_T;

namespace Lexer {
    void init(std::string source);
    void lexer();
}