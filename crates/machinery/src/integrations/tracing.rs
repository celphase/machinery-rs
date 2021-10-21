use std::{ffi::CString, fmt::Write};

use machinery_api::foundation::{
    LoggerApi, TM_LOG_TYPE_DEBUG, TM_LOG_TYPE_ERROR, TM_LOG_TYPE_INFO,
};
use tracing::{
    field::{Field, Visit},
    span, Id, Level, Subscriber,
};

use crate::Plugin;

/// Initialize a global default subscriber for tracing that prints to The Machinery logging API.
pub fn initialize(plugin: &Plugin) {
    let subscriber = TmSubscriber::new(plugin);
    tracing::subscriber::set_global_default(subscriber).unwrap();
}

struct TmSubscriber {
    logger: *const LoggerApi,
}

unsafe impl Send for TmSubscriber {}
unsafe impl Sync for TmSubscriber {}

impl TmSubscriber {
    pub fn new(plugin: &Plugin) -> Self {
        Self {
            logger: plugin.get_api(),
        }
    }
}

impl Subscriber for TmSubscriber {
    fn enabled(&self, metadata: &tracing::Metadata<'_>) -> bool {
        // Assume anything DEBUG level or lower does not need to be logged
        let level = *metadata.level();
        !(level == Level::DEBUG || level == Level::TRACE)
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

            // Add the message data
            event.record(&mut visitor);

            // Add module and file information to the message
            visitor.message.push_str("\n    at ");
            if let Some(path) = event.metadata().module_path() {
                visitor.message.push_str(path);
            }

            visitor.message.push_str(" (");
            if let Some(file) = event.metadata().file() {
                visitor.message.push_str(file);

                if let Some(line) = event.metadata().line() {
                    visitor.message.push(':');
                    visitor.message.push_str(&line.to_string());
                }
            }
            visitor.message.push(')');

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
