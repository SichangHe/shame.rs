#![doc = include_str!("../README.md")]
use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};

pub mod clone_on_write;

pub use clone_on_write::CW;

#[cfg(test)]
mod tests;
