use std::sync::Arc;

use crate::{bytecode::command::{core::ListGeneratorCommand, stack_manipulators::StackPusherCommand, CommandExecutable, CommandMeta, DescribedCommand}, runtime::value::Value, utils::{ArcHolder, Holder}};

use super::{command_map::{CommandMap}, lexer::{CommandToken, Token}};

pub struct CompileTime {
  pub command_map: CommandMap,
  list_opener: Arc<DescribedCommand>, // StackPusherCommand{value_to_push: Value::OpenListIdentifier}
  list_generator: Arc<DescribedCommand>,
  stack_pusher_meta: Arc<CommandMeta>
}


impl CompileTime {
  pub fn new() -> Self {
    CompileTime { 
      command_map: CommandMap::new(), 

      list_opener: Arc::new(DescribedCommand {
        execution: Box::new(StackPusherCommand {
          value_to_push: Value::OpenListIdentifier
        }),
        meta: Arc::new(CommandMeta {
          name: '(',
          aliases: vec![
            String::from("listopen")
          ]
        })
      }), 

      list_generator: Arc::new(DescribedCommand {
        execution: Box::new(ListGeneratorCommand {}),
        meta: Arc::new(CommandMeta {
          name: ')',
          aliases: vec![
            String::from("listgen")
          ]
        })
      }),

      stack_pusher_meta: Arc::new(CommandMeta {
        name: 'ъ',
        aliases: vec![
          String::from("pushstack")
        ]
      })
    }
  }

  pub fn compile(&self, tokens: Vec<Token>) -> Option<Vec<Arc<DescribedCommand>>> {
    let mut iter  = tokens.into_iter();
    let commands = self.parse_commands(&mut iter);
    if let Some(_) = iter.next() {return None} // todo: error message (unparsed tokens after ")")
    commands
  }

  fn parse_command<'a>(
      &'a self, 
      token: CommandToken, 
      tokens: &mut impl Iterator<Item = Token>) -> Option<Arc<DescribedCommand>> {
    

    Some(match token {
      CommandToken::Number(num) => {
        Arc::new(DescribedCommand {
          execution: Box::new(StackPusherCommand {
            value_to_push: Value::Number(num)
          }),
          meta: self.stack_pusher_meta.clone() // TODO: Another meta
        })
      },

      CommandToken::Function => {
        if let Token::CommandToken(token) = tokens.next()? {
          Arc::new(DescribedCommand {
            execution: Box::new(StackPusherCommand {
              value_to_push: Value::CommandContainer(self.parse_command(token, tokens)?)
            }),
            meta: self.stack_pusher_meta.clone()
          })
        } else {return None;} // TODO: Error
      },

      CommandToken::Command(name) =>
        self.command_map.get(name)?,
      
      CommandToken::CommandOrAlias(alias) =>
        self.command_map.get(self.command_map.get_alias(&alias)?)?,

      CommandToken::ListOpenBracket =>
        self.list_opener.clone(),

      CommandToken::ListCloseBracket =>
        self.list_generator.clone(),
    })
  }

  // TODO: change Vec to impl Iterator<CommandExecutable>
  fn parse_commands(
    &self, 
    tokens: &mut impl Iterator<Item = Token>) -> Option<Vec<Arc<DescribedCommand>>> {
    
    let mut commands = Vec::new(); // Use iterator

    while let Some(token) = tokens.next() {
      match token {
        Token::WhiteSpace(_) => {},
        Token::FunctionCloseBracket => return Some(commands),
        Token::FunctionOpenBracket => {
          // TODO: Iter
          let sublist = self.parse_commands(tokens)?;
          for exec in sublist {
            commands.push(Arc::new(DescribedCommand {
              execution: Box::new(StackPusherCommand {
                value_to_push: Value::CommandContainer(exec)
              }),
              meta: self.stack_pusher_meta.clone()
            }));
          };
        },
        Token::CommandToken(cmd) => 
          commands.push(self.parse_command(cmd, tokens)?),
      }

      return Some(commands); // No FunctionCloseBracket token was found
    }
    None
  }
}