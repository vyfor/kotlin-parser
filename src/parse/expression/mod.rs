use crate::ast::*;
use binary_op::binary_op_parser;
use call::call_expr_parser;
use chumsky::prelude::*;

mod binary_op;
mod call;
mod unary;
mod variable;

pub fn expr_parser() -> impl Parser<char, Expr, Error = Simple<char>> {
    recursive(|expr| {
        choice((call_expr_parser(expr.clone()), binary_op_parser(expr)))
            .padded()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_binary_expr() {
        println!("{:#?}", expr_parser().parse("1 + 2 * 3").unwrap());

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
