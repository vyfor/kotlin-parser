use crate::ast::Literal;
use chumsky::prelude::*;

pub fn char_literal() -> impl Parser<char, Literal, Error = Simple<char>> {
    just('\'')
        .ignore_then(none_of('\''))
        .then_ignore(just('\''))
        .map(Literal::Char)
}
