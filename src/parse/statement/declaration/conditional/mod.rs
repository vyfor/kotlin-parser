use crate::ast::Stmt;
use chumsky::prelude::*;

pub mod if_stmt;
pub mod while_stmt;

pub fn conditional_parser(
    stmt_parser: impl Parser<char, Stmt, Error = Simple<char>> + Clone,
) -> impl Parser<char, Stmt, Error = Simple<char>> {
    if_stmt::if_stmt_parser(stmt_parser.clone())
        .or(while_stmt::while_stmt_parser(stmt_parser))
}
