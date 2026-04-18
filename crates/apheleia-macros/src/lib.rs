use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(Extension)]
pub fn derive_extension(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics apheleia_ui::extensions::traits::Extension for #name #ty_generics #where_clause {}
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(Resource)]
pub fn derive_resource(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics apheleia_ui::resources::traits::Resource for #name #ty_generics #where_clause {}
    };

    TokenStream::from(expanded)
}
