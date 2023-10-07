use proc_macro::{Span, TokenStream};

use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    let name = derive_input.ident;
    let name_builder = Ident::new(&format!("{}Builder", name), Span::call_site().into());
    let expanded = quote! {
        pub struct #name_builder {
            executable: Option<String>,
            args: Option<Vec<String>>,
            env: Option<Vec<String>>,
            current_dir: Option<String>,
        }

        impl #name {
            pub fn builder() -> #name_builder {
                #name_builder {
                    executable: None,
                    args: None,
                    env: None,
                    current_dir: None,
                }
            }
        }

        impl #name_builder {
            pub fn executable(&mut self, executable: String) -> &mut Self {
                self.executable = Some(executable);
                self
            }
            pub fn args(&mut self, args: Vec<String>) -> &mut Self {
                self.args = Some(args);
                self
            }
            pub fn env(&mut self, env: Vec<String>) -> &mut Self {
                self.env = Some(env);
                self
            }
            pub fn current_dir(&mut self, current_dir: String) -> &mut Self {
                self.current_dir = Some(current_dir);
                self
            }
            pub fn build(&mut self) -> Result<#name, Box<dyn std::error::Error>> {
                let executable = self.executable.clone().ok_or("")?;
                let args = self.args.clone().ok_or("")?;
                let env = self.env.clone().ok_or("")?;
                let current_dir = self.current_dir.clone().ok_or("")?;
                Ok(#name {
                    executable,
                    args,
                    env,
                    current_dir,
                })
            }
        }

    };
    expanded.into()
}
