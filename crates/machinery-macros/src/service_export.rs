use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, ItemImpl, Result, Type,
};

use crate::utils::generate_wrappers;

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
    let input = parse_macro_input!(item as ServiceExportInput);

    let ty_name = if let Type::Path(ref path) = &*input.item.self_ty {
        path.path.segments.last().unwrap().ident.clone()
    } else {
        panic!("Invalid target for tm_service_export")
    };

    // Go over all functions in the input
    let wrappers = generate_wrappers(&ty_name, &input.item);

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
