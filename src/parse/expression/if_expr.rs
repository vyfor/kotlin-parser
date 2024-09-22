use crate::ast::*;
use chumsky::prelude::*;

use super::call::block_parser;

pub fn if_expr_parser(
    stmt_parser: impl Parser<char, Statement, Error = Simple<char>> + Clone,
    expr_parser: impl Parser<char, Expression, Error = Simple<char>> + Clone,
) -> impl Parser<char, Expression, Error = Simple<char>> {
    let if_branch = just("if")
        .padded()
        .ignore_then(
            expr_parser
                .clone()
                .delimited_by(just('(').padded(), just(')').padded()),
        )
        .then(block_parser(stmt_parser.clone()))
        .map(|(condition, body)| (condition, body));

    let otherwise = just("else")
        .padded()
        .ignore_then(expr_parser)
        .map(Box::new)
        .or_not();

    if_branch.then(otherwise).map(|(if_branch, otherwise)| {
        Expression::If(IfExpression {
            expr: if_branch.0.into(),
            then: if_branch.1,
            otherwise,
        })
    })
}
