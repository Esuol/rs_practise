use napi::{CallContext, Env, JsNumber, JsObject, Result, Task};
use napi_derive::{js_function, module_exports};

struct ComputeFib {
  n: u32,
}

fn fibonacci_native(n: u32) -> u32 {
  match n {
    1 | 2 => 1,
    _ => fibonacci_native(n - 1) + fibonacci_native(n - 2),
  }
}

impl ComputeFib {
  pub fn new(n: u32) -> ComputeFib {
    ComputeFib { n }
  }
}

impl Task for ComputeFib {
  type Output = u32;
  type JsValue = JsNumber;

  fn compute(&mut self) -> Result<Self::Output> {
    Ok(fibonacci_native(self.n))
  }

  fn resolve(&mut self, env: Env, output: Self::Output) -> Result<Self::JsValue> {
    env.create_uint32(output)
  }
}

// 在这段代码中，我们定义了一个名为 native_fib 的函数，它被标记为 #[js_function(0)]。这个标记表明该函数是一个 JavaScript 函数，可以从 JavaScript 环境中调用，并且它不接受任何参数（参数数量为 0）。
#[js_function(0)]
pub fn native_fib(ctx: CallContext) -> Result<JsObject> {
  let task = ComputeFib::new(20);
  // 使用 ctx.env.spawn(task)? 将这个任务提交给 JavaScript 环境的异步运行时。spawn 方法会返回一个 async_promise，它是一个包含异步操作结果的承诺对象。
  let async_promise = ctx.env.spawn(task)?;
  // 函数返回 async_promise.promise_object()，这是一个 JavaScript 的 Promise 对象，表示异步操作的结果。通过这种方式，Rust 代码可以与 JavaScript 代码进行异步交互，并返回一个 Promise，以便在 JavaScript 中处理异步计算的结果。
  Ok(async_promise.promise_object())
}

// 这个标记表明该函数用于导出模块中的方法，使其可以在 JavaScript 环境中使用。
#[module_exports]
pub fn register_js(mut exports: JsObject) -> Result<()> {
  // create_named_method 方法将 native_fib 函数绑定到 exports 对象上，使其可以通过 exports.nativeUVFib 在 JavaScript 中调用。
  exports.create_named_method("nativeUVFib", native_fib)?;
  Ok(())
}
