pub mod annotation;
pub mod enum_entry;
pub mod function;

use crate::ast::*;
use annotation::annotations_parser;
use chumsky::prelude::*;
use enum_entry::enum_entry_parser;
use function::function_parser;

pub fn declaration_parser<'a>(
    stmt_parser: impl Parser<char, Statement, Error = Simple<char>> + Clone + 'a,
) -> impl Parser<char, Declaration, Error = Simple<char>> + 'a {
    let stmt_parser = stmt_parser.clone();
    recursive(|decl| {
        annotations_parser()
            .repeated()
            .or_not()
            .then(choice((
                function_parser(stmt_parser.clone())
                    .map(DeclarationKind::Function),
                enum_entry_parser(decl.clone()).map(DeclarationKind::EnumEntry),
            )))
            .map(|(annotations, kind)| Declaration {
                annotations: annotations.unwrap_or_default(),
                kind,
            })
    })
}

pub fn modifier_parser() -> impl Parser<char, Modifier, Error = Simple<char>> {
    choice((
        just("public").to(Modifier::Public),
        just("internal").to(Modifier::Internal),
        just("private").to(Modifier::Private),
        just("protected").to(Modifier::Protected),
    ))
    .or(choice((
        just("abstract").to(Modifier::Abstract),
        just("final").to(Modifier::Final),
        just("open").to(Modifier::Open),
        just("annotation").to(Modifier::Annotation),
        just("sealed").to(Modifier::Sealed),
        just("data").to(Modifier::Data),
        just("override").to(Modifier::Override),
        just("lateinit").to(Modifier::Lateinit),
        just("inner").to(Modifier::Inner),
        just("in").to(Modifier::In),
        just("out").to(Modifier::Out),
        just("noinline").to(Modifier::NoInline),
        just("crossinline").to(Modifier::CrossInline),
        just("vararg").to(Modifier::Vararg),
        just("reified").to(Modifier::Reified),
        just("tailrec").to(Modifier::Tailrec),
        just("operator").to(Modifier::Operator),
        just("infix").to(Modifier::Infix),
        just("inline").to(Modifier::Inline),
        just("external").to(Modifier::External),
        just("suspend").to(Modifier::Suspend),
        just("const").to(Modifier::Const),
        just("actual").to(Modifier::Actual),
        just("expect").to(Modifier::Expect),
    )))
}
