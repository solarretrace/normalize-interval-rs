// Copyright 2018 Skylor R. Schermer.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
////////////////////////////////////////////////////////////////////////////////
//!
//! Provides an interval type for doing set selection and iteration.
//!
////////////////////////////////////////////////////////////////////////////////

// Internal library imports.
use crate::raw_interval::RawInterval;


////////////////////////////////////////////////////////////////////////////////
// Finite
////////////////////////////////////////////////////////////////////////////////
/// Provides the methods needed to iterate over an type's points. Used
/// to [`Normalize`] finite types used in [`Interval`] bounds.
///
/// [`Normalize`]: trait.Normalize.html
/// [`Interval`]: ../interval/struct.Interval.html
pub trait Finite: Sized {
    /// The minimum value of the type.
    const MINIMUM: Self;

    /// The maximum value of the type.
    const MAXIMUM: Self;

    /// Returns the previous element before the given one.
    fn pred(&self) -> Option<Self>;

    /// Returns the next element after the given one.
    fn succ(&self) -> Option<Self>;
}


////////////////////////////////////////////////////////////////////////////////
// Normalize
////////////////////////////////////////////////////////////////////////////////
/// Provides normalization capabilities for an [`Interval`].
/// 
/// [`Interval`]: ../interval/struct.Interval.html
pub trait Normalize {
    /// Normalizes the given interval in place.
    fn normalize(&mut self);

    /// Denormalizes the given interval in place.
    fn denormalize(&mut self);

    /// Returns a normalized copy of the given interval.
    fn normalized(mut self) -> Self where Self: Sized {
        self.normalize();
        self
    }

    /// Returns a denormalized copy of the given interval.
    fn denormalized(mut self) -> Self where Self: Sized {
        self.denormalize();
        self
    }
}


////////////////////////////////////////////////////////////////////////////////
// Normalize implementations
////////////////////////////////////////////////////////////////////////////////
// /// General 'do nothing' implementation for all intervals.
// impl<T> Normalize for RawInterval<T> {
//     default fn normalize(&mut self) {/* Do nothing. */}
//     default fn denormalize(&mut self) {/* Do nothing. */}
// }

/// Specialization for [`Finite`] intervals.
impl<T> Normalize for RawInterval<T> where T: Finite {
    fn normalize(&mut self) {
        use RawInterval::*;
        *self = match std::mem::replace(self, Empty) {
            Empty           => Empty,
            Point(p)        => Point(p),
            Open(l, r)      => match (l.succ(), r.pred()) {
                (Some(l), Some(r)) => Closed(l, r),
                _                  => Empty,
            },
            LeftOpen(l, r)  => l.succ().map_or(Empty, |l| Closed(l, r)),
            RightOpen(l, r) => r.pred().map_or(Empty, |r| Closed(l, r)),
            Closed(l, r)    => Closed(l, r),
            UpTo(r)         => r.pred().map_or(Empty, |r| Closed(T::MINIMUM, r)),
            UpFrom(l)       => l.succ().map_or(Empty, |l| Closed(l, T::MAXIMUM)),
            To(p)           => Closed(T::MINIMUM, p),
            From(p)         => Closed(p, T::MAXIMUM),
            Full            => Closed(T::MINIMUM, T::MAXIMUM),
        }
    }

    fn denormalize(&mut self) {
        use RawInterval::*;
        *self = match std::mem::replace(self, Empty) {
            Empty           => Empty,
            Point(p)        => match (p.pred(), p.succ()) {
                (Some(l), Some(r)) => Open(l, r),
                (Some(l), None)    => UpFrom(l),
                (None, Some(r))    => UpTo(r),
                _                  => Full,
            },
            Open(l, r)      => Open(l, r),
            LeftOpen(l, r)  => match r.succ() {
                Some(r) => Open(l, r),
                None    => UpFrom(l),
            },
            RightOpen(l, r) => match l.pred() {
                Some(l) => Open(l, r),
                None    => UpTo(r),
            },
            Closed(l, r)    => match (l.pred(), r.succ()) {
                (Some(l), Some(r)) => Open(l, r),
                (Some(l), None)    => UpFrom(l),
                (None, Some(r))    => UpTo(r),
                _                  => Full,
            },
            UpTo(r)         => UpTo(r),
            UpFrom(l)       => UpFrom(l),
            To(p)           => p.pred().map_or(Empty, |r| UpTo(r)),
            From(p)         => p.succ().map_or(Empty, |l| UpFrom(l)),
            Full            => Full,
        }
    }
}


////////////////////////////////////////////////////////////////////////////////
// Standard integer Finite implementations
////////////////////////////////////////////////////////////////////////////////

// Implements basic normalization for a single builtin integer type.
macro_rules! std_integer_finite_impl {
    // For each given type...
    ($($t:ident),*) => {
        $(impl Finite for $t {
            const MINIMUM: $t = {std::$t::MIN};
            const MAXIMUM: $t = {std::$t::MAX};

            fn pred(&self) -> Option<Self> {
                if *self != std::$t::MIN {Some(self - 1)} else {None}
            }

            fn succ(&self) -> Option<Self> {
                if *self != std::$t::MAX {Some(self + 1)} else {None}
            }
        })*
    };
}

// Provide implementations of Finite for builtin integer types.
std_integer_finite_impl![
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize
];


// TODO: Use nextUp and nextDown IEEE 754 functions to normalize float values?
