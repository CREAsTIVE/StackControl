use crate::runtime::{stack::Stack, value::{Array, Value}};

use super::{core::define_commands, RuntimeException};
use indoc::indoc;

fn _test(stack: &mut Stack) -> Result<(), RuntimeException> {
      if let Value::Number(num) = stack.pop()? {
        stack.push(Value::Array(Array::from(
          (0..(num.round() as i32)).map(|n| Value::Number(n.into())).collect()
        )));
      } Err(RuntimeException::WrongElementType)
}

define_commands!(append_iters
  EachCommand('∴', ["map", "each"]) {
    with {
      description: String::from(indoc! {"
        Applies function to each value
      "})
    }
    stack {
      let f = stack.pop()?;
      if let Value::Array(arr) = stack.pop()? {
        let old = arr.move_out();
        let mut new = vec![];
        for e in old {
          stack.push(e);
          f.invoke(stack)?;
          new.push(stack.pop()?);
        }
        stack.push(Value::Array(Array::from(new)));
      } else { return Err(RuntimeException::WrongElementType) }
      Ok(())
    }
  },

  EmptyArrayCommand('∅', ["aempty", "emptyarr"]) {
    with {
      description: String::from(indoc! {"
        Places an empty array on top of stack
      "})
    }
    stack {
      stack.push(Value::Array(Array::new()));
      Ok(())
    }
  },

  Range('⇡', ["range"]) {
    with {

    }
    stack {
      if let Value::Number(num) = stack.pop()? {
        stack.push(Value::Array(Array::from(
          (0..(num.round() as i32)).map(|n| Value::Number(n.into())).collect()
        )));
        return Ok(())
      } Err(RuntimeException::WrongElementType)
    }
  }
);