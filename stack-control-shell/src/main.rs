use itertools::Itertools;
use prompted::input;
use stack_control::{bytecode::commands::core::bind_default_commands, compiletime::{compiler::Scope, lexer::split_string_to_tokens}, runtime::stack::Stack, utils::execution::{execute_code, join, simplify, ExecutionException}};

fn main() {
  loop {
    let input = input!("> ");
    if input == "exit" {break;}

    let mut scope = Scope::new();
    bind_default_commands(&mut scope.command_map);

    let mut stack = Stack::new();

    let tokens = split_string_to_tokens(&input);
    println!("< {}", simplify(&tokens, &scope.command_map).map(|e| e.to_string()).join(""));
    

    match execute_code(&input, &mut stack) {
      Err(ExecutionException::Compilation(compile_error)) =>
        println!("Compilation error: {:?}", compile_error.to_string()),
      Err(ExecutionException::Runtime(runtime_error)) =>
        println!("Runtime error: {:?}", Into::<&'static str>::into(runtime_error)),

      Ok(()) => {
        let stack_strin = stack.copy().iter()
          .map(|e| e.to_string())
          .join(", ");

          println!("Stack: {stack_strin}");
        }
    };
  }
}
