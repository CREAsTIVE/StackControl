use std::sync::Arc;

use crate::{bytecode::command::{CommandExecutable, DescribedCommand}, utils::{ArcHolder, Holder}};

#[derive(Clone)]
pub enum Value {
  Number(f64),
  Array(Vec<Value>),
  OpenListIdentifier,
  CommandContainer(Arc<DescribedCommand>)
}