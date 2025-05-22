use indoc::indoc;
use crate::{compiletime::command_map::CommandMap, runtime::{stack::Stack, value::{Array, Value}}};

use super::{math::append_math, stack_manipulators::append_stack_manipulators, CommandExecutable, CommandMeta, DescribedCommand, DescribedCommandMaker, RuntimeException};

pub fn bind_default_commands(map: &mut CommandMap) {
  append_stack_manipulators(map);
  append_math(map);
}

fn _test(_stack: &mut Stack) -> Result<(), RuntimeException> {
      //
      Ok(())
}

/*
macro_rules! define_commands {
  (define $group:ident $(($metadata:tt $stack:ident $defenitions:block)),+) => {
    define_commands!{defenition $(($metadata $stack $defenitions)),+}
    pub fn $group(cmd_map: &mut $crate::compiletime::command_map::CommandMap) {
      define_commands!{meta cmd_map $(($metadata $stack $defenitions)),+}
    }
  };

  // With additional fields
  (meta $command_map:ident ([$name:ident $key:tt [$($alias:expr),*] {$($vkey:ident : $value:expr),+}] $stack:ident $defenition:block)) => {
    $command_map.set($crate::bytecode::commands::DescribedCommand {
      execution: Box::new($name {}),
      meta: std::sync::Arc::new($crate::bytecode::commands::CommandMeta {
        key: String::from($key),
        aliases: [$($alias),*].iter().map(|s: &&str| s.to_string()).collect::<Vec<String>>(),
        $($vkey : $value),+,
        ..core::default::Default::default()
      })
    });
  };

  // No additional fields
  (meta $command_map:ident ([$name:ident $key:tt [$($alias:expr),*] {}] $stack:ident $defenition:block)) => {
    $command_map.set($crate::bytecode::commands::DescribedCommand {
      execution: Box::new($name {}),
      meta: std::sync::Arc::new($crate::bytecode::command::CommandMeta {
        key: $key,
        aliases: [$($alias),*].iter().map(|s: &&str| s.to_string()).collect::<Vec<String>>(),
        ..core::default::Default::default()
      })
    });
  };

  (meta $command_map:ident ($metadata:tt $stack:ident $defenition:block), $(($metadatas:tt $stacks:ident $defenitions:block)),+) => {{
    define_commands!{meta $command_map ($metadata $stack $defenition)}
    define_commands!{meta $command_map $(($metadatas $stacks $defenitions)),+}
  }};

  (defenition ([$name:ident $key:tt [$($alias:expr),*] $others:tt] $stack:ident $defenition:block)) => {
    pub struct $name {}
    impl $crate::bytecode::commands::CommandExecutable for $name {
      fn execute(&self, $stack: &mut $crate::runtime::stack::Stack) -> Result<(), $crate::bytecode::commands::RuntimeException> $defenition

      fn to_string(&self) -> String {
        String::from($key)
      }
    }
  };

  (defenition ($metadata:tt $stack:ident $defenition:block), $(($metadatas:tt $stacks:ident $defenitions:block)),+) => {
    define_commands!{defenition ($metadata $stack $defenition)}
    define_commands!{defenition $(($metadatas $stacks $defenitions)),+}
  };
}
*/

macro_rules! define_commands {
  ($group:ident $($name:ident $args:tt $body:tt),*) => {
    pub fn $group(cmd_map: &mut $crate::compiletime::command_map::CommandMap) {
      $(cmd_map.set(<$name as $crate::bytecode::commands::DescribedCommandMaker>::make_described_command()));*
    }
    define_commands! {_ defenitions $($name $args $body),*}
  };

  (_ defenitions $($name:ident $args:tt $body:tt),*) => {
    $(define_commands! {_ defenition $name $args $body})*
  };

  (_ defenition $name:ident ($key:expr, [$($alias:tt),*]) {with {$($pkey:ident : $pvalue:expr),*} $stack:ident $code:block}) => {
    pub struct $name {}

    impl $crate::bytecode::commands::CommandExecutable for $name {
      fn execute(&self, $stack: &mut $crate::runtime::stack::Stack) -> 
        Result<(), $crate::bytecode::commands::RuntimeException> 
      $code

      fn to_string(&self) -> String {
        String::from($key)
      }
    }

    impl $crate::bytecode::commands::DescribedCommandMaker for $name {
      fn make_described_command() -> $crate::bytecode::commands::DescribedCommand {
        $crate::bytecode::commands::DescribedCommand {
          execution: Box::new(Self {}),
          meta: std::sync::Arc::new($crate::bytecode::commands::CommandMeta {
            key: String::from($key),
            aliases: [$($alias),*].iter().map(|s: &&str| s.to_string()).collect::<Vec<String>>(),
            $($pkey : $pvalue,)*
            ..core::default::Default::default()
          })
        }
      }
    }
  };
}

pub(crate) use define_commands;

// define_commands!(append_test_commands 
//   RandomMacro ('k', ["alias", "otheralias"]) {
//     with {
//       description: String::from(indoc! {"
//         This is test command.
//         That does nothing.
//       "})
//     }
//     _stack {
//       Ok(())
//     }
//   },
//   RandomMacro2 ('k', ["alias", "otheralias"]) {
//     with {
//       description: String::from(indoc! {"
//         This is test command.
//         That does nothing.
//       "})
//     }
//     _stack {
//       Ok(())
//     }
//   }
// );

// Always implemented command (strictly required for compilation)

pub struct ListGeneratorCommand {}

impl crate::bytecode::commands::CommandExecutable for ListGeneratorCommand {
  fn execute(&self, stack: &mut crate::runtime::stack::Stack) -> Result<(), crate::bytecode::commands::RuntimeException> {
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

impl crate::bytecode::commands::DescribedCommandMaker for ListGeneratorCommand {
  fn make_described_command() -> DescribedCommand {
    crate::bytecode::commands::DescribedCommand {
      execution: Box::new(Self {}),
      meta: std::sync::Arc::new(crate::bytecode::commands::CommandMeta {
        key: String::from("]"),
        aliases: vec![String::from("listgen")],
        ..core::default::Default::default()
      })
    }
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