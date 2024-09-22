use crate::{
    ast::*,
    parse::{
        expression::call::{block_parser, inner_block_parser, vars_parser},
        ty::{type_bounds_parser, type_params_parser, type_parser},
    },
};
use chumsky::prelude::*;

use super::{annotation::annotations_parser, modifier_parser};

pub fn property_parser(
    stmt_parser: impl Parser<char, Statement, Error = Simple<char>> + Clone,
    expr_parser: impl Parser<char, Expression, Error = Simple<char>> + Clone,
) -> impl Parser<char, PropertyDeclaration, Error = Simple<char>> {
    let modifiers = modifier_parser().repeated().or_not();
    let is_const = just("const")
        .padded()
        .or_not()
        .map(|is_const| is_const.is_some());
    let is_mutable = choice((just("var").to(true), just("val").to(false)));
    let type_params = type_params_parser(expr_parser.clone());
    let receiver = type_parser()
        .padded()
        .then_ignore(just('.').padded())
        .or_not();
    let vars = vars_parser();
    let init = choice((just("by").to(true), just('=').to(false)))
        .then(expr_parser.clone())
        .or_not();
    let bounds = type_bounds_parser().or_not();
    let accessors = choice((
        get_accessor_parser(stmt_parser.clone(), expr_parser.clone()).then(
            set_accessor_parser(stmt_parser.clone(), expr_parser.clone())
                .or_not(),
        ),
        set_accessor_parser(stmt_parser.clone(), expr_parser.clone()).then(
            get_accessor_parser(stmt_parser.clone(), expr_parser.clone())
                .or_not(),
        ),
    ))
    .or_not();

    modifiers
        .then(is_const)
        .then(is_mutable)
        .then(type_params)
        .then(receiver)
        .then(vars)
        .then(init)
        .then(bounds)
        .then(accessors)
        .map(
            |(
                (
                    (
                        (
                            (
                                (
                                    ((modifiers, is_const), is_mutable),
                                    type_params,
                                ),
                                receiver,
                            ),
                            vars,
                        ),
                        body,
                    ),
                    bounds,
                ),
                accessors_pair,
            )| {
                let (is_delegated, init) =
                    body.map(|(a, b)| (a, Some(b))).unwrap_or((false, None));

                let mut accessors = Vec::with_capacity(2);
                if let Some((a, b)) = accessors_pair {
                    accessors.push(a);
                    if let Some(b) = b {
                        accessors.push(b);
                    }
                }

                PropertyDeclaration {
                    modifiers: modifiers.unwrap_or_default(),
                    is_const,
                    is_mutable,
                    is_delegated,
                    type_params,
                    receiver,
                    vars,
                    init,
                    bounds: bounds.unwrap_or_default(),
                    accessors,
                }
            },
        )
}

pub fn get_accessor_parser(
    stmt_parser: impl Parser<char, Statement, Error = Simple<char>>,
    expr_parser: impl Parser<char, Expression, Error = Simple<char>> + Clone,
) -> impl Parser<char, PropertyAccessor, Error = Simple<char>> {
    annotations_parser(expr_parser)
        .repeated()
        .or_not()
        .then(modifier_parser().repeated().or_not())
        .then_ignore(just("get").padded())
        .then_ignore(
            just('(').padded().then_ignore(just(')').padded()).or_not(),
        )
        .then(just(':').ignore_then(type_parser()).or_not())
        .then(block_parser(stmt_parser).or_not())
        .map(|(((annotations, modifiers), return_ty), body)| {
            PropertyAccessor::Getter {
                modifiers: modifiers.unwrap_or_default(),
                annotations: annotations.unwrap_or_default(),
                return_ty,
                body,
            }
        })
}

pub fn set_accessor_parser(
    stmt_parser: impl Parser<char, Statement, Error = Simple<char>>,
    expr_parser: impl Parser<char, Expression, Error = Simple<char>> + Clone,
) -> impl Parser<char, PropertyAccessor, Error = Simple<char>> {
    annotations_parser(expr_parser)
        .repeated()
        .or_not()
        .then(modifier_parser().repeated().or_not())
        .then_ignore(just("set").padded())
        .then(
            text::ident()
                .padded()
                .then(just(':').padded().ignore_then(type_parser()).or_not())
                .delimited_by(just('(').padded(), just(')').padded())
                .map(|(name, ty)| PropertySetterField { name, ty })
                .or_not(),
        )
        .then(just(':').padded().ignore_then(type_parser()).or_not())
        .then(
            just('{')
                .padded()
                .ignore_then(inner_block_parser(stmt_parser))
                .then_ignore(just('}').padded())
                .or_not(),
        )
        .map(|((((annotations, modifiers), field), return_ty), body)| {
            PropertyAccessor::Setter {
                modifiers: modifiers.unwrap_or_default(),
                annotations: annotations.unwrap_or_default(),
                field,
                return_ty,
                body,
            }
        })
}
