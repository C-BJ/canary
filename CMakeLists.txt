cmake_minimum_required(VERSION 3.22)

project(Jaye)

set(CMAKE_CXX_STANDARD 23)

set(SOURCES
        src/compiler/lexer/lexer.h
        src/compiler/lexer/lexer.cpp
        src/compiler/lexer/tokens.h
        src/main.cpp)

add_executable(Jaye src/main.cpp)