use std::{error, sync::Arc};

use thiserror::Error;

use crate::{runtime::stack::Stack, utils::{MArc, MBox}};

pub mod stack_manipulators;
pub mod core;
pub mod math;
pub mod iters;
pub mod conditionals;

#[derive(Error, Debug)]
pub enum RuntimeError {
  #[error("No elements ahead of stack")]
  NoElementsAheadOfStack,
  #[error("No elements on stack")]
  NoElementsOnStack,
  #[error("Wrong element type")]
  WrongElementType,
  #[error("Not implemented")]
  NotImplemented
}

pub trait CommandExecutable<'c> {
  fn execute(&self, stack: &mut Stack<'c>) -> Result<(), RuntimeError>;
  fn to_string(&self) -> String;
}

pub trait DescribedCommandMaker<'c> {
  fn make_described_command_holder() -> DescribedCommandHolder<'c>;
}

pub struct DescribedCommandHolder<'c> {
  pub execution: Box<dyn CommandExecutable<'c> + 'c>,
  pub meta: CommandMeta
}

impl<'c> DescribedCommandHolder<'c> {
  pub fn make_ref<'a: 'c>(&'a self) -> DescribedCommand<'a> {
    DescribedCommand {
      execution: &*self.execution,
      meta: &self.meta
    }
  }
}

#[derive(Clone)]
pub struct DescribedCommand<'h> {
  pub execution: &'h (dyn CommandExecutable<'h> + 'h),
  pub meta: &'h CommandMeta
}

pub struct CommandMeta {
  pub key: String,
  pub aliases: Vec<String>,
  pub description: String
}

impl Default for CommandMeta {
  fn default() -> Self {
    CommandMeta {
      key: String::from('_'),
      aliases: vec![],
      description: String::from("No description provided")
    }
  }
}