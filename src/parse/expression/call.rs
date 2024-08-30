use crate::{ast::*, parse::ty::type_parser};
use chumsky::prelude::*;

pub fn call_expr_parser(
    expr: impl Parser<char, Expr, Error = Simple<char>>,
) -> impl Parser<char, Expr, Error = Simple<char>> {
    text::ident()
        .then(
            type_parser()
                .separated_by(just(',').padded())
                .delimited_by(just('<'), just('>'))
                .padded()
                .or_not(),
        )
        .then(
            expr.separated_by(just(',').padded())
                .delimited_by(just('('), just(')'))
                .padded(),
        )
        .map(|((name, attributes), args)| {
            Expr::Call(name, attributes.unwrap_or_default(), args)
        })
}
