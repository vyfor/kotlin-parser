use crate::{
    ast::*,
    parse::{
        expression::call::{block_parser, call_arg_parser},
        ty::function::function_params_parser,
    },
};
use chumsky::prelude::*;

use super::modifier_parser;

pub fn constructor_parser(
    stmt_parser: impl Parser<char, Statement, Error = Simple<char>>,
    expr_parser: impl Parser<char, Expression, Error = Simple<char>> + Clone,
) -> impl Parser<char, ConstructorDeclaration, Error = Simple<char>> {
    modifier_parser()
        .repeated()
        .or_not()
        .then_ignore(just("constructor").padded())
        .then(
            function_params_parser(expr_parser.clone())
                .delimited_by(just('(').padded(), just(')').padded()),
        )
        .then(constructor_delegate_parser(expr_parser).or_not())
        .then(block_parser(stmt_parser).or_not())
        .map(
            |(((modifiers, params), delegate), body)| ConstructorDeclaration {
                modifiers: modifiers.unwrap_or_default(),
                params,
                delegate,
                body,
            },
        )
}

pub fn constructor_delegate_parser(
    expr_parser: impl Parser<char, Expression, Error = Simple<char>>,
) -> impl Parser<char, ConstructorDelegate, Error = Simple<char>> {
    just(':')
        .padded()
        .ignore_then(choice((
            just("this").to(ConstructorDelegateKind::This),
            just("super").to(ConstructorDelegateKind::Super),
        )))
        .then(
            call_arg_parser(expr_parser)
                .separated_by(just(',').padded())
                .or_not()
                .delimited_by(just('(').padded(), just(')').padded()),
        )
        .map(|(kind, args)| ConstructorDelegate {
            kind,
            args: args.unwrap_or_default(),
        })
}
