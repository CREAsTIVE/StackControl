use std::sync::Arc;

use indoc::indoc;
use thiserror::Error;

use crate::{bytecode::commands::{core::{ListGeneratorCommand, StackPusherCommand}, CommandMeta, DescribedCommand}, runtime::value::Value};

use super::{command_map::{CommandMap}, lexer::{CommandToken, Token}};

pub struct Scope {
  pub command_map: CommandMap,
  list_opener: Arc<DescribedCommand>, // StackPusherCommand{value_to_push: Value::OpenListIdentifier}
  list_generator: Arc<DescribedCommand>,
  stack_pusher_meta: Arc<CommandMeta>
}

#[derive(Error, Debug)]
pub enum CompilationException {
  #[error("Unexpected End Token {0}")]
  UnexpectedEndToken(String),
  #[error("Function token required")]
  FunctionTokenRequired,
  #[error("Command {0} not found")]
  CommandNotFound(String),
}

impl Scope {
  pub fn new() -> Self {
    Scope { 
      command_map: CommandMap::new(), 

      list_opener: Arc::new(DescribedCommand {
        execution: Box::new(StackPusherCommand {
          value_to_push: Value::OpenListIdentifier
        }),
        meta: Arc::new(CommandMeta {
          key: String::from('['),
          aliases: vec![
            String::from("listopen")
          ],
          description: String::from(indoc! {"
            Pushes `[` onto stack
          "})
        })
      }), 

      list_generator: Arc::new(DescribedCommand {
        execution: Box::new(ListGeneratorCommand {}),
        meta: Arc::new(CommandMeta {
          key: String::from(']'),
          aliases: vec![
            String::from("listgen")
          ],
          description: String::from(indoc! {"
            Consumes all values up to `[` and generates reversed list out of them
          "})
        })
      }),

      stack_pusher_meta: Arc::new(CommandMeta {
        key: String::from('↓'),
        aliases: vec![
          String::from("pushstack")
        ],
        description: String::from(indoc! {"
          Pushes constant value onto stack.
        "})
      })
    }
  }

  pub fn compile<'a>(&self, mut iter: impl Iterator<Item = &'a Token>) -> Result<Vec<Arc<DescribedCommand>>, CompilationException> {
    let commands = self.parse_commands(&mut iter)?;
    if let Some(e) = iter.next() {return Err(CompilationException::UnexpectedEndToken(e.to_string()))} // todo: error message (unparsed tokens after ")")
    Ok(commands)
  }

  fn parse_command<'a>(
      &self, 
      token: &CommandToken, 
      tokens: &mut impl Iterator<Item = &'a Token>) -> Result<Arc<DescribedCommand>, CompilationException> {
    

    Ok(match token {
      CommandToken::Number(num) => {
        Arc::new(DescribedCommand {
          execution: Box::new(StackPusherCommand {
            value_to_push: Value::Number(*num)
          }),
          meta: self.stack_pusher_meta.clone() // TODO: Another meta
        })
      },

      CommandToken::Function => {
        if let Token::CommandToken(token) = tokens.next().ok_or(CompilationException::FunctionTokenRequired)? {
          Arc::new(DescribedCommand {
            execution: Box::new(StackPusherCommand {
              value_to_push: Value::CommandContainer(self.parse_command(token, tokens)?)
            }),
            meta: self.stack_pusher_meta.clone()
          })
        } else {return Err(CompilationException::FunctionTokenRequired);} // TODO: Error
      },
      
      CommandToken::CommandOrAlias(alias) =>
        self.command_map.get(&alias)
          .ok_or(CompilationException::CommandNotFound(alias.clone()))?,

      CommandToken::ListOpenBracket =>
        self.list_opener.clone(),

      CommandToken::ListCloseBracket =>
        self.list_generator.clone(),
    })
  }

  // TODO: change Vec to impl Iterator<CommandExecutable>
  fn parse_commands<'a>(
    &self, 
    tokens: &mut impl Iterator<Item = &'a Token>) -> Result<Vec<Arc<DescribedCommand>>, CompilationException> {
    
    let mut commands = Vec::new(); // Use iterator

    while let Some(token) = tokens.next() {
      match token {
        Token::WhiteSpace(_) => {},
        Token::FunctionCloseBracket => return Ok(commands),
        Token::FunctionOpenBracket => {
          // TODO: Iter
          commands.push(self.list_opener.clone());
          let sublist_commands = self.parse_commands(tokens)?;
          for exec in sublist_commands {
            commands.push(Arc::new(DescribedCommand {
              execution: Box::new(StackPusherCommand {
                value_to_push: Value::CommandContainer(exec)
              }),
              meta: self.stack_pusher_meta.clone()
            }));
          };
          commands.push(self.list_generator.clone());
        },
        Token::CommandToken(cmd) => 
          commands.push(self.parse_command(cmd, tokens)?),
      } // No FunctionCloseBracket token was found
    }
    Ok(commands)
  }

  pub fn default_commands() {
    
  }
}