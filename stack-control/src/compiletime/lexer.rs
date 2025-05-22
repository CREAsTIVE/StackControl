/*
Lexer:

Splits input string into separate tokens, no other logic involved
*/

use std::iter::Peekable;

#[derive(Clone)]
pub enum Token {
  FunctionOpenBracket,
  FunctionCloseBracket,
  WhiteSpace(char),
  CommandToken(CommandToken)
}

impl ToString for Token {
  fn to_string(&self) -> String {
    match self {
      Self::FunctionOpenBracket => String::from("("),
      Self::FunctionCloseBracket => String::from(")"),
      Self::WhiteSpace(s) => String::from(*s),
      Self::CommandToken(cmd) => cmd.to_string()
    }
  }
}

#[derive(Clone)]
pub enum CommandToken {
  CommandOrAlias(String),
  ListOpenBracket,
  ListCloseBracket,
  Function,
  Number(f64),
}

impl ToString for CommandToken {
  fn to_string(&self) -> String {
    match self {
      Self::CommandOrAlias(alias) => alias.clone(),
      Self::ListOpenBracket => String::from("["),
      Self::ListCloseBracket => String::from("]"),
      Self::Function => String::from("#"),
      Self::Number(num) => num.to_string()
    }
  }
}

pub struct UnhandledParseException {}

/*
pub fn split_to_symbols<'a>(source: &'a str) -> Peekable<impl Iterator<Item = &'a str> + Clone> {
  source.graphemes(true).peekable()
}
*/

// TODO: Parse special tokens from dictionary or something like that

pub fn split_string_to_tokens(string: &str) -> Vec<Token> {
  split_to_tokens(string.chars().peekable())
}

pub fn split_to_tokens<'a>(mut symbols: Peekable<impl Iterator<Item = char> + Clone>) -> Vec<Token> {
  let mut result: Vec<Token> = Vec::new();

  let p = parse_special_symbol;

  while let Some(_) = symbols.peek() {
    result.push(
      // if one of special symbols
                  p(&mut symbols, '[', Token::CommandToken(CommandToken::ListOpenBracket))
      .or_else(|| p(&mut symbols, ']', Token::CommandToken(CommandToken::ListCloseBracket)))
      .or_else(|| p(&mut symbols, '(', Token::FunctionOpenBracket))
      .or_else(|| p(&mut symbols, ')', Token::FunctionCloseBracket))
      // TODO: Change # to CommandContainer symbol
      .or_else(|| p(&mut symbols, '#', Token::CommandToken(CommandToken::Function)))

      // Or whitespace
      .or_else(|| {
        if symbols.peek()?.is_whitespace() { return Some(Token::WhiteSpace(symbols.next()?)) }
        None
      })
      .or_else(|| {
        if !symbols.peek()?.is_alphanumeric() { 
          return Some(Token::CommandToken(CommandToken::CommandOrAlias(String::from(symbols.next()?)))) 
        } None
      })
      // Or one of "complex" tokens
      .or_else(|| parse_number(&mut symbols))
      .unwrap_or_else(|| parse_command_or_alias(&mut symbols).expect("Internal exception!"))
    );
  };

  return result;
}

fn parse_special_symbol(iter: &mut Peekable<impl Iterator<Item = char> + Clone>, symbol: char, out_token: Token) -> Option<Token> {
  if *iter.peek()? == symbol {iter.next(); return Some(out_token);}
  return None
}

fn parse_command_or_alias<'a>(iter: &mut Peekable<impl Iterator<Item = char> + Clone>) -> Option<Token> {
  let p = *iter.peek()?;
  if p.is_numeric() || p.is_whitespace() || p == '.' {return None}
  let mut result = String::new();
  while let Some(symbol) = iter.peek() {
    if !symbol.is_alphanumeric() {break;}
    result.push(*symbol);
    iter.next();
  };

  return Some(Token::CommandToken(CommandToken::CommandOrAlias(result)))
}

fn parse_number<'a>(iter: &mut Peekable<impl Iterator<Item = char> + Clone>) -> Option<Token> {
  let next = *iter.peek()?;
  if !(next.is_numeric()) && next != '.' {return None}
  let mut first = String::from("0");
  let mut second = String::new();
  while let Some(symbol) = iter.peek() {
    if !symbol.is_numeric() {break;}
    first.push(*symbol);
    iter.next();
  }

  if let Some('.') = iter.peek() {
    iter.next();
    while let Some(symbol) = iter.peek() {
      if !symbol.is_numeric() {break;}
      second.push(*symbol);
      iter.next();
    }
  }

  second.push('0');

  if first.len() == 1 && second.len() == 1 {return None}

  return Some(Token::CommandToken(CommandToken::Number((first + "." + &second).parse::<f64>().expect("Unexcpected Number parsing error"))));
}