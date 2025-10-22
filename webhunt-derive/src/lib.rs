use syn::{Data, DeriveInput, Fields, parse_macro_input};

#[proc_macro_derive(Hunt, attributes(field))]
pub fn hunt_derive(input: TokenStream) -> TokenStream {
    todo!()
}
