pub mod annotation;
pub mod constructor;
pub mod entity;
pub mod enum_entry;
pub mod function;
pub mod init;
pub mod typealias;

use crate::ast::*;
use annotation::annotations_parser;
use chumsky::prelude::*;
use constructor::constructor_parser;
use entity::entity_parser;
use enum_entry::enum_entry_parser;
use function::function_parser;
use init::init_block_parser;
use typealias::typealias_parser;

use super::expression::expression_parser;

pub fn declaration_parser<'a>(
    stmt_parser: impl Parser<char, Statement, Error = Simple<char>> + Clone + 'a,
) -> impl Parser<char, Declaration, Error = Simple<char>> + 'a {
    let expr_parser = expression_parser(stmt_parser.clone()).boxed();
    recursive(|decl| {
        annotations_parser(expr_parser.clone())
            .repeated()
            .or_not()
            .then(choice((
                function_parser(stmt_parser.clone(), expr_parser.clone())
                    .map(DeclarationKind::Function),
                enum_entry_parser(decl.clone(), expr_parser.clone())
                    .map(DeclarationKind::EnumEntry),
                init_block_parser(stmt_parser.clone())
                    .map(DeclarationKind::InitBlock),
                entity_parser(stmt_parser.clone(), expr_parser.clone())
                    .map(DeclarationKind::Entity),
                constructor_parser(stmt_parser.clone(), expr_parser.clone())
                    .map(DeclarationKind::Constructor),
                typealias_parser(expr_parser.clone())
                    .map(DeclarationKind::TypeAlias),
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
    .padded()
}
