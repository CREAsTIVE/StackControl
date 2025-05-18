use crate::{runtime::{stack::Stack, value::Value}};

use super::{core::define_commands, CommandExecutable, ExecutionResult};

fn test(stack: &mut Stack) -> ExecutionResult {
  stack.move_left();
  ExecutionResult::Success
}

define_commands!(define stack_manipulators 
  (
    MoveLeftCommand ('←') ["mvl"] to stack {
      stack.move_left();
      ExecutionResult::Success
    }
  ), (
    MoveRightCommand ('→') ["mvr"] to stack {
      stack.move_right();
      ExecutionResult::Success
    }
  )
);