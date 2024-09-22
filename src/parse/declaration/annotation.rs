use crate::{
    ast::*,
    parse::expression::{
        call::invocation_args_parser, path::path_parser,
    },
};
use chumsky::prelude::*;

pub fn annotations_parser(
    expr_parser: impl Parser<char, Expression, Error = Simple<char>>,
) -> impl Parser<char, AnnotationSet, Error = Simple<char>> {
    just('@')
        .ignore_then(annotation_site_parser().then_ignore(just(':')).or_not())
        .then(annotation_parser(expr_parser).repeated())
        .map(|(site, annotations)| AnnotationSet { site, annotations })
}

pub fn file_annotations_parser(
    expr_parser: impl Parser<char, Expression, Error = Simple<char>>,
) -> impl Parser<char, AnnotationSet, Error = Simple<char>> {
    just('@')
        .ignore_then(just("file:"))
        .ignore_then(annotation_parser(expr_parser).repeated())
        .map(|annotations| AnnotationSet {
            site: None,
            annotations,
        })
}

pub fn annotation_site_parser(
) -> impl Parser<char, AnnotationSite, Error = Simple<char>> {
    choice((
        just("field").to(AnnotationSite::Field),
        just("property").to(AnnotationSite::Property),
        just("get").to(AnnotationSite::Get),
        just("set").to(AnnotationSite::Set),
        just("receiver").to(AnnotationSite::Receiver),
        just("param").to(AnnotationSite::Param),
        just("setparam").to(AnnotationSite::SetParam),
        just("delegate").to(AnnotationSite::Delegate),
    ))
}

pub fn annotation_parser(
    expr_parser: impl Parser<char, Expression, Error = Simple<char>>,
) -> impl Parser<char, Annotation, Error = Simple<char>> {
    path_parser()
        .then(
            just('(')
                .ignore_then(invocation_args_parser(expr_parser))
                .then_ignore(just(')'))
                .or_not(),
        )
        .map(|(path, args)| Annotation {
            path,
            args: args.unwrap_or_default(),
        })
}
