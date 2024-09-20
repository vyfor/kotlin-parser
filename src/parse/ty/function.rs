use crate::{ast::*, parse::declaration::annotation::annotations_parser};
use chumsky::prelude::*;

use super::type_parser;

pub fn function_type_parser() -> impl Parser<char, Type, Error = Simple<char>> {
    let core_function_type = (type_parser().then_ignore(just('.')))
        .or_not()
        .then(anonymous_params_parser())
        .delimited_by(just('('), just(')'))
        .then_ignore(just("->"))
        .then(type_parser());

    just('(')
        .or_not()
        .then(core_function_type)
        .then(just(')').or_not())
        .then(just('?').or_not())
        .try_map(
            |(((left_paren, function), right_paren), is_nullable), span| {
                if left_paren.is_some() && right_paren.is_none() {
                    return Err(Simple::custom(span, "Expected ')'"));
                } else if left_paren.is_none() && right_paren.is_some() {
                    return Err(Simple::custom(span, "Expected '('"));
                }

                let ((receiver, params), return_ty) = function;

                Ok(Type::Function(Box::new(FunctionType {
                    receiver,
                    params,
                    return_ty,
                    is_nullable: is_nullable.is_some(),
                })))
            },
        )
}

pub fn anonymous_params_parser(
) -> impl Parser<char, Vec<AnonymousParam>, Error = Simple<char>> {
    text::ident()
        .padded()
        .then_ignore(just(':'))
        .padded()
        .or_not()
        .then(type_parser())
        .separated_by(just(',').padded())
        .map(|params| {
            params
                .into_iter()
                .map(|(name, ty)| AnonymousParam { name, ty })
                .collect()
        })
}

pub fn function_params_parser(
) -> impl Parser<char, Vec<Param>, Error = Simple<char>> {
    param_parser().separated_by(just(',').padded()).collect()
}

pub fn param_parser() -> impl Parser<char, Param, Error = Simple<char>> {
    annotations_parser()
        .repeated()
        .or_not()
        .then(
            text::ident()
                .padded()
                .then_ignore(just(':'))
                .padded()
                .then(type_parser()),
        )
        .map(|(annotations, (name, ty))| Param {
            annotations: annotations.unwrap_or_default(),
            name,
            ty,
        })
}
