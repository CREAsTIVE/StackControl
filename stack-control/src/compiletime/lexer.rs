/*
Lexer:

Splits input string into separate tokens, no other logic involved
*/

use std::iter::Peekable;

pub enum Token {
  FunctionOpenBracket,
  FunctionCloseBracket,
  WhiteSpace(char),
  CommandToken(CommandToken)
}

pub enum CommandToken {
  Command(char),
  CommandOrAlias(String),
  ListOpenBracket,
  ListCloseBracket,
  Function,
  Number(f64),
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

  while let Some(_) = symbols.peek() {
    result.push(
      parse_alias(&mut symbols)
      .or_else(|| parse_number(&mut symbols))
      .or_else(|| parse_special_symbol(&mut symbols, '[', Token::CommandToken(CommandToken::ListOpenBracket))) 
      .or_else(|| parse_special_symbol(&mut symbols, ']', Token::CommandToken(CommandToken::ListCloseBracket)))
      .or_else(|| parse_special_symbol(&mut symbols, '(', Token::FunctionOpenBracket))
      .or_else(|| parse_special_symbol(&mut symbols, ')', Token::FunctionCloseBracket))
      .or_else(|| parse_special_symbol(&mut symbols, '#', Token::CommandToken(CommandToken::Function)))
      .or_else(|| {
        if (symbols.peek()?.is_whitespace()) {return Some(Token::WhiteSpace(symbols.next()?))}
        None
      })
      .unwrap_or_else(|| Token::CommandToken(CommandToken::Command(symbols.next().expect("Unexcpected exception: peek is lier!"))))
    );
  };

  return result;
}

fn parse_special_symbol(iter: &mut Peekable<impl Iterator<Item = char> + Clone>, symbol: char, out_token: Token) -> Option<Token> {
  if *iter.peek()? == symbol {return Some(out_token);}
  return None
}

fn parse_alias<'a>(iter: &mut Peekable<impl Iterator<Item = char> + Clone>) -> Option<Token> {
  if !iter.peek()?.is_alphabetic() {return None}
  let mut result = String::new();
  while let Some(symbol) = iter.peek() {
    if !symbol.is_alphanumeric() {break;}
    result.push(*symbol);
    iter.next();
  };

  return Some(Token::CommandToken(CommandToken::CommandOrAlias(result)))
}

fn parse_number<'a>(iter: &mut Peekable<impl Iterator<Item = char> + Clone>) -> Option<Token> {
  if !(iter.peek()?.is_alphanumeric()) || *iter.peek()? != '.' {return None}
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

  return Some(Token::CommandToken(CommandToken::Number((first + "." + &second).parse::<f64>().expect("Unexcpected Number parsing error"))));
}