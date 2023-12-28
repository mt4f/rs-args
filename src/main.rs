use rs_args::lexing;

fn main() {
    let input = "1223.4556 asdf \"Hello, World!\"";
    let tokens = lexing::tokenise(String::from(input));

    for token in tokens {
        println!("{:?}", token);
    }
}
