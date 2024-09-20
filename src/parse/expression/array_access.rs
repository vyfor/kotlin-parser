use crate::ast::{ArrayAccessExpression, Expression};
use chumsky::prelude::*;

use super::expression_parser;

pub fn array_access_parser(
) -> impl Parser<char, Expression, Error = Simple<char>> {
    just('[')
        .padded()
        .ignore_then(expression_parser())
        .then_ignore(just(']').padded())
        .map(|expr| {
            Expression::ArrayAccess(ArrayAccessExpression { expr: expr.into() })
        })
}
