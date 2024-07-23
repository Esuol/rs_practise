use std::ptr;
use sys::{napi_create_double, napi_env, napi_get_value_double, napi_value};

// NapiValue特质旨在为与N-API交互提供抽象，N-API是一个C语言编写的Node.js API，允许原生模块与JavaScript代码进行交互。
pub trait NapiValue {
    // get_value_from_raw：这个方法接受两个参数，env和value，分别代表N-API的环境句柄和N-API值。它返回特质实现者自身的类型。这个方法的目的是从原始的N-API值中提取Rust类型的值。
    fn get_value_from_raw(env: napi_env, value: napi_value) -> Self;
    // try_into_raw：这个方法也接受两个参数，env和value，其中value是特质实现者自身的类型。它返回一个N-API值。这个方法的目的是将Rust类型的值转换为N-API值。
    fn try_into_raw(env: napi_env, value: Self) -> napi_value;
}

impl NapiValue for f64 {
    fn get_value_from_raw(env: napi_env, value: napi_value) -> f64 {
        // 首先定义了一个f64类型的变量res，初始值为0.0。
        let mut res: f64 = 0.0;
        // 块调用napi_get_value_double函数，这是N-API提供的一个函数，用于从N-API的值中提取出双精度浮点数（f64）
        unsafe {
            napi_get_value_double(env, value, &mut res);
        };
        // 即从N-API值中提取出的f64值。
        res
    }

    fn try_into_raw(env: napi_env, value: f64) -> napi_value {
        // 首先通过 ptr::null_mut() 创建一个空指针 res，这个指针将用来存储函数 napi_create_double 的结果。
        let mut res = ptr::null_mut();
        unsafe {
            //这个函数调用的目的是将传入的浮点数 value 转换为一个 Node.js 可以理解的 napi_value 类型的值，并将这个值的指针存储在 res 中。
            napi_create_double(env, value, &mut res);
        };
        res
    }
}
