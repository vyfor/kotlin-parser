use crate::ast::*;
use super::expressions::expr_parser;
use super::types::type_parser;
use chumsky::prelude::*;

pub fn stmt_parser() -> impl Parser<char, Stmt, Error = Simple<char>> + Clone {
    recursive(|stmt| {
        choice((
            var_decl_parser(),
            fun_decl_parser(stmt.clone()),
            if_stmt_parser(stmt.clone()),
            while_stmt_parser(stmt.clone()),
            for_stmt_parser(stmt.clone()),
            return_stmt_parser(),
            expr_stmt_parser(),
        ))
        .padded()
        .then_ignore(just(';').or(just('\n')).or(just('\r')).repeated())
        .padded()
    })
}

fn expr_stmt_parser() -> impl Parser<char, Stmt, Error = Simple<char>> {
    let expr = expr_parser();
    expr.map(|expr| Stmt::ExprStmt(Box::new(expr)))
}

fn var_decl_parser() -> impl Parser<char, Stmt, Error = Simple<char>> {
    let mutable = just("const")
        .padded()
        .or_not()
        .then(just("val"))
        .to(false)
        .or(just("var").to(true))
        .padded();
    let name = text::ident().padded();
    let ty = just(':')
        .padded()
        .ignore_then(type_parser())
        .or_not()
        .padded();
    let init = just("=")
        .or(just("by"))
        .padded()
        .ignore_then(expr_parser())
        .map(Box::new)
        .or_not()
        .padded();

    mutable.then(name).then(ty).then(init).map(
        |(((mutable, name), ty), init)| Stmt::VarDecl {
            mutable,
            name,
            ty,
            init,
        },
    )
}

fn fun_decl_parser(
    stmt_parser: impl Parser<char, Stmt, Error = Simple<char>> + Clone,
) -> impl Parser<char, Stmt, Error = Simple<char>> {
    let name = text::ident().padded();
    let params = text::ident()
        .padded()
        .then(just(':').padded().ignore_then(type_parser()))
        .padded()
        .separated_by(just(',').padded())
        .delimited_by(just('('), just(')'))
        .padded();
    let ty = just(':')
        .padded()
        .ignore_then(type_parser())
        .or_not()
        .padded();
    let body = choice((
        just("=")
            .padded()
            .ignore_then(stmt_parser.clone())
            .map(|stmt| Block {
                statements: vec![Box::new(stmt)],
            }),
        just("{")
            .padded()
            .ignore_then(stmt_parser.map(Box::new).repeated())
            .then_ignore(just("}").padded())
            .map(|stmts| Block { statements: stmts }),
    ));

    just("fun")
        .padded()
        .ignore_then(name)
        .then(params)
        .then(ty)
        .then(body)
        .map(|(((name, params), ty), body)| Stmt::FunDecl {
            name,
            params,
            ty,
            body,
        })
}

fn if_stmt_parser(
    stmt_parser: impl Parser<char, Stmt, Error = Simple<char>> + Clone,
) -> impl Parser<char, Stmt, Error = Simple<char>> {
    let if_branch = just("if")
        .padded()
        .then_ignore(just('(').padded())
        .ignore_then(expr_parser())
        .then_ignore(just(')').padded())
        .then(block_parser(stmt_parser.clone()))
        .map(|(condition, body)| (condition, body));

    let else_if_branches = just("else")
        .padded()
        .then(just("if").padded())
        .then_ignore(just('(').padded())
        .ignore_then(expr_parser())
        .then_ignore(just(')').padded())
        .then(block_parser(stmt_parser.clone()))
        .map(|(condition, body)| (condition, body))
        .repeated();

    let otherwise = just("else")
        .padded()
        .ignore_then(block_parser(stmt_parser)) // Use block_parser for the else body
        .or_not();

    if_branch.then(else_if_branches).then(otherwise).map(
        |((if_branch, else_if_branches), otherwise)| Stmt::If {
            branches: vec![if_branch]
                .into_iter()
                .chain(else_if_branches)
                .collect(),
            otherwise,
        },
    )
}

