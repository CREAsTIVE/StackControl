use crate::{bytecode::command::{stack_manipulators, RuntimeException}, compiletime::{self, command_map::CommandMap, compiler::{CompilationException, CompileTime}, lexer::split_string_to_tokens}, runtime::stack::Stack};

pub enum ExecutionException {
  Runtime(RuntimeException),
  Compilation(CompilationException)
}

pub fn execute(code: &str, stack: &mut Stack) -> Result<(), ExecutionException> {
  let tokens = split_string_to_tokens(code);
  let mut compiletime = CompileTime::new();

  bind_default_commands(&mut compiletime.command_map);

  match compiletime.compile(tokens) {
    Ok(commands) => {
      commands
        .iter()
        .try_for_each(|command| { 
          command.execution.execute(stack).or_else(|e| Err(ExecutionException::Runtime(e)))
        })?;
      Ok(())
    },

    Err(err) => {
      Err(ExecutionException::Compilation(err))
    }
  }
}

pub fn bind_default_commands(map: &mut CommandMap) {
  stack_manipulators::stack_manipulators_group(map);
}