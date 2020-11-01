extern crate proc_macro;

use proc_macro_error::abort_call_site;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields, FieldsNamed, Type, TypePath};

#[proc_macro_derive(Builder)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let fields = get_fields(input.clone());
    let get_current_dir = gen_getter_string(fields, "current_dir".to_string());

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
            #get_current_dir

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
                let current_dir = #internal_struct::get_current_dir(value)?;

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

fn get_fields(input: DeriveInput) -> FieldsNamed {
    match input.data {
        Data::Struct(DataStruct { fields, .. }) => match fields {
            Fields::Named(fields) => fields,
            _ => abort_call_site!("only named fields are supported"),
        },
        _ => abort_call_site!("only structs are supported"),
    }
}

fn gen_getter_string(fields: FieldsNamed, ident: String) -> proc_macro2::TokenStream {
    let getter = format_ident!("get_{}", ident);

    let field = fields.named.into_iter().find(|f| match &f.ident {
        Some(i) => i.to_string() == ident,
        _ => false,
    });

    if field.is_none() {
        let result = quote! {
            pub fn #getter(&mut self) -> Result<Option<String>, String> { Ok(None) }
        };

        return result.into();
    }

    let field_ident = field.clone().unwrap().ident.unwrap();

    let optional: bool = field
        .map(|f| match f.ty {
            Type::Path(TypePath { path, .. }) => !path
                .segments
                .into_iter()
                .any(|s| s.ident.to_string() == "Option".to_string()),
            _ => true,
        })
        .unwrap_or_else(|| true);

    let result = if optional {
        quote! {
            pub fn #getter(&mut self) -> Result<Option<String>, String> {
                Ok(self.#field_ident)
            }
        }
    } else {
        let msg = format!("Missing field: {}", ident);
        quote! {
            pub fn #getter(&mut self) -> Result<Option<String>, String> {
                self.#field_ident.ok_or_else(|| #msg)
            }
        }
    };

    result.into()
}
