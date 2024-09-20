use crate::ast::Expression;
use chumsky::prelude::*;

use super::statement::statement_parser;

pub mod array_access;
pub mod break_expr;
pub mod call;
pub mod continue_expr;
pub mod for_expr;
pub mod if_expr;
pub mod label;
pub mod literal;
pub mod path;
pub mod try_expr;
pub mod when_expr;
pub mod while_expr;

pub fn expression_parser() -> impl Parser<char, Expression, Error = Simple<char>>
{
    let stmt_parser = statement_parser().boxed();
    recursive(|expr| {
        choice((
            break_expr::break_expr_parser(),
            continue_expr::continue_expr_parser(),
            for_expr::for_expr_parser(stmt_parser.clone(), expr.clone()),
            if_expr::if_expr_parser(stmt_parser.clone(), expr.clone()),
            try_expr::try_expr_parser(stmt_parser.clone()),
            when_expr::when_expr_parser(stmt_parser.clone(), expr.clone()),
            while_expr::while_expr_parser(stmt_parser.clone(), expr.clone()),
            array_access::array_access_parser(),
            call::call_parser(stmt_parser.clone(), expr.clone()),
        ))
        .padded()
        .map(Expression::from)
    })
}
