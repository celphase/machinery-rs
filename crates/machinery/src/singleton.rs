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
