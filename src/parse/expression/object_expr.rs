use crate::{
    ast::*,
    parse::declaration::{
        annotation::annotations_parser, entity::extends_parser,
    },
};
use chumsky::prelude::*;

use super::call::block_parser;

pub fn object_expr_parser(
    stmt_parser: impl Parser<char, Statement, Error = Simple<char>>,
    expr_parser: impl Parser<char, Expression, Error = Simple<char>>,
) -> impl Parser<char, Expression, Error = Simple<char>> {
    annotations_parser(expr_parser)
        .repeated()
        .or_not()
        .then(
            just("object")
                .padded()
                .ignore_then(extends_parser().or_not())
                .then(block_parser(stmt_parser)),
        )
        .map(|(annotations, (extends, body))| {
            Expression::Object(ObjectExpression {
                annotations: annotations.unwrap_or_default(),
                extends: extends.unwrap_or_default(),
                body,
            })
        })
}
