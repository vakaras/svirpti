use proc_macro::TokenStream;
use quote::quote;
use svirpti_vir::high::parse as high;
use syn::parse_macro_input;

#[proc_macro]
pub fn vir_high(input: TokenStream) -> TokenStream {
    let declarations = parse_macro_input!(input as high::ProgramFragment);
    (quote! {
        {
            use svirpti_vir::high::*;
            #declarations
        }
    })
    .into()
}
