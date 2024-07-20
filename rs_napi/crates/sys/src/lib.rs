#![allow(non_camel_case_types)]

use std::os::raw::{c_char, c_int, c_uint, c_void};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct napi_value__ {
    _unused: [u8; 0],
}

pub type napi_value = *mut napi_value__;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct napi_env__ {
    _unused: [u8; 0],
}

pub type napi_env = *mut napi_env__;

pub type napi_status = i32;
