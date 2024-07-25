// extern crate napi_build; 这一行引入了一个名为 napi_build 的外部 crate。
// napi_build 是一个用于构建 Node.js 原生模块的工具库，它简化了与 Node.js 的集成过程。
extern crate napi_build;

fn main() {
  // 这个函数调用配置并准备构建环境，以便正确地编译和链接 Node.js 原生模块。
  napi_build::setup();
}
