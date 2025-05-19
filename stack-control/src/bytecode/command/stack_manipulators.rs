use crate::{runtime::{stack::Stack, value::Value}};
use indoc::indoc;
use super::{core::define_commands, CommandExecutable, ExecutionResult};

fn test(stack: &mut Stack) -> ExecutionResult {
  stack.pop();
  ExecutionResult::Success
}

define_commands!(define stack_manipulators 
  (
    [
      MoveLeftCommand '←' ["mvl"]
      (indoc! {"
        Moves stack head cursor onto left
      "})
    ] to stack {
      stack.move_left();
      ExecutionResult::Success
    }
  ), (
    [
      MoveRightCommand '→' ["mvr"]
      (indoc! {"
        Moves stack head cursor onto right
      "})
    ] to stack {
      stack.move_right();
      ExecutionResult::Success
    }
  )
);