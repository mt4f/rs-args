// TODO: Add documentation to all functions and structs.
// TODO: Add the ability to give a custom input to the `parse_arguments` function ( and child functions ).

use lexing::errors::LexicalError;
use parsing::Argument;

pub mod lexing;
pub mod parsing;
pub mod tests;

#[deprecated(since = "0.3.0", note = "Use standalone functions in `rs_args`.")]
pub struct CommandArguments {
    arguments: Vec<parsing::Argument>,
}

#[allow(deprecated)]
impl CommandArguments {
    pub fn new(arguments: Vec<parsing::Argument>) -> CommandArguments {
        CommandArguments { arguments }
    }

    pub fn from_input(input: &str) -> Result<CommandArguments, LexicalError> {
        let tokens = lexing::tokenise(String::from(input));
        let arguments = parsing::parse(tokens?);
        Ok(CommandArguments::new(arguments))
    }

    pub fn default_new() -> Result<CommandArguments, LexicalError> {
        let mut raw_arguments = std::env::args().collect::<Vec<String>>();
        let raw_arguments = raw_arguments
            .drain(1..)
            .collect::<Vec<String>>()
            .join(" ");
        let tokens = lexing::tokenise(raw_arguments);
        let arguments = parsing::parse(tokens?);
        Ok(CommandArguments::new(arguments))
    }

    #[deprecated(since = "0.3.0", note = "Use `rs_args::get_named` instead.")]
    pub fn get_named(&self, name: &str) -> Option<&str> {
        self.arguments
            .iter()
            .find(
                |it|
                    it.get_name().is_some() &&
                    it.get_name().unwrap() == name &&
                    it.get_value().is_some()
            )
            .map(|it| it.get_value().unwrap())
    }

    #[deprecated(since = "0.3.0", note = "Use `rs_args::get_positional` instead.")]
    pub fn get_positional(&self, position: usize) -> Option<&str> {
        self.arguments
            .iter()
            .find(|it| it.get_position().is_some() && it.get_position().unwrap() == position)
            .map(|it| it.get_value().unwrap())
    }

    #[deprecated(since = "0.3.0", note = "Use `rs_args::get_positional_count` instead.")]
    pub fn get_positional_count(&self) -> usize {
        self.arguments
            .iter()
            .filter(|it| it.get_position().is_some())
            .count()
    }

    #[deprecated(since = "0.3.0", note = "Use `rs_args::get_positional_values` instead.")]
    pub fn get_positional_values(&self) -> Vec<&str> {
        self.arguments
            .iter()
            .filter(|it| it.get_position().is_some())
            .map(|it| it.get_value().unwrap())
            .collect()
    }

    #[deprecated(since = "0.3.0", note = "Use `rs_args::get_named_count` instead.")]
    pub fn get_named_count(&self) -> usize {
        self.arguments
            .iter()
            .filter(|it| it.get_name().is_some())
            .count()
    }

    #[deprecated(since = "0.3.0", note = "Use `rs_args::get_named_values` instead.")]
    pub fn get_named_values(&self) -> Vec<&str> {
        self.arguments
            .iter()
            .filter(|it| it.get_name().is_some())
            .map(|it| it.get_value().unwrap())
            .collect()
    }

    #[deprecated(since = "0.3.0", note = "Use `rs_args::parse_arguments` instead.")]
    pub fn get_all(&self) -> Vec<Argument> {
        self.arguments.to_vec()
    }

    #[deprecated(since = "0.3.0", note = "Use `rs_args::get_flag_names` instead.")]
    pub fn get_flag_names(&self) -> Vec<&str> {
        self.arguments
            .iter()
            .filter(|it| it.get_name().is_some() && it.get_value().is_none())
            .map(|it| it.get_name().unwrap())
            .collect()
    }

