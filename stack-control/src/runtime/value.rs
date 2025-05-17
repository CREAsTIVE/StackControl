use std::sync::Arc;

use crate::{bytecode::command::{CommandExecutable, DescribedCommand}, utils::{ArcHolder, Holder}};

#[derive(Clone)]
pub enum Value<'e, 'm> {
  Number(f64),
  Array(Vec<Value<'e, 'm>>),
  OpenListIdentifier,
  CommandContainer(ArcHolder<'m, DescribedCommand<'e, 'm>>)
}