pub mod declaration;

use crate::ast::*;
use chumsky::prelude::*;
use declaration::declaration_parser;

use super::expression::expr_parser;

pub fn stmt_parser() -> impl Parser<char, Stmt, Error = Simple<char>> + Clone {
    recursive(|stmt| {
        choice((
            declaration_parser(stmt.clone()),
            expr_stmt_parser(),
            return_parser(),
            scope_parser(stmt),
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

pub fn return_parser() -> impl Parser<char, Stmt, Error = Simple<char>> {
    just("return")
        .padded()
        .ignore_then(expr_parser().or_not())
        .map(|expr| Stmt::Return(expr.map(Box::new)))
}

pub fn block_parser(
    stmt_parser: impl Parser<char, Stmt, Error = Simple<char>> + Clone,
) -> impl Parser<char, Block, Error = Simple<char>> {
    choice((
        just('{')
            .padded()
            .ignore_then(stmt_parser.clone().map(Box::new).repeated())
            .then_ignore(just('}').padded())
            .or_not()
            .map(|stmts| Block {
                statements: stmts.unwrap_or_default(),
            }),
        stmt_parser.or_not().map(|stmt| Block {
            statements: stmt.map_or_else(Vec::new, |stmt| vec![Box::new(stmt)]),
        }),
    ))
}

pub fn scope_parser(
    stmt_parser: impl Parser<char, Stmt, Error = Simple<char>> + Clone,
) -> impl Parser<char, Stmt, Error = Simple<char>> {
    just('{')
        .padded()
        .ignore_then(stmt_parser.clone().map(Box::new).repeated())
        .then_ignore(just('}').padded())
        .or_not()
        .map(|stmts| {
            Stmt::Scope(Block {
                statements: stmts.unwrap_or_default(),
            })
        })
}

pub fn visibility_parser(
) -> impl Parser<char, VisibilityModifier, Error = Simple<char>> {
    choice((
        just("public").to(VisibilityModifier::Public),
        just("internal").to(VisibilityModifier::Internal),
        just("protected").to(VisibilityModifier::Protected),
        just("private").to(VisibilityModifier::Private),
    ))
    .padded()
}

pub fn inheritance_parser(
) -> impl Parser<char, InheritanceModifier, Error = Simple<char>> {
    choice((
        just("open").to(InheritanceModifier::Open),
        just("final").to(InheritanceModifier::Final),
        just("abstract").to(InheritanceModifier::Abstract),
    ))
    .padded()
}
