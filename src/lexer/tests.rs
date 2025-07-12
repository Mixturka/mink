use super::*;

#[test]
fn tokenize_left_paren() {
    let mut lexer = Lexer::new();
    let tokens = lexer.tokenize("(").unwrap();
    assert_eq!(tokens, &[SpannedToken::new(Token::LeftParen, 0, 0, 0)])
}

#[test]
fn tokenize_right_paren() {
    let mut lexer = Lexer::new();
    let tokens = lexer.tokenize(")").unwrap();
    assert_eq!(tokens, &[SpannedToken::new(Token::RightParen, 0, 0, 0)])
}

#[test]
fn tokenize_colon() {
    let mut lexer = Lexer::new();
    let tokens = lexer.tokenize(":").unwrap();
    assert_eq!(tokens, &[SpannedToken::new(Token::Colon, 0, 0, 0)])
}

#[test]
fn tokenize_identifier() {
    let mut lexer = Lexer::new();
    let tokens = lexer.tokenize("random_ident123").unwrap();
    assert_eq!(
        tokens,
        &[SpannedToken::new(
            Token::Identifier("random_ident123".into()),
            0,
            0,
            14
        )]
    )
}

#[test]
fn tokenize_left_curly_brace() {
    let mut lexer = Lexer::new();
    let tokens = lexer.tokenize("{").unwrap();
    assert_eq!(tokens, &[SpannedToken::new(Token::LeftCurlyBrace, 0, 0, 0)])
}

#[test]
fn tokenize_right_curly_brace() {
    let mut lexer = Lexer::new();
    let tokens = lexer.tokenize("}").unwrap();
    assert_eq!(
        tokens,
        &[SpannedToken::new(Token::RightCurlyBrace, 0, 0, 0)]
    )
}

#[test]
fn tokenize_positive_integer() {
    let mut lexer = Lexer::new();
    let tokens = lexer.tokenize("123").unwrap();
    assert_eq!(tokens, &[SpannedToken::new(Token::Integer(123), 0, 0, 2)])
}

#[test]
fn tokenize_fn_keyword() {
    let mut lexer = Lexer::new();
    let tokens = lexer.tokenize("fn").unwrap();
    assert_eq!(tokens, &[SpannedToken::new(Token::Fn, 0, 0, 1)])
}

#[test]
fn tokenize_return_keyword() {
    let mut lexer = Lexer::new();
    let tokens = lexer.tokenize("return").unwrap();
    assert_eq!(tokens, &[SpannedToken::new(Token::Return, 0, 0, 5)])
}

#[test]
fn tokenize_void_keyword() {
    let mut lexer = Lexer::new();
    let tokens = lexer.tokenize("void").unwrap();
    assert_eq!(tokens, &[SpannedToken::new(Token::Void, 0, 0, 3)])
}

#[test]
fn tokenize_i32_keyword() {
    let mut lexer = Lexer::new();
    let tokens = lexer.tokenize("i32").unwrap();
    assert_eq!(tokens, &[SpannedToken::new(Token::I32, 0, 0, 2)])
}
