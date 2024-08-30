use crate::ast::*;
use chumsky::prelude::*;

pub fn variable_expr_parser() -> impl Parser<char, Expr, Error = Simple<char>> {
    text::ident().map(Expr::Variable)
}
