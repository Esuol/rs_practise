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

    // 通过REGISTER_INIT.load(Ordering::SeqCst)检查REGISTER_INIT的值。
    // 这里使用的Ordering::SeqCst保证了这个操作在多线程环境下的内存顺序性，确保这个操作看起来是在一个单一的、全局的操作序列中执行的。
    let init = match REGISTER_INIT.load(Ordering::SeqCst) {
        // 如果REGISTER_INIT的值为false，表示模块尚未注册，那么就执行注册逻辑。
        false => {
            // 过REGISTER_INIT.store(true, Ordering::SeqCst)将REGISTER_INIT的值设置为true，以防止后续的重复注册。
            REGISTER_INIT.store(true, Ordering::SeqCst);
            quote! {
                // 代码定义了一个名为napi_register_module_v1的外部"C"函数，这个函数是Node.js原生模块注册的入口点
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
                        // 并通过sys::napi_module_register函数将其注册到Node.js环境中。
                        sys::napi_module_register(&mut modules);
                    };
                }
            }
        }
        _ => {
            quote!()
        }
    };

    // ast.sig获取函数的签名
    let sig = &ast.sig;
    // 获取函数参数
    let params = &ast.sig.inputs;
    // 获取函数返回值
    let result = &ast.sig.output;
    // 获取函数块
    let fn_blocks = &ast.block;

    // 生成原始函数返回值
    let ret_ty = match result {
        syn::ReturnType::Type(_, ty) => quote! { #ty },
        syn::ReturnType::Default => quote! { () },
    };

    // 生成原始函数签名
    let org_sig = quote! { #sig };

    // 生成原始函数块
    let org_block = quote! { #fn_blocks };

    // 生成原始函数参数
    let args = params
        .iter()
        .filter_map(|arg| match arg {
            // ref关键字用于在模式匹配中创建对匹配值的引用，而不是获取其所有权。这在处理不想获取所有权但需要访问数据的场景中非常有用。
            // ref p表示创建一个名为p的变量，它是对匹配到的值的引用，而不是值本身的所有权。这里的p是对Typed变体中包含的值的引用。
            syn::FnArg::Typed(ref p) => {
                // 里的ident是对Ident中包含的值的引用。
                // 在syn库中，Ident是用来表示Rust程序中的标识符的类型。
                // syn::Pat::Ident包含了关于标识符的信息，比如它的名称。
                if let Ident(ref ident) = *p.pat {
                    Some(NapiFnArgs {
                        _ident: ident.ident.clone(),
                        ty: p.ty.clone().deref().clone(),
                    })
                } else {
                    None
                }
            }
            syn::FnArg::Receiver(ref _p) => None,
        })
        .collect::<Vec<NapiFnArgs>>();
}
