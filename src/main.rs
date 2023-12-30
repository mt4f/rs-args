fn main() {
    let args = rs_args::parse_arguments();
    for arg in args {
        println!("{:?}", arg);
    }
}
