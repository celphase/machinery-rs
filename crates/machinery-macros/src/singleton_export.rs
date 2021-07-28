use proc_macro2::Span;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, FnArg, Ident, ImplItem, ItemImpl, Result, Type,
};

struct ExportSingletonFnsInput {
    item: ItemImpl,
}

impl Parse for ExportSingletonFnsInput {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            item: input.parse()?,
        })
    }
}

pub fn export_singleton_fns(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut input = parse_macro_input!(item as ExportSingletonFnsInput);

    let ty_name = if let Type::Path(ref path) = &*input.item.self_ty {
        path.path.segments.last().unwrap().ident.clone()
    } else {
        panic!("Invalid target for export_plugin_fn")
    };

    let mut wrappers = Vec::new();

    // Go over all functions in the input
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
                    use machinery::Singleton;
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
