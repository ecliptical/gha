//! Minimalist utilities for writing [custom GitHub Actions](https://docs.github.com/en/actions/creating-actions/about-custom-actions).

mod env;
mod func;
mod message;

pub use env::*;
pub use func::*;
