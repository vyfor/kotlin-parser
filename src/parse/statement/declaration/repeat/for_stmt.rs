use crate::{
    ast::*,
    parse::{expression::expr_parser, statement::block_parser},
};
use chumsky::prelude::*;

pub fn for_stmt_parser(
    stmt_parser: impl Parser<char, Stmt, Error = Simple<char>> + Clone,
) -> impl Parser<char, Stmt, Error = Simple<char>> {
    just("for")
        .padded()
        .then_ignore(just('(').padded())
        .ignore_then(expr_parser())
        .then_ignore(just(')').padded())
        .then(block_parser(stmt_parser))
        .map(|(condition, body)| {
            Stmt::Repeat(RepeatKind::For {
                condition: Box::new(condition),
                body,
            })
        })
}
