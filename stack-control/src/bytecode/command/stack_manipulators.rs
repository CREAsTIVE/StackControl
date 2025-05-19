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

define_commands!(define stack_manipulators_group
  (
    [
      MoveLeftCommand '←' ["mvl"]
      (indoc! {"
        Moves stack head cursor onto left
      "})
    ] to stack {
      stack.move_left()?;
      Ok(())
    }
  ), (
    [
      MoveRightCommand '→' ["mvr"]
      (indoc! {"
        Moves stack head cursor onto right
      "})
    ] to stack {
      stack.move_right()?;
      Ok(())
    }
  ), (
    [
      DupCommand '.' ["dup"]
      (indoc! {"
        Duplicates value on stack
      "})
    ] to stack {
      let v = stack.pop()?;
      stack.push(v.clone());
      stack.push(v);
      Ok(())
    }
  ), (
    [
      Dup2Command ':' ["ddup", "dup2"]
      (indoc! {"
        Duplicates 2 values on stack
      "})
    ] to stack {
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