use crate::runtime::{stack::Stack, value::Value};

use super::{CommandExecutable, ExecutionResult};

pub struct StackPusherCommand<'e, 'm> {
  pub value_to_push: Value<'e, 'm>
}

impl<'e, 'm> CommandExecutable<'e, 'm> for StackPusherCommand<'e, 'm> {
  fn execute(&self, stack: &mut Stack<'e, 'm>) -> super::ExecutionResult {
    stack.push(self.value_to_push.clone());
    ExecutionResult::Success
  }
}
