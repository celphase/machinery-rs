use std::fmt::Write;

use machinery_sys::foundation::{TM_LOG_TYPE_DEBUG, TM_LOG_TYPE_ERROR, TM_LOG_TYPE_INFO};
use tracing::{
    field::{Field, Visit},
    span, Id, Level, Subscriber,
};

use crate::{foundation::ApiRegistryApi, generated::foundation::LoggerApi};

pub struct MachinerySubscriber {
    logger: LoggerApi,
}

impl MachinerySubscriber {
    pub fn new(registry: &ApiRegistryApi) -> Self {
        Self {
            logger: registry.get(),
        }
    }
}

impl Subscriber for MachinerySubscriber {
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
            let mut visitor = Visitor {
                message: String::new(),
            };
            event.record(&mut visitor);

            let level = match *event.metadata().level() {
                Level::TRACE => TM_LOG_TYPE_DEBUG,
                Level::DEBUG => TM_LOG_TYPE_DEBUG,
                Level::INFO => TM_LOG_TYPE_INFO,
                Level::WARN => TM_LOG_TYPE_ERROR,
                Level::ERROR => TM_LOG_TYPE_ERROR,
            };

            self.logger.print(level, &visitor.message);
        }
    }

    fn enter(&self, _span: &span::Id) {}

    fn exit(&self, _span: &span::Id) {}
}

struct Visitor {
    message: String,
}

impl Visit for Visitor {
    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        if field.name() == "message" {
            write!(&mut self.message, "{:?}", value).unwrap();
        }
    }
}
