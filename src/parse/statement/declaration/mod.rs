use chumsky::prelude::*;
use conditional::conditional_parser;
use function::function_parser;
use property::property_parser;
use repeat::repeat_parser;

use crate::ast::Stmt;

pub mod conditional;
mod function;
mod property;
pub mod repeat;

pub fn declaration_parser(
    stmt_parser: impl Parser<char, Stmt, Error = Simple<char>> + Clone,
) -> impl Parser<char, Stmt, Error = Simple<char>> {
    property_parser()
        .or(function_parser(stmt_parser.clone()))
        .or(repeat_parser(stmt_parser.clone()))
        .or(conditional_parser(stmt_parser))
}
