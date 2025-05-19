use std::sync::Arc;

use crate::{runtime::stack::Stack, utils::Holder};

pub mod stack_manipulators;
pub mod core;
pub enum ExecutionResult {
  Success,
  Error(&'static str)
}
pub trait CommandExecutable {
  fn execute(&self, stack: &mut Stack) -> ExecutionResult;
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