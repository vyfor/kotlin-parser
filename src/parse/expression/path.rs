use chumsky::prelude::*;

pub fn path_parser() -> impl Parser<char, Vec<String>, Error = Simple<char>> {
    text::ident()
        .then(just('.').ignore_then(text::ident()).repeated())
        .map(|(first, rest)| [first].into_iter().chain(rest).collect())
}
