#![doc = include_str!("../README.md")]
use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};

pub use derive_where::derive_where;
pub use pub_fields::pub_fields;

pub mod clone_on_write;

pub use clone_on_write::CW;

#[cfg(test)]
mod tests;
