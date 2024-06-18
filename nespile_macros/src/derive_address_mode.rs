use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Fields, ItemEnum};



fn get_address_mode_pattern(address_mode: &str) -> proc_macro2::TokenStream {
    match address_mode {
        "Accumulator" => quote!{ 0x0a | 0x2a | 0x4a | 0x6a },
        "Absolute" => quote!{
            0x20 |
            0x0c..=0x0f | 0x2c..=0x2f | 0x4c..=0x4f | 0x6d..=0x6f |
            0x8c..=0x8f | 0xac..=0xaf | 0xcc..=0xcf | 0xec..=0xef
        },
        "AbsoluteX" => quote!{
            0x1c..=0x1f | 0x3c..=0x3f | 0x5c..=0x5f | 0x7d..=0x7f |
            0x9c | 0x9d | 0xbc | 0xbd | 0xdc..=0xdf | 0xfc..=0xff
        },
        "AbsoluteY" => quote! { b if (b & 0x1d == 0x19) || (b & 0xde == 0x9e) },
        "Immediate" => quote! { b if (b & 0x1d == 0x09) || (b & 0x9d == 0x80) },
        "Implied" => quote! {
            b if (b & 0x1f == 0x08) || (b & 0x1f == 0x0a) || (b & 0x1f == 0x12) || (b & 0x1f == 0x18) || (b & 0x1f == 0x0a) ||
                 (b & 0x9f == 0x02) || (b & 0x9f == 0x00 && b != 0x20)
        },
        "Indirect" => quote!{ 0x6c },
        "IndirectX" => quote!{ b if (b & 0x1d == 0x01) },
        "IndirectY" => quote!{ b if (b & 0x1d == 0x11) },
        "Relative" => quote!{ b if (b & 0x1f == 0x10) },
        "ZeroPage" => quote!{ b if (b & 0x1c == 0x04) },
        "ZeroPageX" => quote!{
            0x14..=0x17 | 0x34..=0x37 | 0x54..=0x57 | 0x74..=0x77 |
            0x94 | 0x95 | 0xb4 | 0xb5 | 0xd4..=0xd7 | 0xf4..=0xf7
        },
        "ZeroPageY" => quote!{ 0x96 | 0x97 | 0xb6 | 0xb7 },
        _ => panic!("Unsupported address mode: {}", address_mode)
    }
}



pub fn derive_address_mode_parse(item: TokenStream) -> TokenStream {
    let enum_type = parse_macro_input!(item as ItemEnum);
    let enum_ident = enum_type.ident.clone();
    let enum_name = enum_ident.to_string();

    let variant_matchers = enum_type.variants.into_iter()
        .map(|variant| {
            let variant_ident = variant.ident;
            let pattern = get_address_mode_pattern(&variant_ident.to_string());

            match variant.fields {
                Fields::Unit => 
                    quote!{ #pattern => Ok(#enum_ident::#variant_ident) },
                Fields::Unnamed(fields) => {
                    let field_parsers = fields.unnamed.iter()
                        .map(|field| {
                            let field_type = &field.ty;
                            quote! { #field_type::read_options(reader, endian, ())? }
                        })
                        .collect::<Vec<proc_macro2::TokenStream>>();
                        
                    quote!{ #pattern => Ok(#enum_ident::#variant_ident(#(#field_parsers),*)) }
                },
                _ =>
                    panic!("You cannot derive BinReadAddressMode for an enum that has variants with named fields"),
            }
            
        })
        .collect::<Vec<proc_macro2::TokenStream>>();


    TokenStream::from(quote! {
        impl BinRead for #enum_ident {
            // The opcode, for determining address mode.
            type Args<'a> = u8;
        
            /// Based on https://www.nesdev.org/wiki/CPU_unofficial_opcodes
            fn read_options<R: io::Read + io::Seek>(
                reader: &mut R,
                endian: binrw::Endian,
                args: Self::Args<'_>,
            ) -> binrw::BinResult<Self> {
                match args {
                    #(#variant_matchers,)*
                    _ => Err(binrw::Error::AssertFail{
                        pos: reader.stream_position()?,
                        message: format!("Unexpected byte for address mode {}: 0x{:02x}", #enum_name, args)
                    })
                }
            }
        }
    })
}


