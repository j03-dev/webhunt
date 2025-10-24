mod types;

use deluxe::ExtractAttributes;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

use crate::types::Select;

#[proc_macro_derive(Hunt, attributes(select))]
pub fn hunt_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = &input.ident;
    let fields = match input.data {
        syn::Data::Struct(ref data) => match data.fields {
            syn::Fields::Named(ref fields) => &fields.named,
            _ => panic!("Model derive macro only supports structs with named fields"),
        },
        _ => panic!("Model derive macro only supports structs"),
    };

    let field_assignments = fields.iter().map(|field| {
        let field_ident = field.ident.as_ref().unwrap();
        let field_type = &field.ty;

        let mut field_attrs = field.attrs.clone();
        let args = Select::extract_attributes(&mut field_attrs).unwrap();

        let tag = args.tag;

        match args.attr {
            Some(attr) => {
                quote! {
                    #field_ident: webhunt::get_element_attribute::<#field_type>(html, #tag, #attr)?
                }
            }
            _ => {
                quote! {
                    #field_ident: webhunt::get_element_inner_html::<#field_type>(html, #tag)?
                }
            }
        }
    });

    let expanded = quote! {
        impl Hunt for #struct_name {
            fn from_html(html: &webhunt::Html) -> Result<Self, webhunt::Error>
            {
                Ok(
                    Self {
                        #(#field_assignments),*
                    }
                )
            }
        }
    };

    TokenStream::from(expanded)
}
