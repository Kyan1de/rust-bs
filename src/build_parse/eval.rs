use crate::prelude::*;

impl BuildParser {
    pub fn eval(tree: BSAst) {
        match tree {
            // root node, program
            BSAst::Prog(lines) => {},
            // identifier
            BSAst::Ident(name) => {},
            // batch of commands to run at once, once generated
            BSAst::Batch(lines) => {},
            
            // literals
            // number literal
            BSAst::Num(n) => {},
            // string literal
            BSAst::Str(s) => {},
            // array literal
            BSAst::Arr(items) => {},
            
            // operations
            // set <Iden> = <Ident||Num||Str||Expr||Generate>
            BSAst::SetVar(ident, value) => {},
            // gen <((Iden )||(*Iden ))*>
            BSAst::Generate(command) => {},
            // *iden from the above, unpacks an array
            BSAst::Unpack(value) => {},
            
            // <ExprAdd> <+||-> <ExprMul> || <ExprMul> (bool is to do the inverse op)
            BSAst::ExprAdd(arg_a, arg_b, inverse) => {},
            // <ExprMul> <*||/> <ExprTerm> || <ExprTerm> (bool is to do the inverse op)
            BSAst::ExprMul(arg_a, arg_b, inverse) => {},
            
            // used in empty lines, ignore. 
            BSAst::None => {},
        }
    }
}