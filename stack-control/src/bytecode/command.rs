use std::sync::Arc;

use crate::{runtime::stack::Stack, utils::Holder};

pub mod stack_manipulators;
pub mod core;

#[derive(strum_macros::IntoStaticStr)]
pub enum RuntimeException {
  NoElementsAheadOfStack,
  NoElementsOnStack
}
pub trait CommandExecutable {
  fn execute(&self, stack: &mut Stack) -> Result<(), RuntimeException>;
  fn to_string(&self) -> String;
}

pub struct DescribedCommand {
  pub execution: Box<dyn CommandExecutable>,
  pub meta: Arc<CommandMeta>
}

pub struct CommandMeta {
  pub key: char,
  pub aliases: Vec<String>,
  pub description: String
}