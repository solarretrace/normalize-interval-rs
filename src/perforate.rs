// Copyright 2018 Skylor R. Schermer.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
////////////////////////////////////////////////////////////////////////////////
//!
//! Provides a trait for splitting intervals at natural boundaries.
//!
////////////////////////////////////////////////////////////////////////////////




////////////////////////////////////////////////////////////////////////////////
// Perforate
////////////////////////////////////////////////////////////////////////////////
/// Defines operations for splitting an `Interval` at a natural boundary.
pub trait Perforate where Self: Sized {
	/// Returns the start boundary point of the next perforation zone.
	fn next_zone(&self) -> Option<Self>;
	/// Returns the start boundary point of the previous perforation zone.
	fn prev_zone(&self) -> Option<Self>;
}
