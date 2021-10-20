use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, ItemStruct, Result,
};

struct DeriveServiceInput {
    item: ItemStruct,
}

impl Parse for DeriveServiceInput {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            item: input.parse()?,
        })
    }
}

pub fn tm_derive_service(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as DeriveServiceInput);
    let ident = input.item.ident;

    // Generate the implementation
    let implementation = quote! {
        static INSTANCE: std::sync::atomic::AtomicPtr<#ident> =
            std::sync::atomic::AtomicPtr::new(std::ptr::null_mut());

        impl machinery::Service for #ident {
            fn set_ptr(value: *const Self) {
                let result = INSTANCE.swap(
                    value as *mut _,
                    std::sync::atomic::Ordering::SeqCst,
                );

                if !result.is_null() {
                    panic!("Service was already set!");
                }
            }

            fn unset_ptr() {
                let result = INSTANCE.swap(
                    std::ptr::null_mut(),
                    std::sync::atomic::Ordering::SeqCst,
                );

                if result.is_null() {
                    panic!("Service was not yet set!");
                }
            }

            unsafe fn ptr() -> *const #ident {
                INSTANCE.load(std::sync::atomic::Ordering::SeqCst)
            }
        }
    };

    implementation.into()
}
