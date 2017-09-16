
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Literals
    StrLiteral,
    IntLiteral,
    DecLiteral,
    RangeLiteral,
    VecLiteral,
    BoolLiteral,
    SExpr,
    Comment,
    CommentDoc,

    // Identificatives
    Ident,
    ClassLiteral,

    // Special symbols
    Comma,      // `,`
    Smc,        // `;`
    Dot,        // `.`
    Dbc,        // `:`
    Arrow,      // `->`
    Equal,      // `=`
    EqualIs,    // `=?`
    EqualEsc,   // `=!`
    Plus,       // `+`
    Dash,       // `-`
    Star,       // `*`
    Slash,      // `/`
    Esc,        // `!`
}
