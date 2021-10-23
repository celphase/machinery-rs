mod identifier;
mod instance_export;
mod plugin;
mod service_derive;
mod service_export;
mod service_impl;
mod utils;

/// Generates constants for a The Machinery identifier.
#[proc_macro]
pub fn tm_identifier(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    identifier::tm_identifier(item)
}

#[proc_macro_attribute]
pub fn tm_instance_export(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    instance_export::tm_instance_export(attr, item)
}

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

/// Generates C exported wrapper functions for service methods.
///
/// All methods in the annotated block will receive a generated wrapper, named `[origin]_export`.
/// The wrapper uses the service singleton pointer to call into the method.
#[proc_macro_attribute]
pub fn tm_service_export(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    service_export::tm_service_export(item)
}

/// Generates C exported wrapper functions for service methods, and creates a constant function
/// table for a given interface using those function wrappers.
///
/// The interface implementation will be available as an associated constant with the name of the
/// interface in CAPITAL_SNAKE_CASE.
#[proc_macro_attribute]
pub fn tm_service_impl(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    service_impl::tm_service_impl(attr, item)
}
