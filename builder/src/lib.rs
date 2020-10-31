extern crate proc_macro;

use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Builder)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = input.ident;
    let internal_struct = format_ident!("{}Builder", struct_name);

    let expanded = quote! {
        impl #struct_name {
            pub fn builder() -> #internal_struct {
                #internal_struct{
                    executable: None,
                    args: None,
                    env: None,
                    current_dir: None,
                }
            }
        }

        pub struct #internal_struct {
            executable: Option<String>,
            args: Option<Vec<String>>,
            env: Option<Vec<String>>,
            current_dir: Option<String>,
        };
    };

    proc_macro::TokenStream::from(expanded)
}
