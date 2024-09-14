#![doc = include_str!("../README.md")]
use std::{
    io::stderr,
    ops::{Deref, DerefMut},
    sync::Arc,
};

pub use derive_everything;
pub use derive_where;
pub use pub_fields;

pub mod clone_on_write;

pub use anyhow;
pub use thiserror;
pub use tracing;
pub use tracing_subscriber;
pub use regex;
use tracing_subscriber::EnvFilter;

pub mod prelude;
