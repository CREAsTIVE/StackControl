use std::{fmt::{self, format}, sync::Arc};

use crate::{bytecode::command::{core::{ListGeneratorCommand, StackPusherCommand}, CommandExecutable, CommandMeta, DescribedCommand, ExecutionResult}, runtime::value::Value, utils::{ArcHolder, Holder}};

use super::{command_map::{CommandMap}, lexer::{CommandToken, Token}};

pub struct CompileTime {
  pub command_map: CommandMap,
  list_opener: Arc<DescribedCommand>, // StackPusherCommand{value_to_push: Value::OpenListIdentifier}
  list_generator: Arc<DescribedCommand>,
  stack_pusher_meta: Arc<CommandMeta>
}

pub enum CompilationError {
  UnexcpectedEndToken(String),
  FunctionTokenRequired,
  CommandNotFound(char),
  AliasNotFound(String),
}

impl ToString for CompilationError {
  fn to_string(&self) -> String {
    match self {
      CompilationError::AliasNotFound(alias) =>
        format!("Alias {alias} not found"),
      CompilationError::CommandNotFound(cmd) => 
        format!("Command {cmd} not found"),
      
      CompilationError::FunctionTokenRequired =>
        format!("Function token required"),

      CompilationError::UnexcpectedEndToken(t) => 
        format!("Unexcpected END token: {t}")
    }
  }
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

  pub fn compile(&self, tokens: Vec<Token>) -> Result<Vec<Arc<DescribedCommand>>, CompilationError> {
    let mut iter  = tokens.into_iter();
    let commands = self.parse_commands(&mut iter)?;
    if let Some(e) = iter.next() {return Err(CompilationError::UnexcpectedEndToken(e.to_string()))} // todo: error message (unparsed tokens after ")")
    Ok(commands)
  }

  fn parse_command<'a>(
      &'a self, 
      token: CommandToken, 
      tokens: &mut impl Iterator<Item = Token>) -> Result<Arc<DescribedCommand>, CompilationError> {
    

    Ok(match token {
      CommandToken::Number(num) => {
        Arc::new(DescribedCommand {
          execution: Box::new(StackPusherCommand {
            value_to_push: Value::Number(num)
          }),
          meta: self.stack_pusher_meta.clone() // TODO: Another meta
        })
      },

      CommandToken::Function => {
        if let Token::CommandToken(token) = tokens.next().ok_or(CompilationError::FunctionTokenRequired)? {
          Arc::new(DescribedCommand {
            execution: Box::new(StackPusherCommand {
              value_to_push: Value::CommandContainer(self.parse_command(token, tokens)?)
            }),
            meta: self.stack_pusher_meta.clone()
          })
        } else {return Err(CompilationError::FunctionTokenRequired);} // TODO: Error
      },

      CommandToken::Command(name) =>
        self.command_map.get(name).ok_or(CompilationError::CommandNotFound(name))?,
      
      CommandToken::CommandOrAlias(alias) =>
        self.command_map.get(
          self.command_map.get_alias(&alias).ok_or(CompilationError::AliasNotFound(alias))?
        ).ok_or(CompilationError::CommandNotFound('0'))?,

      CommandToken::ListOpenBracket =>
        self.list_opener.clone(),

      CommandToken::ListCloseBracket =>
        self.list_generator.clone(),
    })
  }

  // TODO: change Vec to impl Iterator<CommandExecutable>
  fn parse_commands(
    &self, 
    tokens: &mut impl Iterator<Item = Token>) -> Result<Vec<Arc<DescribedCommand>>, CompilationError> {
    
    let mut commands = Vec::new(); // Use iterator

    while let Some(token) = tokens.next() {
      match token {
        Token::WhiteSpace(_) => {},
        Token::FunctionCloseBracket => return Ok(commands),
        Token::FunctionOpenBracket => {
          // TODO: Iter
          commands.push(self.list_opener.clone());
          let mut sublist_commands = self.parse_commands(tokens)?;
          sublist_commands.reverse();
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