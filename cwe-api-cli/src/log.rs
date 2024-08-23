//! Control the application's logging logic.

use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _};

pub fn setup_logging() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();
}
