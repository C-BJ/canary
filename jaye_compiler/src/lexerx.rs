use crate::tokens::*;

pub struct Lexer {
    src: Vec<char>,
    pos: usize,
    next: usize,
}

impl Lexer {
    pub fn new(s: String) -> Self {
        Self {
            src: s.chars().collect(),
            pos: 0,
            next: 1,
        }
    }

    pub fn lex(&mut self) -> Vec<Tok> {
        let mut tokens = Vec::<Tok>::new();   // Stores generated tokens
        
        loop {
            if self.pos > self.src.len()-1 { break }

            if self.src[self.pos] == '/' && self.src[self.next] == '/' {
                while self.src[self.pos] != '\n' {
                    self.advance();
                }
            }

            if self.src[self.pos] == '\n' {
                tokens.push(Tok{kind: TokKind::NewLine, lit: "$N$"});
            }

            let token: Tok = match self.src[self.pos] {
                '=' => { if self.src[self.next] == '=' {
                            self.advance();
                            Tok{kind: TokKind::Equal, lit: "=="}
                         } else { Tok{kind: TokKind::Assign, lit: "="} } }

                '+' => { if self.src[self.next] == '=' {
                            self.advance();
                            Tok{kind: TokKind::PlusAssign, lit: "+="}
                         } else { Tok{kind: TokKind::Plus, lit: "+"} } }

                '-' => { if self.src[self.next] == '=' {
                            self.advance();
                            Tok{kind: TokKind::MinusAssign, lit: "-="}
                         } else { Tok{kind: TokKind::Minus, lit: "-"} } }

                '/' => { if self.src[self.next] == '=' {
                            self.advance();
                            Tok{kind: TokKind::DivideAssign, lit: "/="}
                         } else { Tok{kind: TokKind::Divide, lit: "/"} } }

                '*' => { if self.src[self.next] == '=' {
                            self.advance();
                            Tok{kind: TokKind::MulAssign, lit: "*="}
                         } else { Tok{kind: TokKind::Mul, lit: "*"} } }

                '>' => { if self.src[self.next] == '=' {
                            self.advance();
                            Tok{kind: TokKind::GreaterEq, lit: ">="}
                         } else { Tok{kind: TokKind::Greater, lit: ">"} } }
                '<' => { if self.src[self.next] == '=' {
                            self.advance();
                            Tok{kind: TokKind::SmallerEq, lit: "<="}
                         } else { Tok{kind: TokKind::Smaller, lit: "<"} } }


                '%' => { if self.src[self.next] == '=' {
                            self.advance();
                            Tok{kind: TokKind::ModAssign, lit: "%="}
                         } else { Tok{kind: TokKind::Mod, lit: "%"} } }


                '(' => Tok{kind: TokKind::LParen, lit: "("},
                ')' => Tok{kind: TokKind::RParen, lit: ")"},
                '[' => Tok{kind: TokKind::LSqParen, lit: "["},
                ']' => Tok{kind: TokKind::RSqParen, lit: "]"},
                '{' => Tok{kind: TokKind::LBrace, lit: "{"},
                '}' => Tok{kind: TokKind::RBrace, lit: "}"},
                ':' => Tok{kind: TokKind::Colon, lit: ":"},
                ',' => Tok{kind: TokKind::Comma, lit: ","},
                '_' => Tok{kind: TokKind::Placeholder, lit: "_"},

                '!' => { if self.src[self.next] == '=' {
                            self.advance();
                            Tok{kind: TokKind::NotEq, lit: "!="}
                         } else { Tok{kind: TokKind::Not, lit: "!"} } }

                '|' => { if self.src[self.next] == '|' {
                            self.advance();
                            Tok{kind: TokKind::DoublePipe, lit: "||"}
                         } else { Tok{kind: TokKind::Pipe, lit: "|"} } }
                
                '&' => { if self.src[self.next] == '&' {
                            self.advance();
                            Tok{kind: TokKind::DoubleAnd, lit: "&&"}
                         } else { Tok{kind: TokKind::And, lit: "&"} } }
                
                _ if self.src[self.pos].is_alphabetic() => {
                    Tok{kind: TokKind::Identifier, lit: "f"}
                }


                _ => Tok{kind: TokKind::Space, lit: "$S$"},
            };

            tokens.push(token);
            self.advance();
        }

        return tokens
    }

    pub fn advance(&mut self) {
        self.pos = self.next;
        self.next += 1;
    }
}
