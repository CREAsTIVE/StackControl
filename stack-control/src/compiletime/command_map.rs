use std::{collections::HashMap, sync::Arc};

use crate::bytecode::command::DescribedCommand;

pub struct CommandMap {
  // I know hashmap of arcs is bs
  // BUT i waste like 2-3 days trying to do everything on lifetimes and pure references
  // Yeah, i m stupid, maybe later i will fix that
  // but for now i fcking tired, so forgive me once
  collection: HashMap<char, Arc<DescribedCommand>>,
  aliases_collection: HashMap<String, char>
}

impl CommandMap {
  pub fn new() -> CommandMap {
    CommandMap { 
      collection: HashMap::new(),
      aliases_collection: HashMap::new()
    }
  }

  pub fn set(&mut self, command: DescribedCommand) {
    command.meta.aliases.iter()
      .for_each(|e| { self.aliases_collection.insert(e.clone(), command.meta.key);});
    self.collection.insert(command.meta.key, Arc::new(command));
  }

  pub fn get<'a>(&'a self, name: char) -> Option<Arc<DescribedCommand>> {
    Some(self.collection.get(&name)?.clone())
  }

  pub fn get_alias(&self, alias: &str) -> Option<char> {
    self.aliases_collection.get(alias).copied()
  }
}