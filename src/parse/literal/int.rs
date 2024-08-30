use crate::ast::Literal;
use chumsky::prelude::*;

pub fn int_literal() -> impl Parser<char, Literal, Error = Simple<char>> {
    text::int(10).map(|s: String| Literal::Integer(s.parse().unwrap()))
}
