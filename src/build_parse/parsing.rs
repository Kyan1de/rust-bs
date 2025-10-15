use crate::prelude::*;
use std::iter::Peekable;

impl BuildParser {

    /// takes a vector of token strings and constructs an AST
    pub fn parse(input: &Vec<BSToken>) -> BSAst {
        BSAst::Prog(Self::parse_lines(&mut input.split(|t|*t==BSToken::Break)))
    }

    /// parses lines of tokens
    fn parse_lines(global_toks: &mut (dyn Iterator<Item = &[BSToken]> + '_)) -> Vec<BSAst> {
        
        let mut parsed = vec![];

        loop {
            let Some(statement) = global_toks.next() else {return parsed;};
            parsed.push(Self::parse_part(&mut statement.iter().peekable(), global_toks));
        }
    }

    /// parses a part of the tokens
    fn parse_part<'a>(statement: &mut Peekable<impl Iterator<Item = &'a BSToken>>, global_toks: &mut (dyn Iterator<Item = &[BSToken]> + '_)) -> BSAst {
        match statement.peek() {
            Some(BSToken::Word(t)) if t.eq("batch") => {
                statement.next();
                let mut inner = vec![];
                loop {
                    match global_toks.next() {
                        Some([BSToken::Word(s)]) if s.eq("end") => {break},
                        Some([BSToken::Word(s)]) if s.eq("batch") => panic!("syntax error"),
                        Some([tokens @ ..]) => inner.append(&mut Vec::from(tokens)),
                        None => panic!("syntax error"),
                    }
                }
                BSAst::Batch(Self::parse_lines(&mut inner.split(|t| *t==BSToken::Break)))
            },
            Some(BSToken::Word(t)) if t.eq("set") => {
                statement.next();
                let BSToken::Word(iden) = statement.next().expect("Syntax Error") else {panic!("Syntax Error")};
                if statement.next().expect("Syntax Error").ne(&BSToken::Operator("=".to_string())) {panic!("Syntax Error")}
                BSAst::SetVar(Box::new(BSAst::Ident((*iden).clone())), Box::new(Self::parse_part(statement, global_toks)))
            },
            Some(BSToken::Word(t)) if t.eq("gen") => {
                statement.next();
                let mut args = vec![];
                let tail = statement.collect::<Vec<&BSToken>>();
                let tail = tail.split(|t| t.eq(&&BSToken::Operator(",".to_string())));
                for term in tail {
                    args.push(match term {
                        [BSToken::Operator(o), tail @ ..] if o=="*" => {
                            let mut inner = vec![];
                            for e in tail {inner.push((**e).clone());} 
                            BSAst::Unpack(Box::new(Self::parse_part(&mut inner.iter().peekable(), global_toks)))
                        },
                        [tail @ ..] => {
                            let mut inner = vec![];
                            for e in tail {inner.push((**e).clone());} 
                            Self::parse_part(&mut inner.iter().peekable(), global_toks)
                        },
                    });
                }
                BSAst::Generate(args)
            },
            Some(BSToken::Operator(t)) if t.eq("[") => {
                statement.next();
                let mut content = vec![];
                let tail = statement.collect::<Vec<&BSToken>>();
                let tail = tail.split(|t| t.eq(&&BSToken::Operator(",".to_string())));
                for term in tail {
                    let mut inner = vec![];
                    for e in term {inner.push((**e).clone());} 
                    content.push(Self::parse_part(&mut inner.iter().peekable(), global_toks))
                }
                BSAst::Arr(content)
            },
            Some(_) => {
                Self::parse_expr(statement)
            },
            _ => BSAst::None
        }
    }

    /// parses a math expression
    fn parse_expr<'a>(expr: &mut Peekable<impl Iterator<Item = &'a BSToken>>) -> BSAst {
        Self::parse_add_expr(expr)
    }

    fn parse_add_expr<'a>(expr: &mut Peekable<impl Iterator<Item = &'a BSToken>>) -> BSAst {
        let mut res = Self::parse_mul_expr(expr);
        loop {
            match expr.peek() {
                Some(BSToken::Operator(o)) if o == "+" => {
                    expr.next();
                    res = BSAst::ExprAdd(Box::new(res), Box::new(Self::parse_mul_expr(expr)), false);
                },
                Some(BSToken::Operator(o)) if o == "-" => {
                    expr.next();
                    res = BSAst::ExprAdd(Box::new(res), Box::new(Self::parse_mul_expr(expr)), true);
                },
                _ => break res,
            }
        }
    }

    fn parse_mul_expr<'a>(expr: &mut Peekable<impl Iterator<Item = &'a BSToken>>) -> BSAst {
        let mut res = Self::parse_term_expr(expr);
        loop {
            match expr.peek() {
                Some(BSToken::Operator(o)) if o == "*" => {
                    expr.next();
                    res = BSAst::ExprMul(Box::new(res), Box::new(Self::parse_mul_expr(expr)), false);
                },
                Some(BSToken::Operator(o)) if o == "/" => {
                    expr.next();
                    res = BSAst::ExprMul(Box::new(res), Box::new(Self::parse_mul_expr(expr)), true);
                },
                _ => break res,
            }
        }
    }

    fn parse_term_expr<'a>(expr: &mut Peekable<impl Iterator<Item = &'a BSToken>>) -> BSAst {
        match expr.peek() {
            Some(BSToken::Operator(o)) if o == "(" => {
                expr.next();
                let res = Self::parse_add_expr(expr);
                if expr.next().ne(&Some(&BSToken::Operator(")".to_string()))) {panic!("Syntax error!")}
                res
            },
            Some(BSToken::Numeric(s)) => {
                expr.next();
                BSAst::Num((*s).clone())
            },
            Some(BSToken::String(s)) => {
                expr.next();
                BSAst::Str((*s).clone())
            },
            Some(BSToken::Word(s)) if !["gen", "set", "batch", "end"].contains(&(s.as_str())) => {
                expr.next();
                BSAst::Ident((*s).clone())
            },
            _ => BSAst::None
        }
        
    }

}