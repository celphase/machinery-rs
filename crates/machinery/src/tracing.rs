use std::{ffi::CString, fmt::Write};

use machinery_api::foundation::{
    ApiRegistryApi, LoggerApi, TM_LOG_TYPE_DEBUG, TM_LOG_TYPE_ERROR, TM_LOG_TYPE_INFO,
};
use tracing::{
    field::{Field, Visit},
    span, Id, Level, Subscriber,
};

use crate::get_api;

/// Initialize a global default subscriber for tracing that prints to The Machinery logging API.
pub fn initialize(registry: &ApiRegistryApi) {
    let subscriber = TmSubscriber::new(registry);
    tracing::subscriber::set_global_default(subscriber).unwrap();
}

struct TmSubscriber {
    logger: *const LoggerApi,
}

unsafe impl Send for TmSubscriber {}
unsafe impl Sync for TmSubscriber {}

impl TmSubscriber {
    pub fn new(registry: &ApiRegistryApi) -> Self {
        Self {
            logger: get_api(registry),
        }
    }
}

impl Subscriber for TmSubscriber {
    fn enabled(&self, _metadata: &tracing::Metadata<'_>) -> bool {
        true
    }

    fn new_span(&self, _span: &span::Attributes<'_>) -> Id {
        Id::from_u64(1)
    }

    fn record(&self, _span: &span::Id, _values: &span::Record<'_>) {}

    fn record_follows_from(&self, _span: &span::Id, _follows: &span::Id) {}

    fn event(&self, event: &tracing::Event<'_>) {
        unsafe {
            let mut visitor = TmVisitor {
                message: String::new(),
            };

            // Prefix with the module
            if let Some(path) = event.metadata().module_path() {
                visitor.message.push_str(path);
                visitor.message.push_str(": ");
            }

            // Add the message data
            event.record(&mut visitor);

            // Add file metadata to the message
            if let Some(file) = event.metadata().file() {
                visitor.message.push_str("\n    at ");
                visitor.message.push_str(file);

                if let Some(file) = event.metadata().line() {
                    visitor.message.push(':');
                    visitor.message.push_str(&file.to_string());
                }
            }

            // Convert from tracing level to machinery log type
            let level = match *event.metadata().level() {
                Level::TRACE => TM_LOG_TYPE_DEBUG,
                Level::DEBUG => TM_LOG_TYPE_DEBUG,
                Level::INFO => TM_LOG_TYPE_INFO,
                Level::WARN => TM_LOG_TYPE_ERROR,
                Level::ERROR => TM_LOG_TYPE_ERROR,
            };

            let cstr = CString::new(visitor.message).unwrap();
            (*self.logger).print(level, cstr.as_ptr());
        }
    }

    fn enter(&self, _span: &span::Id) {}

    fn exit(&self, _span: &span::Id) {}
}

struct TmVisitor {
    message: String,
}

impl Visit for TmVisitor {
    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        if field.name() == "message" {
            write!(&mut self.message, "{:?}", value).unwrap();
        } else {
            write!(&mut self.message, "\n    {} = {:?}", field.name(), value).unwrap();
        }
    }
}
