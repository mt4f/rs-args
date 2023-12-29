pub mod errors;

#[cfg(test)]
pub mod tests;

use crate::lexing::{ Token, TokenKind };

/// Arguments can either be positional or named. Positional arguments are
/// identified by their position relative to the command name. Note that
/// named arguments are not counted towards the positional argument count.
/// Named arguments are identified by a flag, which is an identifier prefixed
/// by a dash. The flag may be followed by a value, which is separated by a
/// space. If the value is not present, the argument is considered a flag.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Argument {
    name: Option<String>,
    value: Option<String>,
    position: Option<usize>,
}

impl Argument {
    pub fn get_name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn get_value(&self) -> Option<&str> {
        self.value.as_deref()
    }

    pub fn get_position(&self) -> Option<usize> {
        self.position
    }
}

pub fn parse(mut input: Vec<Token>) -> Vec<Argument> {
    let mut arguments: Vec<Argument> = Vec::new();
    while !input.is_empty() {
        let result = parse_argument(
            &mut input,
            arguments
                .iter()
                .filter(|it| it.position.is_some())
                .count()
        );

        arguments.push(result);
    }

    arguments
}

fn parse_argument(input: &mut Vec<Token>, positional_count: usize) -> Argument {
    let token = input.get(0).unwrap();
    match token.get_kind() {
        TokenKind::Dash => {
            input.remove(0);
            parse_named_argument(input)
        }
        _ => parse_positional_argument(input, positional_count),
    }
}

fn parse_named_argument(input: &mut Vec<Token>) -> Argument {
    let key = input.remove(0);
    if input.is_empty() || *input[0].get_kind() != TokenKind::Equals {
        return Argument { name: Some(key.get_value().to_string()), value: None, position: None };
    }

    let _ = input.remove(0);

    let value = input.remove(0);

    Argument {
        name: Some(key.get_value().clone()),
        value: Some(value.get_value().clone()),
        position: None,
    }
}

fn parse_positional_argument(input: &mut Vec<Token>, count: usize) -> Argument {
    let value = input.remove(0);

    Argument {
        name: None,
        value: Some(value.get_value().clone()),
        position: Some(count),
    }
}
