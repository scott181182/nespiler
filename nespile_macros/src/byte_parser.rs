use std::collections::HashMap;

use proc_macro::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, Fields, Ident, ItemEnum, Token};
use syn::parse::{Parse, ParseStream};



struct Args {
    match_cases: Vec<Ident>
}
impl Parse for Args {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let vars = Punctuated::<Ident, Token![,]>::parse_terminated(input)?;
        Ok(Args {
            match_cases: vars.into_iter().collect(),
        })
    }
}
impl Args {
    pub fn to_matches(self) -> [Ident; 256] {
        self.match_cases.into_iter().cycle().take(256).collect::<Vec<Ident>>()
            .try_into().expect("Could not convert Vec to [Ident; 256]")
    }
}



pub fn parse_byte_with(arg_tokens: TokenStream, enum_tokens: TokenStream) -> TokenStream {
    let enum_type = parse_macro_input!(enum_tokens as ItemEnum);
    let args = parse_macro_input!(arg_tokens as Args);

    let ItemEnum{
        ident: enum_ident,
        variants: enum_variants,
        ..
    } = enum_type.clone();
    let variant_map = enum_variants.into_iter()
        .map(|v| (v.ident, v.fields))
        .collect::<HashMap<Ident, Fields>>();

    let match_lines = args.to_matches().into_iter()
        .enumerate()
        .map(|(idx, ident)| {
            let variant_fields = variant_map.get(&ident)
                .expect("Could not find variant with name");
            let byte = idx as u8;


            match variant_fields {
                Fields::Unit => 
                    quote! { #byte => Ok(#enum_ident::#ident) },
                Fields::Unnamed(fields) => {
                    let variant_parsers = fields.unnamed.iter()
                        .map(|field| {
                            let field_type = &field.ty;
                            quote! { #field_type::read_options(reader, endian, byte)? }
                        })
                        .collect::<Vec<proc_macro2::TokenStream>>();
                        
                    quote! { #byte => Ok(#enum_ident::#ident(#(#variant_parsers),*)) }
                },
                _ =>
                    panic!("You cannot use `parse_byte_with` with an enum that has named fields in variants"),
            }
        })
        .collect::<Vec<proc_macro2::TokenStream>>();
    
    TokenStream::from(quote! {
        #enum_type

        impl BinRead for #enum_ident {
            type Args<'a> = ();
            
            fn read_options<R: std::io::Read + std::io::Seek>(
                reader: &mut R,
                endian: binrw::Endian,
                args: Self::Args<'_>,
            ) -> binrw::BinResult<Self> {
                let byte = u8::read_options(reader, endian, ())?;

                match byte {
                    #(#match_lines),*
                }
            }
        }
    })

    // enum_type.to_token_stream().into()
}
