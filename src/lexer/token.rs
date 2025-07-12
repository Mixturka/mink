#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub(crate) enum Token {
    Fn,
    Identifier(Box<str>),
    LeftParen,
    RightParen,
    LeftCurlyBrace,
    RightCurlyBrace,
    Return,
    Integer(i64),
    Colon,
    Void,
    I32,
}

impl Token {}

/// start and end fields represent position of character starting from line beginning
#[derive(Debug, PartialEq)]
pub(crate) struct SpannedToken {
    pub(crate) token: Token,
    pub(crate) line: usize,
    pub(crate) start: usize,
    pub(crate) end: usize,
}

impl SpannedToken {
    pub(crate) fn new(token: Token, line: usize, start: usize, end: usize) -> SpannedToken {
        Self {
            token,
            line,
            start,
            end,
        }
    }

    pub(crate) fn new_single(token: Token, line: usize, pos: usize) -> SpannedToken {
        Self {
            token,
            line,
            start: pos,
            end: pos,
        }
    }
}
