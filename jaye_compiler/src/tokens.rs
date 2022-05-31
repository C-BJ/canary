#[derive(Debug, PartialEq, Clone)]
pub enum TokKind {
    NewLine,      // Use for identifing new lines in code
    Space,        // Use for identifing spaces in code

    Identifier,   // [name]
    Number,       // float, int, uint, double
    String,       // string
    Boolean,      // bool
    Function,     // fn

    Structure,    // struct
    Enumerator,   // enum
    Constant,     // const
    Variable,     // var
    Value,        // val

    If,           // if
    Else,         // else
    Switch,       // switch
    Break,        // break
    Default,      // default
    For,          // for
    While,        // while
    Loop,         // loop
    Return,       // return

    Assign,       // = 
    Plus,         // + 
    Minus,        // - 
    Mul,          // * 
    Divide,       // /  
    Mod,          // % 
    PlusAssign,   // += 
    MinusAssign,  // -= 
    MulAssign,    // *=  
    DivideAssign, // /=  
    ModAssign,    // %=  

    LParen,       // ( 
    RParen,       // )  
    LSqParen,     // [ 
    RSqParen,     // ]  
    LBrace,       // { 
    RBrace,       // } 
    Comma,        // , 
    Colon,        // : 
    Placeholder,  // _ 

    Not,          // ! 
    Pipe,         // | 
    Greater,      // > 
    Smaller,      // < 
    And,          // & 

    DoublePipe,   // || 
    DoubleAnd,    // && 
    NotEq,        // != 
    GreaterEq,    // >= 
    SmallerEq,    // <= 
    Equal,        // == 
}

#[derive(Debug, PartialEq, Clone)]
pub struct Tok {
    pub kind: TokKind,
    pub lit: String,
}

impl Tok {
    pub fn new(k: TokKind, l: &str) -> Self {
        Self {
            kind: k,
            lit: l.to_string(),
        }
    }
}
