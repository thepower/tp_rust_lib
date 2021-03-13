extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use quote::Tokens;
use syn::{FnArg, Ident, Item};

#[proc_macro_attribute]
pub fn power_method(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let item: syn::Item = syn::parse(input).expect("failed to parse input");

    let mut output = quote! { #item };
    match item {
        Item::Fn(ref ifn) => {
            let fn_name = ifn.ident;
            let wrapper_name: Ident = format!("{}_wrapper", &fn_name).into();

            if ifn.decl.inputs.len() > 0 {
                let mut ps: Vec<Tokens> = Vec::new();
                let mut types: Vec<Tokens> = Vec::new();
                for (index, arg) in ifn.decl.inputs.iter().enumerate() {
                    match arg {
                        FnArg::Captured(ref c) => {
                            let ct = &c.ty;
                            ps.push(quote!{ args.#index });
                            types.push(quote!{ #ct });
                        }
                        _ => {}
                    }
                }

                output = quote!{
                  #[no_mangle]
                  pub fn #wrapper_name(){
                    let args : (#(#types),* ,) = get_args();
                    set_return(#fn_name(#(#ps),*));
                  }

                  #output
                };
            }else{
                output = quote!{
                  #[no_mangle]
                  pub fn #wrapper_name(){
                    set_return(#fn_name());
                  }

                  #output
                };

            }
        }
        _ => {}
    }
    output.into()
}
