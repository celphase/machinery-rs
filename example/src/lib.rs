use machinery::{foundation::ApiRegistryApi, plugin, Plugin};
use tracing::{event, Level};

plugin!(ExamplePlugin);

struct ExamplePlugin {}

impl Plugin for ExamplePlugin {
    fn load(_registry: &ApiRegistryApi) -> Self {
        event!(Level::INFO, "Example rust plugin loaded.");
        event!(Level::DEBUG, "Debug level message!");
        event!(Level::WARN, "Warn level message!");

        Self {}
    }
}

impl Drop for ExamplePlugin {
    fn drop(&mut self) {
        event!(Level::INFO, "Example rust plugin unloaded.");
    }
}
