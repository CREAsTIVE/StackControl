use std::collections::LinkedList;

use super::value::Value;

pub struct Stack<'e, 'm> {
  ahead_stack: Vec<Value<'e, 'm>>,
  stack: Vec<Value<'e, 'm>>
}

impl<'e, 'm> Stack<'e, 'm> {
  pub fn push(&mut self, value: Value<'e, 'm>) {
    self.stack.push(value);
  }

  pub fn pop(&mut self) -> Option<Value<'e, 'm>> {
    self.stack.pop()
  }

  pub fn move_right(&mut self) {
    self.stack.push(self.ahead_stack.pop().expect("FIXME"));
  }

  pub fn move_left(&mut self) {
    self.ahead_stack.push(self.stack.pop().expect("FIXME"));
  }
}