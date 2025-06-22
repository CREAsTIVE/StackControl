
use itertools::{join, Itertools};
use stack_control::{bytecode::commands::core::bind_default_commands, compiletime::{compiler::Scope, lexer::split_string_to_tokens}, runtime::stack::Stack, utils::execution::{execute_commands, simplify_tokens}};
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
  fn alert(s: &str);
  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);
}

#[wasm_bindgen]
pub fn ensure() {
  log("stack-control wasm is loaded!");
}

#[wasm_bindgen]
pub struct JSScope {
  #[wasm_bindgen(skip)]
  pub scope: Scope
}

#[wasm_bindgen]
pub fn make_scope() -> JSScope {
  let mut scope = JSScope { scope: Scope::new() };
  bind_default_commands(&mut scope.scope.command_map);
  scope
}

#[wasm_bindgen(getter_with_clone)]
pub struct ExecutionResult {
  stack: Stack,
  pub simplified: Option<String>
}

#[wasm_bindgen]
impl ExecutionResult {
  #[wasm_bindgen]
  pub fn join_stack(&self, sep: &str) -> String {
    self.stack.copy().into_iter()
      .map(|e| e.to_string())
      .join(sep)
  }
}

#[wasm_bindgen]
pub fn execute(code: &str, scope: &mut JSScope, simplify: bool) -> Result<ExecutionResult, String> {
  let mut stack = Stack::new();
  let tokens = split_string_to_tokens(code);
  let commands = scope.scope.compile(tokens.iter())
    .or_else(|e| Err(format!("Compilation exception: {}", e.to_string())))?;
  execute_commands(commands, &mut stack)
    .or_else(|e| Err(format!("Runtime exception: {}", e.to_string())))?;

  Ok(ExecutionResult {
    stack,
    simplified: 
      if simplify { 
        Some(join(simplify_tokens(&tokens, &scope.scope.command_map).map(|t| t.to_string()), ""))
      } else {None}
  })
}