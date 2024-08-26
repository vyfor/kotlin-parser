use crate::ast::*;
use super::literals::literal_parser;
use super::types::type_parser;
use chumsky::prelude::*;
use chumsky::text::ident;

pub fn expr_parser() -> impl Parser<char, Expr, Error = Simple<char>> {
    recursive(|expr| {
        let literal = literal_parser().map(Expr::Literal);

        let variable = text::ident().map(Expr::Variable);

        let unary_op = choice((
            just("!").to(UnaryOperator::Not),
            just("++").to(UnaryOperator::Increment),
            just("--").to(UnaryOperator::Decrement),
            just("+").to(UnaryOperator::Plus),
            just("-").to(UnaryOperator::Minus),
        ));

        let unary_expr = unary_op
            .then(expr.clone())
            .map(|(op, rhs)| Expr::UnaryOp(op, Box::new(rhs)));

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
            just("==").to(BinaryOperator::Equal),
            just("!=").to(BinaryOperator::NotEqual),
            just("===").to(BinaryOperator::ReferenceEqual),
            just("!==").to(BinaryOperator::ReferenceNotEqual),
            just("<").to(BinaryOperator::LessThan),
            just("<=").to(BinaryOperator::LessThanOrEqual),
            just(">").to(BinaryOperator::GreaterThan),
            just(">=").to(BinaryOperator::GreaterThanOrEqual),
            just("&&").to(BinaryOperator::And),
            just("||").to(BinaryOperator::Or),
            just("xor").to(BinaryOperator::Xor),
            just("shl").to(BinaryOperator::Shl),
            just("shr").to(BinaryOperator::Shr),
            just("ushr").to(BinaryOperator::UShr),
            just("in").to(BinaryOperator::In),
            just("!in").to(BinaryOperator::NotIn),
            just("is").to(BinaryOperator::Is),
            just("is!").to(BinaryOperator::IsNot),
            just("..").to(BinaryOperator::RangeTo),
            just("..<").to(BinaryOperator::RangeUntil),
            just("+").to(BinaryOperator::Add),
            just("-").to(BinaryOperator::Subtract),
            just("*").to(BinaryOperator::Multiply),
            just("/").to(BinaryOperator::Divide),
            just("%").to(BinaryOperator::Modulo),
        ));

        let atom = choice((literal, variable, unary_expr)).boxed();

        let op = assign_op.or(binary_op).padded();
        let binary_expr = recursive(|expr| {
            atom.then(op.then(expr.clone()).repeated()).map(
                |(lhs, ops): (Expr, Vec<(BinaryOperator, Expr)>)| {
                    ops.into_iter().fold(lhs, |lhs, (op, rhs)| {
                        Expr::BinaryOp(Box::new(lhs), op, Box::new(rhs))
                    })
                },
            )
        });

        let call_expr = ident()
            .then(
                type_parser()
                    .separated_by(just(',').padded())
                    .delimited_by(just('<'), just('>'))
                    .padded()
                    .or_not(),
            )
            .then(
                expr.separated_by(just(',').padded())
                    .delimited_by(just('('), just(')'))
                    .padded(),
            )
            .map(|((name, attributes), args)| {
                Expr::Call(name, attributes.unwrap_or_default(), args)
            });

        choice((call_expr, binary_expr))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_binary_expr() {
        println!("{:#?}", expr_parser().parse("++1").unwrap());
        println!("{:#?}", expr_parser().parse("1 + 2 - 3").unwrap());

        // assert_eq!(
        //     expr_parser().parse("1 + 2 - 3"),
        //     Ok(Expr::BinaryOp(
        //         Box::new(Expr::Literal(Literal::Integer(1))),
        //         BinaryOperator::Add,
        //         Box::new(Expr::BinaryOp(
        //             Box::new(Expr::Literal(Literal::Integer(2))),
        //             BinaryOperator::Subtract,
        //             Box::new(Expr::Literal(Literal::Integer(3)))
        //         ))
        //     ))
        // );

        println!("{:#?}", expr_parser().parse("1 + 2 - 3").unwrap());
    }

    #[test]
    fn parse_function_call() {
        assert_eq!(
            expr_parser().parse("mapOf<String, Int?>(a, b, c)"),
            Ok(Expr::Call(
                "mapOf".to_string(),
                vec![
                    Type {
                        kind: TypeKind::String,
                        nullable: false,
                        attributes: vec![],
                    },
                    Type {
                        kind: TypeKind::Int,
                        nullable: true,
                        attributes: vec![],
                    }
                ],
                vec![
                    Expr::Variable("a".to_string()),
                    Expr::Variable("b".to_string()),
                    Expr::Variable("c".to_string())
                ]
            ))
        )
    }
}
