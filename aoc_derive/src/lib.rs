use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn aoc(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = parse_macro_input!(item as ItemFn);

    let start = quote!(__measure_time_start_instant);
    let expanded = quote! {
        #(#attrs)*
        #vis #sig {
            let #start = ::std::time::Instant::now();
            let ret = #block;
            let duration = #start.elapsed();

            if duration.as_secs() > 0 {
                ::std::println!("Elapsed: \x1b[1m{}s\x1b[0m", duration.as_secs());
            }
            else if duration.as_millis() > 0 {
                ::std::println!("Elapsed: \x1b[1m{}ms\x1b[0m", duration.as_millis());
            } else {
                ::std::println!("Elapsed: \x1b[1m{}μs\x1b[0m", duration.as_micros());
            }

            ret
        }
    };

    TokenStream::from(expanded)
}
