use crate::ast::*;
use chumsky::prelude::*;

pub fn simple_type_parser() -> impl Parser<char, Type, Error = Simple<char>> {
    recursive(|ty| {
        text::ident()
            .padded()
            .then(type_args_parser(ty).or_not())
            .then(just('?').or_not())
            .map(|((name, type_args), is_nullable)| {
                Type::Simple(Box::new(SimpleType {
                    name,
                    type_args: type_args.unwrap_or_default(),
                    is_nullable: is_nullable.is_some(),
                }))
            })
    })
}

pub fn type_args_parser(
    ty: impl Parser<char, Type, Error = Simple<char>>,
) -> impl Parser<char, Vec<Type>, Error = Simple<char>> {
    ty.padded()
        .separated_by(just(',').padded())
        .delimited_by(just('<').padded(), just('>').padded())
}
