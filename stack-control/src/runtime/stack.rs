use std::collections::LinkedList;

use super::value::Value;

pub struct Stack {
  ahead_stack: Vec<Value>,
  stack: Vec<Value>
}

impl Stack {
  pub fn push(&mut self, value: Value) {
    self.stack.push(value);
  }

  pub fn pop(&mut self) -> Option<Value> {
    self.stack.pop()
  }

  pub fn move_right(&mut self) {
    self.stack.push(self.ahead_stack.pop().expect("FIXME"));
  }

  pub fn move_left(&mut self) {
    self.ahead_stack.push(self.stack.pop().expect("FIXME"));
  }
}