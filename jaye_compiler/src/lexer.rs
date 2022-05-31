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

            if self.src[self.pos] == '#' {
                while self.src[self.pos] != '\n' {
                    self.advance();
                }
            }

            if self.src[self.pos] == '\n' {
                tk.push(Tok::new(TokKind::NewLine, "$N$"));
            }

            match self.src[self.pos] {
            
                '=' => { if self.src[self.next] == '=' {
                            self.advance();
                            tk.push(Tok::new(TokKind::Equal, "=="))
                         } else { tk.push(Tok::new(TokKind::Assign, "=")) } }

                '+' => { if self.src[self.next] == '=' {
                            self.advance();
                            tk.push(Tok::new(TokKind::PlusAssign, "+="))
                         } else { tk.push(Tok::new(TokKind::Plus, "+")) } }

                '-' => { if self.src[self.next] == '=' {
                            self.advance();
                            tk.push(Tok::new(TokKind::MinusAssign, "-="))
                         } else { tk.push(Tok::new(TokKind::Minus, "-")) } }

                '/' => { if self.src[self.next] == '=' {
                            self.advance();
                            tk.push(Tok::new(TokKind::DivideAssign, "/="))
                         } else { tk.push(Tok::new(TokKind::Divide, "/")) } }

                '*' => { if self.src[self.next] == '=' {
                            self.advance();
                            tk.push(Tok::new(TokKind::MulAssign, "*="))
                         } else { tk.push(Tok::new(TokKind::Mul, "*")) } }

                '>' => { if self.src[self.next] == '=' {
                            self.advance();
                            tk.push(Tok::new(TokKind::GreaterEq, ">="))
                         } else { tk.push(Tok::new(TokKind::Greater, ">")) } }
                '<' => { if self.src[self.next] == '=' {
                            self.advance();
                            tk.push(Tok::new(TokKind::SmallerEq, "<="))
                         } else { tk.push(Tok::new(TokKind::Smaller, "<")) } }


                '%' => { if self.src[self.next] == '=' {
                            self.advance();
                            tk.push(Tok::new(TokKind::ModAssign, "%="))
                         } else { tk.push(Tok::new(TokKind::Mod, "%")) } }


                '(' => tk.push(Tok::new(TokKind::LParen, "(")),
                ')' => tk.push(Tok::new(TokKind::RParen, ")")),
                '[' => tk.push(Tok::new(TokKind::LSqParen, "[")),
                ']' => tk.push(Tok::new(TokKind::RSqParen, "]")),
                '{' => tk.push(Tok::new(TokKind::LBrace, "{")),
                '}' => tk.push(Tok::new(TokKind::RBrace, "}")),
                ':' => tk.push(Tok::new(TokKind::Colon, ":")),
                ',' => tk.push(Tok::new(TokKind::Comma, ",")),
                '_' => tk.push(Tok::new(TokKind::Placeholder, "_")),

                '!' => { if self.src[self.next] == '=' {
                            self.advance();
                            tk.push(Tok::new(TokKind::NotEq, "!="))
                         } else { tk.push(Tok::new(TokKind::Not, "!")) } }

                '|' => { if self.src[self.next] == '|' {
                            self.advance();
                            tk.push(Tok::new(TokKind::DoublePipe, "||"))
                         } else { tk.push(Tok::new(TokKind::Pipe, "|")) } }
                
                '&' => { if self.src[self.next] == '&' {
                            self.advance();
                            tk.push(Tok::new(TokKind::DoubleAnd, "&&"))
                         } else { tk.push(Tok::new(TokKind::And, "&")) } }

                /* Lex alphabet */
                _ if self.src[self.pos].is_alphabetic() => {
                    while self.src.len()-1 > self.pos && self.src[self.pos].is_alphabetic() {
                        buf.push(self.src[self.pos]);
                        self.advance();
                    }
                    
                    match &buf[..] {
                        "fn" => tk.push(Tok::new(TokKind::Function, "fn")),
                        "var" => tk.push(Tok::new(TokKind::Variable, "var")),
                        "val" => tk.push(Tok::new(TokKind::Value, "val")),
                        "const" => tk.push(Tok::new(TokKind::Constant, "const")),
                        "struct" => tk.push(Tok::new(TokKind::Structure, "struct")),
                        "enum" => tk.push(Tok::new(TokKind::Enumerator, "enum")),
                        
                        "if" => tk.push(Tok::new(TokKind::If, "if")), 
                        "else" => tk.push(Tok::new(TokKind::Else, "else")),
                        "switch" => tk.push(Tok::new(TokKind::Switch, "switch")),
                        "break" => tk.push(Tok::new(TokKind::Break, "break")),
                        "default" => tk.push(Tok::new(TokKind::Default, "default")),
                        "for" => tk.push(Tok::new(TokKind::For, "for")),
                        "while" => tk.push(Tok::new(TokKind::While, "while")),
                        "loop" => tk.push(Tok::new(TokKind::Loop, "loop")),
                        "return" => tk.push(Tok::new(TokKind::Return, "return")),
                        
                        _ =>  tk.push(Tok::new(TokKind::Identifier, &buf)),
                    }
                    
                    self.pos -= 1;
                    self.next -= 1;
                }
                
                /* Lex numbers */
                _ if self.src[self.pos].is_numeric() => {
                    while self.src.len()-1 > self.pos && self.src[self.pos].is_numeric() {
                        buf.push(self.src[self.pos]);
                        self.advance();
                    }
                    
                    self.pos -= 1;
                    self.next -= 1;
                    tk.push(Tok::new(TokKind::Number, &buf))
                }
              

                _ => (),
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
