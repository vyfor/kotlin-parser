use crate::{ast::*, parse::ty::function::param_parser};
use chumsky::prelude::*;

use super::call::block_parser;

pub fn try_expr_parser(
    stmt_parser: impl Parser<char, Statement, Error = Simple<char>> + Clone,
    expr_parser: impl Parser<char, Expression, Error = Simple<char>>,
) -> impl Parser<char, Expression, Error = Simple<char>> {
    just("try")
        .padded()
        .ignore_then(block_parser(stmt_parser.clone()))
        .then(catch_expr_parser(stmt_parser.clone(), expr_parser).repeated())
        .then(finally_expr_parser(stmt_parser).or_not())
        .map(|((body, catches), finally)| {
            Expression::Try(TryExpression {
                body,
                catches,
                finally,
            })
        })
}

pub fn catch_expr_parser(
    stmt_parser: impl Parser<char, Statement, Error = Simple<char>>,
    expr_parser: impl Parser<char, Expression, Error = Simple<char>>,
) -> impl Parser<char, CatchExpression, Error = Simple<char>> {
    just("catch")
        .padded()
        .ignore_then(param_parser(expr_parser))
        .then(block_parser(stmt_parser))
        .map(|(param, body)| CatchExpression { param, body })
}

pub fn finally_expr_parser(
    stmt_parser: impl Parser<char, Statement, Error = Simple<char>>,
) -> impl Parser<char, Block, Error = Simple<char>> {
    just("finally")
        .padded()
        .ignore_then(block_parser(stmt_parser))
}
