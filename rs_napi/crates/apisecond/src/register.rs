use once_cell::sync::Lazy;
use std::sync::RwLock;
use sys::{napi_callback, napi_env, napi_value};

// Lazy：来自 once_cell crate，用于延迟初始化静态变量。Lazy 确保 REGISTER_FN 在首次访问时才会被初始化，并且初始化的结果会被缓存起来，后续访问直接使用缓存的结果。
// 这行代码定义了一个线程安全的、延迟初始化的静态变量，用于存储一组可能在程序的多个地方注册和使用的回调函数
pub(crate) static REGISTER_FN: Lazy<RwLock<Vec<(&'static str, napi_callback)>>> =
    Lazy::new(Default::default);

//它接受两个参数：js_name 和 cb。js_name 是一个静态生命周期的字符串切片，表示要注册的 JavaScript 函数名。
// cb 是一个类型为 napi_callback 的回调函数，这是一个与 Node.js 的原生 API 接口（N-API）相关的类型，用于定义 JavaScript 调用的原生函数。
pub fn register_fn(js_name: &'static str, cb: napi_callback) {
    REGISTER_FN.write().unwrap().push((js_name, cb));
}

// 其目的是在Node.js的N-API环境中注册一系列的函数。
// 这个过程涉及到几个关键步骤，包括获取全局函数注册表、创建N-API函数，并将这些函数绑定到一个导出对象上。
pub fn gen_fn(env: napi_env, exports: napi_value) {
    let register = REGISTER_FN.write().unwrap();
    register.iter().for_each(|(name, cb)| {
        // let mut fn_ptr = std::ptr::null_mut();初始化一个空指针，它将用于存储N-API创建的函数对象的引用。
        let mut fn_ptr = std::ptr::null_mut();
        unsafe {
            //首先通过format!("{}\0", *name).as_ptr().cast();创建一个以null终止的字符串，这是C语言字符串的要求。这个字符串用作N-API函数的名称。cast()方法将字符串指针转换为适当的类型。
            let n = format!("{}\0", *name).as_ptr().cast();
            // 调用创建一个新的N-API函数
            // 创建的函数对象引用存储在fn_ptr中。
            sys::napi_create_function(env, n, name.len(), *cb, std::ptr::null_mut(), &mut fn_ptr);
            // 新创建的函数对象作为一个命名属性添加到exports对象上。这样，当模块被导入到Node.js环境时，这些函数就会作为模块的导出可用。
            sys::napi_set_named_property(env, exports, n, fn_ptr);
        };
    })
}
