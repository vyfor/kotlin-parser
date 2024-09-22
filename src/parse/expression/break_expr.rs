use crate::ast::*;
use chumsky::prelude::*;

use super::label::after_label_parser;

pub fn break_expr_parser() -> impl Parser<char, Expression, Error = Simple<char>>
{
    just("break")
        .ignore_then(after_label_parser().or_not())
        .map(|label| Expression::Break(BreakExpression { label }))
}
