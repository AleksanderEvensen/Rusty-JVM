use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn mapping(args: TokenStream, input: TokenStream) -> TokenStream {
    println!("Args: {:#?}", args);
    println!("Input: {:#?}", input);

    TokenStream::new()
}
