use proc_macro2::Span;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, FnArg, Ident, ImplItem, ItemImpl, Result, Type,
};

struct InstanceExportInput {
    item: ItemImpl,
}

impl Parse for InstanceExportInput {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            item: input.parse()?,
        })
    }
}

pub fn tm_instance_export(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let target = attr.to_string();
    let mut input = parse_macro_input!(item as InstanceExportInput);

    let ty_name = if let Type::Path(ref path) = &*input.item.self_ty {
        path.path.segments.last().unwrap().ident.clone()
    } else {
        panic!("Invalid target for export_instance_fns")
    };

    // Get the target out of the attribute
    let target_ident = if target.is_empty() {
        ty_name.clone()
    } else {
        Ident::new(&target, Span::call_site())
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
                unsafe extern "C" fn #original_name(
                    inst: *mut #target_ident,
                    #(#args_with_types),*
                ) #ret_val {
                    (*(inst as *mut #ty_name)).#internal_ident(#(#args_without_types),*)
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
