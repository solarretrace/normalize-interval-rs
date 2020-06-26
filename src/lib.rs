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
#![warn(anonymous_parameters)]
#![warn(bad_style)]
#![warn(bare_trait_objects)]
#![warn(const_err)]
#![warn(dead_code)]
#![warn(elided_lifetimes_in_paths)]
#![warn(improper_ctypes)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
#![warn(no_mangle_generic_items)]
#![warn(non_shorthand_field_patterns)]
#![warn(overflowing_literals)]
#![warn(path_statements)]
#![warn(patterns_in_fns_without_body)]
#![warn(private_in_public)]
#![warn(rust_2018_idioms)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unconditional_recursion)]
#![warn(unreachable_pub)]
#![warn(unused)]
#![warn(unused_allocation)]
#![warn(unused_comparisons)]
#![warn(unused_parens)]
#![warn(unused_qualifications)]
#![warn(unused_results)]
#![warn(variant_size_differences)]
#![warn(while_true)]

// NOTE: Specialization is used to allow normalization to be specialized for
// Finite types. and remain a no-op for others.
#![feature(specialization)]

// NOTE: Associated type bounds are used to get max/min of finite types in
//generic contexts without providing an instance.
#![feature(associated_type_bounds)]

// // Internal modules.
pub(crate) mod raw_interval;
pub(crate) mod tine;
pub(crate) mod tine_tree;
pub(crate) mod utility;

#[cfg(test)]
mod test;

// Public modules.
pub mod bound;
pub mod interval;
pub mod normalize;
pub mod selection;

// Exports.
pub use bound::Bound;
pub use interval::Interval;
pub use selection::Selection;
