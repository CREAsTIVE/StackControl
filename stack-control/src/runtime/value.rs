use std::{cell::{Cell, Ref, RefCell, RefMut}, rc::Rc};

use itertools::Itertools;

use crate::bytecode::commands::{DescribedCommand, RuntimeError};


pub struct Array<'a> {
  pointer: Rc<RefCell<Vec<Value<'a>>>>,
  owned: Cell<bool>
}

impl<'a> Array<'a> {
  pub fn new() -> Self {
    Array { pointer: Rc::new(RefCell::new(Vec::new())), owned: Cell::new(true) }
  }

  pub fn from(vec: Vec<Value<'a>>) -> Self {
    Array { pointer: Rc::new(RefCell::new(vec)), owned: Cell::new(true) }
  }

  pub fn from_ref(vec: Rc<RefCell<Vec<Value<'a>>>>) -> Self {
    Array { pointer: vec, owned: Cell::new(false) }
  }

  pub fn get(&self) -> Ref<Vec<Value<'a>>> {
    self.pointer.borrow()
  }

  pub fn get_mut(&mut self) -> RefMut<Vec<Value<'a>>> {
    if !self.owned.get() { self.own(); }
    
    self.pointer.borrow_mut()
  }

  pub fn move_out(self) -> Vec<Value<'a>> {
    self.pointer.take()
  }

  fn own(&mut self) {
    let clone = self.pointer.borrow().clone();
    self.set(clone);
  }
  
  pub fn set(&mut self, new_vec: Vec<Value<'a>>) {
    self.pointer = Rc::new(RefCell::new(new_vec));
    self.owned.set(true);
  }
}

impl<'a> Clone for Array<'a> {
  fn clone(&self) -> Self {
    // TODO: Custom RC or something like that
    // That will be owned only when exists 1 or less refs
    self.owned.set(false);  // On clone that array didn't owned. Temporary solution
    Array { pointer: self.pointer.clone(), owned: Cell::new(false) }
  }
}

#[derive(Clone)]
pub enum Value<'a> {
  Number(f64),
  Array(Array<'a>), // TODO: Use Cow instead
  OpenListIdentifier,
  CommandContainer(DescribedCommand<'a>)
}

impl<'a> Value<'a> {
  pub fn b_true() -> Value<'a> { Value::Number(1.) }
  pub fn b_false() -> Value<'a> { Value::Number(0.) }

  pub fn bool(&self) -> bool {
    match self {
      Value::Number(n) => *n != 0.,
      Value::Array(arr) => arr.get().len() > 0,
      Value::CommandContainer(_) => panic!("Not implemented!"),
      Value::OpenListIdentifier => true
    }
  }

  pub fn invoke(&self, stack: &mut super::stack::Stack<'a>) -> Result<(), RuntimeError> {
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

impl<'a> PartialEq for Value<'a> {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Self::Number(l0), Self::Number(r0)) => l0 == r0,
      (Self::Array(l0), Self::Array(r0)) => *l0.get() == *r0.get(),
      (Self::CommandContainer(l0), Self::CommandContainer(r0)) => 
        panic!("Command == Command not implemented yet!"),
      _ => core::mem::discriminant(self) == core::mem::discriminant(other),
    }
  }
}

impl<'a> ToString for Value<'a> {
  fn to_string(&self) -> String {
    match self {
      Self::Number(num) => num.to_string(),
      Self::Array(arr) => {
        let strarr = arr.get().iter()
          .map(|e| e.to_string())
          .join(", ");

        format!("[{strarr}]")
      },
      Self::CommandContainer(cmd) =>
        String::from("◉") + &cmd.execution.to_string(),
      
      Self::OpenListIdentifier => String::from("[")
        
    }
  }
}