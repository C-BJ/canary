#[derive(Debug, PartialEq, Clone)]
pub enum TokKind {
    
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
    Boolean,      // bool

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
    Loop,         // Loop
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
    MODAssign,    // %=

    LParen,       // (
    RParen,       // )
    LSqParen,     // [
    RSqParen,     // ]
    LBrace,       // {
    RBrace,       // }
    Comma,        // ,
    Semicolon,    // ;

    Not,          // !
    Pipe,         // |
    Greater,      // >
    Smaller,      // <
    And,          // &
    Power,        // ^
    Reverse,      // ~
    ShiftLeft,    // <<
    ShiftRight,   // >>



    DoublePipe,   // ||
    DoubleAnd,    // &&
    PowerEq,      // ^=
    ShiftLeftEq,  // <<=
    ShiftRightEq, // >>=
    NotEq,        // !=
    GreaterEq,    // >=
    SmallerEq,    // <=
    Equal,        // ==
}

#[derive(Debug, PartialEq, Clone)]
pub struct Tok {
    pub kind: TokKind,
    pub literal: String,
}

impl Tok {
    pub fn new(k: TokKind, l: &str) -> Self {
        Self {
            kind: k,
            literal: l.to_string(),
        }
    }
}
