use crate::ast::*;
use chumsky::prelude::*;

use super::call::block_parser;

pub fn while_expr_parser<'a>(
    stmt_parser: impl Parser<char, Statement, Error = Simple<char>> + Clone + 'a,
    expr_parser: impl Parser<char, Expression, Error = Simple<char>> + 'a,
) -> impl Parser<char, Expression, Error = Simple<char>> + 'a {
    let condition = just("while")
        .padded()
        .ignore_then(
            expr_parser.delimited_by(just('(').padded(), just(')').padded()),
        )
        .boxed();

    let while_loop = condition.clone().then(block_parser(stmt_parser.clone()));
    let do_while = just("do")
        .padded()
        .ignore_then(block_parser(stmt_parser))
        .then(condition);

    choice((
        while_loop.map(|(expr, body)| {
            Expression::While(WhileExpression {
                expr: expr.into(),
                body,
                is_do_while: false,
            })
        }),
        do_while.map(|(body, expr)| {
            Expression::While(WhileExpression {
                expr: expr.into(),
                body,
                is_do_while: true,
            })
        }),
    ))
}
