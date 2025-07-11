use std::{sync::Arc};

use thiserror::Error;

use crate::runtime::stack::Stack;

pub mod stack_manipulators;
pub mod core;
pub mod math;
pub mod iters;
pub mod conditionals;

#[derive(Error, Debug)]
pub enum RuntimeException {
  #[error("No elements ahead of stack")]
  NoElementsAheadOfStack,
  #[error("No elements on stack")]
  NoElementsOnStack,
  #[error("Wrong element type")]
  WrongElementType,
  #[error("Not implemented")]
  NotImplemented
}

pub trait CommandExecutable {
  fn execute(&self, stack: &mut Stack) -> Result<(), RuntimeException>;
  fn to_string(&self) -> String;
}

pub trait DescribedCommandMaker where Self : CommandExecutable {
  fn make_described_command() -> DescribedCommand;
}
pub struct DescribedCommand {
  pub execution: Box<dyn CommandExecutable>,
  pub meta: Arc<CommandMeta>
}

pub struct CommandMeta {
  pub key: String,
  pub aliases: Vec<String>,
  pub description: String
}

impl Default for CommandMeta {
  fn default() -> Self {
    CommandMeta {
      key: String::from('d'),
      aliases: vec![],
      description: String::from("No description provided")
    }
  }
}