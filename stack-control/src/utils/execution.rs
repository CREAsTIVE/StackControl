use std::sync::Arc;

use itertools::Itertools;

use crate::{bytecode::commands::{core::bind_default_commands, DescribedCommand, RuntimeException}, compiletime::{command_map::{self, CommandMap}, compiler::{CompilationException, Scope}, lexer::{split_string_to_tokens, CommandToken, Token}}, runtime::stack::Stack};

pub enum ExecutionException {
  Runtime(RuntimeException),
  Compilation(CompilationException)
}

pub fn simplify(tokens: &Vec<Token>, map: &CommandMap) -> impl Iterator<Item = Token> {
  tokens.iter().map(|e| match e {
    Token::CommandToken(CommandToken::CommandOrAlias(e)) => 
      Token::CommandToken(CommandToken::CommandOrAlias(String::from(map.get_by_alias(&e).unwrap_or(e)))),
    any => any.clone()
  })
}

pub fn join(tokens: impl Iterator<Item = Token>) -> String {
  tokens
    .map(|e| e.to_string())
    .join(", ")
}

pub fn execute_commands(commands: Vec<Arc<DescribedCommand>>, stack: &mut Stack) -> Result<(), RuntimeException> {
  for command in commands {
    command.execution.execute(stack)?;
  }
  Ok(())
}

pub fn execute_code(code: &str, stack: &mut Stack) -> Result<(), ExecutionException> {
  let tokens = split_string_to_tokens(code);
  let mut compiletime = Scope::new();

  bind_default_commands(&mut compiletime.command_map);

  match compiletime.compile(tokens) {
    Ok(commands) => 
      execute_commands(commands, stack).or_else(|e| Err(ExecutionException::Runtime(e))),

    Err(err) => {
      Err(ExecutionException::Compilation(err))
    }
  }
}

