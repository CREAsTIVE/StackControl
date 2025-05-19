use super::value::Value;

pub struct Stack {
  ahead_stack: Vec<Value>,
  stack: Vec<Value>
}

pub struct NoElementsAheadOfStackException {}
pub struct NoElementsOnStackException {}

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

  pub fn move_right(&mut self) -> Result<(), NoElementsAheadOfStackException> {
    self.stack.push(self.ahead_stack.pop().ok_or(NoElementsAheadOfStackException {})?);
    Ok(())
  }

  pub fn move_left(&mut self) -> Result<(), NoElementsOnStackException> {
    self.ahead_stack.push(self.stack.pop().ok_or(NoElementsOnStackException{})?);
    Ok(())
  }
}