    #[deprecated(since = "0.3.0", note = "Use `rs_args::get_flags` instead.")]
    pub fn get_flags(&self) -> Vec<Argument> {
        self.arguments
            .iter()
            .filter(|it| it.get_name().is_some() && it.get_value().is_none())
            .cloned()
            .collect()
    }

    #[deprecated(since = "0.3.0", note = "Use `rs_args::get_flag_names` instead.")]
    pub fn has_flag(&self, name: &str) -> bool {
        self.arguments
            .iter()
            .any(
                |it|
                    it.get_name().is_some() &&
                    it.get_name().unwrap() == name &&
                    it.get_value().is_none()
            )
    }

    #[deprecated(since = "0.3.0", note = "Use `rs_args::has_named` instead.")]
    pub fn has_named(&self, name: &str) -> bool {
        self.arguments
            .iter()
            .any(
                |it|
                    it.get_name().is_some() &&
                    it.get_name().unwrap() == name &&
                    it.get_value().is_some()
            )
    }

    #[deprecated(since = "0.3.0", note = "Use `rs_args::has_positional` instead.")]
    pub fn has_positional(&self, position: usize) -> bool {
        self.arguments
            .iter()
            .any(|it| it.get_position().is_some() && it.get_position().unwrap() == position)
    }
}

pub fn parse_arguments() -> Vec<Argument> {
    let mut raw_arguments = std::env::args().collect::<Vec<String>>();
    let raw_arguments = raw_arguments
        .drain(1..)
        .collect::<Vec<String>>()
        .join(" ");
    let tokens = lexing::tokenise(raw_arguments);
    parsing::parse(tokens.unwrap())
}

pub fn get_named(name: &str) -> Option<String> {
    parse_arguments()
        .iter()
        .find(
            |it|
                it.get_name().is_some() &&
                it.get_name().unwrap() == name &&
                it.get_value().is_some()
        )
        .map(|it| it.get_value().unwrap().to_owned())
}

pub fn get_positional(position: usize) -> Option<String> {
    parse_arguments()
        .iter()
        .find(|it| it.get_position().is_some() && it.get_position().unwrap() == position)
        .map(|it| it.get_value().unwrap().to_owned())
}

pub fn get_positional_count() -> usize {
    parse_arguments()
        .iter()
        .filter(|it| it.get_position().is_some())
        .count()
}

pub fn get_positional_values() -> Vec<String> {
    parse_arguments()
        .iter()
        .filter(|it| it.get_position().is_some())
        .map(|it| it.get_value().unwrap().to_owned())
        .collect()
}

pub fn get_named_count() -> usize {
    parse_arguments()
        .iter()
        .filter(|it| it.get_name().is_some())
        .count()
}

pub fn get_named_values() -> Vec<String> {
    parse_arguments()
        .iter()
        .filter(|it| it.get_name().is_some())
        .map(|it| it.get_value().unwrap().to_owned())
        .collect()
}

pub fn get_flag_names() -> Vec<String> {
    parse_arguments()
        .iter()
        .filter(|it| it.get_name().is_some() && it.get_value().is_none())
        .map(|it| it.get_name().unwrap().to_owned())
        .collect()
}

pub fn get_flags() -> Vec<Argument> {
    parse_arguments()
        .iter()
        .filter(|it| it.get_name().is_some() && it.get_value().is_none())
        .cloned()
        .collect()
}

pub fn has_flag(name: &str) -> bool {
    parse_arguments()
        .iter()
        .any(
            |it|
                it.get_name().is_some() &&
                it.get_name().unwrap() == name &&
                it.get_value().is_none()
        )
}

pub fn has_named(name: &str) -> bool {
    parse_arguments()
        .iter()
        .any(
            |it|
                it.get_name().is_some() &&
                it.get_name().unwrap() == name &&
                it.get_value().is_some()
        )
}

pub fn has_positional(position: usize) -> bool {
    parse_arguments()
        .iter()
        .any(|it| it.get_position().is_some() && it.get_position().unwrap() == position)
}
