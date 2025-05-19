use indoc::indoc;
use crate::runtime::{stack::Stack, value::{Array, Value}};

use super::{CommandExecutable, RuntimeException};

macro_rules! define_commands {
  (define $group:ident $(($metadata:tt to $stack:ident $defenitions:block)),+) => {
    define_commands!{defenition $(($metadata to $stack $defenitions)),+}
    pub fn $group(cmd_map: &mut $crate::compiletime::command_map::CommandMap) {
      define_commands!{meta cmd_map $(($metadata to $stack $defenitions)),+}
    }
  };

  (meta $command_map:ident ([$name:ident $key:tt [$($alias:expr),*] $description:expr] to $stack:ident $defenition:block)) => {
    $command_map.set($crate::bytecode::command::DescribedCommand {
      execution: Box::new($name {}),
      meta: std::sync::Arc::new($crate::bytecode::command::CommandMeta {
        key: $key,
        aliases: [$($alias),*].iter().map(|s: &&str| s.to_string()).collect::<Vec<String>>(),
        description: String::from($description)
      })
    });
  };

  (meta $command_map:ident ($metadata:tt to $stack:ident $defenition:block), $(($metadatas:tt to $stacks:ident $defenitions:block)),+) => {{
    define_commands!{meta $command_map ($metadata to $stack $defenition)}
    define_commands!{meta $command_map $(($metadatas to $stacks $defenitions)),+}
  }};

  (defenition ([$name:ident $key:tt [$($alias:expr),*] $description:tt] to $stack:ident $defenition:block)) => {
    struct $name {}
    impl CommandExecutable for $name {
      fn execute(&self, $stack: &mut $crate::runtime::stack::Stack) -> Result<(), RuntimeException> $defenition

      fn to_string(&self) -> String {
        String::from($key)
      }
    }
  };

  (defenition ($metadata:tt to $stack:ident $defenition:block), $(($metadatas:tt to $stacks:ident $defenitions:block)),+) => {
    define_commands!{defenition ($metadata to $stack $defenition)}
    define_commands!{defenition $(($metadatas to $stacks $defenitions)),+}
  };
}

pub(crate) use define_commands;




define_commands!(define test_group 
  (
    [
      RandomMacro 'k' ["alias", "otheralias"]
      (indoc! {"
        This is test command.
        That does nothing.
      "})
    ] to stack {
      Ok(())
    }
  )
);

// Always implemented command (strictly required for compilation)

pub struct ListGeneratorCommand {}

impl CommandExecutable for ListGeneratorCommand {
  fn execute(&self, stack: &mut crate::runtime::stack::Stack) -> Result<(), RuntimeException> {
    let mut new_list = Vec::new();
    loop {
      let v = stack.pop()?;
      if let Value::OpenListIdentifier = v {break;}
      new_list.push(v);
    }
    new_list.reverse();
    
    stack.push(Value::Array(Array::from(new_list)));
    Ok(())
  }

  fn to_string(&self) -> String {
    String::from("]")
  }
}


pub struct StackPusherCommand {
  pub value_to_push: Value
}
impl CommandExecutable for StackPusherCommand {
  fn execute(&self, stack: &mut Stack) -> Result<(), RuntimeException> {
    stack.push(self.value_to_push.clone());
    Ok(())
  }

  fn to_string(&self) -> String {
    let val = self.value_to_push.to_string();
    format!("↓{val}")
  }
}