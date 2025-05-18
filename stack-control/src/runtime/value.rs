use std::{fmt::format, sync::Arc};

use itertools::Itertools;

use crate::bytecode::command::DescribedCommand;

#[derive(Clone)]
pub enum Value {
  Number(f64),
  Array(Vec<Value>),
  OpenListIdentifier,
  CommandContainer(Arc<DescribedCommand>)
}

impl ToString for Value {
  fn to_string(&self) -> String {
    match self {
      Self::Number(num) => num.to_string(),
      Self::Array(arr) => {
        let strarr = arr.iter()
          .map(|e| e.to_string())
          .join(", ");

        format!("[{strarr}]")
      },
      Self::CommandContainer(cmd) =>
        String::from("◉") + &cmd.execution.to_string(),
      
      Self::OpenListIdentifier => String::from("[")
        
    }
  }
}