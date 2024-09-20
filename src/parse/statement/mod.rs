use crate::ast::*;
use chumsky::prelude::*;

use super::{declaration::declaration_parser, expression::expression_parser};

pub fn statement_parser() -> impl Parser<char, Statement, Error = Simple<char>>
{
    recursive(|stmt| {
        choice((
            declaration_parser(stmt).map(Statement::Declaration),
            expression_parser().map(Statement::Expression),
        ))
    })
}
