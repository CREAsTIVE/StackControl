use crate::{compiletime::{compiler::CompileTime, lexer::split_string_to_tokens}, runtime::stack::Stack};

use super::Holder;

fn test() {
  let copmiletime = CompileTime::new();
  let e = copmiletime.compile(split_string_to_tokens("abc"));
}