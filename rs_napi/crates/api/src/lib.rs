use std::ffi::CString;
use std::ptr;
use sys::{napi_callback_info, napi_env, napi_value};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}
