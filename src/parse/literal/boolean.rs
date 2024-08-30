use crate::ast::Literal;
use chumsky::prelude::*;

pub fn boolean_literal() -> impl Parser<char, Literal, Error = Simple<char>> {
    choice((
        just("true").to(Literal::Boolean(true)),
        just("false").to(Literal::Boolean(false)),
    ))
}
