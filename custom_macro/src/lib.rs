use proc_macro::TokenStream;
use proc_macro2::Ident;

// 声明为派生宏并且命名为 MyDebug
#[proc_macro_derive(MyDebug)]
pub fn debug(item: TokenStream) -> TokenStream {}
