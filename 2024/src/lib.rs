use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident, ItemFn, LitInt};

#[proc_macro_attribute]
pub fn main(args: TokenStream, input: TokenStream) -> TokenStream {
    let day = parse_macro_input!(args as LitInt)
        .base10_parse::<u8>()
        .expect("integer argument");

    let input_path = format!("../../inputs/{day:02}.in");

    let mut aoc_solution = parse_macro_input!(input as ItemFn);
    aoc_solution.sig.ident = Ident::new("aoc_solution", aoc_solution.sig.ident.span());

    let tokens = quote! {
        const INPUT: &str = include_str!(#input_path);
        #aoc_solution
        fn main() {
            let now = ::std::time::Instant::now();
            let (p1, p2) = aoc_solution(INPUT.trim_end());
            let time = now.elapsed().as_millis();
            println!("Part one: {}", p1);
            println!("Part two: {}", p2);
            println!("Time: {}ms", time);
        }
    };
    TokenStream::from(tokens)
}
