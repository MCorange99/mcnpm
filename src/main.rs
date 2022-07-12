
#[path = "arg_parser.rs"] mod arg_parser;
#[path = "utils.rs"] mod utils;
// use std::result::Result;
fn main() {
    let options = arg_parser::parse();
    // println!("{:?}", options);
    // println!("{}", options.help);
    if options.err != "none" {
        println!("ERROR: {}", options.err);
        return;
    }
}
