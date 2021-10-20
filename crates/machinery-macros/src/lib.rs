mod identifier;
mod instance_export;
mod plugin;
mod service_derive;
mod service_export;

#[proc_macro_attribute]
pub fn tm_plugin(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    plugin::tm_plugin(item)
}

#[proc_macro_derive(Service)]
pub fn tm_service_derive(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    service_derive::tm_service_derive(item)
}

#[proc_macro_attribute]
pub fn tm_service_export(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    service_export::tm_service_export(item)
}

#[proc_macro_attribute]
pub fn tm_instance_export(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    instance_export::tm_instance_export(attr, item)
}

/// Generates constants for a The Machinery identifier.
#[proc_macro]
pub fn tm_identifier(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    identifier::tm_identifier(item)
}
