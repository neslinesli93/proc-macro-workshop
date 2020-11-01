extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro_error::abort_call_site;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    if let DeriveInput {
        ref ident,
        data:
            syn::Data::Struct(syn::DataStruct {
                fields: syn::Fields::Named(ref fields),
                ..
            }),
        ..
    } = input
    {
        let internal_struct_name = format_ident!("{}Builder", ident.to_string());

        let internal_struct_fields = internal_struct_fields(fields);
        let internal_struct_values = internal_struct_values(fields);
        let internal_struct_methods = internal_struct_methods(fields);

        let internal_struct = quote! {
            pub struct #internal_struct_name {
                #(#internal_struct_fields,)*
            }
        };

        let setters = quote! {
            impl #internal_struct_name {
                #(#internal_struct_methods)*
            }
        };

        let builder = quote! {
            impl #ident {
                pub fn builder() -> #internal_struct_name {
                    #internal_struct_name {
                        #(#internal_struct_values,)*
                    }
                }

            }
        };

        let result = quote! {
            #internal_struct

            #setters

            #builder
        };

        result.into()
    } else {
        abort_call_site!("builder macro only supports structs");
    }
}

fn internal_struct_fields(fields: &syn::FieldsNamed) -> Vec<proc_macro2::TokenStream> {
    fields
        .named
        .iter()
        .map(|f| {
            let syn::Field {
                ref ident, ref ty, ..
            } = f;

            quote! { #ident: Option<#ty> }
        })
        .collect::<_>()
}

fn internal_struct_values(fields: &syn::FieldsNamed) -> Vec<proc_macro2::TokenStream> {
    fields
        .named
        .iter()
        .map(|f| {
            let syn::Field { ref ident, .. } = f;

            quote! { #ident: None }
        })
        .collect::<_>()
}

fn internal_struct_methods(fields: &syn::FieldsNamed) -> Vec<proc_macro2::TokenStream> {
    fields
        .named
        .iter()
        .map(|f| {
            let syn::Field {
                ref ident, ref ty, ..
            } = f;

            quote! {
               fn #ident(&mut self, value: #ty) -> &mut Self {
                   self.#ident = Some(value);
                   self
               }
            }
        })
        .collect::<_>()
}
