#pragma once

#include <string>

enum TokenKind {
    Identifier,   // [name]
    Integer,      // int
    Integer8,     // i8
    Integer16,    // i16 
    Integer32,    // i32
    Integer64,    // i64
    UInteger,     // uint
    UInteger8,    // u8
    UInteger16,   // u16
    UInteger32,   // u32
    UInteger64,   // u64
    Float,        // float
    Float64,      // f64
    Float32,      // f32
    String,       // string

    Constant,     // const
    Let,          // let
    Mutable,      // mut

    If,           // if
    Else,         // else
    Switch,       // switch
    Break,        // break
    Default,      // default
    For,          // for
    While,        // while
    Return,       // return

    Assign,       // =
    Plus,         // +
    Minus,        // -
    Mul,          // *
    Divide,       // /
    PlusAssign,   // +=
    MinusAssign,  // -=
    MulAssign,    // *=
    DivideAssign, // /=

    LParen,       // (
    RParen,       // )
    LSqParen,     // [
    RSqParen,     // ]
    LBrace,       // {
    RBrace,       // }
    Comma,        // ,
    SemiColon,    // :
};

struct Token {
    TokenKind kind;
    std::string literal;
};