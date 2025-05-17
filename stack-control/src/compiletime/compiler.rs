use std::sync::Arc;

use crate::{bytecode::command::{core::ListGeneratorCommand, stack_manipulators::StackPusherCommand, CommandExecutable, CommandMeta, DescribedCommand}, runtime::value::Value, utils::{ArcHolder, Holder}};

use super::{command_map::{CommandMap}, lexer::{CommandToken, Token}};

pub struct CompileTime<'e, 'm> {
  pub command_map: CommandMap<'e, 'm>,
  list_opener: Box<DescribedCommand<'e, 'm>>, // StackPusherCommand{value_to_push: Value::OpenListIdentifier}
  list_generator: Box<DescribedCommand<'e, 'm>>,
  stack_pusher_meta: Box<CommandMeta>
}


impl<'a> CompileTime<'a, 'a> {
  pub fn new() -> CompileTime<'a, 'a> {
    CompileTime { 
      command_map: CommandMap::new(), 

      list_opener: Box::new(DescribedCommand {
        execution: Box::new(StackPusherCommand {
          value_to_push: Value::OpenListIdentifier
        }),
        meta: Holder::Val(Box::new(CommandMeta {
          name: '(',
          aliases: vec![
            String::from("listopen")
          ]
        }))
      }), 

      list_generator: Box::new(DescribedCommand {
        execution: Box::new(ListGeneratorCommand {}),
        meta: Holder::Val(Box::new(CommandMeta {
          name: ')',
          aliases: vec![
            String::from("listgen")
          ]
        }))
      }),

      stack_pusher_meta: Box::new(CommandMeta {
        name: 'ъ',
        aliases: vec![
          String::from("pushstack")
        ]
      })
    }
  }

  pub fn compile<'s: 'a>(&self, tokens: Vec<Token>) -> Option<Vec<ArcHolder<'s, DescribedCommand>>> {
    let mut iter  = tokens.into_iter();
    let commands = self.parse_commands(&mut iter);
    if let Some(_) = iter.next() {return None} // todo: error message (unparsed tokens after ")")
    commands
  }

  fn parse_command<'s: 'a>(
      &self, 
      token: CommandToken, 
      tokens: &mut impl Iterator<Item = Token>) -> Option<ArcHolder<'s, DescribedCommand>> {

    /*
    if let CommandToken::Command(name) = token {
      return Some(Holder::Ref(self.command_map.get(name)?))
    }
    */

    Some(match token {
      CommandToken::Number(num) => {
        ArcHolder::Val(Arc::new(DescribedCommand {
          execution: Box::new(StackPusherCommand {
            value_to_push: Value::Number(num)
          }),
          meta: Holder::Ref(self.stack_pusher_meta.as_ref()) // TODO: Another meta
        }))
      },

      CommandToken::Function => {
        if let Token::CommandToken(token) = tokens.next()? {
          ArcHolder::Val(Arc::new(DescribedCommand {
            execution: Box::new(StackPusherCommand {
              value_to_push: Value::CommandContainer(self.parse_command(token, tokens)?)
            }),
            meta: Holder::Ref(&self.stack_pusher_meta)
          }))
        } else {return None;} // TODO: Error
      },

      CommandToken::Command(name) =>
        ArcHolder::Ref(self.command_map.get(name)?),
      
      CommandToken::CommandOrAlias(alias) =>
        ArcHolder::Ref(self.command_map.get(self.command_map.get_alias(&alias)?)?),

      CommandToken::ListOpenBracket =>
        ArcHolder::Ref(self.list_opener.as_ref()),

      CommandToken::ListCloseBracket =>
        ArcHolder::Ref(self.list_generator.as_ref()),
    })
  }

  // TODO: change Vec to impl Iterator<CommandExecutable>
  fn parse_commands<'s: 'a>(
    &self, 
    tokens: &mut impl Iterator<Item = Token>) -> Option<Vec<ArcHolder<'s, DescribedCommand>>> {
    
    let mut commands = Vec::new(); // Use iterator

    while let Some(token) = tokens.next() {
      match token {
        Token::WhiteSpace(_) => {},
        Token::FunctionCloseBracket => return Some(commands),
        Token::FunctionOpenBracket => {
          // TODO: Iter
          let sublist = self.parse_commands(tokens)?;
          for exec in sublist {
            commands.push(ArcHolder::Val(Arc::new(DescribedCommand {
              execution: Box::new(StackPusherCommand {
                value_to_push: Value::CommandContainer(exec)
              }),
              meta: Holder::Ref(self.stack_pusher_meta.as_ref())
            })));
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