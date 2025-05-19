use crate::runtime::stack::Stack;
use indoc::indoc;
use super::{core::define_commands, CommandExecutable, RuntimeException};

fn test(stack: &mut Stack) -> Result<(), RuntimeException> {
      //
      let a = stack.pop()?;
      let b = stack.pop()?;
      stack.push(b.clone());
      stack.push(a.clone());
      stack.push(b);
      stack.push(a);
      Ok(())
}

define_commands!(define append_stack_manipulators
  (
    [
      MoveLeftCommand '←' ["mvl"]
      {
        description: String::from(indoc! {"
          Moves stack head cursor onto left
        "})
      }
      
    ] stack {
      stack.move_left()?;
      Ok(())
    }
  ), (
    [
      MoveRightCommand '→' ["mvr"]
      {
        description: String::from(indoc! {"
          Moves stack head cursor onto right
        "})
      }
    ] stack {
      stack.move_right()?;
      Ok(())
    }
  ), (
    [
      DupCommand '.' ["dup"]
      {
        description: String::from(indoc! {"
          Duplicates value on stack
        "})
      }
    ] stack {
      let v = stack.pop()?;
      stack.push(v.clone());
      stack.push(v);
      Ok(())
    }
  ), (
    [
      Dup2Command ':' ["ddup", "dup2"]
      {
        description: String::from(indoc! {"
          Duplicates 2 values on stack
        "})
      }
    ] stack {
      let a = stack.pop()?;
      let b = stack.pop()?;
      stack.push(b.clone());
      stack.push(a.clone());
      stack.push(b);
      stack.push(a);
      Ok(())
    }
  )
);