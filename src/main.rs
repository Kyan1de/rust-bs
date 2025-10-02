use rust_bs::*;

fn main() {
    use std::fs;
    let contents = fs::read_to_string("./buildsys/test.rbs").unwrap();
    
    println!("{:#?}", BuildParser::parse(BuildParser::lex(&contents)));
}
