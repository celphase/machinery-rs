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

pub fn tm_service_impl(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as ServiceImplInput);

    let _ty_name = if let Type::Path(ref path) = &*input.item.self_ty {
        path.path.segments.last().unwrap().ident.clone()
    } else {
        panic!("Invalid target for tm_service_impl")
    };

    // Parse the target type and additional flags
    let attr = attr.to_string();
    let mut attrs = attr.split(',').map(|v| v.trim());

    let _target_name = if let Some(v) = attrs.next() {
        v
    } else {
        panic!("Target type attribute parameter required");
    };
    let target_type = parse_type(attrs.next().unwrap_or("table"));

    if target_type == TargetType::Table {
        panic!("Table type not yet supported");
    }

    // Create a constant function table with the wrappers
    // TODO

    // Generate the new code
    let original = input.item;
    let output = quote! {
        #original
    };

    output.into()
}

fn parse_type(ty: &str) -> TargetType {
    match ty {
        "table" => TargetType::Table,
        "functional" => TargetType::Functional,
        _ => panic!("Invalid target type parameter"),
    }
}

#[derive(PartialEq, Eq)]
enum TargetType {
    Table,
    Functional,
}
