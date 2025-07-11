use crate::{compiletime::command_map::CommandMap, runtime::{stack::Stack, value::{Array, Value}}};

use super::{conditionals::append_conditionals, iters::append_iters, math::append_math, stack_manipulators::append_stack_manipulators, CommandExecutable, DescribedCommand, DescribedCommandMaker, RuntimeException};

use indoc::indoc;

pub fn bind_default_commands(map: &mut CommandMap) {
  map.set(InvokeCommand::make_described_command());
  
  append_stack_manipulators(map);
  append_math(map);
  append_iters(map);
  append_conditionals(map);
}

fn _test(stack: &mut Stack) -> Result<(), RuntimeException> {
      //
      let truly = stack.pop()?;
      let val = stack.pop()?.bool();
      if val {truly.invoke(stack)?;}
      Ok(())
}

macro_rules! define_command {
  ($name:ident ($key:expr, [$($alias:tt),*]) {with {$($pkey:ident : $pvalue:expr),*} $stack:ident $code:block}) => {
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

macro_rules! define_commands {
  ($group:ident $($name:ident $args:tt $body:tt),*) => {
    pub fn $group(cmd_map: &mut $crate::compiletime::command_map::CommandMap) {
      $(cmd_map.set(<$name as $crate::bytecode::commands::DescribedCommandMaker>::make_described_command()));*
    }
    $(crate::bytecode::commands::core::define_command! {$name $args $body})*
  };
}

pub(crate) use define_commands;
pub(crate) use define_command;

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

define_command!(
  InvokeCommand ('!', ["invoke"]) {
    with {
      description: String::from(indoc!("
        Consumes value and invokes it
      "))
    }
    stack {
      Ok(stack.pop()?.invoke(stack)?)
    }
  }
);