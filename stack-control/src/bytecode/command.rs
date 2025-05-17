use crate::{runtime::stack::Stack, utils::Holder};

pub mod stack_manipulators;
pub mod core;
pub enum ExecutionResult {
  Success,
  Error(&'static str)
}
pub trait CommandExecutable<'e, 'm> {
  fn execute(&self, stack: &mut Stack<'e, 'm>) -> ExecutionResult;
}

pub struct DescribedCommand<'e, 'm> {
  pub execution: Box<dyn CommandExecutable<'e, 'm> + 'e>,
  pub meta: Holder<'m, CommandMeta>
}

pub struct CommandMeta {
  pub name: char,
  pub aliases: Vec<String>,
}

impl CommandMeta {
  pub fn make_command<'e, 'm>(&'m self, execution: Box<dyn CommandExecutable<'e, 'm>>) -> DescribedCommand<'e, 'm> {
    DescribedCommand {
      execution: execution,
      meta: Holder::Ref(self)
    }
  }
}