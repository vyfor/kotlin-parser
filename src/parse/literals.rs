use crate::ast::*;
use chumsky::prelude::*;

pub fn literal_parser() -> impl Parser<char, Literal, Error = Simple<char>> {
    choice((
        float_literal(),
        int_literal(),
        string_literal(),
        char_literal(),
        boolean_literal(),
        null_literal(),
    ))
}

fn int_literal() -> impl Parser<char, Literal, Error = Simple<char>> {
    text::int(10).map(|s: String| Literal::Integer(s.parse().unwrap()))
}

fn float_literal() -> impl Parser<char, Literal, Error = Simple<char>> {
    text::int(10)
        .then(just('.').then(text::int(10)))
        .map(|(int, (_, frac))| {
            Literal::Float(format!("{}.{}", int, frac).parse().unwrap())
        })
}

fn string_literal() -> impl Parser<char, Literal, Error = Simple<char>> {
    just('"')
        .ignore_then(none_of('"').repeated())
        .then_ignore(just('"'))
        .collect::<String>()
        .map(Literal::String)
}

fn char_literal() -> impl Parser<char, Literal, Error = Simple<char>> {
    just('\'')
        .ignore_then(none_of('\''))
        .then_ignore(just('\''))
        .map(Literal::Char)
}

fn boolean_literal() -> impl Parser<char, Literal, Error = Simple<char>> {
    choice((
        just("true").to(Literal::Boolean(true)),
        just("false").to(Literal::Boolean(false)),
    ))
}

fn null_literal() -> impl Parser<char, Literal, Error = Simple<char>> {
    just("null").to(Literal::Null)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_literal() {
        assert_eq!(literal_parser().parse("123"), Ok(Literal::Integer(123)));

        assert_eq!(
            literal_parser().parse("123.456"),
            Ok(Literal::Float(123.456))
        );

        assert_eq!(
            literal_parser().parse("\"abc\""),
            Ok(Literal::String("abc".to_string()))
        );

        assert_eq!(literal_parser().parse("'a'"), Ok(Literal::Char('a')));

        assert_eq!(literal_parser().parse("true"), Ok(Literal::Boolean(true)));

        assert_eq!(
            literal_parser().parse("false"),
            Ok(Literal::Boolean(false))
        );

        assert_eq!(literal_parser().parse("null"), Ok(Literal::Null));
    }
}
