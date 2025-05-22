use crate::runtime::{stack::Stack, value::Value};

use super::{core::define_commands, RuntimeException};
use indoc::indoc;

// TODO: logic for (arr + val) and (arr + arr)
fn math_op2(stack: &mut Stack, f: impl Fn(f64, f64) -> f64) -> Result<(), RuntimeException> {
  if let Value::Number(a) = stack.pop()? {
    if let Value::Number(b) = stack.pop()? {
      return Ok(stack.push(Value::Number(f(a, b))));
    }
  }
  Err(RuntimeException::WrongElementType)
}

fn math_op1(stack: &mut Stack, f: impl Fn(f64) -> f64) -> Result<(), RuntimeException> {
  if let Value::Number(a) = stack.pop()? {
    return Ok(stack.push(Value::Number(f(a))));
  }
  Err(RuntimeException::WrongElementType)
}

fn math_op2_bool(stack: &mut Stack, f: impl Fn(f64, f64) -> bool) -> Result<(), RuntimeException> {
  math_op2(stack, |a, b| if f(a, b) {1.} else {0.})
}

define_commands!(define append_math
    // op2
  (
    [
      OpAddCommand '+' ["add", "plus"]
      {
        description: String::from(indoc! {"
          (.. b a) => (.. (b + a))
        "})
      }
      
    ] stack {
      math_op2(stack, |a, b| a + b)
    }
  ), (
    [
      OpSubCommand '-' ["sub", "minus"]
      {
        description: String::from(indoc! {"
          (.. b a) => (.. (a - b))
        "})
      }
      
    ] stack {
      math_op2(stack, |a, b| a - b)
    }
  ), (
    [
      OpRSubCommand '∸' ["rsub", "rminus"]
      {
        description: String::from(indoc! {"
          (.. b a) => (.. (b - a))
        "})
      }
      
    ] stack {
      math_op2(stack, |a, b| b - a)
    }
  ), (
    [
      OpMulCommand '×' ["mul"]
      {
        description: String::from(indoc! {"
          (.. b a) => (.. (a × b))
        "})
      }
      
    ] stack {
      math_op2(stack, |a, b| a * b)
    }
  ), (
    [
      OpDivCommand '÷' ["div"]
      {
        description: String::from(indoc! {"
          (.. b a) => (.. (a ÷ b))
        "})
      }
      
    ] stack {
      math_op2(stack, |a, b| a / b)
    }
  ), (
    [
      OpRDivCommand '∺' ["rdiv"]
      {
        description: String::from(indoc! {"
          (.. b a) => (.. (b ÷ a))
        "})
      }
      
    ] stack {
      math_op2(stack, |a, b| b / a)
    }
  ), (
    [
      OpRootCommand '؆' ["root"]
      {
        description: String::from(indoc! {"
          NOT IMPLEMENTED! (.. b a) => (.. (b√ a))
        "})
      }
      
    ] stack {
      math_op2(stack, |a, b| b / a)
    }
  ),
    // op2 eq
  (
    [
      OpHigherCommand '>' ["hi"]
      {
        description: String::from(indoc! {"
          (.. b a) => (.. (a > b))
        "})
      }
      
    ] stack {
      math_op2_bool(stack, |a, b| a > b)
    }
  ), (
    [
      OpLessCommand '<' ["le"]
      {
        description: String::from(indoc! {"
          (.. b a) => (.. (a < b))
        "})
      }
      
    ] stack {
      math_op2_bool(stack, |a, b| a < b)
    }
  ), (
    [
      OpHigherOrEqualsCommand '≥' ["heq"]
      {
        description: String::from(indoc! {"
          (.. b a) => (.. (a ≥ b))
        "})
      }
      
    ] stack {
      math_op2_bool(stack, |a, b| a >= b)
    }
  ), (
    [
      OpLessOrEqualsCommand '≤' ["leq"]
      {
        description: String::from(indoc! {"
          (.. b a) => (.. (a ≤ b))
        "})
      }
      
    ] stack {
      math_op2_bool(stack, |a, b| a <= b)
    }
  ),

    // op1
  (
    [
      OpSqrtCommand '√' ["sqrt"]
      {
        description: String::from(indoc! {"
          (.. a) => (.. (√ a))
        "})
      }
      
    ] stack {
      math_op1(stack, |a| a.sqrt())
    }
  )
);

