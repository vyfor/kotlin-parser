use crate::ast::*;
use chumsky::prelude::*;
use text::newline;

use super::call::expr_block_parser;

pub fn when_expr_parser(
    stmt_parser: impl Parser<char, Statement, Error = Simple<char>> + Clone,
    expr_parser: impl Parser<char, Expression, Error = Simple<char>> + Clone,
) -> impl Parser<char, Expression, Error = Simple<char>> {
    just("when")
        .padded()
        .ignore_then(
            expr_parser
                .clone()
                .delimited_by(just('(').padded(), just(')').padded())
                .map(Box::new)
                .or_not(),
        )
        .then(
            when_entry_parser(stmt_parser.clone(), expr_parser.clone())
                .separated_by(
                    newline().or(just(';').ignored()).padded().repeated(),
                ),
        )
        .map(|(expr, entries)| {
            Expression::When(WhenExpression { expr, entries })
        })
}

pub fn when_entry_parser(
    stmt_parser: impl Parser<char, Statement, Error = Simple<char>> + Clone,
    expr_parser: impl Parser<char, Expression, Error = Simple<char>> + Clone,
) -> impl Parser<char, WhenEntry, Error = Simple<char>> {
    expr_parser
        .separated_by(just(',').padded())
        .at_least(1)
        .or(just("else").map(|_| vec![]))
        .then(
            just("->")
                .padded()
                .ignore_then(expr_block_parser(stmt_parser)),
        )
        .map(|(exprs, body)| WhenEntry { exprs, body })
}
