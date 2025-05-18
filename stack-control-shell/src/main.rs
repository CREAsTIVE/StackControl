use std::io::stdin;

use itertools::Itertools;
use prompted::input;
use stack_control::{runtime::stack::Stack, utils::execution::execute};

fn main() {
  loop {
    let input = input!("> ");
    if input == "exit" {break;}

    let mut stack = Stack::new();

    execute(&input, &mut stack);
    let t = stack.copy().iter()
      .map(|e| e.to_string())
      .join(", ");

    println!("Stack: {t}");
  }
}
