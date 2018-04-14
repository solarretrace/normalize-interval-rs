// Copyright 2018 Skylor R. Schermer.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
////////////////////////////////////////////////////////////////////////////////
//!
//! Provides a basic bounded interval type for doing complex set selections.
//!
////////////////////////////////////////////////////////////////////////////////
#![doc(html_root_url = "https://docs.rs/interval/0.12.5")]
#![feature(specialization)]
#![feature(conservative_impl_trait)]
#![warn(missing_docs)]

// Public modules.
pub mod bound;
pub mod interval;
pub mod normalize;
pub mod selection;

// Internal modules.
pub(crate) mod raw_interval;
pub(crate) mod tine;
pub(crate) mod tine_tree;
pub(crate) mod utilities;

// Test module declarations.
#[cfg(test)]
mod test;

// Exports.
pub use bound::Bound;
pub use interval::Interval;
pub use selection::Selection;