/* Certain symbols removed due to this being new
 * just DM me to add them for you */

#[derive(Debug, PartialEq, Clone)]
pub enum TokKind {
    NewLine,      // Use for identifing new lines in code
    Space,        // Use for identifing spaces in code

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
pub struct Tok<'a> {
    pub kind: TokKind,
    pub lit: &'a str,
}
