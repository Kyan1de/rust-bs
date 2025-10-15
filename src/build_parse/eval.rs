use crate::prelude::*;
use std::collections::HashSet;

impl BuildParser {
    
    fn validate_part(tree: &BSAst, in_batch: bool, var_table: &mut HashSet<String>) -> bool {
        // A few general rules:
        //  1. A variable must be defined before its use.
        //  2. A variable cannot be changed within a batch.
        //  3. A batch may not contain another batch

        match (*tree).clone() {
            BSAst::Prog(bsasts) => bsasts.iter().all(|t|{Self::validate_part(&t, in_batch, var_table)}),
            BSAst::Batch(bsasts) => !in_batch && bsasts.iter().all(|t|{Self::validate_part(&t, true, var_table)}),
            BSAst::Arr(bsasts) => bsasts.iter().all(|t|{Self::validate_part(&t, in_batch, var_table)}),
            BSAst::SetVar(bsast, bsast1) => {
                if in_batch {return false}
                let BSAst::Ident(iden) = *bsast else {return false};
                var_table.insert(iden);
                Self::validate_part(&bsast1, in_batch, var_table)
            },
            BSAst::Generate(bsasts) => bsasts.iter().all(|t|{Self::validate_part(&t, in_batch, var_table)}),
            BSAst::Unpack(bsast) => {
                match *bsast {
                    BSAst::Arr(_) => Self::validate_part(&bsast, in_batch, var_table),
                    BSAst::Ident(_) => Self::validate_part(&bsast, in_batch, var_table),
                    _ => false
                }
            },

            BSAst::ExprAdd(a, b, _) => 
            Self::validate_part(&a, in_batch, var_table) && Self::validate_part(&b, in_batch, var_table),
            BSAst::ExprMul(a, b, _) => 
            Self::validate_part(&a, in_batch, var_table) && Self::validate_part(&b, in_batch, var_table),
            
            BSAst::Ident(s) => var_table.contains(&s),

            BSAst::Num(_) => true,
            BSAst::Str(_) => true,
            BSAst::None => true,
        }
    }

    pub fn validate(tree: &BSAst) -> bool {
        let mut var_table: HashSet<String> = HashSet::new();

        Self::validate_part(tree, false, &mut var_table)
    }

}