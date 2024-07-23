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
                    let desc = crate::register::gen_fn(env,exports);
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

    // 生成原始函数参数的长度
    let arg_cnt = args.len();

    // 这段Rust代码是在一个宏定义中使用的，它的目的是将Rust函数的参数转换为Node.js的N-API值。
    let js_args = args.iter().enumerate().map(|(index, &ref ident)| {
        let arg = syn::Ident::new(
            format!("arg_{}", index).as_str(),
            proc_macro2::Span::call_site(),
        );
        let ty = &ident.ty.clone();
        quote! {
            let #arg = <#ty as crate::value::NapiValue>::get_value_from_raw(env,args[#index]);
        }
    });

    // 这段代码的目的是在宏内部动态生成一个新的标识符，其名称基于输入的 name，
    let js_name = syn::Ident::new(
        format!("js_{}", name).as_str(),
        proc_macro2::Span::call_site(),
    );

    // 创建一个新的标识符（syn::Ident），其名称是通过将给定的 name 前缀加上 "_napi_" 来构造的。这里使用了 format! 宏来拼接字符串 "_napi_" 和 name 的值，然后通过 .as_str() 方法将其转换为字符串切片，因为 syn::Ident::new 函数的第一个参数需要的是一个字符串切片 (&str)。
    let init_js_fn = syn::Ident::new(
        format!("_napi_{}", name).as_str(),
        proc_macro2::Span::call_site(),
    );

    let run_args = args.iter().enumerate().map(|(index, _ident)| {
        // 使用 format!("arg_{}", index) 来创建一个新的字符串，该字符串以 "arg_" 开头，后跟元素的索引。这个字符串用于创建一个新的 syn::Ident 实例，表示一个标识符。syn::Ident::new 函数的第一个参数是标识符的名称，第二个参数是一个 Span，在这里使用 proc_macro2::Span::call_site() 来获取调用宏的位置。

        // syn::Ident::new 函数返回一个 Ident 类型的实例，这个实例可以在宏的输出中被用作变量名、函数名等标识符。
        let arg = syn::Ident::new(
            format!("arg_{}", index).as_str(),
            proc_macro2::Span::call_site(),
        );
        quote! {
           #arg
        }
    });

    //，quote! { ... }; 用于在宏中生成代码，
    // # 符号用于插入变量值。
    // 展示了如何使用quote!宏来生成包含不安全外部函数的Rust代码，
    // 如何在Rust中定义一个与Node.js的N-API交互的外部"C"函数，包括如何接收参数、调用Rust逻辑，并将结果返回给JavaScript。这是创建Node.js本地插件的关键步骤之一，允许Rust代码高效地与JavaScript代码交互。
    let expanded = quote! {
        #init
        #org_sig
        #org_block

        // 这两个参数的类型分别是sys::napi_env和sys::napi_callback_info，这表明该函数可能是为了与Node.js的N-API交互而设计的。N-API是一个C语言接口，允许创建独立于Node.js版本的本地插件。
        unsafe extern "C" fn #js_name(
            env: sys::napi_env,
            callback: sys::napi_callback_info,
        ) -> sys::napi_value {
            unsafe {
                let mut args = [std::ptr::null_mut(); #arg_cnt];
                // 调用sys::napi_get_cb_info函数来填充这个数组，这个函数从Node.js环境中获取回调信息，包括传递给函数的参数。
                sys::napi_get_cb_info(
                    env,
                    callback,
                    &mut #arg_cnt,
                    args.as_mut_ptr(),
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                );

                // #(#js_args)*是一个宏替换片段，用于处理或转换JavaScript传递过来的参数。
                #(#js_args)*

                //用另一个Rust函数（或可能是同一个函数的不同部分），这个函数执行实际的逻辑处理，并返回一个结果。
                let ret = #name(#(#run_args),*);

                // 将处理结果转换为N-API可以识别的值类型，以便将结果返回给JavaScript环境。
                // 这里#ret_ty是返回值的类型，
                // try_into_raw方法负责将Rust类型转换为N-API的值类型。
                <#ret_ty as crate::value::NapiValue>::try_into_raw(env,ret)
            }
        }

        // 函数#init_js_fn()（这里的#init_js_fn是一个占位符，表示实际的函数名将在宏展开时被替换）的主要任务是调用crate::register::register_fn函数。这个调用传递了两个参数：#org_name_str和Some(#js_name)。这里的#org_name_str和#js_name同样是占位符，分别代表原始函数名的字符串表示和一个可能的JavaScript函数名

        // crate::register::register_fn函数的作用是将一个Rust函数注册为可以从JavaScript调用的函数。这通常是在创建Node.js的本地扩展时进行的，允许JavaScript代码直接调用Rust代码。第一个参数指定了要注册的Rust函数的名称，而第二个参数Some(#js_name)提供了这个函数在JavaScript中的可选名称。如果提供了这个名称，JavaScript代码就可以通过这个名称来调用Rust函数。

        // 这段代码的目的是在程序启动时自动注册一个Rust函数，使其可以被JavaScript代码调用。这是在Rust中创建Node.js本地扩展的常见步骤之一，允许开发者利用Rust的性能优势在Node.js应用中执行高效的后端逻辑。
        #[ctor::ctor]
        fn #init_js_fn() {
            crate::register::register_fn(#org_name_str,Some(#js_name));
        }
    };

    expanded.into()
}
