/* Jaye Programming Language
 * lexer.rs  -  Lexes the input string into tokens
 * UPL v1.0 Licence see on LICENCE for more .
 */

use crate::tokens::*;


pub struct Lexer {
    source: Vec<char>,
    current: usize,
}

impl Lexer {
    pub fn new(s: String) -> Self {
        Self {
            source: s.chars().collect(),
            current: 0,
        }
    }

    pub fn lex(&mut self) -> Vec<Tok> {
        let mut tokens = Vec::<Tok>::new();

        loop {
            if self.current > self.source.len()-1 { break };
            
            let c = self.get_char();

            if c.to_string().is_empty() { break }

            if c == '\n' { tokens.push(Tok::new(TokKind::NEWLINE, "$N$")) }

            match c {
                // = and ==
                '=' => {
                    self.current += 1;
                    if self.get_char() == '=' {
                        tokens.push(Tok::new(TokKind::Equal, "=="));
                    } else {
                        tokens.push(Tok::new(TokKind::Assign, "="));
                        self.current -= 1;
                    }
                }
                // + and +=
                '+' => {
                    self.current += 1;
                    if self.get_char() == '=' {
                        tokens.push(Tok::new(TokKind::PlusAssign, "+="));
                    } else {
                        tokens.push(Tok::new(TokKind::Plus, "+"));
                        self.current -= 1;
                    }
                }
                // - and -=
                '-' => {
                    self.current += 1;
                    if self.get_char() == '=' {
                        tokens.push(Tok::new(TokKind::MinusAssign, "-="));
                    } else {
                        tokens.push(Tok::new(TokKind::Minus, "-"));
                        self.current -= 1;
                    }
                }
                // / and /=
                '/' => {
                    self.current += 1;
                    if self.get_char() == '=' {
                        tokens.push(Tok::new(TokKind::DivideAssign, "/="));
                    } else {
                        tokens.push(Tok::new(TokKind::Divide, "/"));
                        self.current -= 1;
                    }
                }
                // * and *=
                '*' => {
                    self.current += 1;
                    if self.get_char() == '=' {
                        tokens.push(Tok::new(TokKind::MulAssign, "*="));
                    } else {
                        tokens.push(Tok::new(TokKind::Mul, "*"));
                        self.current -= 1;
                    }
                }
                // % and %=
                '%' => {
                    self.current += 1;
                    if self.get_char() == '=' {
                        tokens.push(Tok::new(TokKind::MODAssign, "%="));
                    } else {
                        tokens.push(Tok::new(TokKind::Mod, "%"));
                        self.current -= 1;
                    }
                }
                // ! and !=
                '!' => {
                    self.current += 1;
                    if self.get_char() == '=' {
                        tokens.push(Tok::new(TokKind::NotEq, "!="));
                    } else {
                        tokens.push(Tok::new(TokKind::Not, "!"));
                        self.current -= 1;
                    }
                }
                // ^ and ^=
                '^' => {
                    self.current += 1;
                    if self.get_char() == '=' {
                        tokens.push(Tok::new(TokKind::PowerEq, "^="));
                    } else {
                        tokens.push(Tok::new(TokKind::Power, "^"));
                        self.current -= 1;
                    }
                }
                // << and <<= and < and <=
                '<' => {
                    self.current += 1;
                    if self.get_char() == '<' {
                        self.current += 1;
                        if self.get_char() == '=' {
                            tokens.push(Tok::new(TokKind::ShiftLeftEq, "<<="));
                        } else {
                            tokens.push(Tok::new(TokKind::ShiftLeft, "<<"));
                            self.current -= 1;
                        }
                    } else {
                        if self.get_char() == '=' {
                            tokens.push(Tok::new(TokKind::SmallerEq, "<="));
                        } else {
                            tokens.push(Tok::new(TokKind::Smaller, "<"));
                            self.current -= 1;
                        }
                    }
                    
                }
                // >> and >>= and > and >=
                '>' => {
                    self.current += 1;
                    if self.get_char() == '>' {
                        self.current += 1;
                        if self.get_char() == '=' {
                            tokens.push(Tok::new(TokKind::ShiftRightEq, ">>="));
                        } else {
                            tokens.push(Tok::new(TokKind::ShiftRight, ">>"));
                            self.current -= 1;
                        }
                    } else {
                        if self.get_char() == '=' {
                            tokens.push(Tok::new(TokKind::GreaterEq, ">="));
                        } else {
                            tokens.push(Tok::new(TokKind::Greater, ">"));
                            self.current -= 1;
                        }
                    }
                    
                }


                '(' => tokens.push(Tok::new(TokKind::LParen, "(")),
                ')' => tokens.push(Tok::new(TokKind::RParen, ")")),
                '[' => tokens.push(Tok::new(TokKind::RSqParen, "[")),
                ']' => tokens.push(Tok::new(TokKind::LSqParen, "]")),
                '{' => tokens.push(Tok::new(TokKind::LBrace, "{")),
                '}' => tokens.push(Tok::new(TokKind::RBrace, "}")),
                ',' => tokens.push(Tok::new(TokKind::Comma, ",")),
                ';' => tokens.push(Tok::new(TokKind::Semicolon, ";")),
                '~' => tokens.push(Tok::new(TokKind::Reverse, "~")),
                
                // | and ||
                '|' => {
                    self.current += 1;
                    if self.get_char() == '|' {
                        tokens.push(Tok::new(TokKind::DoublePipe, "||"));
                    } else {
                        tokens.push(Tok::new(TokKind::Pipe, "|"));
                        self.current -= 1;
                    }
                }
                // & and &&
                '&' => {
                    self.current += 1;
                    if self.get_char() == '&' {
                        tokens.push(Tok::new(TokKind::DoubleAnd, "&&"));
                    } else {
                        tokens.push(Tok::new(TokKind::And, "&"));
                        self.current -= 1;
                    }
                }

                '\"' => {
                    let mut buf = String::new();
                    self.current += 1;

                    while self.get_char() != '\"' {
                        buf.push(self.get_char());
                        self.current += 1;
                    }

                    tokens.push(Tok::new(TokKind::String, &buf[..]));
                }
               
                // _
                '_' => tokens.push(Tok::new(TokKind::Placeholder, "_")),


                // Detecting numbers
                _ if c.is_numeric() || c == '-' => {
                    let mut buf = String::new();

                    loop {
                        if !self.get_char().is_numeric() ||
                        self.current > self.source.len()-1 { break; }

                        buf.push(self.get_char());
                        self.current += 1;
                    }

                    tokens.push(Tok::new(TokKind::Integer, &buf[..]));
                    self.current -= 1;
                }

                // Detecting alphabet
                _ if c.is_alphabetic() => {
                    let mut buf = String::new();

                    loop {
                        if !self.get_char().is_alphabetic() ||
                        self.current > self.source.len()-1 { break; }

                        buf.push(self.get_char());
                        self.current += 1;
                    }

                    match &buf[..] {
                        "int" => tokens.push(Tok::new(TokKind::Integer, "int")),
                        "i8" => tokens.push(Tok::new(TokKind::Integer8, "i8")),
                        "i16" => tokens.push(Tok::new(TokKind::Integer16, "i16")),
                        "i32" => tokens.push(Tok::new(TokKind::Integer32, "i32")),
                        "i64" => tokens.push(Tok::new(TokKind::Integer64, "i64")),

                        "uint" => tokens.push(Tok::new(TokKind::UInteger, "uint")),
                        "u8" => tokens.push(Tok::new(TokKind::UInteger8, "u8")),
                        "u16" => tokens.push(Tok::new(TokKind::UInteger16, "u16")),
                        "u32" => tokens.push(Tok::new(TokKind::UInteger32, "u32")),
                        "u64" => tokens.push(Tok::new(TokKind::UInteger64, "u64")),

                        "float" => tokens.push(Tok::new(TokKind::Float, "float")),
                        "f32" => tokens.push(Tok::new(TokKind::Float32, "f32")),
                        "f64" => tokens.push(Tok::new(TokKind::Float64, "f64")),

                        "string" => tokens.push(Tok::new(TokKind::String, "string")),
                        "bool" => tokens.push(Tok::new(TokKind::Boolean, "bool")),

                        "var" => tokens.push(Tok::new(TokKind::Variable, "var")),
                        "val" => tokens.push(Tok::new(TokKind::Value, "var")),
                        "const" => tokens.push(Tok::new(TokKind::Constant, "const")),
                        "struct" =>  tokens.push(Tok::new(TokKind::Structure, "struct")),
                        "enum" => tokens.push(Tok::new(TokKind::Enumerator, "enum")),

                        "if" => tokens.push(Tok::new(TokKind::If, "if")),                   
                        "else" => tokens.push(Tok::new(TokKind::Else, "else")),             
                        "for" => tokens.push(Tok::new(TokKind::For, "for")),
                        "while" => tokens.push(Tok::new(TokKind::While, "while")),
                        "loop" => tokens.push(Tok::new(TokKind::Loop, "loop")),
                        "switch" => tokens.push(Tok::new(TokKind::Switch, "switch")),       
                        "break" => tokens.push(Tok::new(TokKind::Break, "break")),          
                        "default" => tokens.push(Tok::new(TokKind::Default, "default")),    
                        "return" => tokens.push(Tok::new(TokKind::Return, "return")),             

                        _ => tokens.push(Tok::new(TokKind::Identifier, &buf[..])),
                    }

                    self.current -= 1;
                }

                _ =>(),
            }

            self.current += 1;
        }

        println!("{:?}", tokens);
        return tokens;
    }

    pub fn get_char(&self) -> char {
        self.source[self.current]
    }

    pub fn get_next(&self) -> char {
        self.source[self.current+1]
    }
}
