use std::{cell::{Cell, Ref, RefCell, RefMut}, rc::Rc, sync::Arc};

use itertools::Itertools;

use crate::bytecode::commands::DescribedCommand;


pub struct Array {
  pointer: Rc<RefCell<Vec<Value>>>,
  owned: Cell<bool>
}

impl Array {
  pub fn new() -> Self {
    Array { pointer: Rc::new(RefCell::new(Vec::new())), owned: Cell::new(true) }
  }

  pub fn from(vec: Vec<Value>) -> Self{
    Array { pointer: Rc::new(RefCell::new(vec)), owned: Cell::new(true) }
  }

  pub fn from_ref(vec: Rc<RefCell<Vec<Value>>>) -> Self {
    Array { pointer: vec, owned: Cell::new(false) }
  }

  pub fn get(&self) -> Ref<Vec<Value>> {
    self.pointer.borrow()
  }

  pub fn get_mut(&mut self) -> RefMut<Vec<Value>> {
    if !self.owned.get() { self.own(); }
    
    self.pointer.borrow_mut()
  }

  fn own(&mut self) {
    let clone = self.pointer.borrow().clone();
    self.set(clone);
  }
  
  pub fn set(&mut self, new_vec: Vec<Value>) {
    self.pointer = Rc::new(RefCell::new(new_vec));
    self.owned.set(true);
  }
}

impl Clone for Array {
  fn clone(&self) -> Self {
    // TODO: Custom RC or something like that
    // That will be owned only when exists 1 or less refs
    self.owned.set(false);  // On clone that array didn't owned. Temporary solution
    Array { pointer: self.pointer.clone(), owned: Cell::new(false) }
  }
}

#[derive(Clone)]
pub enum Value {
  Number(f64),
  Array(Array), // TODO: Use Cow instead
  OpenListIdentifier,
  CommandContainer(Arc<DescribedCommand>)
}

impl ToString for Value {
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