use crate::ast::{BracketExpression, Expression};
use chumsky::prelude::*;

use super::expression_parser;

pub fn bracket_expr_parser(
) -> impl Parser<char, Expression, Error = Simple<char>> {
    just('[')
        .padded()
        .ignore_then(expression_parser())
        .then_ignore(just(']').padded())
        .map(|expr| {
            Expression::Bracket(BracketExpression { expr: expr.into() })
        })
}
