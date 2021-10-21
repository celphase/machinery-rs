use proc_macro2::Span;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, FnArg, Ident, ImplItem, ItemImpl, Result, Type,
};

struct ServiceExportInput {
    item: ItemImpl,
}

impl Parse for ServiceExportInput {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            item: input.parse()?,
        })
    }
}

pub fn tm_service_export(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut input = parse_macro_input!(item as ServiceExportInput);

    let ty_name = if let Type::Path(ref path) = &*input.item.self_ty {
        path.path.segments.last().unwrap().ident.clone()
    } else {
        panic!("Invalid target for tm_service_export")
    };

    // Go over all functions in the input
    let mut wrappers = Vec::new();
    for item in &mut input.item.items {
        if let ImplItem::Method(fun_item) = item {
            // Rename the function, since we're only going to call it internally
            let original_name = fun_item.sig.ident.clone();
            let internal_ident =
                Ident::new(&format!("{}_internal", original_name), Span::call_site());
            fun_item.sig.ident = internal_ident.clone();

            // Extract the arguments
            let args_with_types: Vec<_> = fun_item
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
            let ret_val = &fun_item.sig.output;
            let wrapper = quote! {
                unsafe extern "C" fn #original_name(#(#args_with_types),*) #ret_val {
                    use machinery::Service;
                    (*#ty_name::ptr()).#internal_ident(#(#args_without_types),*)
                }
            };
            wrappers.push(wrapper);
        }
    }

    // Generate the new code
    let original = input.item;
    let output = quote! {
        #original

        impl #ty_name {
            #(#wrappers)*
        }
    };

    output.into()
}