pub(crate) mod token;
pub(crate) use token::Token;
pub mod tests;

use std::{
    collections::{HashMap, HashSet},
    iter::Peekable,
    str::Chars,
};

use crate::lexer::token::SpannedToken;

#[derive(Debug)]
pub(crate) enum LexerError {
    FailedToParseInteger,
    UnrecognizedLexeme,
}

pub(crate) struct Lexer {
    tokens: Vec<SpannedToken>,
    cur_char: usize,
    cur_char_at_cur_line: usize,
    line: usize,
    keywords: HashMap<&'static str, Token>,
}

#[allow(unused)]
impl Lexer {
    pub(crate) fn new() -> Lexer {
        Lexer {
            tokens: Vec::new(),
            cur_char: 0,
            cur_char_at_cur_line: 0,
            line: 0,
            keywords: HashMap::from([
                ("fn", Token::Fn),
                ("i32", Token::I32),
                ("return", Token::Return),
                ("void", Token::Void),
            ]),
        }
    }

    pub(crate) fn tokenize(&mut self, source: &str) -> Result<&[SpannedToken], LexerError> {
        let mut iter: Peekable<Chars<'_>> = source.chars().peekable();

        while let Some(ch) = &iter.next() {
            self.cur_char += 1;
            self.cur_char_at_cur_line += 1;
            self.scan_token(ch, &mut iter);
        }

        Ok(&self.tokens)
    }

    fn scan_token(&mut self, ch: &char, iter: &mut Peekable<Chars<'_>>) -> Result<(), LexerError> {
        match ch {
            '(' => self.tokens.push(SpannedToken::new_single(
                Token::LeftParen,
                self.line,
                self.cur_char_at_cur_line - 1,
            )),
            ')' => self.tokens.push(SpannedToken::new_single(
                Token::RightParen,
                self.line,
                self.cur_char_at_cur_line - 1,
            )),
            '{' => self.tokens.push(SpannedToken::new_single(
                Token::LeftCurlyBrace,
                self.line,
                self.cur_char_at_cur_line - 1,
            )),
            '}' => self.tokens.push(SpannedToken::new_single(
                Token::RightCurlyBrace,
                self.line,
                self.cur_char_at_cur_line - 1,
            )),
            ':' => self.tokens.push(SpannedToken::new_single(
                Token::Colon,
                self.line,
                self.cur_char_at_cur_line - 1,
            )),
            c if c.is_alphabetic() => {
                let identifier = self.scan_identifier(ch, iter);
                self.tokens.push(identifier);
            }
            c if c.is_numeric() || *c == '_' => {
                let number = self.scan_number(ch, iter)?;
                self.tokens.push(number);
            }
            '\n' => {
                self.line += 1;
                self.cur_char_at_cur_line = 0;
            }
            _ => return Err(LexerError::UnrecognizedLexeme),
        }
        Ok(())
    }

    fn scan_identifier(&mut self, ch: &char, iter: &mut Peekable<Chars<'_>>) -> SpannedToken {
        let identifier_start: usize = self.cur_char_at_cur_line - 1;
        let mut token = String::new();
        token.push(*ch);

        while let Some(ch) = iter.next()
            && (ch.is_alphanumeric() || ch == '_')
        {
            self.cur_char += 1;
            self.cur_char_at_cur_line += 1;
            token.push(ch);
        }

        if let Some(keyword) = self.check_keyword(&token, identifier_start) {
            keyword
        } else {
            SpannedToken::new(
                Token::Identifier(token.into_boxed_str()),
                self.line,
                identifier_start,
                self.cur_char_at_cur_line - 1,
            )
        }
    }

    fn check_keyword(&mut self, token: &str, keyword_start: usize) -> Option<SpannedToken> {
        if let Some(keyword) = self.keywords.get(token) {
            let spanned_token = SpannedToken::new(
                keyword.clone(),
                self.line,
                keyword_start,
                self.cur_char_at_cur_line - 1,
            );
            Some(spanned_token)
        } else {
            None
        }
    }

    fn scan_number(
        &mut self,
        ch: &char,
        iter: &mut Peekable<Chars<'_>>,
    ) -> Result<SpannedToken, LexerError> {
        let number_start: usize = self.cur_char_at_cur_line - 1;
        let mut token = String::new();
        token.push(*ch);

        while let Some(ch) = iter.next()
            && ch.is_numeric()
        {
            self.cur_char += 1;
            self.cur_char_at_cur_line += 1;
            token.push(ch);
        }

        Ok(SpannedToken::new(
            Token::Integer(
                token
                    .parse::<i64>()
                    .map_err(|_| LexerError::FailedToParseInteger)?,
            ),
            self.line,
            number_start,
            self.cur_char_at_cur_line - 1,
        ))
    }
}
