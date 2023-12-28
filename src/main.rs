use rs_args::lexing;

fn main() {
    let input = "";
    let tokens = lexing::tokenise(String::from(input));

    for token in tokens {
        println!("{:?}", token);
    }
}
