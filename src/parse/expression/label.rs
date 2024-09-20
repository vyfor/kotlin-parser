use chumsky::prelude::*;

pub fn after_label_parser() -> impl Parser<char, String, Error = Simple<char>> {
    just('@').ignore_then(text::ident())
}

pub fn before_label_parser() -> impl Parser<char, String, Error = Simple<char>>
{
    text::ident().then_ignore(just('@'))
}
