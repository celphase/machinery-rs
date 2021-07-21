use machinery_sys::foundation::{tm_api_registry_api, tm_logger_api, TM_LOG_TYPE_INFO};

#[no_mangle]
pub unsafe extern "C" fn tm_load_plugin(reg: *const tm_api_registry_api, load: bool) {
    let logger = (*reg).get(tm_logger_api::NAME) as *const tm_logger_api;

    let text = format!("Example rust plugin {}.", if load { "loaded" } else { "unloaded" });
    (*logger).printf(TM_LOG_TYPE_INFO, &text);
}
