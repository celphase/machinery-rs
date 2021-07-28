/// Singleton utility for exposing C callbacks.
///
/// # Warning
/// Avoid singletons as much as you can, this utility exists only for exposing `extern "C"`
/// callbacks for APIs, interfaces, and vtables.
pub trait Singleton: Sized + Send + Sync {
    /// # Safety
    /// This should only be called once.
    unsafe fn create(value: Self);

    /// # Safety
    /// This should only be called once.
    unsafe fn destroy();

    /// # Safety
    /// The data behind this pointer will only be valid for as long as the singleton is alive.
    unsafe fn ptr() -> *mut Self;
}

/// Implements Singleton for a type.
/// TODO: Derive
#[macro_export]
macro_rules! singleton {
    ($ty:ident) => {
        static INSTANCE: std::sync::atomic::AtomicPtr<$ty> =
            std::sync::atomic::AtomicPtr::new(std::ptr::null_mut());

        impl machinery::Singleton for $ty {
            unsafe fn create(value: Self) {
                let boxed = Box::new(value);
                let result = INSTANCE.swap(
                    Box::into_raw(boxed),
                    std::sync::atomic::Ordering::SeqCst,
                );

                if !result.is_null() {
                    panic!("Plugin was already loaded!");
                }
            }

            unsafe fn destroy() {
                let plugin = INSTANCE.swap(
                    std::ptr::null_mut(),
                    std::sync::atomic::Ordering::SeqCst,
                );
                let _ = Box::from_raw(plugin);
            }

            unsafe fn ptr() -> *mut $ty {
                INSTANCE.load(std::sync::atomic::Ordering::SeqCst)
            }
        }
    };
}
