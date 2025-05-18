use crate::runtime::{stack::Stack, value::Value};

use super::{CommandExecutable, ExecutionResult};

pub struct StackPusherCommand {
  pub value_to_push: Value
}

impl CommandExecutable for StackPusherCommand {
  fn execute(&self, stack: &mut Stack) -> super::ExecutionResult {
    stack.push(self.value_to_push.clone());
    ExecutionResult::Success
  }
}
