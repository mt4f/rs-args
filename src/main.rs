fn main() {
    let input = "1.2";
    let tokens = rs_args::lexing::tokenise(String::from(input));
    if tokens.is_err() {
        panic!("Error during lexing: {:?}", tokens.err().unwrap());
    }
    let arguments = rs_args::parsing::parse(tokens.unwrap());

    for arg in arguments.unwrap() {
        println!("{:?}", arg);
    }
}
