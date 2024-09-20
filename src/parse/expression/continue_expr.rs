use crate::ast::*;
use chumsky::prelude::*;

use super::label::after_label_parser;

pub fn continue_expr_parser(
) -> impl Parser<char, Expression, Error = Simple<char>> {
    just("continue")
        .ignore_then(after_label_parser().or_not())
        .map(|label| Expression::Continue(ContinueExpression { label }))
}
