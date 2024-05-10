use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub(crate) fn process_enum_from(input: DeriveInput) -> TokenStream {
    // get the ident
    let name = input.ident;
    // get the generics
    let generics = input.generics;
    // get enum variants
    let variants = match input.data {
        syn::Data::Enum(data) => data.variants,
        _ => panic!("EnumFrom only works with enums"),
    };
    // for each variant, get the ident and fields
    let from_impls = variants.iter().map(|variant| {
        let ident = &variant.ident;
        match &variant.fields {
            syn::Fields::Unnamed(fields) => {
                // only support one field
                if fields.unnamed.len() != 1 {
                    quote! {}
                } else {
                    let field = fields.unnamed.first().expect("should have 1 field");
                    let ty = &field.ty;
                    quote! {
                             impl #generics From<#ty> for #name #generics {
                                    fn from(v: #ty) -> Self {
                                                    Self::#ident(v)
                                    }
                            }
                    }
                }
            }

            _ => quote! {},
        }
    });
    quote! {
            #(#from_impls)*
    }
}
