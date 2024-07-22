use std::ffi::CString;
use std::ptr;
use sys::{napi_callback_info, napi_env, napi_value, napi_value__};

pub fn add(left: f64, right: f64) -> f64 {
    left + right
}

unsafe extern "C" fn add_unsafe_code(env: napi_env, callback: napi_callback_info) -> napi_value {
    let mut a: f64 = 0.0;
    let mut b: f64 = 0.0;

    unsafe {
        let mut args = [ptr::null_mut(); 2];
        sys::napi_get_cb_info(
            env,
            callback,
            &mut 2,
            args.as_mut_ptr(),
            ptr::null_mut(),
            ptr::null_mut(),
        );
        sys::napi_get_value_double(env, args[0], &mut a);
        sys::napi_get_value_double(env, args[1], &mut b);
    };
    let v = add(a, b);
    let mut res = ptr::null_mut();
    unsafe {
        sys::napi_create_double(env, v, &mut res);
    };
    res
}
