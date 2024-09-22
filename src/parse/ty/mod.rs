use crate::ast::*;
use chumsky::prelude::*;

use super::declaration::annotation::annotations_parser;

pub mod function;
pub mod simple;

pub fn type_parser() -> impl Parser<char, Type, Error = Simple<char>> {
    choice((
        function::function_type_parser(),
        simple::simple_type_parser(),
    ))
    .map(Type::from)
    .boxed()
}

pub fn type_params_parser(
    expr_parser: impl Parser<char, Expression, Error = Simple<char>>,
) -> impl Parser<char, Vec<TypeParam>, Error = Simple<char>> {
    annotations_parser(expr_parser)
        .repeated()
        .or_not()
        .then(
            text::ident()
                .padded()
                .then(just(':').padded().ignore_then(type_parser()).or_not()),
        )
        .separated_by(just(',').padded())
        .delimited_by(just('<').padded(), just('>').padded())
        .map(|types| {
            types
                .into_iter()
                .map(|(annotations, (name, ty))| TypeParam {
                    annotations: annotations.unwrap_or_default(),
                    name,
                    ty,
                })
                .collect()
        })
}

pub fn class_type_params_parser(
    expr_parser: impl Parser<char, Expression, Error = Simple<char>>,
) -> impl Parser<char, Vec<BoundedTypeParam>, Error = Simple<char>> {
    annotations_parser(expr_parser)
        .repeated()
        .or_not()
        .then(
            choice((
                just("in").to(BoundKind::In),
                just("out").to(BoundKind::Out),
            ))
            .or_not(),
        )
        .then(
            text::ident()
                .padded()
                .then(just(':').padded().ignore_then(type_parser()).or_not()),
        )
        .separated_by(just(',').padded())
        .delimited_by(just('<').padded(), just('>').padded())
        .map(|types| {
            types
                .into_iter()
                .map(|((annotations, bound), (name, ty))| BoundedTypeParam {
                    annotations: annotations.unwrap_or_default(),
                    bounds: vec![TypeBound {
                        name,
                        ty,
                        kind: bound,
                    }],
                })
                .collect()
        })
}

pub fn type_bounds_parser(
) -> impl Parser<char, Vec<TypeBound>, Error = Simple<char>> {
    just("where")
        .padded()
        .ignore_then(
            text::ident()
                .padded()
                .then(just(':').padded().ignore_then(type_parser()))
                .separated_by(just(',').padded()),
        )
        .map(|bounds| {
            bounds
                .into_iter()
                .map(|(name, ty)| TypeBound {
                    name,
                    ty: ty.into(),
                    kind: None,
                })
                .collect()
        })
}
