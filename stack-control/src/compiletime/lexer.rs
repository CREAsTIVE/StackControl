/*
Lexer:

Splits input string into separate tokens, no other logic involved
*/
use std::fmt::Display;
use std::iter::Peekable;

#[derive(Clone)]
pub enum Token {
  FunctionOpenBracket,
  FunctionCloseBracket,
  WhiteSpace(char),
  CommandToken(CommandToken)
}

impl Display for Token {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let str = match self {
      Self::FunctionOpenBracket => String::from("("),
      Self::FunctionCloseBracket => String::from(")"),
      Self::WhiteSpace(s) => String::from(*s),
      Self::CommandToken(cmd) => cmd.to_string()
    };
    write!(f, "{}", str)
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

impl Display for CommandToken {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let str = match self {
      Self::CommandOrAlias(alias) => alias.clone(),
      Self::ListOpenBracket => String::from("["),
      Self::ListCloseBracket => String::from("]"),
      Self::Function => String::from("#"),
      Self::Number(num) => num.to_string()
    };
    write!(f, "{}", str)
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

  result
}

fn parse_special_symbol(iter: &mut Peekable<impl Iterator<Item = char> + Clone>, symbol: char, out_token: Token) -> Option<Token> {
  if *iter.peek()? == symbol {iter.next(); return Some(out_token);}
  None
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

  Some(Token::CommandToken(CommandToken::CommandOrAlias(result)))
}

fn parse_number<'a>(iter: &mut Peekable<impl Iterator<Item = char> + Clone>) -> Option<Token> {
  let next = *iter.peek()?;
  if !next.is_numeric() && next != '.' {return None}
  
  fn parse_uint<'a>(initial: &str, iter: &mut Peekable<impl Iterator<Item = char> + Clone>) -> String {
    let mut result = String::from(initial);

    while let Some(symbol) = iter.peek() {
      if !symbol.is_numeric() {break;}
      result.push(*symbol);
      iter.next();
    }
    
    result
  }
  
  let first = parse_uint("0", iter);
  
  let second = match iter.peek() {
    Some('.') => {
      iter.next();
      parse_uint("", iter)
    }
    _ => String::from("0")
  };

  if first.len() == 1 && second.len() == 1 {return None}

  Some(Token::CommandToken(CommandToken::Number((first + "." + &second).parse::<f64>().expect("Unexpected Number parsing error"))))
}