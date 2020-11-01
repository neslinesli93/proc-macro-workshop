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

            pub fn build(&mut self) -> Result<#struct_name, Box<dyn std::error::Error>> {
                use std::convert::TryFrom;

                #struct_name::try_from(self).map_err(|error| error.into())
            }
        }

        impl std::convert::TryFrom<&mut #internal_struct> for #struct_name {
            type Error = String;

            fn try_from(value: &mut #internal_struct) -> Result<#struct_name, Self::Error> {
                let executable = value.executable.to_owned().ok_or_else(|| "Missing field: 'executable'".to_string())?;
                let args = value.args.to_owned().ok_or_else(|| "Missing field: 'args'".to_string())?;
                let env = value.env.to_owned().ok_or_else(|| "Missing field: 'env'".to_string())?;
                let current_dir = value.current_dir.to_owned().ok_or_else(|| "Missing field: 'current_dir'".to_string())?;

                Ok(#struct_name {
                    executable,
                    args,
                    env,
                    current_dir,
                })
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
