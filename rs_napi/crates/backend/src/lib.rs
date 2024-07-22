use proc_macro::TokenStream;
use quote::quote;
use std::ops::Deref;
use std::sync::atomic::{AtomicBool, Ordering};
use syn::{ItemFn, Pat::Ident, Type};

struct NapiFnArgs {
    _ident: syn::Ident,
    ty: Type,
}

// 声明原子操作 用于确保当前为第一个宏展开
static REGISTER_INIT: AtomicBool = AtomicBool::new(false);                          
