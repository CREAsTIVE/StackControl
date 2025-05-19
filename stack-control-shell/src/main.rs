use itertools::Itertools;
use prompted::input;
use stack_control::{runtime::stack::Stack, utils::execution::{execute, ExecutionException}};

fn main() {
  loop {
    let input = input!("> ");
    if input == "exit" {break;}

    let mut stack = Stack::new();

    match execute(&input, &mut stack) {
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
