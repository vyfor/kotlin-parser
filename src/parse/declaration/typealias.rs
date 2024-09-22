use crate::{
    ast::*,
    parse::ty::{type_params_parser, type_parser},
};
use chumsky::prelude::*;

use super::modifier_parser;

pub fn typealias_parser(
    expr_parser: impl Parser<char, Expression, Error = Simple<char>>,
) -> impl Parser<char, TypeAliasDeclaration, Error = Simple<char>> {
    modifier_parser()
        .repeated()
        .or_not()
        .then(
            just("typealias")
                .padded()
                .ignore_then(text::ident().padded())
                .then(type_params_parser(expr_parser).or_not())
                .then_ignore(just('='))
                .then(type_parser()),
        )
        .map(
            |(modifiers, ((name, type_params), ty))| TypeAliasDeclaration {
                modifiers: modifiers.unwrap_or_default(),
                name,
                type_params: type_params.unwrap_or_default(),
                ty,
            },
        )
}
