use crate::runtime::value::Value;

use super::CommandExecutable;

pub struct ListGeneratorCommand {}

impl<'e, 's> CommandExecutable<'e, 's> for ListGeneratorCommand {
  fn execute(&self, stack: &mut crate::runtime::stack::Stack) -> super::ExecutionResult {
    let mut new_list = Vec::new();
    while let Some(v) = stack.pop() {
      if let Value::OpenListIdentifier = v {break;}
      new_list.push(v);
    }
    
    stack.push(Value::Array(new_list));
    super::ExecutionResult::Success
  }
}