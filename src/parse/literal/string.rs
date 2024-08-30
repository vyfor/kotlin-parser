use crate::ast::Literal;
use chumsky::prelude::*;

pub fn string_literal() -> impl Parser<char, Literal, Error = Simple<char>> {
    just('"')
        .ignore_then(none_of('"').repeated())
        .then_ignore(just('"'))
        .collect::<String>()
        .map(Literal::String)
}
