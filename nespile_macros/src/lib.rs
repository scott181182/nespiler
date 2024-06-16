extern crate proc_macro;
use proc_macro::TokenStream;



#[proc_macro]
pub fn byte_parser(item: TokenStream) -> TokenStream {
    println!("{:?}", item);
    item
}
