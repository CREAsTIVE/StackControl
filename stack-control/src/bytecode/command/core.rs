use std::sync::Arc;

use crate::{compiletime::command_map::{self, CommandMap}, runtime::{stack::Stack, value::Value}};

use super::{CommandExecutable, CommandMeta, DescribedCommand, ExecutionResult};

macro_rules! define_commands {
  (define $group:ident $(($names:ident ($keys:expr) [$($aliass:expr),*] to $stack:ident $defenitions:block)),+) => {
    define_commands!{defenition $(($names ($keys) [$($aliass),*] to $stack $defenitions)),+}
    pub fn $group(cmd_map: &mut crate::compiletime::command_map::CommandMap) {
      define_commands!{meta cmd_map $(($names ($keys) [$($aliass),*] to $stack $defenitions)),+}
    }
  };

  (meta $command_map:ident ($name:ident ($key:expr) [$($alias:expr),*] to $stack:ident $defenition:block)) => {
    $command_map.set(crate::bytecode::command::DescribedCommand {
      execution: Box::new($name {}),
      meta: std::sync::Arc::new(crate::bytecode::command::CommandMeta {
        name: $key,
        aliases: [$($alias),*].iter().map(|s: &&str| s.to_string()).collect::<Vec<String>>()
      })
    });
  };

  (meta $command_map:ident ($name:ident ($key:expr) [$($alias:expr),*] to $stack:ident $defenition:block), $(($names:ident ($keys:expr) [$($aliass:expr),*] to $stacks:ident $defenitions:block)),+) => {{
    define_commands!{meta $command_map ($name ($key) [$($alias),*] to $stack $defenition)}
    define_commands!{meta $command_map $(($names ($keys) [$($aliass),*] to $stacks $defenitions)),+}
  }};

  (defenition ($name:ident ($key:expr) [$($alias:expr),*] to $stack:ident $defenition:block)) => {
    struct $name {}
    impl CommandExecutable for $name {
      fn execute(&self, $stack: &mut Stack) -> ExecutionResult $defenition

      fn to_string(&self) -> String {
        String::from($key)
      }
    }
  };

  (defenition ($name:ident ($key:expr) [$($alias:expr),*] to $stack:ident $defenition:block), $(($names:ident ($keys:expr) [$($aliass:expr),*] to $stacks:ident $defenitions:block)),+) => {
    define_commands!{defenition ($name ($key) [$($alias),*] to $stack $defenition)}
    define_commands!{defenition $(($names ($keys) [$($aliass),*] to $stacks $defenitions)),+}
  };
}

pub(crate) use define_commands;

/*
define_commands!(define test_group 
  (
    RandomMacro ('r') ["RandomMacro", "Other"] to $stack:ident {
      ExecutionResult::Success
    }
  ),
  (
    AnotherMacro ('e') [] to $stack:ident {
      ExecutionResult::Success
    }
  )
);
*/

// Always implemented command (strictly required for compilation)

pub struct ListGeneratorCommand {}

impl CommandExecutable for ListGeneratorCommand {
  fn execute(&self, stack: &mut crate::runtime::stack::Stack) -> super::ExecutionResult {
    let mut new_list = Vec::new();
    while let Some(v) = stack.pop() {
      if let Value::OpenListIdentifier = v {break;}
      new_list.push(v);
    }
    
    stack.push(Value::Array(new_list));
    super::ExecutionResult::Success
  }

  fn to_string(&self) -> String {
    String::from("]")
  }
}


pub struct StackPusherCommand {
  pub value_to_push: Value
}
impl CommandExecutable for StackPusherCommand {
  fn execute(&self, stack: &mut Stack) -> super::ExecutionResult {
    stack.push(self.value_to_push.clone());
    ExecutionResult::Success
  }

  fn to_string(&self) -> String {
    let val = self.value_to_push.to_string();
    format!("↓{val}")
  }
}