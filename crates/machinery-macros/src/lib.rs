mod identifier;
mod singleton_derive;
mod singleton_export;
mod instance_export;

#[proc_macro_derive(Singleton)]
pub fn derive_singleton(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    singleton_derive::derive_singleton(item)
}

/// Generates `extern "C"` wrappers for singleton member functions.
#[proc_macro_attribute]
pub fn export_singleton_fns(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    singleton_export::export_singleton_fns(item)
}

/// Generates `extern "C"` wrappers for instance member functions.
#[proc_macro_attribute]
pub fn export_instance_fns(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    instance_export::export_instance_fns(attr, item)
}

/// Generates constants for a The Machinery identifier.
#[proc_macro]
pub fn identifier(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    identifier::identifier(item)
}
