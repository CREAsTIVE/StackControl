use std::{collections::HashMap, sync::Arc};

use crate::bytecode::commands::DescribedCommand;

pub struct CommandMap {
  // I know the hashmap of arcs is bs
  // BUT I waste like 2-3 days trying to do everything on lifetimes and pure references
  // Yeah, i m stupid, maybe later I will fix that
  // but for now i fcking tired, so forgive me once
  pub collection: HashMap<String, Arc<DescribedCommand>>,
  pub aliases_collection: HashMap<String, String>
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
      .for_each(|e| { self.aliases_collection.insert(e.clone(), command.meta.key.clone());});
    self.collection.insert(command.meta.key.clone(), Arc::new(command));
  }

  pub fn get_by_name(&self, name: &str) -> Option<Arc<DescribedCommand>> {
    Some(self.collection.get(name)?.clone())
  }

  pub fn get_by_alias(&self, alias: &str) -> Option<&str> {
    self.aliases_collection.get(alias).map(|s| s.as_str())
  }

  pub fn get(&self, ident: &str) -> Option<Arc<DescribedCommand>> {
    self.get_by_name(ident).or_else(|| self.get_by_name(self.get_by_alias(ident)?))
  }
}