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

pub(crate) fn keyword(text: &str, pos: Pos) -> Option<Token> {
    match text {
        "and" => Some(Token::And(pos)),
        "class" => Some(Token::Class(pos)),
        "else" => Some(Token::Else(pos)),
        "false" => Some(Token::False(pos)),
        "for" => Some(Token::For(pos)),
        "fun" => Some(Token::Fun(pos)),
        "if" => Some(Token::If(pos)),
        "nil" => Some(Token::Nil(pos)),
        "or" => Some(Token::Or(pos)),
        "print" => Some(Token::Print(pos)),
        "return" => Some(Token::Return(pos)),
        "super" => Some(Token::Super(pos)),
        "this" => Some(Token::This(pos)),
        "true" => Some(Token::True(pos)),
        "var" => Some(Token::Var(pos)),
        "while" => Some(Token::While(pos)),
        _ => None,
    }
}
