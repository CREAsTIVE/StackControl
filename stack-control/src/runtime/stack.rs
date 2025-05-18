
use std::fmt::Debug;

use super::value::Value;

pub struct Stack {
  ahead_stack: Vec<Value>,
  stack: Vec<Value>
}

impl Stack {
  pub fn new() -> Self {
    Stack {
      ahead_stack: vec![],
      stack: vec![]
    }
  }

  pub fn copy(&self) -> Vec<Value> {
    let mut clone = self.stack.clone();
    let mut ahead = self.ahead_stack.clone(); ahead.reverse();
    clone.append(&mut ahead);
    clone
  }

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