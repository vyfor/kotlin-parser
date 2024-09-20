use crate::{
    ast::*,
    parse::ty::{
        function::function_params_parser, type_bounds_parser,
        type_params_parser, type_parser,
    },
};
use chumsky::prelude::*;

use super::modifier_parser;

pub fn function_parser(
    stmt_parser: impl Parser<char, Statement, Error = Simple<char>> + Clone,
) -> impl Parser<char, FunctionDeclaration, Error = Simple<char>> {
    let modifiers = modifier_parser().repeated().or_not();
    let name = text::ident().padded();
    let type_params = type_params_parser().or_not();
    let receiver = type_parser().then_ignore(just('.').padded()).or_not();
    let params = function_params_parser()
        .delimited_by(just('(').padded(), just(')').padded())
        .or_not();
    let return_ty = just(':')
        .padded()
        .ignore_then(type_parser())
        .padded()
        .or_not();
    let body = choice((
        just('=')
            .padded()
            .ignore_then(stmt_parser.clone())
            .map(|statement| Block {
                statements: vec![statement],
            }),
        just('{')
            .padded()
            .ignore_then(stmt_parser.repeated())
            .then_ignore(just('}').padded())
            .map(|statements: Vec<Statement>| Block { statements }),
    ))
    .or_not();
    let bounds = type_bounds_parser().or_not();

    modifiers
        .then_ignore(just("fun"))
        .then(name)
        .then(type_params)
        .then(receiver)
        .then(params)
        .then(return_ty)
        .then(body)
        .then(bounds)
        .map(
            |(
                (
                    (
                        ((((modifiers, name), type_params), receiver), params),
                        return_ty,
                    ),
                    body,
                ),
                bounds,
            )| {
                FunctionDeclaration {
                    modifiers: modifiers.unwrap_or_default(),
                    name: name.into(),
                    type_params: type_params.unwrap_or_default(),
                    receiver,
                    params: params.unwrap_or_default(),
                    return_ty,
                    body,
                    bounds: bounds.unwrap_or_default(),
                }
            },
        )
}
