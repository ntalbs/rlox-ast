#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) struct Pos {
    pub line: usize,
    pub col: usize,
}

#[derive(Debug, PartialEq)]
pub(crate) enum Token {
    LeftParen(Pos),
    RightParen(Pos),
    LeftBrace(Pos),
    RightBrace(Pos),
    Comma(Pos),
    Dot(Pos),
    Minus(Pos),
    Plus(Pos),
    Semicolon(Pos),
    Slash(Pos),
    Star(Pos),

    Bang(Pos),
    BangEqual(Pos),
    Equal(Pos),
    EqualEqual(Pos),
    Greater(Pos),
    GreaterEqual(Pos),
    Less(Pos),
    LessEqual(Pos),

    Identifier { lexeme: String, pos: Pos },
    String { lexeme: String, pos: Pos },
    Number { lexeme: String, val: f64, pos: Pos },
    And(Pos),
    Class(Pos),
    Else(Pos),
    False(Pos),
    Fun(Pos),
    For(Pos),
    If(Pos),
    Nil(Pos),
    Or(Pos),
    Print(Pos),
    Return(Pos),
    Super(Pos),
    This(Pos),
    True(Pos),
    Var(Pos),
    While(Pos),

    Eof,
}
