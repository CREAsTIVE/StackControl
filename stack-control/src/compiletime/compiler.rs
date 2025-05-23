use std::sync::Arc;

use indoc::indoc;
use thiserror::Error;

use crate::{bytecode::commands::{core::{ListGeneratorCommand, StackPusherCommand}, CommandExecutable, CommandMeta, DescribedCommand, DescribedCommandHolder, DescribedCommandMaker}, runtime::value::Value};

use super::{command_map::{CommandMap}, lexer::{CommandToken, Token}};

#[derive(Error, Debug)]
pub enum CompilationError {
  #[error("Unexcpected End Token {0}")]
  UnexcpectedEndToken(String),
  #[error("Function token required")]
  FunctionTokenRequired,
  #[error("Command {0} not found")]
  CommandNotFound(String),
}

pub struct Scope<'c> {
  pub command_map: CommandMap<'c>,
  list_opener: Box<DescribedCommandHolder<'c>>, // StackPusherCommand{value_to_push: Value::OpenListIdentifier}
  list_generator: Box<DescribedCommandHolder<'c>>,
  stack_pusher_meta: Box<CommandMeta>,

  container: Vec<Box<dyn CommandExecutable<'c>>>
}

impl<'c> Scope<'c> {
  pub fn store<'a>(&'a mut self, exec: impl CommandExecutable<'c> + 'static) -> &'a (dyn CommandExecutable<'c> + 'c) {
    self.container.push(Box::new(exec));
    self.container.last().unwrap().as_ref()
  }

  pub fn new() -> Self {
    Scope::<'c> { 
      command_map: CommandMap::new(), 

      list_opener: Box::new(DescribedCommandHolder {
        execution: Box::new(StackPusherCommand {
          value_to_push: Value::OpenListIdentifier
        }),
        meta: CommandMeta {
          key: String::from('['),
          aliases: vec![
            String::from("listopen")
          ],
          description: String::from(indoc! {"
            Pushes `[` onto stack
          "})
        }
      }), 

      list_generator: Box::new(ListGeneratorCommand::make_described_command_holder()),

      stack_pusher_meta: Box::new(CommandMeta {
        key: String::from('↓'),
        aliases: vec![
          String::from("pushstack")
        ],
        description: String::from(indoc! {"
          Pushes constant value onto stack.
        "})
      }),
      container: vec![]
    }
  }

  pub fn compile<'a, 't>(
    &'a mut self, 
    mut iter: impl Iterator<Item = &'t Token>,
  ) -> Result<Vec<DescribedCommand<'a>>, CompilationError> {
    Ok(self.parse_commands(&mut iter)?)
  }

  fn parse_command<'a, 't>(
    &'a mut self, 
    token: &CommandToken, 
    tokens: &mut impl Iterator<Item = &'t Token>
  ) -> Result<DescribedCommand<'a>, CompilationError> {
    Ok(match token {
      CommandToken::Number(num) => {
        let meta = self.stack_pusher_meta.as_ref();
        DescribedCommand::<'a> {
          execution: self.store(StackPusherCommand {
            value_to_push: Value::Number(*num)
          }),
          meta
        }
      },

      CommandToken::Function => {
        if let Token::CommandToken(token) = tokens.next().ok_or(CompilationError::FunctionTokenRequired)? {
          let exec = self.store(StackPusherCommand {
            value_to_push: Value::CommandContainer(self.parse_command(token, tokens)?)
          });
          DescribedCommand {
            execution: exec,
            meta: &*self.stack_pusher_meta
          }
        } else {return Err(CompilationError::FunctionTokenRequired);} // TODO: Error
      },
      
      CommandToken::CommandOrAlias(alias) =>
        self.command_map.get(&alias)
          .ok_or(CompilationError::CommandNotFound(alias.clone()))?,

      CommandToken::ListOpenBracket =>
        self.list_opener.make_ref(),

      CommandToken::ListCloseBracket =>
        self.list_generator.make_ref(),
    })
  }

  // TODO: change Vec to impl Iterator<CommandExecutable>
  fn parse_commands<'a, 't>(
    &'a self, 
    tokens: &mut impl Iterator<Item = &'t Token>,
  ) -> Result<Vec<DescribedCommand<'a>>, CompilationError> {
    
    let mut commands = Vec::new(); // Use iterator

    while let Some(token) = tokens.next() {
      match token {
        Token::WhiteSpace(_) => {},
        Token::FunctionCloseBracket => return Ok(commands),
        Token::FunctionOpenBracket => {
          // TODO: Iter
          commands.push(self.list_opener.make_ref());
          let sublist_commands = self.parse_commands(tokens)?;

          for exec in sublist_commands {
            commands.push(DescribedCommand {
              execution: self.store(StackPusherCommand {
                value_to_push: Value::CommandContainer(exec)
              }),
              meta: &*self.stack_pusher_meta
            });
          };

          commands.push(self.list_generator.make_ref());
        },
        Token::CommandToken(cmd) => 
          commands.push(self.parse_command(cmd, tokens)?),
      } // No FunctionCloseBracket token was found
    }
    Ok(commands)
  }
}