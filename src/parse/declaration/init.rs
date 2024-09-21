use crate::{ast::*, parse::expression::call::block_parser};
use chumsky::prelude::*;

pub fn init_block_parser(
    stmt_parser: impl Parser<char, Statement, Error = Simple<char>>,
) -> impl Parser<char, Block, Error = Simple<char>> {
    just("init").ignore_then(block_parser(stmt_parser))
}
