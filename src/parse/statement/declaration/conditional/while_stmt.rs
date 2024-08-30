use crate::{
    ast::*,
    parse::{expression::expr_parser, statement::block_parser},
};
use chumsky::prelude::*;

pub fn while_stmt_parser(
    stmt_parser: impl Parser<char, Stmt, Error = Simple<char>> + Clone,
) -> impl Parser<char, Stmt, Error = Simple<char>> {
    let condition = just("while")
        .padded()
        .then_ignore(just('(').padded())
        .ignore_then(expr_parser())
        .then_ignore(just(')').padded())
        .boxed();

    let while_loop = condition.clone().then(block_parser(stmt_parser.clone()));
    let do_while = just("do")
        .padded()
        .ignore_then(block_parser(stmt_parser))
        .then(condition);

    while_loop
        .map(|(condition, body)| {
            Stmt::Repeat(RepeatKind::While {
                condition: Box::new(condition),
                body,
                do_while: false,
            })
        })
        .or(do_while.map(|(body, condition)| {
            Stmt::Repeat(RepeatKind::While {
                condition: Box::new(condition),
                body,
                do_while: true,
            })
        }))
}
