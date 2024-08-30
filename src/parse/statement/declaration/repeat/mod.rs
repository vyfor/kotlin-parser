use crate::ast::Stmt;
use chumsky::prelude::*;

pub mod for_stmt;

pub fn repeat_parser(
    stmt_parser: impl Parser<char, Stmt, Error = Simple<char>> + Clone,
) -> impl Parser<char, Stmt, Error = Simple<char>> {
    for_stmt::for_stmt_parser(stmt_parser)
}
