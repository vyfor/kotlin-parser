use crate::ast::Literal;
use chumsky::prelude::*;

pub fn float_literal() -> impl Parser<char, Literal, Error = Simple<char>> {
    text::int(10)
        .then(just('.').then(text::int(10)))
        .map(|(int, (_, frac))| {
            Literal::Float(format!("{}.{}", int, frac).parse().unwrap())
        })
}
