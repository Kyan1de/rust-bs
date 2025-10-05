use std::collections::HashMap;

/// represents the value that a variable holds
pub enum VarVal {
    String(String),
    NumberF(f64),
    NumberI(i64),
    Arr(Vec<VarVal>)
}

/// lookup table for variables
pub type VarTable = HashMap<String, VarVal>;

/// generates BuildSys structs from .rbs files 
#[derive(Debug)]
pub struct BuildParser;

// used to construct the AST for the parser
#[derive(Debug)]
pub enum BSAst {
    Prog(Vec<BSAst>), // root node, program
    Ident(String), // identifier
    Batch(Vec<BSAst>), // batch of commands to run at once, once generated
    
    // literals
    Num(String), // number literal
    Str(String), // string literal
    Arr(Vec<BSAst>), // array literal
    
    // operations
    SetVar(Box<BSAst>, Box<BSAst>), // set <Iden> = <Ident||Num||Str||Expr||Generate>
    Generate(Vec<BSAst>), // gen <((Iden )||(*Iden ))*>
    Unpack(Box<BSAst>), // *iden from the above, unpacks an array
    None,

    ExprAdd(Box<BSAst>, Box<BSAst>, bool), // <ExprAdd> <+||-> <ExprMul> || <ExprMul> (bool is to do the inverse op)
    ExprMul(Box<BSAst>, Box<BSAst>, bool), // <ExprMul> <*||/> <ExprTerm> || <ExprTerm> (bool is to do the inverse op)
}

/// used to make tokens
#[derive(Debug, PartialEq, Clone)]
pub enum BSToken {
    Operator(String), // single character things
    String(String), // string literals
    Numeric(String), // numeric literals
    Word(String), // any other thing (keywords, identifiers)
    Break, // break between lines, basically a \n
}

mod lexing;
mod parsing;
mod eval;