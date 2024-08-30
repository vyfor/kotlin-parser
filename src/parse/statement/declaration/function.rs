use crate::ast::*;
use crate::parse::statement::inheritance_parser;
use crate::parse::statement::visibility_parser;
use crate::parse::ty::type_parser;
use chumsky::prelude::*;
use chumsky::text;

pub fn function_parser(
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

    visibility_parser()
        .or_not()
        .then(inheritance_parser().or_not())
        .then_ignore(just("fun").padded())
        .then(name)
        .then(params)
        .then(ty)
        .then(body)
        .map(
            |(((((visibility, inheritance), name), params), ty), body)| {
                Stmt::Declaration(DeclarationKind::Function {
                    visibility,
                    inheritance,
                    name,
                    params,
                    ty,
                    body,
                })
            },
        )
}
