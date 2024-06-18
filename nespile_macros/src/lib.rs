extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Fields, ItemEnum};



mod byte_parser;
mod derive_address_mode;



#[proc_macro_attribute]
pub fn parse_byte_with(arg_tokens: TokenStream, enum_tokens: TokenStream) -> TokenStream {
    crate::byte_parser::parse_byte_with(arg_tokens, enum_tokens)
}

#[proc_macro_derive(BinReadAddressMode)]
pub fn derive_address_mode_parse(item: TokenStream) -> TokenStream {
    crate::derive_address_mode::derive_address_mode_parse(item)
}
#[proc_macro_derive(OpcodeArgs)]
pub fn derive_opcode_arguments(item: TokenStream) -> TokenStream {
    let enum_type = parse_macro_input!(item as ItemEnum);
    let enum_ident = enum_type.ident;

    let variant_matchers = enum_type.variants.into_iter()
        .filter_map(|variant| {
            let variant_ident = variant.ident;

            match variant.fields {
                Fields::Unnamed(_) => {
                    Some(quote! {
                        &#enum_ident::#variant_ident(ref f) => Some((*f).into())
                    })           
                },
                _ => None,
            }
        })
        .collect::<Vec<proc_macro2::TokenStream>>();

    TokenStream::from(quote! {
        impl #enum_ident {
            pub fn argument(&self) -> Option<AddressMode> {
                match self {
                    #(#variant_matchers,)*
                    _ => None
                }
            }
        }
    })
}



#[proc_macro_derive(VariantNames)]
pub fn derive_enum_variant_names(item: TokenStream) -> TokenStream {
    let enum_type = parse_macro_input!(item as ItemEnum);
    let enum_ident = enum_type.ident;
    

    let variant_matchers = enum_type.variants.into_iter()
        .map(|variant| {
            let variant_ident = variant.ident;
            let variant_name = variant_ident.to_string();

            if variant.fields.is_empty() {
                quote! { &#enum_ident::#variant_ident => #variant_name }            
            } else {
                quote! { &#enum_ident::#variant_ident(_) => #variant_name }            
            }
        })
        .collect::<Vec<proc_macro2::TokenStream>>();

    TokenStream::from(quote! {
        impl #enum_ident {
            pub fn variant_name(&self) -> &str {
                match self {
                    #(#variant_matchers),*
                }
            }
        }
    })
}
