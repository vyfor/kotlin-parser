use super::ast::*;
use chumsky::prelude::*;

use crate::parse::statements::stmt_parser;

pub fn parser() -> impl Parser<char, Vec<Stmt>, Error = Simple<char>> {
    stmt_parser().repeated().then_ignore(end())
}
