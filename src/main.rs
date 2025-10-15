use rust_bs::prelude::*;

fn main() {
    use std::fs;
    let contents = fs::read_to_string("./buildsys/test.rbs").unwrap();
    let lexxed = BuildParser::lex(&contents);
    let parsed = BuildParser::parse(&lexxed);
    let is_valid = BuildParser::validate(&parsed);
    println!("{:#?}", lexxed);
    println!("{:#?}", parsed);
    println!("{:#?}", is_valid);
}
