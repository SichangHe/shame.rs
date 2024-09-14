use super::*;

pub use anyhow::{bail, Context, Result};
pub use clone_on_write::CW;
pub use derive_everything::*;
pub use derive_new::new;
pub use derive_where::derive_where;
pub use pub_fields::pub_fields;
pub use regex::{bytes::Regex as BytesRegex, Regex};
pub use thiserror::Error;
pub use tracing::{debug, error, info, instrument, trace, warn};

pub fn init_tracing() {
    tracing_subscriber::fmt()
        .with_writer(stderr)
        .with_env_filter(EnvFilter::from_default_env())
        .init();
}
