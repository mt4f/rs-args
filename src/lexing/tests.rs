use crate::lexing::{ self, TokenKind };

#[test]
fn test_lexer_identifier() {
    let input = "asdf";
    let tokens = lexing::tokenise(String::from(input)).unwrap();

    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::Identifier);
    assert_eq!(tokens[0].value, String::from("asdf"));
}

#[test]
fn test_lexer_number() {
    let input = "1234";
    let tokens = lexing::tokenise(String::from(input)).unwrap();

    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::Number);
    assert_eq!(tokens[0].value, String::from("1234"));
}

#[test]
fn test_lexer_string() {
    let input = "\"Hello, World!\"";
    let tokens = lexing::tokenise(String::from(input)).unwrap();

    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::String);
    assert_eq!(tokens[0].value, String::from("Hello, World!"));
}

#[test]
fn test_lexer_multiple() {
    let input = "1234 asdf \"Hello, World!\"";
    let tokens = lexing::tokenise(String::from(input)).unwrap();

    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0].kind, TokenKind::Number);
    assert_eq!(tokens[0].value, String::from("1234"));
    assert_eq!(tokens[1].kind, TokenKind::Identifier);
    assert_eq!(tokens[1].value, String::from("asdf"));
    assert_eq!(tokens[2].kind, TokenKind::String);
    assert_eq!(tokens[2].value, String::from("Hello, World!"));
}

#[test]
#[should_panic(expected = "called `Result::unwrap()` on an `Err` value: InvalidDecimalPoint(2)")]
fn test_lexer_invalid_decimal() {
    let input = "1234.567.890";
    let tokens = lexing::tokenise(String::from(input)).unwrap();

    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::Number);
    assert_eq!(tokens[0].value, String::from("1234.567.890"));
}

#[test]
#[should_panic(expected = "called `Result::unwrap()` on an `Err` value: InvalidDecimalPoint(2)")]
fn test_lexer_invalid_decimal_multiple() {
    let input = "1234.567.890 asdf";
    let tokens = lexing::tokenise(String::from(input)).unwrap();

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].kind, TokenKind::Number);
    assert_eq!(tokens[0].value, String::from("1234.567.890"));
    assert_eq!(tokens[1].kind, TokenKind::Identifier);
    assert_eq!(tokens[1].value, String::from("asdf"));
}
