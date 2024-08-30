use crate::{ast::*, parse::literal::literal_expr_parser};
use chumsky::prelude::*;

use super::{unary::unary_expr_parser, variable::variable_expr_parser};

pub fn binary_op_parser<'a>(
    expr: impl Parser<char, Expr, Error = Simple<char>> + 'a + Clone,
) -> impl Parser<char, Expr, Error = Simple<char>> + 'a {
    let assign_op = choice((
        just("=").to(BinaryOperator::Assign),
        just("+=").to(BinaryOperator::AddAssign),
        just("-=").to(BinaryOperator::SubtractAssign),
        just("*=").to(BinaryOperator::MultiplyAssign),
        just("/=").to(BinaryOperator::DivideAssign),
        just("%=").to(BinaryOperator::ModuloAssign),
    ));

    // TODO: prioritize binary ops based on precedence
    let binary_op = choice((
        just("as").to(BinaryOperator::As),
        just("as?").to(BinaryOperator::AsNullable),
        just("*").to(BinaryOperator::Multiply),
        just("/").to(BinaryOperator::Divide),
        just("%").to(BinaryOperator::Modulo),
        just("+").to(BinaryOperator::Add),
        just("-").to(BinaryOperator::Subtract),
        just("..").to(BinaryOperator::RangeTo),
        just("..<").to(BinaryOperator::RangeUntil),
        just("?:").to(BinaryOperator::Elvis),
        just("in").to(BinaryOperator::In),
        just("!in").to(BinaryOperator::NotIn),
        just("is").to(BinaryOperator::Is),
        just("!is").to(BinaryOperator::IsNot),
        just("<").to(BinaryOperator::LessThan),
        just(">").to(BinaryOperator::GreaterThan),
        just("<=").to(BinaryOperator::LessThanOrEqual),
        just(">=").to(BinaryOperator::GreaterThanOrEqual),
        just("==").to(BinaryOperator::Equal),
        just("!=").to(BinaryOperator::NotEqual),
        just("===").to(BinaryOperator::ReferenceEqual),
        just("!==").to(BinaryOperator::ReferenceNotEqual),
        just("&&").to(BinaryOperator::And),
        just("||").to(BinaryOperator::Or),
    ));

    let literal = literal_expr_parser();
    let variable = variable_expr_parser();
    let unary = unary_expr_parser(expr.clone());

    let atom = choice((variable, literal, unary)).boxed();

    let op = binary_op.or(assign_op).padded();
    atom.clone()
        .then(op.then(atom).repeated())
        .foldl(|lhs, (op, rhs)| {
            Expr::BinaryOp(Box::new(lhs), op, Box::new(rhs))
        })
}
