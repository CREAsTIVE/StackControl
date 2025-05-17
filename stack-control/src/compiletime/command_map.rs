use std::collections::HashMap;

use crate::bytecode::command::DescribedCommand;

pub struct CommandMap<'e, 'm> {
  collection: HashMap<char, DescribedCommand<'e, 'm>>,
  aliases_collection: HashMap<String, char>
}

impl<'e, 'm> CommandMap<'e, 'm> {
  pub fn new() -> CommandMap<'e, 'm> {
    CommandMap { 
      collection: HashMap::new(),
      aliases_collection: HashMap::new()
    }
  }

  pub fn set(&mut self, command: DescribedCommand<'e, 'm>) {
    command.meta.to_ref().aliases.iter()
      .for_each(|e| { self.aliases_collection.insert(e.clone(), command.meta.to_ref().name);});
    self.collection.insert(command.meta.to_ref().name, command);
  }

  pub fn get(&self, name: char) -> Option<&DescribedCommand<'e, 'm>> {
    self.collection.get(&name)
  }

  pub fn get_alias(&self, alias: &str) -> Option<char> {
    self.aliases_collection.get(alias).copied()
  }
}