use crate::ast::{BracketExpression, Expression, Statement};
use chumsky::prelude::*;

use super::expression_parser;

pub fn bracket_expr_parser<'a>(
    stmt_parser: impl Parser<char, Statement, Error = Simple<char>> + Clone + 'a,
) -> impl Parser<char, Expression, Error = Simple<char>> + 'a {
    just('[')
        .padded()
        .ignore_then(expression_parser(stmt_parser))
        .then_ignore(just(']').padded())
        .map(|expr| {
            Expression::Bracket(BracketExpression { expr: expr.into() })
        })
}
