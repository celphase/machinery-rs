use machinery::{Plugin, foundation::{ApiRegistryApi, LoggerApi}, plugin};
use machinery_sys::foundation::TM_LOG_TYPE_INFO;

plugin!(ExamplePlugin);

struct ExamplePlugin {
    logger: LoggerApi,
}

impl Plugin for ExamplePlugin {
    fn load(registry: &ApiRegistryApi) -> Self {
        let logger: LoggerApi = registry.get();

        unsafe {
            logger.print(TM_LOG_TYPE_INFO, "Example rust plugin loaded.");
        }

        Self { logger }
    }
}

impl Drop for ExamplePlugin {
    fn drop(&mut self) {
        unsafe {
            self.logger
                .print(TM_LOG_TYPE_INFO, "Example rust plugin unloaded.");
        }
    }
}
