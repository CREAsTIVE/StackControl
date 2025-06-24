use std::{cell::{Cell, Ref, RefCell, RefMut}, rc::Rc, sync::Arc};
use std::fmt::Display;
use itertools::Itertools;

use crate::bytecode::commands::{DescribedCommand, RuntimeException};


pub struct Array {
  pointer: Rc<RefCell<Vec<Value>>>,
  unique: Cell<bool>
}

impl Array {
  pub fn new() -> Self {
    Array { pointer: Rc::new(RefCell::new(Vec::new())), unique: Cell::new(true) }
  }

  pub fn from(vec: Vec<Value>) -> Self{
    Array { pointer: Rc::new(RefCell::new(vec)), unique: Cell::new(true) }
  }

  pub fn from_ref(vec: Rc<RefCell<Vec<Value>>>) -> Self {
    Array { pointer: vec, unique: Cell::new(false) }
  }

  pub fn get(&self) -> Ref<Vec<Value>> {
    self.pointer.borrow()
  }

  pub fn get_mut(&mut self) -> RefMut<Vec<Value>> {
    self.own().pointer.borrow_mut()
  }

  pub fn move_out(mut self) -> Vec<Value> {
    self.own();
    self.pointer.take()
  }

  fn own(&mut self) -> &mut Self {
    if self.unique.get() { return self; }
    let clone = self.pointer.borrow().clone();
    self.set(clone);
    self
  }
  
  pub fn set(&mut self, new_vec: Vec<Value>) {
    self.pointer = Rc::new(RefCell::new(new_vec));
    self.unique.set(true);
  }
}

impl Clone for Array {
  fn clone(&self) -> Self {
    // TODO: Custom RC or something like that
    // That will be owned only when exists 1 or less refs
    self.unique.set(false);  // On clone that array didn't owned. Temporary solution
    Array { pointer: self.pointer.clone(), unique: Cell::new(false) }
  }
}

#[derive(Clone)]
pub enum Value {
  Number(f64),
  Array(Array), // TODO: Use Cow instead
  OpenListIdentifier,
  CommandContainer(Arc<DescribedCommand>)
}

impl Value {
  pub fn b_true() -> Value { Value::Number(1.) }
  pub fn b_false() -> Value {Value::Number(0.)}

  pub fn bool(&self) -> bool {
    match self {
      Value::Number(n) => *n != 0.,
      Value::Array(arr) => arr.get().len() > 0,
      Value::CommandContainer(_) => panic!("Not implemented!"),
      Value::OpenListIdentifier => true
    }
  }

  pub fn invoke(&self, stack: &mut super::stack::Stack) -> Result<(), RuntimeException> {
    match self {
      Value::Array(array) => {
        for e in array.get().iter() {
          e.invoke(stack)?;
        }
        Ok(())
      },

      Value::CommandContainer(described_command) => 
        described_command.execution.execute(stack),

      any => {
        let other = stack.pop()?;
        stack.push(if other == *any { Value::b_true() } else { Value::b_false() });
        Ok(())
      }
    }
  }
}

impl PartialEq for Value {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Self::Number(l0), Self::Number(r0)) => l0 == r0,
      (Self::Array(l0), Self::Array(r0)) => *l0.get() == *r0.get(),
      (Self::CommandContainer(_l0), Self::CommandContainer(_r0)) => 
        panic!("Command == Command not implemented yet!"),
      _ => core::mem::discriminant(self) == core::mem::discriminant(other),
    }
  }
}

impl Display for Value {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Number(num) => write!(f, "{}", num),
      Self::Array(arr) =>
        write!(f, "[{}]", 
          arr.get().iter()
            .map(|e| e.to_string())
            .join(" ")
        )
      ,
      Self::CommandContainer(cmd) =>
        write!(f, "{}{}", "◉", &cmd.execution.to_string()),

      Self::OpenListIdentifier => write!(f, "{}", "[")
    }
  }
}