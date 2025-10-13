use rust_bs::prelude::*;

fn main() {
    use std::fs;
    let contents = fs::read_to_string("./buildsys/test.rbs").unwrap();
    
    println!("{:#?}", BuildParser::validate(BuildParser::parse(BuildParser::lex(&contents))));
}
