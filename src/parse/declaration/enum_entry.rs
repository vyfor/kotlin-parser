use chumsky::prelude::*;

use crate::{ast::*, parse::expression::call::call_args_parser};

use super::modifier_parser;

pub fn enum_entry_parser(
    decl_parser: impl Parser<char, Declaration, Error = Simple<char>>,
    expr_parser: impl Parser<char, Expression, Error = Simple<char>>,
) -> impl Parser<char, EnumEntryDeclaration, Error = Simple<char>> {
    modifier_parser()
        .repeated()
        .or_not()
        .then(text::ident().padded())
        .then(call_args_parser(expr_parser))
        .then(
            decl_parser
                .repeated()
                .delimited_by(just('{').padded(), just('}').padded())
                .or_not(),
        )
        .map(|(((modifiers, name), args), decls)| EnumEntryDeclaration {
            modifiers: modifiers.unwrap_or_default(),
            name,
            args,
            body: decls.unwrap_or_default(),
        })
}
