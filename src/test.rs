// Copyright 2018 Skylor R. Schermer.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
////////////////////////////////////////////////////////////////////////////////
//!
//! Testing module.
//!
////////////////////////////////////////////////////////////////////////////////

// Needs to be defined before submodule declarations.

/// Variant of `assert_eq` that collects items into a HashSet before comparing.
macro_rules! assert_eq_u {
    ($left:expr, $right:expr) => ({
        use std::collections::HashSet;
        let l = $left.collect::<Vec<_>>();
        let r = $right;
        let a: HashSet<_> = l.iter().collect();
        let b: HashSet<_> = r.iter().collect();
        assert_eq!(a, b);
    })
}

/// Variant of `assert_eq` that converts to an iterator and collects items into
/// a HashSet before comparing.
macro_rules! assert_eq_i {
    ($left:expr, $right:expr) => ({
        use std::collections::HashSet;
        let l = $left.into_iter().collect::<Vec<_>>();
        let r = $right;
        let a: HashSet<_> = l.iter().collect();
        let b: HashSet<_> = r.iter().collect();
        assert_eq!(a, b);
    })
}

// Module declarations.
mod raw_interval;
mod tine_tree;
