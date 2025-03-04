use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn log_time_elapsed(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    let fn_body = &input.block;

    let output = quote! {
        fn #fn_name() {
            let start = std::time::Instant::now();
            #fn_body
            let duration = start.elapsed();
            println!("Execution time of {}: {:?}", stringify!(#fn_name), duration);
        }
    };

    output.into()
}