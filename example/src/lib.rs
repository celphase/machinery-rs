use machinery::foundation::{ApiRegistryApi, LoggerApi};
use machinery_sys::foundation::{tm_api_registry_api, tm_logger_api, TM_LOG_TYPE_INFO};

#[no_mangle]
pub unsafe extern "C" fn tm_load_plugin(registry: *const tm_api_registry_api, load: bool) {
    let registry = ApiRegistryApi(registry);

    let logger = LoggerApi(registry.get(LoggerApi::NAME) as *const tm_logger_api);

    let text = format!(
        "Example rust plugin {}.",
        if load { "loaded" } else { "unloaded" }
    );
    logger.printf(TM_LOG_TYPE_INFO, &text);
}
