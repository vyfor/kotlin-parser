use crate::{
    ast::*,
    parse::ty::{
        simple::{simple_type_parser, type_args_parser},
        type_parser,
    },
};
use chumsky::prelude::*;

use super::{label::before_label_parser, path::path_parser};

pub fn call_parser(
    stmt_parser: impl Parser<char, Statement, Error = Simple<char>>,
    expr_parser: impl Parser<char, Expression, Error = Simple<char>>,
) -> impl Parser<char, Expression, Error = Simple<char>> {
    path_parser()
        .padded()
        .then(type_args_parser(simple_type_parser()).or_not())
        .then(call_args_parser(expr_parser).or_not())
        .then(lambda_parser(stmt_parser).or_not())
        .map(|(((path, type_args), args), block)| {
            Expression::Call(CallExpression {
                path,
                args: args.unwrap_or_default(),
                type_args: type_args.unwrap_or_default(),
                lambda: block.map(Box::new),
            })
        })
}

pub fn call_arg_parser(
    expr_parser: impl Parser<char, Expression, Error = Simple<char>>,
) -> impl Parser<char, CallArg, Error = Simple<char>> {
    text::ident()
        .padded()
        .then_ignore(just('='))
        .or_not()
        .then(just('*').or_not())
        .then(expr_parser)
        .map(|((name, is_spread), expr)| CallArg {
            name,
            value: Box::new(expr),
            is_spread: is_spread.is_some(),
        })
}

pub fn call_args_parser(
    expr_parser: impl Parser<char, Expression, Error = Simple<char>>,
) -> impl Parser<char, Vec<CallArg>, Error = Simple<char>> {
    call_arg_parser(expr_parser)
        .separated_by(just(',').padded())
        .delimited_by(just('(').padded(), just(')').padded())
}

pub fn invocation_args_parser(
    expr_parser: impl Parser<char, Expression, Error = Simple<char>>,
) -> impl Parser<char, Vec<InvocationArg>, Error = Simple<char>> {
    text::ident()
        .padded()
        .then_ignore(just('='))
        .or_not()
        .then(expr_parser)
        .separated_by(just(',').padded())
        .delimited_by(just('(').padded(), just(')').padded())
        .map(|args| {
            args.into_iter()
                .map(|(name, expr)| InvocationArg {
                    name,
                    value: Box::new(expr),
                })
                .collect()
        })
}

pub fn lambda_parser(
    stmt_parser: impl Parser<char, Statement, Error = Simple<char>>,
) -> impl Parser<char, LambdaBlock, Error = Simple<char>> {
    before_label_parser()
        .or_not()
        .then(
            just('{')
                .ignore_then(vars_parser().then_ignore(just("->")).or_not())
                .then(inner_block_parser(stmt_parser).or_not())
                .then_ignore(just('}')),
        )
        .map(|(label, (vars, body))| LambdaBlock { label, vars, body })
}

pub fn vars_parser() -> impl Parser<char, Vars, Error = Simple<char>> {
    let params = text::ident()
        .padded()
        .then(just(':').padded().ignore_then(type_parser()).or_not())
        .separated_by(just(',').padded());

    params
        .delimited_by(just('(').padded(), just(')').padded())
        .map(|params| {
            let vars = params
                .into_iter()
                .map(|(name, ty)| Var { name, ty })
                .collect();

            Vars {
                is_destructured: true,
                vars,
            }
        })
}

pub fn block_parser(
    stmt_parser: impl Parser<char, Statement, Error = Simple<char>>,
) -> impl Parser<char, Block, Error = Simple<char>> {
    stmt_parser
        .repeated()
        .delimited_by(just('{').padded(), just('}').padded())
        .map(|statements| Block { statements })
}

pub fn inner_block_parser(
    stmt_parser: impl Parser<char, Statement, Error = Simple<char>>,
) -> impl Parser<char, Block, Error = Simple<char>> {
    stmt_parser
        .repeated()
        .map(|statements| Block { statements })
}

pub fn expr_block_parser(
    stmt_parser: impl Parser<char, Statement, Error = Simple<char>> + Clone,
) -> impl Parser<char, Block, Error = Simple<char>> {
    choice((
        just('{')
            .padded()
            .ignore_then(stmt_parser.clone().repeated())
            .then_ignore(just('}').padded())
            .or_not()
            .map(|stmts| Block {
                statements: stmts.unwrap_or_default(),
            }),
        stmt_parser.or_not().map(|stmt| Block {
            statements: stmt.map_or_else(Vec::new, |stmt| vec![stmt]),
        }),
    ))
}
