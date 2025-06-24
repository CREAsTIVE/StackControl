/*
Lexer:

Splits input string into separate tokens, no other logic involved
*/
use std::collections::{HashMap, HashSet};
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
        match self {
            Self::FunctionOpenBracket => write!(f, "("),
            Self::FunctionCloseBracket => write!(f, ")"),
            Self::WhiteSpace(s) => write!(f, "{}", s),
            Self::CommandToken(cmd) => write!(f, "{}", cmd)
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

impl Display for CommandToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CommandOrAlias(alias) => write!(f, "{}", alias),
            Self::ListOpenBracket => write!(f, "["),
            Self::ListCloseBracket => write!(f, "]"),
            Self::Function => write!(f, "◉"),
            Self::Number(num) => write!(f, "{}", num)
        }
    }
}

pub struct UnhandledParseError {}

pub struct ParsingError {
    pub position: usize,
    pub error: ParsingErrorType
}

pub enum ParsingErrorType {

}

/*
pub fn split_to_symbols<'a>(source: &'a str) -> Peekable<impl Iterator<Item = &'a str> + Clone> {
  source.graphemes(true).peekable()
}
*/

// TODO: Parse special tokens from dictionary or something like that

pub struct Lexer {
    pub special_tokens: HashMap<char, Token>
}

impl Lexer {
    pub fn new() -> Self {
        Self {
            special_tokens: HashMap::from([
                ('[', Token::CommandToken(CommandToken::ListOpenBracket)),
                (']', Token::CommandToken(CommandToken::ListCloseBracket)),
                ('#', Token::CommandToken(CommandToken::Function)),
                ('◉', Token::CommandToken(CommandToken::Function)),
                ('(', Token::FunctionOpenBracket),
                (')', Token::FunctionCloseBracket),
            ]),
        }
    }
    
    pub fn split_string_to_tokens(&self, string: &str) -> Vec<Token> {
        self.split_to_tokens(string.char_indices().peekable())
    }

    pub fn split_to_tokens<'a>(&self, mut iter: Peekable<impl Iterator<Item = (usize, char)> + Clone>) -> Vec<Token> {
        let mut result: Vec<Token> = Vec::new();

        while let Some(token) = self.parse_token(&mut iter) {
            result.push(token);
        }

        result
    }



    fn parse_token(&self, iter: &mut Peekable<impl Iterator<Item = (usize, char)> + Clone>) -> Option<Token> {
        if let None = iter.peek() {return None;}
        // Is special token
        Some(
            self.parse_special_tokens(iter)
                // Or whitespace
                .or_else(|| {
                    if iter.peek()?.1.is_whitespace() { return Some(Token::WhiteSpace(iter.next()?.1)) }
                    None
                })
                // Or Number
                .or_else(|| Self::parse_number(iter))
                .or_else(|| Self::parse_command_or_alias(iter))
    
                // Not a character OR a number
                .or_else(|| {
                    Some(Token::CommandToken(CommandToken::CommandOrAlias(String::from(iter.next()?.1))))
                })
            
                .expect("Uncaptured token!")
        )
    }


    fn parse_special_tokens(&self, iter: &mut Peekable<impl Iterator<Item = (usize, char)> + Clone>) -> Option<Token> {
        match self.special_tokens.get(&iter.peek()?.1) {
            Some(token) => {
                iter.next();
                Some(token.clone())
            },
            None => None,
        }
    }

    fn parse_command_or_alias<'a>(iter: &mut Peekable<impl Iterator<Item = (usize, char)> + Clone>) -> Option<Token> {
        let p = iter.peek()?.1;
        if !p.is_alphabetic() {return None}
        let mut result = String::new();
        while let Some((_, symbol)) = iter.peek() {
            if !symbol.is_alphanumeric() {break;}
            result.push(*symbol);
            iter.next();
        };

        Some(Token::CommandToken(CommandToken::CommandOrAlias(result)))
    }

    fn parse_number<'a>(iter: &mut Peekable<impl Iterator<Item = (usize, char)> + Clone>) -> Option<Token> {
        let next = iter.peek()?.1;
        if !(next.is_numeric() || next == '.') {return None}

        fn parse_uint<'a>(initial: &str, iter: &mut Peekable<impl Iterator<Item = (usize, char)> + Clone>) -> String {
            let mut result = String::from(initial);

            while let Some((_, symbol)) = iter.peek() {
                if !symbol.is_numeric() {break;}
                result.push(*symbol);
                iter.next();
            }

            result
        }

        let first = parse_uint("0", iter);

        let second = match iter.peek() {
            Some((_, '.')) => {
                iter.next();
                parse_uint("", iter)
            }
            _ => String::from("0")
        };

        if first.len() == 1 && second.len() == 1 {return None}

        Some(Token::CommandToken(CommandToken::Number((first + "." + &second).parse::<f64>().expect("Unexpected Number parsing error"))))
    }
}

