use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, ItemImpl, Result, Type,
};

struct ServiceImplInput {
    item: ItemImpl,
}

impl Parse for ServiceImplInput {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            item: input.parse()?,
        })
    }
}

pub fn tm_service_impl(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut input = parse_macro_input!(item as ServiceImplInput);

    let _ty_name = if let Type::Path(ref path) = &*input.item.self_ty {
        path.path.segments.last().unwrap().ident.clone()
    } else {
        panic!("Invalid target for tm_service_impl")
    };

    // Extract the target, as this will be the interface struct's name
    let _target_name = if let Some((_, path, _)) = input.item.trait_.take() {
        path.segments.into_iter().last().unwrap().ident
    } else {
        panic!("Invalid target for tm_service_impl")
    };

    // Create a constant function table with the wrappers
    // TODO

    // Generate the new code
    let original = input.item;
    let output = quote! {
        #original
    };

    output.into()
}
