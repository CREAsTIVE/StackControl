use itertools::Itertools;
use prompted::input;
use stack_control::{bytecode::commands::core::bind_default_commands, compiletime::{compiler::Scope, lexer::split_string_to_tokens}, runtime::stack::Stack, utils::execution::{execute_code, simplify_tokens, ExecutionException}};

fn main() {
  loop {
    let input = input!("> ");
    if input == "exit" {break;}

    let mut scope = Scope::new();
    bind_default_commands(&mut scope.command_map);

    let mut stack = Stack::new();

    let tokens = split_string_to_tokens(&input);
    println!("< {}", simplify_tokens(&tokens, &scope.command_map).map(|e| e.to_string()).join(""));
    

    match execute_code(&input, &mut stack) {
      Err(ExecutionException::Compilation(compile_error)) =>
        println!("Compilation error: {:?}", compile_error.to_string()),
      Err(ExecutionException::Runtime(runtime_error)) =>
        println!("Runtime error: {:?}", runtime_error.to_string()),

      Ok(()) => {
        let stack_string = stack.copy().iter()
          .map(|e| e.to_string())
          .join(", ");

          println!("Stack: {stack_string}");
        }
    };
  }
}
