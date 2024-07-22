use std::ffi::CString;
use std::ptr;
use sys::{napi_callback_info, napi_env, napi_value};

pub fn add(left: f64, right: f64) -> f64 {
    left + right
}

unsafe extern "C" fn add_unsafe_code(env: napi_env, callback: napi_callback_info) -> napi_value {
    // 初始化两个浮点数变量a和b为0.0，这两个变量将用于存储从JavaScript传入的数值。
    let mut a: f64 = 0.0;
    let mut b: f64 = 0.0;

    unsafe {
        // 使用sys::napi_get_cb_info函数从回调中获取参数。
        // 这个函数需要参数的数量和一个数组来存储参数值的指针。
        // 这里，args数组被初始化为包含两个空指针，用于接收从JavaScript传入的参数。
        let mut args = [ptr::null_mut(); 2];
        sys::napi_get_cb_info(
            env,
            callback,
            &mut 2,
            args.as_mut_ptr(),
            ptr::null_mut(),
            ptr::null_mut(),
        );
        // 使用sys::napi_get_value_double函数两次，分别获取args数组中的两个参数，并将它们的值存储在a和b变量中。
        sys::napi_get_value_double(env, args[0], &mut a);
        sys::napi_get_value_double(env, args[1], &mut b);
    };
    // 调用add函数（未在代码片段中定义）将a和b相加，结果存储在变量v中。
    let v = add(a, b);

    let mut res = ptr::null_mut();
    unsafe {
        // 使用sys::napi_create_double函数将加法结果v转换为N-API可以识别的nap i_value类型，以便将结果返回给JavaScript。
        sys::napi_create_double(env, v, &mut res);
    };
    res
}

// unsafe extern "C" fn register_fn(env: napi_env, exports: napi_value) -> napi_value {：定义了一个不安全的外部 "C" 函数 register_fn，这意味着该函数可以从 C 代码中调用。它接收两个参数：env（一个表示 Node.js 环境的 napi_env 类型的变量）和 exports（一个表示模块导出的 napi_value 类型的变量）。函数返回一个 napi_value 类型的值，即模块的导出。
unsafe extern "C" fn register_fn(env: napi_env, exports: napi_value) -> napi_value {
    // ：创建一个新的 CString，包含要注册的函数名 "add"。CString 用于与 C 代码交互，因为它保证了字符串的结尾有一个空字符
    let name = CString::new("add").unwrap();
    let desc = [sys::napi_property_descriptor {
        utf8name: name.as_ptr().cast(),
        name: ptr::null_mut(),
        getter: None,
        setter: None,
        method: Some(add_unsafe_code),
        attributes: 0,
        value: ptr::null_mut(),
        data: ptr::null_mut(),
    }];

    //将上面定义的属性（函数）添加到模块的导出中。这个函数接收环境变量 env、模块导出 exports、属性描述符数组的长度以及数组的指针
    sys::napi_define_properties(env, exports, desc.len(), desc.as_ptr());
    exports
}

// 这段Rust代码展示了如何使用ctor库来在Rust中定义一个在加载时自动执行的函数，以及如何使用Rust的FFI（Foreign Function Interface）功能与Node.js的N-API交互，从而注册一个原生模块
#[ctor::ctor]
fn export_module() {
    let name = CString::new("api").unwrap();
    let mut modules = sys::napi_module {
        nm_version: 1,
        nm_filename: ptr::null_mut(),
        nm_flags: 0,
        nm_modname: name.as_ptr().cast(),
        nm_priv: ptr::null_mut() as *mut _,
        nm_register_func: Some(register_fn),
        reserved: [ptr::null_mut() as *mut _; 4],
    };
    unsafe {
        // ，将modules结构体注册为一个N-API模块。这个调用是不安全的，因为它涉及到FFI调用和裸指针操作，这是Rust中潜在的不安全操作。
        sys::napi_module_register(&mut modules);
    }
}
