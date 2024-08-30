use crate::ast::Literal;
use chumsky::prelude::*;

pub fn null_literal() -> impl Parser<char, Literal, Error = Simple<char>> {
    just("null").to(Literal::Null)
}
