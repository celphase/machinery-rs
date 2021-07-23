use proc_macro2::{Literal, Span};
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, FnArg, Ident, ImplItem, ItemImpl, Result, Type,
};

struct TmExportPluginFnsInput {
    item: ItemImpl,
}

impl Parse for TmExportPluginFnsInput {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            item: input.parse()?,
        })
    }
}

/// Generates `extern "C"` wrappers for plugin member functions.
#[proc_macro_attribute]
pub fn tm_export_plugin_fns(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut input = parse_macro_input!(item as TmExportPluginFnsInput);

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
                    (*#ty_name::as_ptr())
                        .#internal_ident(#(#args_without_types),*)
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

    return output.into();
}

struct TmIdentInput {
    item: Literal,
}

impl Parse for TmIdentInput {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            item: input.parse()?,
        })
    }
}


/// Generates constants for a The Machinery identifier.
#[proc_macro]
pub fn tm_ident(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as TmIdentInput);
    let literal = input.item;

    // Extract the inner string value
    let literal_str = literal.to_string();
    if !(literal_str.starts_with('"') && literal_str.ends_with('"')) {
        panic!("tm_ident requires a string literal")
    }
    let value = &literal_str[1..literal_str.len()-1];

    // Hash the value using the hash function used by the machinery
    let hash = murmurhash64::murmur_hash64a(value.as_bytes(), 0);

    let wrapper = quote! {
        machinery::Identifier {
            name: const_cstr::const_cstr!(#literal),
            hash: machinery::tm::foundation::StrhashT { u64_: #hash },
        }
    };

    wrapper.into()
}
