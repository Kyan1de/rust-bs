use crate::prelude::*;
use regex::Regex;

impl BuildParser {
    /// takes a string input and splits it into token strings
    pub fn lex(input: &str) -> Vec<BSToken> {
        let mut out = vec![];

        input.split("\n").for_each(|l|{
            let split: Vec<&str> = Regex::new("(\\d+\\.\\d+)|(\\\".*?\\\")|(#.*)|[\\+\\-\\*\\/\\=\\(\\)\\[\\]\\,]|(\\b\\S+?\\b)").unwrap()
                                   .find_iter(&l).map(|mat|{
                                       if mat.as_str().ends_with("\r") {&mat.as_str()[..(mat.len()-1)]} else {mat.as_str()}
                                   })
                                   .collect();
            split.iter().for_each(|s| {
                match s.chars().nth(0) {
                    Some('+'|'-'|'*'|'/'|'='|'('|')'|'['|']'|',') => {out.push(BSToken::Operator(s.to_string()));},
                    Some('0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9') => {out.push(BSToken::Numeric(s.to_string()));},
                    Some('"') => {out.push(BSToken::String(s.to_string()));},
                    None | Some('#') => {},
                    _ => {out.push(BSToken::Word(s.to_string()));},
                }
            });
            out.push(BSToken::Break);
        });
        
        out
    }
}