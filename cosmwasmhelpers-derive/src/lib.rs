use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Meta};

#[proc_macro_derive(Instantiate)]
pub fn derive_instantiate(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl WasmInstantiate for #ident {}
    };
    output.into()
}

#[proc_macro_derive(Execute)]
pub fn derive_execute(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl WasmExecute for #ident {}
    };
    output.into()
}

#[proc_macro_derive(Query, attributes(Response))]
pub fn derive_query(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, attrs, .. } = parse_macro_input!(input);

    let mut output = quote!();

    if let Some(attr) = attrs.iter().find(|a| a.path().is_ident("Response")) {
        if let Meta::List(meta_list) = &attr.meta {
            // TODO: figure this out
            meta_list.parse_nested_meta(|meta| {
                if let Some(res_ident) = meta.path.get_ident() {
                    output = quote! {
                        impl WasmQuery for #ident {
                            type Response = #res_ident;
                        }
                    };
                }
                Ok(())
            });
        } else {
            panic!("MetaList is only allowed")
        }
    } else {
        panic!("Missing #[Response(ResponseMsg)]")
    }
    output.into()
}