use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, ItemFn, Result,
};

struct TmPluginInput {
    item: ItemFn,
}

impl Parse for TmPluginInput {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            item: input.parse()?,
        })
    }
}

pub fn tm_plugin(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as TmPluginInput);
    let item = input.item;
    let name = item.sig.ident.clone();

    let code = quote! {
        #[no_mangle]
        pub unsafe extern "C" fn tm_load_plugin(
            registry: *const machinery_api::foundation::ApiRegistryApi,
            mode_load: bool,
        ) {
            machinery::load_plugin(#name, registry, mode_load);
        }

        #item
    };

    code.into()
}
