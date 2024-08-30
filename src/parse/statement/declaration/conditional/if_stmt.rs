use crate::{
    ast::*,
    parse::{expression::expr_parser, statement::block_parser},
};
use chumsky::prelude::*;

pub fn if_stmt_parser(
    stmt_parser: impl Parser<char, Stmt, Error = Simple<char>> + Clone,
) -> impl Parser<char, Stmt, Error = Simple<char>> {
    let if_branch = just("if")
        .padded()
        .then_ignore(just('(').padded())
        .ignore_then(expr_parser())
        .then_ignore(just(')').padded())
        .then(block_parser(stmt_parser.clone()))
        .map(|(condition, body)| (condition, body));

    let else_if_branches = just("else")
        .padded()
        .then(just("if").padded())
        .then_ignore(just('(').padded())
        .ignore_then(expr_parser())
        .then_ignore(just(')').padded())
        .then(block_parser(stmt_parser.clone()))
        .map(|(condition, body)| (condition, body))
        .repeated();

    let otherwise = just("else")
        .padded()
        .ignore_then(block_parser(stmt_parser)) // Use block_parser for the else body
        .or_not();

    if_branch.then(else_if_branches).then(otherwise).map(
        |((if_branch, else_if_branches), otherwise)| {
            Stmt::Conditional(ConditionalKind::If {
                branches: vec![if_branch]
                    .into_iter()
                    .chain(else_if_branches)
                    .collect(),
                otherwise,
            })
        },
    )
}
