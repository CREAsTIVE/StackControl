use crate::bytecode::commands::RuntimeError;

use super::value::Value;

pub struct Stack<'a> {
  ahead_stack: Vec<Value<'a>>,
  stack: Vec<Value<'a>>
}

impl<'a> Stack<'a> {
  pub fn new() -> Self {
    Stack {
      ahead_stack: vec![],
      stack: vec![]
    }
  }

  pub fn copy(&self) -> Vec<Value<'a>> {
    let mut clone = self.stack.clone();
    let mut ahead = self.ahead_stack.clone(); ahead.reverse();
    clone.append(&mut ahead);
    clone
  }

  pub fn push(&mut self, value: Value<'a>) {
    self.stack.push(value);
  }

  pub fn pop(&mut self) -> Result<Value<'a>, RuntimeError> {
    self.stack.pop().ok_or(RuntimeError::NoElementsOnStack)
  }

  pub fn move_right(&mut self) -> Result<(), RuntimeError> {
    self.stack.push(self.ahead_stack.pop().ok_or(RuntimeError::NoElementsAheadOfStack)?);
    Ok(())
  }

  pub fn move_left(&mut self) -> Result<(), RuntimeError> {
    self.ahead_stack.push(self.stack.pop().ok_or(RuntimeError::NoElementsOnStack)?);
    Ok(())
  }
}