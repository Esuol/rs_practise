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

#[proc_macro_attribute]
pub fn api(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let ast = syn::parse::<ItemFn>(input).unwrap();

    // 函数名
    let name = &ast.sig.ident;
    let org_name_str = quote! {#name}.to_string();
    println!("{name:?}", name);
    println!("{org_name_str:?}", org_name_str);

    let init = match REGISTER_INIT.load(Ordering::SeqCst) {
        false => {
            REGISTER_INIT.store(true, Ordering::SeqCst);
            quote! {
                unsafe extern "C" fn napi_register_module_v1(
                    env: sys::napi_env,
                    exports: sys::napi_value,
                ) -> sys::napi_value {
                    let desc = crate::register::gen_fn(env. exports);
                    exports
                }
                #[ctor::ctor]
                fn init() {
                    let name = std::ffi::CString::new("api").unwrap();
                    let mut modules = sys::napi_module {
                        nm_version: 1,
                        nm_filename: std::ptr::null_mut(),
                        nm_flags: 0,
                        nm_modname: name.as_ptr().cast(),
                        nm_priv: std::ptr::null_mut() as *mut _,
                        nm_register_func: Some(napi_register_module_v1),
                        reserved: [std::ptr::null_mut() as *mut _; 4],
                    };
                    unsafe {
                        sys::napi_module_register(&mut modules);
                    };
                }
            }
        }
        _ => {
            quote!()
        }
    };
}
