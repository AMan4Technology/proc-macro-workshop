use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Ident, Type};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let builder = format_ident!("{}Builder", name);

    let methods = match input.data {
        Data::Enum(_) => quote!(/* Enum */),
        Data::Union(_) => quote!(/* Union */),
        Data::Struct(struct_data) => {
            let fields: Vec<(&Ident, &Type)> = struct_data
                .fields
                .iter()
                .filter(|f| f.ident.is_some())
                .map(|f| (f.ident.as_ref().unwrap(), &f.ty))
                .collect();

            let names: Vec<&Ident> = fields.iter().map(|f| f.0).collect();
            let tys: Vec<&Type> = fields.iter().map(|f| f.1).collect();

            quote! {
                pub fn build(&mut self) -> Result<Command, Box<dyn Error>> {
                    Ok(#name{
                        #(
                            #names: self.#names.to_owned().unwrap(),
                        )*
                    })
                }


                #(
                    fn #names(&mut self, #names: #tys) -> &mut Self {
                        self.#names = Some(#names);
                        self
                    }
                )*
            }
        }
    };

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        use std::error::Error;

        impl #name {
            pub fn builder() -> #builder {
                #builder {
                   executable: None,
                   args: None,
                   env: None,
                   current_dir: None,
                }
            }
        }

        pub struct #builder {
            executable: Option<String>,
            args: Option<Vec<String>>,
            env: Option<Vec<String>>,
            current_dir: Option<String>,
        }

        impl #builder {
            #methods
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
