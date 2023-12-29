use self::errors::LexicalError;

pub mod errors;

/// A token is a single unit of a command, such as a word, number or symbol.
/// This is used to convert single characters in a more machine-readable format.
/// For example, the string "ls -l" would be converted to a list of tokens like
/// so: ["ls", "-l"].
///
/// The token kind is a simple enum, which is used to distinguish between the
/// different types of tokens. The token value is a string, which contains the
/// actual value of the token. The token position is a simple integer, which
/// contains the position of the token in the input string. This is used for
/// error reporting.
///
#[derive(Debug)]
#[allow(dead_code)] // TODO: Remove one the parser is implemented
pub struct Token {
    kind: TokenKind,
    value: String,
}

impl Token {
    pub fn get_kind(&self) -> &TokenKind {
        &self.kind
    }
    pub fn get_value(&self) -> &String {
        &self.value
    }
    pub fn unknown(c: &char) -> Token {
        Token { kind: TokenKind::Unknown, value: c.to_string() }
    }
}

/// As we are only implementing a very simple shell, we only need a few token
/// kinds. These are the token kinds we are going to use:
/// 1. Identifier: A word, such as "ls" or "echo".
/// 2. String: A string, such as "Hello, world!".
/// 3. Number: A number, such as "123".
/// 4. Equals: The equals sign, used for key-value pairs.
/// 5. Dash: The dash sign, used for flags.
#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind {
    Identifier,
    String,
    Number,
    Equals,
    Dash,
    Unknown,
}

pub fn tokenise(input: String) -> Result<Vec<Token>, LexicalError> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut chars = input.chars().collect::<Vec<char>>();

    while let Some(c) = chars.first() {
        let result = match c {
            '-' => Ok(Token { kind: TokenKind::Dash, value: chars.remove(0).to_string() }),
            '=' => Ok(Token { kind: TokenKind::Equals, value: chars.remove(0).to_string() }),
            ' ' => {
                chars.remove(0);
                continue;
            }
            _ => {
                if c.is_numeric() {
                    tokenise_number(&mut chars)
                } else if c.is_alphabetic() {
                    Ok(tokenise_identifier(&mut chars))
                } else if *c == '"' {
                    Ok(tokenise_string(&mut chars))
                } else {
                    Ok(Token::unknown(c))
                }
            }
        };

        if let Ok(value) = &result {
            if let TokenKind::Unknown = value.get_kind() {
                return Err(LexicalError::UnknownToken(String::from(value.get_value())));
            }
        } else {
            return Err(result.err().unwrap());
        }

        tokens.push(result.unwrap());
    }
    Ok(tokens)
}

fn tokenise_identifier(chars: &mut Vec<char>) -> Token {
    let mut value = String::new();

    while let Some(c) = chars.first() {
        if c.is_alphanumeric() {
            value.push(chars.remove(0));
        } else {
            break;
        }
    }

    Token { kind: TokenKind::Identifier, value }
}
fn tokenise_number(chars: &mut Vec<char>) -> Result<Token, LexicalError> {
    let mut value = String::new();

    let mut decimals = 0;
    while let Some(c) = chars.first() {
        if c.is_numeric() || *c == '.' {
            if *c == '.' {
                decimals += 1;
            }
            value.push(chars.remove(0));
        } else {
            break;
        }
    }

    if decimals > 1 {
        return Err(LexicalError::InvalidDecimalPoint(decimals));
    }
    Ok(Token { kind: TokenKind::Number, value })
}

fn tokenise_string(chars: &mut Vec<char>) -> Token {
    chars.remove(0);
    let mut value = String::new();

    while let Some(c) = chars.first() {
        if *c == '"' {
            chars.remove(0);
            break;
        } else {
            value.push(chars.remove(0));
        }
    }

    Token { kind: TokenKind::String, value }
}

#[cfg(test)]
pub mod tests;
