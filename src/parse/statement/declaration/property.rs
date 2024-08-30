use crate::ast::*;
use crate::parse::expression::expr_parser;
use crate::parse::statement::inheritance_parser;
use crate::parse::statement::visibility_parser;
use crate::parse::ty::type_parser;
use chumsky::prelude::*;
use chumsky::text;

pub fn property_parser() -> impl Parser<char, Stmt, Error = Simple<char>> {
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

    visibility_parser()
        .or_not()
        .then(inheritance_parser().or_not())
        .then(mutable)
        .then(name)
        .then(ty)
        .then(init)
        .map(
            |(((((visibility, inheritance), mutable), name), ty), init)| {
                Stmt::Declaration(DeclarationKind::Property {
                    visibility,
                    inheritance,
                    mutable,
                    name,
                    ty,
                    init,
                })
            },
        )
}
