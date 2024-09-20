use crate::ast::*;
use chumsky::prelude::*;

use super::call::{block_parser, vars_parser};

pub fn for_expr_parser(
    stmt_parser: impl Parser<char, Statement, Error = Simple<char>> + Clone,
    expr_parser: impl Parser<char, Expression, Error = Simple<char>> + Clone,
) -> impl Parser<char, Expression, Error = Simple<char>> {
    just("for")
        .padded()
        .ignore_then(
            vars_parser()
                .then_ignore(just("in").padded())
                .then(expr_parser)
                .delimited_by(just('(').padded(), just(')').padded()),
        )
        .then(block_parser(stmt_parser))
        .map(|((vars, expr), body)| {
            Expression::For(ForExpression {
                vars,
                iterable: expr.into(),
                body,
            })
        })
}
