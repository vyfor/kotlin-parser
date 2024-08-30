use crate::ast::*;
use chumsky::prelude::*;

pub fn unary_expr_parser(
    expr: impl Parser<char, Expr, Error = Simple<char>>,
) -> impl Parser<char, Expr, Error = Simple<char>> {
    let unary_op = choice((
        just("!").to(UnaryOperator::Not),
        just("++").to(UnaryOperator::Increment),
        just("--").to(UnaryOperator::Decrement),
        just("+").to(UnaryOperator::Plus),
        just("-").to(UnaryOperator::Minus),
    ));

    unary_op
        .then(expr)
        .map(|(op, rhs)| Expr::UnaryOp(op, Box::new(rhs)))
}
