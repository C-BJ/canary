use crate::statements::Statement;
use crate::tokens::*;

pub struct Parser {
    statement: Statement,
    identifier: String,
    expression: Vec<Tok>,
}

impl Parser {
    pub fn new(s: Statement, i: String, e: Vec<Tok>) -> Self {
        Self {
            statement: s,
            identifier: i,
            expression: e,
        }
    }

    pub fn parse(mut tok_vec: Vec<Tok>) {
        for i in 0..tok_vec.len() {
            let mut buf = Vec::<Tok>::new();

            for i in 0..tok_vec.len() {
                if tok_vec[i].kind == TokKind::NEWLINE {
                    break
                }

                buf.push(tok_vec[i].clone());
            }

            for i in 0..buf.len()+1 {
                if buf.len() == 0 || tok_vec.len() == 0 {
                    break
                }
                tok_vec.remove(0);
            }

            /* Check if it's a function */
            if buf.len() >= 8 && buf[buf.len()-3].kind == TokKind::RParen &&
            buf[4].kind == TokKind::LParen {
                Self::parse_function(buf.clone());    
            }

            /* Check if it's a variable */
            if buf.len() >= 6 && buf[0].kind == TokKind::Variable || buf[0].kind == TokKind::Value || buf[0].kind == TokKind::Constant && buf[4].kind == TokKind::Assign {
                Self::parse_variable(buf.clone());
            }

            println!("PARSER ::: {:?}", buf);
        }
    }


    /* 
     * Parsing functions
     */

    /* Parse functions */
    pub fn parse_function(buf: Vec<Tok>) {
        println!("FUNCTION");   
    }

    pub fn parse_variable(buf: Vec<Tok>) {
        println!("VARIABLE");
    }

}
