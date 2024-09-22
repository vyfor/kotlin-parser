use crate::{
    ast::*,
    parse::{
        expression::call::block_parser,
        ty::{class_type_params_parser, type_bounds_parser, type_parser},
    },
};
use chumsky::prelude::*;

use super::{annotation::annotations_parser, modifier_parser};

pub fn entity_parser(
    stmt_parser: impl Parser<char, Statement, Error = Simple<char>> + Clone,
    expr_parser: impl Parser<char, Expression, Error = Simple<char>> + Clone,
) -> impl Parser<char, EntityDeclaration, Error = Simple<char>> {
    modifier_parser()
        .repeated()
        .or_not()
        .then(entity_kind_parser().padded())
        .then(text::ident().padded().or_not())
        .then(
            class_type_params_parser(expr_parser.clone())
                .padded()
                .or_not(),
        )
        .then(primary_constructor_parser(expr_parser).or_not())
        .then(extends_parser().padded().or_not())
        .then(type_bounds_parser().padded().or_not())
        .then(block_parser(stmt_parser).or_not())
        .map(
            |(
                (
                    (
                        (
                            (((modifiers, kind), name), type_params),
                            primary_constructor,
                        ),
                        extends,
                    ),
                    bounds,
                ),
                inner,
            )| {
                EntityDeclaration {
                    modifiers: modifiers.unwrap_or_default(),
                    kind,
                    name,
                    type_params: type_params.unwrap_or_default(),
                    primary_constructor,
                    bounds: bounds.unwrap_or_default(),
                    extends: extends.unwrap_or_default(),
                    body: inner,
                }
            },
        )
}

pub fn entity_kind_parser(
) -> impl Parser<char, EntityDeclarationKind, Error = Simple<char>> {
    choice((
        just("enum")
            .padded()
            .then(just("class"))
            .to(EntityDeclarationKind::Enum),
        just("interface").to(EntityDeclarationKind::Interface),
        just("class").to(EntityDeclarationKind::Class),
        just("companion")
            .padded()
            .then(just("object"))
            .to(EntityDeclarationKind::CompanionObject),
        just("object").to(EntityDeclarationKind::Object),
    ))
}

pub fn extends_parser() -> impl Parser<char, Vec<Type>, Error = Simple<char>> {
    just(':')
        .padded()
        .ignore_then(type_parser())
        .separated_by(just(',').padded())
}

pub fn primary_constructor_parser(
    expr_parser: impl Parser<char, Expression, Error = Simple<char>>,
) -> impl Parser<char, PrimaryConstructorDeclaration, Error = Simple<char>> {
    modifier_parser()
        .repeated()
        .or_not()
        .then(
            constructor_param_parser(expr_parser)
                .separated_by(just(',').padded())
                .delimited_by(just('(').padded(), just(')').padded()),
        )
        .map(|(modifiers, params)| PrimaryConstructorDeclaration {
            modifiers: modifiers.unwrap_or_default(),
            params,
        })
}

pub fn constructor_param_parser(
    expr_parser: impl Parser<char, Expression, Error = Simple<char>>,
) -> impl Parser<char, ConstructorParam, Error = Simple<char>> {
    annotations_parser(expr_parser)
        .repeated()
        .or_not()
        .then(modifier_parser().repeated().or_not())
        .then(
            choice((
                just("var").to(PropertyType::Var),
                just("val").to(PropertyType::Val),
            ))
            .padded()
            .or_not(),
        )
        .then(
            text::ident()
                .padded()
                .then_ignore(just(':'))
                .padded()
                .then(type_parser()),
        )
        .map(|(((annotations, modifiers), property_type), (name, ty))| {
            ConstructorParam {
                modifiers: modifiers.unwrap_or_default(),
                property_type,
                param: Param {
                    annotations: annotations.unwrap_or_default(),
                    name,
                    ty,
                },
            }
        })
}
