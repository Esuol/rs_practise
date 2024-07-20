use proc_macro::TokenStream;
use proc_macro2::Ident;

// 声明为派生宏并且命名为 MyDebug
#[proc_macro_derive(MyDebug)]
pub fn debug(item: TokenStream) -> TokenStream {
    // 解析 TokenStream 流 转 ast
    let item: DeriveInput = syn::parse(item).unwrap();
    let name = &item.ident;

    // 字段信息
    let mut content: Vec<Ident> = Vec::new();
    // 格式化代码 {}[]
    let mut format_code = proc_macro2::TokenStream::new();

    // 如果是结构体
    if let syn::Data::Struct(s) = item.data {
        s.fields.iter().for_each(|f| {
            // 获取字段名
            let n = f.ident.clone().unwrap();
            // 将字段名存入 content
            content.push(n.clone());
            // 构造成 `字段名:{},` 的 TokenStream 流
            format_code.extend(quote! { #n:{},});
        })
    }

    // 去掉最后一个逗号
    // 使用quote!宏来生成新的TokenStream，其中生成过程中可以使用我们定义的所有变量。
    let prefix = quote!(
        #name format: #format_code
    )
    .to_string();

    let d = quote!(
        // 使用 name 实现对结构体的实现声明
        impl std::fmt::Debug for #name {
            fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
                // 使用 write! 宏来格式化输出
                write!(fmt, #prefix, #(self.#content),*)
            }
        }
    );

    // 使用into实现，尝试进行数据类型转化
    // proc_macro2的数据结构， -> proc_macro
    d.into()
}
