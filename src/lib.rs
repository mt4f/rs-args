use lexing::errors::LexicalError;
use parsing::Argument;

pub mod lexing;
pub mod parsing;
pub mod tests;

pub struct CommandArguments {
    arguments: Vec<parsing::Argument>,
}

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

    pub fn get_positional(&self, position: usize) -> Option<&str> {
        self.arguments
            .iter()
            .find(|it| it.get_position().is_some() && it.get_position().unwrap() == position)
            .map(|it| it.get_value().unwrap())
    }

    pub fn get_positional_count(&self) -> usize {
        self.arguments
            .iter()
            .filter(|it| it.get_position().is_some())
            .count()
    }

    pub fn get_positional_values(&self) -> Vec<&str> {
        self.arguments
            .iter()
            .filter(|it| it.get_position().is_some())
            .map(|it| it.get_value().unwrap())
            .collect()
    }

    pub fn get_named_count(&self) -> usize {
        self.arguments
            .iter()
            .filter(|it| it.get_name().is_some())
            .count()
    }

    pub fn get_named_values(&self) -> Vec<&str> {
        self.arguments
            .iter()
            .filter(|it| it.get_name().is_some())
            .map(|it| it.get_value().unwrap())
            .collect()
    }

    pub fn get_all(&self) -> Vec<Argument> {
        self.arguments.to_vec()
    }

    pub fn get_flag_names(&self) -> Vec<&str> {
        self.arguments
            .iter()
            .filter(|it| it.get_name().is_some() && it.get_value().is_none())
            .map(|it| it.get_name().unwrap())
            .collect()
    }

    pub fn get_flags(&self) -> Vec<Argument> {
        self.arguments
            .iter()
            .filter(|it| it.get_name().is_some() && it.get_value().is_none())
            .cloned()
            .collect()
    }

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

    pub fn has_positional(&self, position: usize) -> bool {
        self.arguments
            .iter()
            .any(|it| it.get_position().is_some() && it.get_position().unwrap() == position)
    }
}
