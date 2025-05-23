use std::{collections::HashMap, sync::Arc};

use crate::bytecode::commands::{DescribedCommand, DescribedCommandHolder};

pub struct CommandMap<'h> {
  pub collection: HashMap<String, Box<DescribedCommandHolder<'h>>>,
  pub aliases_collection: HashMap<String, String>
}

impl<'c> CommandMap<'c> {
  pub fn new() -> CommandMap<'c> {
    CommandMap { 
      collection: HashMap::new(),
      aliases_collection: HashMap::new()
    }
  }

  pub fn set(&mut self, command: DescribedCommandHolder<'c>) {
    command.meta.aliases.iter()
      .for_each(|e| { self.aliases_collection.insert(e.clone(), command.meta.key.clone());});
    self.collection.insert(command.meta.key.clone(), Box::new(command));
  }

  pub fn get_by_name<'a: 'c>(&'a self, name: &str) -> Option<DescribedCommand<'a>> {
    Some(self.collection.get(name)?.make_ref())
  }

  pub fn get_by_alias(&self, alias: &str) -> Option<&str> {
    self.aliases_collection.get(alias).map(|s| s.as_str())
  }

  pub fn get<'a: 'c>(&'a self, ident: &str) -> Option<DescribedCommand<'c>> {
    self.get_by_name(ident).or_else(|| self.get_by_name(self.get_by_alias(ident)?))
  }
}