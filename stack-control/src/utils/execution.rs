use crate::{bytecode::command::stack_manipulators, compiletime::{self, command_map::CommandMap, compiler::CompileTime, lexer::split_string_to_tokens}, runtime::stack::Stack};

pub fn execute(code: &str, stack: &mut Stack) {
  let tokens = split_string_to_tokens(code);
  let mut compiletime = CompileTime::new();

  bind_default_commands(&mut compiletime.command_map);

  match compiletime.compile(tokens) {
    Ok(commands) => 
      commands.iter().for_each(|command| {command.execution.execute(stack); }),
      Err(err) => {let out = err.to_string(); println!("{out}")}
  }
}

pub fn bind_default_commands(map: &mut CommandMap) {
  stack_manipulators::stack_manipulators(map);
}