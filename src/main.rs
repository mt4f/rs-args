fn main() {
    let args = rs_args::CommandArguments::default_new();
    for arg in args.unwrap().get_all() {
        println!("{:?}", arg);
    }
}
