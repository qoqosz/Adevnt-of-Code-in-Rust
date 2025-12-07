use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parser, parse_macro_input, punctuated::Punctuated, Expr, ItemFn, Token};

#[proc_macro_attribute]
pub fn aoc(attr: TokenStream, item: TokenStream) -> TokenStream {
    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = parse_macro_input!(item as ItemFn);

    let exprs = Punctuated::<Expr, Token![,]>::parse_terminated
        .parse(attr)
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();

    let (year, day) = match &*exprs {
        [year, day] => (year, day),
        _ => panic!("aoc: Invalid number of parameters; expected 2"),
    };

    let fn_name = sig.ident.clone();
    let start = quote!(__measure_time_start_instant);
    let expanded = quote! {

        #(#attrs)*
        #vis #sig {
            fn inner() {
                #block
            }

            let #start = ::std::time::Instant::now();
            let ret = inner();
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

        inventory::submit! {
            aoc_core::Solution::new(#year, #day, #fn_name)
        }
    };

    TokenStream::from(expanded)
}
