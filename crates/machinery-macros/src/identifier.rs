use proc_macro2::Literal;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Result,
};

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

pub fn identifier(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as TmIdentInput);
    let literal = input.item;

    // Extract the inner string value
    let literal_str = literal.to_string();
    if !(literal_str.starts_with('"') && literal_str.ends_with('"')) {
        panic!("tm_ident requires a string literal")
    }
    let value = &literal_str[1..literal_str.len() - 1];

    // Hash the value using the hash function used by the machinery
    let hash = murmurhash64::murmur_hash64a(value.as_bytes(), 0);

    let wrapper = quote! {
        machinery::Identifier {
            name: const_cstr::const_cstr!(#literal),
            hash: machinery_api::foundation::StrhashT { u64_: #hash },
        }
    };

    wrapper.into()
}
