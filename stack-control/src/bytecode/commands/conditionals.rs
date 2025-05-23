use super::core::define_commands;
use indoc::indoc;

define_commands!(append_conditionals 
  IfCommand ('?', ["if"]) {
    with {
      description: String::from(indoc! {"
        Consumes function and value, 
        invokes function if value is true
      "})
    }
    stack {
      let truely = stack.pop()?;
      let val = stack.pop()?.bool();
      if val {truely.invoke(stack)?;}
      Ok(())
    }
  },
  IfElseCommand ('⁇', ["ifelse"]) {
    with {
      description: String::from(indoc! {"
        Consumes to functions and value.
        Invokes second if value is true, otherwise invokes first
      "})
    }
    stack {
      let falsely = stack.pop()?;
      let truely = stack.pop()?;
      let val = stack.pop()?.bool();
      (if val {truely} else {falsely})
        .invoke(stack)?;
      Ok(())
    }
  }
);