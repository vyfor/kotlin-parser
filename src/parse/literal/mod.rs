use crate::ast::{Expr, Literal};
use boolean::boolean_literal;
use char::char_literal;
use chumsky::prelude::*;
use float::float_literal;
use int::int_literal;
use null::null_literal;
use string::string_literal;

mod boolean;
mod char;
mod float;
mod int;
mod null;
mod string;

#[allow(dead_code)]
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

pub fn literal_expr_parser() -> impl Parser<char, Expr, Error = Simple<char>> {
    choice((
        float_literal(),
        int_literal(),
        string_literal(),
        char_literal(),
        boolean_literal(),
        null_literal(),
    ))
    .map(Expr::Literal)
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
