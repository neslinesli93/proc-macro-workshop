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

        impl #internal_struct {
            fn executable(&mut self, executable: String) -> &mut Self {
                self.executable = Some(executable);
                self
            }

            fn args(&mut self, args: Vec<String>) -> &mut Self {
                self.args = Some(args);
                self
            }

            fn env(&mut self, env: Vec<String>) -> &mut Self {
                self.env = Some(env);
                self
            }

            fn current_dir(&mut self, current_dir: String) -> &mut Self {
                self.current_dir = Some(current_dir);
                self
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
