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
        let mut tk = Vec::<Tok>::new();   // Stores generated tokens
        
        loop {
            let mut buf = String::new();

            if self.pos > self.src.len()-1 { break }

            if self.src[self.pos] == '/' && self.src[self.next] == '/' {
                while self.src[self.pos] != '\n' {
                    self.advance();
                }
            }

            if self.src[self.pos] == '\n' {
                tk.push(Tok{kind: TokKind::NewLine, lit: "$N$"});
            }

            match self.src[self.pos] {
                '=' => { if self.src[self.next] == '=' {
                            self.advance();
                            tk.push(Tok{kind: TokKind::Equal, lit: "=="})
                         } else { tk.push(Tok{kind: TokKind::Assign, lit: "="}) } }

                '+' => { if self.src[self.next] == '=' {
                            self.advance();
                            tk.push(Tok{kind: TokKind::PlusAssign, lit: "+="})
                         } else { tk.push(Tok{kind: TokKind::Plus, lit: "+"}) } }

                '-' => { if self.src[self.next] == '=' {
                            self.advance();
                            tk.push(Tok{kind: TokKind::MinusAssign, lit: "-="})
                         } else { tk.push(Tok{kind: TokKind::Minus, lit: "-"}) } }

                '/' => { if self.src[self.next] == '=' {
                            self.advance();
                            tk.push(Tok{kind: TokKind::DivideAssign, lit: "/="})
                         } else { tk.push(Tok{kind: TokKind::Divide, lit: "/"}) } }

                '*' => { if self.src[self.next] == '=' {
                            self.advance();
                            tk.push(Tok{kind: TokKind::MulAssign, lit: "*="})
                         } else { tk.push(Tok{kind: TokKind::Mul, lit: "*"}) } }

                '>' => { if self.src[self.next] == '=' {
                            self.advance();
                            tk.push(Tok{kind: TokKind::GreaterEq, lit: ">="})
                         } else { tk.push(Tok{kind: TokKind::Greater, lit: ">"}) } }
                '<' => { if self.src[self.next] == '=' {
                            self.advance();
                            tk.push(Tok{kind: TokKind::SmallerEq, lit: "<="})
                         } else { tk.push(Tok{kind: TokKind::Smaller, lit: "<"}) } }


                '%' => { if self.src[self.next] == '=' {
                            self.advance();
                            tk.push(Tok{kind: TokKind::ModAssign, lit: "%="})
                         } else { tk.push(Tok{kind: TokKind::Mod, lit: "%"}) } }


                '(' => tk.push(Tok{kind: TokKind::LParen, lit: "("}),
                ')' => tk.push(Tok{kind: TokKind::RParen, lit: ")"}),
                '[' => tk.push(Tok{kind: TokKind::LSqParen, lit: "["}),
                ']' => tk.push(Tok{kind: TokKind::RSqParen, lit: "]"}),
                '{' => tk.push(Tok{kind: TokKind::LBrace, lit: "{"}),
                '}' => tk.push(Tok{kind: TokKind::RBrace, lit: "}"}),
                ':' => tk.push(Tok{kind: TokKind::Colon, lit: ":"}),
                ',' => tk.push(Tok{kind: TokKind::Comma, lit: ","}),
                '_' => tk.push(Tok{kind: TokKind::Placeholder, lit: "_"}),

                '!' => { if self.src[self.next] == '=' {
                            self.advance();
                            tk.push(Tok{kind: TokKind::NotEq, lit: "!="})
                         } else { tk.push(Tok{kind: TokKind::Not, lit: "!"}) } }

                '|' => { if self.src[self.next] == '|' {
                            self.advance();
                            tk.push(Tok{kind: TokKind::DoublePipe, lit: "||"})
                         } else { tk.push(Tok{kind: TokKind::Pipe, lit: "|"}) } }
                
                '&' => { if self.src[self.next] == '&' {
                            self.advance();
                            tk.push(Tok{kind: TokKind::DoubleAnd, lit: "&&"})
                         } else { tk.push(Tok{kind: TokKind::And, lit: "&"}) } }

                _ if self.src[self.pos].is_alphabetic() => {
                    while self.src[self.pos].is_alphabetic() {
                        buf.push(self.src[self.pos]);
                        self.advance();
                    }
                    tk.push(Tok{kind: TokKind::Identifier, lit: buf.as_str().clone()});
                    println!("{:?}", buf);
                }

                _ => tk.push(Tok{kind: TokKind::Space, lit: "$S$"}),
            };

            self.advance();
        }

        return tk
    }

    pub fn advance(&mut self) {
        self.pos = self.next;
        self.next += 1;
    }
}
