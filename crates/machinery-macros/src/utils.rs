use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{FnArg, ImplItem, ItemImpl};

pub fn generate_wrappers(ty_name: &Ident, item: &ItemImpl) -> Vec<(Ident, TokenStream)> {
    let mut wrappers = Vec::new();

    for item in &item.items {
        let method_item = if let ImplItem::Method(method_item) = item {
            method_item
        } else {
            continue;
        };

        // Extract the arguments
        let args_with_types: Vec<_> = method_item
            .sig
            .inputs
            .iter()
            .filter_map(|input| {
                if let FnArg::Typed(ref input) = input {
                    Some(input)
                } else {
                    None
                }
            })
            .collect();
        let args_without_types = args_with_types.iter().map(|input| &input.pat);

        // Generate an external C wrapper
        let ident = &method_item.sig.ident;
        let wrapper_ident = Ident::new(&format!("{}_export", ident), ident.span());
        let output = &method_item.sig.output;
        let wrapper = quote! {
            unsafe extern "C" fn #wrapper_ident(#(#args_with_types),*) #output {
                use machinery::Service;
                (*#ty_name::ptr()).#ident(#(#args_without_types),*)
            }
        };
        wrappers.push((wrapper_ident, wrapper));
    }

    wrappers
}