fn while_stmt_parser(
    stmt_parser: impl Parser<char, Stmt, Error = Simple<char>> + Clone,
) -> impl Parser<char, Stmt, Error = Simple<char>> {
    let condition = just("while")
        .padded()
        .then_ignore(just('(').padded())
        .ignore_then(expr_parser())
        .then_ignore(just(')').padded())
        .boxed();

    let while_loop = condition.clone().then(block_parser(stmt_parser.clone()));
    let do_while = just("do")
        .padded()
        .ignore_then(block_parser(stmt_parser))
        .then(condition);

    while_loop
        .map(|(condition, body)| Stmt::While {
            condition: Box::new(condition),
            body,
            do_while: false,
        })
        .or(do_while.map(|(body, condition)| Stmt::While {
            condition: Box::new(condition),
            body,
            do_while: true,
        }))
}

fn for_stmt_parser(
    stmt_parser: impl Parser<char, Stmt, Error = Simple<char>> + Clone,
) -> impl Parser<char, Stmt, Error = Simple<char>> {
    just("for")
        .padded()
        .then_ignore(just('(').padded())
        .ignore_then(expr_parser())
        .then_ignore(just(')').padded())
        .then(block_parser(stmt_parser))
        .map(|(condition, body)| Stmt::For {
            condition: Box::new(condition),
            body,
        })
}

fn return_stmt_parser() -> impl Parser<char, Stmt, Error = Simple<char>> {
    just("return")
        .padded()
        .ignore_then(expr_parser().or_not())
        .map(|expr| Stmt::Return(expr.map(Box::new)))
}

fn block_parser(
    stmt_parser: impl Parser<char, Stmt, Error = Simple<char>> + Clone,
) -> impl Parser<char, Block, Error = Simple<char>> {
    choice((
        just("{")
            .padded()
            .ignore_then(stmt_parser.clone().map(Box::new).repeated())
            .then_ignore(just("}").padded())
            .or_not()
            .map(|stmts| Block {
                statements: stmts.unwrap_or_default(),
            }),
        stmt_parser.or_not().map(|stmt| Block {
            statements: stmt.map_or_else(Vec::new, |stmt| vec![Box::new(stmt)]),
        }),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_variable_declaration() {
        assert_eq!(
            stmt_parser().parse("const val foo: Long = 123"),
            Ok(Stmt::VarDecl {
                mutable: false,
                name: "foo".to_string(),
                ty: Some(Type {
                    kind: TypeKind::Long,
                    nullable: false,
                    attributes: vec![],
                }),
                init: Some(Box::new(Expr::Literal(Literal::Integer(123))))
            })
        );
    }

    #[test]
    fn parse_function_declaration() {
        assert_eq!(
            stmt_parser().parse("fun foo(bar: Bar): Bazz? { return null; }"),
            Ok(Stmt::FunDecl {
                name: "foo".to_string(),
                params: vec![(
                    "bar".to_string(),
                    Type {
                        kind: TypeKind::Object("Bar".to_string()),
                        nullable: false,
                        attributes: vec![],
                    },
                )],
                ty: Some(Type {
                    kind: TypeKind::Object("Bazz".to_string()),
                    nullable: true,
                    attributes: vec![],
                }),
                body: Block {
                    statements: vec![Box::new(Stmt::Return(Some(Box::new(
                        Expr::Literal(Literal::Null),
                    ))))]
                },
            })
        );
    }

    #[test]
    fn parse_if_statement() {
        assert_eq!(
            stmt_parser().parse("if (a) { b } else if (c) { d } else { e }"),
            Ok(Stmt::If {
                branches: vec![
                    (
                        Expr::Variable("a".to_string()),
                        Block {
                            statements: vec![Box::new(Stmt::ExprStmt(
                                Box::new(Expr::Variable("b".to_string()))
                            ))]
                        }
                    ),
                    (
                        Expr::Variable("c".to_string()),
                        Block {
                            statements: vec![Box::new(Stmt::ExprStmt(
                                Box::new(Expr::Variable("d".to_string()))
                            ))]
                        }
                    )
                ],
                otherwise: Some(Block {
                    statements: vec![Box::new(Stmt::ExprStmt(Box::new(
                        Expr::Variable("e".to_string())
                    )))]
                }),
            })
        );
    }

    #[test]
    fn parse_while_statement() {
        assert_eq!(
            stmt_parser().parse("do {} while (true)"),
            Ok(Stmt::While {
                condition: Box::new(Expr::Literal(Literal::Boolean(true))),
                body: Block { statements: vec![] },
                do_while: true,
            })
        );
    }

    // #[test]
    // fn parse_for_statement() {
    //     assert_eq!(
    //         stmt_parser().parse("for (i in 1..10) { c++ }"),
    //         Ok(Stmt::For {
    //             condition: Box::new(Expr::BinaryOp(
}
