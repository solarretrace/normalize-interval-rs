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

// Local imports.
use bound::Bound;
use normalize::Finite;
use normalize::Normalize;
use raw_interval::RawInterval;

// Standard library imports.
use std::convert;
use std::ops::Range;
use std::ops::RangeFrom;
// use std::ops::RangeInclusive; // TODO(Sky): Add when RangeInclusive accessors stabilize.
use std::ops::RangeTo;
use std::ops::RangeToInclusive;
// use std::ops::RangeFull; // NOTE: Excluded due to impl conflict.
use std::ops::Sub;

// Local enum shortcuts.
use raw_interval::RawInterval::*;



////////////////////////////////////////////////////////////////////////////////
// Interval<T>
////////////////////////////////////////////////////////////////////////////////
/// A contiguous interval of the type T.
///
/// `Interval`s are [`Normalize`]d when created. For [`Finite`] types, open
/// bounds will be converted to the nearest contained closed bound.
///
/// [`Normalize`]: ../normalize/trait.Normalize.html
/// [`Finite`]: ../normalize/trait.Finite.html
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Interval<T>(pub (crate) RawInterval<T>)
    where T: PartialOrd + Ord + Clone;

// All mutable operations and constructors on `Interval` must ensure that the
// interval is normalized before returning.
impl<T> Interval<T> 
    where 
        T: PartialOrd + Ord + Clone,
        RawInterval<T>: Normalize 
{
    ////////////////////////////////////////////////////////////////////////////
    // Constructors
    ////////////////////////////////////////////////////////////////////////////
    
    /// Constructs a new `Interval` from the given [`Bound`]s.
    /// 
    /// [`Bound`]: bound/enum.Bound.html
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Bound::*;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<i32> = Interval::new(Include(3), Exclude(7));
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// [`Finite`] types will have their bounds closed:
    ///
    /// [`Finite`]: ../normalize/trait.Finite.html
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Bound::*;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<i32> = Interval::new(Exclude(-3), Exclude(7));
    ///
    /// assert_eq!(interval, Interval::new(Include(-2), Include(6)));
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// If the bounds are out of order, and empty `Interval` will be returned.
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Bound::*;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<i32> = Interval::new(Exclude(7), Exclude(-7));
    ///
    /// assert_eq!(interval, Interval::empty());
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn new(left: Bound<T>, right: Bound<T>) -> Self {
        Interval(RawInterval::new(left, right).normalized())
    }
    
    
    /// Constructs an empty `Interval`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<i32> = Interval::empty();
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn empty() -> Self {
        // Normalization not needed for empty intervals.
        Interval(RawInterval::Empty)
    }
    
    /// Constructs a new degenerate `Interval` containing the given point.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<i32> = Interval::point(3);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn point(point: T) -> Self {
       // Normalization not needed for point intervals.
       Interval(RawInterval::Point(point))
    }
    
    /// Constructs a new bounded open `Interval` from the given points.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<i32> = Interval::open(3, 7);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// [`Finite`] types will have their bounds closed:
    ///
    /// [`Finite`]: ../normalize/trait.Finite.html
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Bound::*;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<i32> = Interval::open(-3, 7);
    ///
    /// assert_eq!(interval, Interval::new(Include(-2), Include(6)));
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// If the bounds are out of order, and empty `Interval` will be returned.
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Bound::*;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<i32> = Interval::open(7, -7);
    ///
    /// assert_eq!(interval, Interval::empty());
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn open(left: T, right: T) -> Self {
        Interval(RawInterval::open(left, right).normalized())
    }
    
    /// Constructs a new bounded left-open `Interval` from the given points.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<i32> = Interval::left_open(3, 7);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// [`Finite`] types will have their bounds closed:
    ///
    /// [`Finite`]: ../normalize/trait.Finite.html
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Bound::*;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<i32> = Interval::left_open(-3, 7);
    ///
    /// assert_eq!(interval, Interval::new(Include(-2), Include(7)));
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// If the bounds are out of order, and empty `Interval` will be returned.
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Bound::*;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<i32> = Interval::left_open(7, -7);
    ///
    /// assert_eq!(interval, Interval::empty());
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    /// 
    /// If the bounds are identical, a point `Interval` will be returned.
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Bound::*;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<i32> = Interval::left_open(5, 5);
    ///
    /// assert_eq!(interval, Interval::point(5));
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn left_open(left: T, right: T) -> Self {
        Interval(RawInterval::left_open(left, right).normalized())
    }
    
    /// Constructs a new bounded right-open `Interval` from the given points.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<i32> = Interval::right_open(3, 7);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// [`Finite`] types will have their bounds closed:
    ///
    /// [`Finite`]: ../normalize/trait.Finite.html
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Bound::*;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<i32> = Interval::right_open(-3, 7);
    ///
    /// assert_eq!(interval, Interval::new(Include(-3), Include(6)));
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// If the bounds are out of order, and empty `Interval` will be returned.
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Bound::*;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<i32> = Interval::right_open(7, -7);
    ///
    /// assert_eq!(interval, Interval::empty());
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    /// 
    /// If the bounds are identical, a point `Interval` will be returned.
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Bound::*;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<i32> = Interval::right_open(5, 5);
    ///
    /// assert_eq!(interval, Interval::point(5));
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn right_open(left: T, right: T) -> Self {
        Interval(RawInterval::right_open(left, right).normalized())
    }

    /// Constructs a new bounded closed `Interval` from the given points.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<i32> = Interval::closed(3, 7);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// If the bounds are out of order, and empty `Interval` will be returned.
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Bound::*;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<i32> = Interval::closed(7, -7);
    ///
    /// assert_eq!(interval, Interval::empty());
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    /// 
    /// If the bounds are identical, a point `Interval` will be returned.
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Bound::*;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<i32> = Interval::closed(5, 5);
    ///
    /// assert_eq!(interval, Interval::point(5));
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn closed(left: T, right: T) -> Self {
        Interval(RawInterval::closed(left, right).normalized())
    }

    /// Constructs a new bounded left-closed `Interval` from the given points.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<i32> = Interval::left_closed(3, 7);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// [`Finite`] types will have their bounds closed:
    ///
    /// [`Finite`]: ../normalize/trait.Finite.html
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Bound::*;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<i32> = Interval::left_closed(-3, 7);
    ///
    /// assert_eq!(interval, Interval::new(Include(-3), Include(6)));
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// If the bounds are out of order, and empty `Interval` will be returned.
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Bound::*;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<i32> = Interval::left_closed(7, -7);
    ///
    /// assert_eq!(interval, Interval::empty());
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    /// 
    /// If the bounds are identical, a point `Interval` will be returned.
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Bound::*;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<i32> = Interval::left_closed(5, 5);
    ///
    /// assert_eq!(interval, Interval::point(5));
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn left_closed(left: T, right: T) -> Self {
        Self::right_open(left, right)
    }
    
    /// Constructs a new bounded right-closed `Interval` from the given points.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<i32> = Interval::right_closed(3, 7);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// [`Finite`] types will have their bounds closed:
    ///
    /// [`Finite`]: ../normalize/trait.Finite.html
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Bound::*;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<i32> = Interval::right_closed(-3, 7);
    ///
    /// assert_eq!(interval, Interval::new(Include(-2), Include(7)));
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// If the bounds are out of order, and empty `Interval` will be returned.
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Bound::*;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<i32> = Interval::right_closed(7, -7);
    ///
    /// assert_eq!(interval, Interval::empty());
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    /// 
    /// If the bounds are identical, a point `Interval` will be returned.
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Bound::*;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<i32> = Interval::right_closed(5, 5);
    ///
    /// assert_eq!(interval, Interval::point(5));
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn right_closed(left: T, right: T) -> Self {
        Self::left_open(left, right)
    }

    /// Constructs a new right-unbounded `Interval` from (and including) the
    /// given point.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<i32> = Interval::unbounded_from(3);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// [`Finite`] types will have their bounds closed:
    ///
    /// [`Finite`]: ../normalize/trait.Finite.html
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Bound::*;
    /// # use interval::Interval;
    /// # use std::i32;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<i32> = Interval::unbounded_from(7);
    ///
    /// assert_eq!(interval, Interval::new(Include(7), Include(i32::MAX)));
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn unbounded_from(point: T) -> Self {
        Interval(RawInterval::From(point).normalized())
    }

    /// Constructs a new left-unbounded `Interval` to (and including) the
    /// given point.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<i32> = Interval::unbounded_to(3);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// [`Finite`] types will have their bounds closed:
    ///
    /// [`Finite`]: ../normalize/trait.Finite.html
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Bound::*;
    /// # use interval::Interval;
    /// # use std::i32;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<i32> = Interval::unbounded_to(7);
    ///
    /// assert_eq!(interval, Interval::new(Include(i32::MIN), Include(7)));
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn unbounded_to(point: T) -> Self {
        Interval(RawInterval::To(point).normalized())
    }

    /// Constructs a new right-unbounded `Interval` from (but excluding) the
    /// given point.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<i32> = Interval::unbounded_up_from(3);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// [`Finite`] types will have their bounds closed:
    ///
    /// [`Finite`]: ../normalize/trait.Finite.html
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Bound::*;
    /// # use interval::Interval;
    /// # use std::i32;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<i32> = Interval::unbounded_up_from(7);
    ///
    /// assert_eq!(interval, Interval::new(Include(8), Include(i32::MAX)));
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn unbounded_up_from(point: T) -> Self {
        Interval(RawInterval::UpFrom(point).normalized())
    }

    /// Constructs a new left-unbounded `Interval` to (but excluding) the
    /// given point.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<i32> = Interval::unbounded_up_to(3);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// [`Finite`] types will have their bounds closed:
    ///
    /// [`Finite`]: ../normalize/trait.Finite.html
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Bound::*;
    /// # use interval::Interval;
    /// # use std::i32;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<i32> = Interval::unbounded_up_to(7);
    ///
    /// assert_eq!(interval, Interval::new(Include(i32::MIN), Include(6)));
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn unbounded_up_to(point: T) -> Self {
        Interval(RawInterval::UpTo(point).normalized())
    }

    /// Constructs a new unbounded `Interval` containing all points.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<i32> = Interval::full();
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// [`Finite`] types will have their bounds closed:
    ///
    /// [`Finite`]: ../normalize/trait.Finite.html
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Bound::*;
    /// # use interval::Interval;
    /// # use std::i32;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<i32> = Interval::full();
    ///
    /// assert_eq!(interval, Interval::new(Include(i32::MIN), Include(i32::MAX)));
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn full() -> Self {
        Interval(RawInterval::Full.normalized())
    }
    
    ////////////////////////////////////////////////////////////////////////////
    // Conversion methods
    ////////////////////////////////////////////////////////////////////////////

    /// Converts the `Interval` into an `Option`, returning `None` if it is 
    /// empty.
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Bound::*;
    /// # use interval::Interval;
    /// # use std::i32;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<i32> = Interval::closed(0, 4);
    /// assert_eq!(interval.into_non_empty(), Some(Interval::closed(0, 4)));
    ///
    /// let interval: Interval<i32> = Interval::empty();
    /// assert_eq!(interval.into_non_empty(), None);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    pub fn into_non_empty(self) -> Option<Self> {
        if self.is_empty() {
            None
        } else {
            Some(self)
        }
    }

    ////////////////////////////////////////////////////////////////////////////
    // Bound accessors
    ////////////////////////////////////////////////////////////////////////////

    /// Returns the lower [`Bound`] of the `Interval`, or `None` if the 
    /// `Interval` is [`empty`].
    ///
    /// [`Bound`]: bound/enum.Bound.html
    /// [`empty`]: #method.empty
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Bound::*;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<i32> = Interval::closed(-3, 5);
    /// assert_eq!(interval.lower_bound(), Some(Include(-3)));
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// [`Finite`] types will have their bounds closed:
    ///
    /// [`Finite`]: ../normalize/trait.Finite.html
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Bound::*;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<i32> = Interval::open(-3, 5);
    /// 
    /// assert_eq!(interval.lower_bound(), Some(Include(-2)));
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn lower_bound(&self) -> Option<Bound<T>> {
        self.0.lower_bound()
    }
    
    /// Returns the upper [`Bound`] of the `Interval`, or `None` if the 
    /// `Interval` is [`empty`].
    ///
    /// [`Bound`]: bound/enum.Bound.html
    /// [`empty`]: #method.empty
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Bound::*;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<i32> = Interval::closed(-3, 5);
    /// assert_eq!(interval.upper_bound(), Some(Include(5)));
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// [`Finite`] types will have their bounds closed:
    ///
    /// [`Finite`]: ../normalize/trait.Finite.html
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Bound::*;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<i32> = Interval::open(-3, 5);
    /// 
    /// assert_eq!(interval.upper_bound(), Some(Include(4)));
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn upper_bound(&self) -> Option<Bound<T>> {
        self.0.upper_bound()
    }
    
    /// Returns the greatest lower bound of the `Interval`, or `None` if the
    /// `Interval` is [`empty`] or unbounded below.
    ///
    /// [`empty`]: #method.empty
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<i32> = Interval::closed(-3, 5);
    /// assert_eq!(interval.infimum(), Some(-3));
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// [`Finite`] types will have their bounds closed:
    ///
    /// [`Finite`]: ../normalize/trait.Finite.html
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<i32> = Interval::open(-3, 5);
    /// 
    /// assert_eq!(interval.infimum(), Some(-2));
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn infimum(&self) -> Option<T> {
        self.0.infimum()
    }
    
    
    /// Returns the least upper bound of the `Interval`, or `None` if the
    /// `Interval` is [`empty`] or unbounded above.
    ///
    /// [`empty`]: #method.empty
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<i32> = Interval::closed(-3, 5);
    /// assert_eq!(interval.supremum(), Some(5));
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// [`Finite`] types will have their bounds closed:
    ///
    /// [`Finite`]: ../normalize/trait.Finite.html
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<i32> = Interval::open(-3, 5);
    /// 
    /// assert_eq!(interval.supremum(), Some(4));
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn supremum(&self) -> Option<T> {
        self.0.supremum()
    }

    /// Returns the size of the `Interval`, or `None` if it is either infinite
    /// or empty.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<i32> = Interval::closed(-3, 7);
    /// assert_eq!(interval.size(), Some(10));
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    pub fn size(&self) -> Option<T> where T: Sub<Output=T> {
        match (self.infimum(), self.supremum()) {
            (Some(l), Some(u)) => Some(u - l),
            _                  => None,
        }
    }

    ////////////////////////////////////////////////////////////////////////////
    // Query operations
    ////////////////////////////////////////////////////////////////////////////

    /// Returns `true` if the interval contains no points.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<i32> = Interval::closed(-3, 5);
    /// assert_eq!(interval.is_empty(), false);
    ///
    /// let interval: Interval<i32> = Interval::empty();
    /// assert_eq!(interval.is_empty(), true);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        match self.0 {
            Empty => true,
            _     => false,
        }
    }

    /// Returns `true` if the interval contains a single point.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<i32> = Interval::closed(-3, 5);
    /// assert_eq!(interval.is_degenerate(), false);
    ///
    /// let interval: Interval<i32> = Interval::point(4);
    /// assert_eq!(interval.is_degenerate(), true);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn is_degenerate(&self) -> bool {
        match self.0 {
            Point(_) => true,
            _        => false,
        }
    }

    /// Returns `true` if the interval contains more than one point.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<i32> = Interval::closed(-3, 5);
    /// assert_eq!(interval.is_proper(), true);
    ///
    /// let interval: Interval<i32> = Interval::point(4);
    /// assert_eq!(interval.is_proper(), false);
    ///
    /// let interval: Interval<i32> = Interval::empty();
    /// assert_eq!(interval.is_proper(), false);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn is_proper(&self) -> bool {
        match self.0 {
            Empty    => false,
            Point(_) => false,
            _        => true,
        }
        
    }

    /// Returns `true` if the interval is open.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<Option<i32>> = Interval::left_open(Some(-3), Some(5));
    /// assert_eq!(interval.is_open(), true);
    ///
    /// let interval: Interval<i32> = Interval::point(4);
    /// assert_eq!(interval.is_open(), false);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    /// 
    /// Note that the empty and full intervals are open:
    /// 
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<i32> = Interval::empty();
    /// assert_eq!(interval.is_open(), true);
    ///
    /// let interval: Interval<Option<i32>> = Interval::full();
    /// assert_eq!(interval.is_open(), true);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn is_open(&self) -> bool {
        match self.0 {
            Point(_)     => false,
            Closed(_, _) => false,
            _            => true,
        }
    }

    /// Returns `true` if the interval is left-open.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<Option<i32>> = Interval::left_open(Some(-3), Some(5));
    /// assert_eq!(interval.is_left_open(), true);
    ///
    /// let interval: Interval<i32> = Interval::closed(2, 4);
    /// assert_eq!(interval.is_left_open(), false);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    /// 
    /// Note that the left-unbounded intervals are considered left-open:
    /// 
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<Option<i32>> = Interval::unbounded_to(Some(4));
    /// assert_eq!(interval.is_left_open(), true);
    ///
    /// let interval: Interval<Option<i32>> = Interval::full();
    /// assert_eq!(interval.is_left_open(), true);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn is_left_open(&self) -> bool {
        match self.0 {
            LeftOpen(_, _) => true,
            UpTo(_)        => true,
            To(_)          => true,
            Full           => true,
            _              => false,
        }
    }

    /// Returns `true` if the interval is right-open.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<Option<i32>> = Interval::right_open(Some(-3), Some(5));
    /// assert_eq!(interval.is_right_open(), true);
    ///
    /// let interval: Interval<i32> = Interval::closed(2, 4);
    /// assert_eq!(interval.is_right_open(), false);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    /// 
    /// Note that the right-unbounded intervals are considered right-open:
    /// 
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<Option<i32>> = Interval::unbounded_from(Some(4));
    /// assert_eq!(interval.is_right_open(), true);
    ///
    /// let interval: Interval<Option<i32>> = Interval::full();
    /// assert_eq!(interval.is_right_open(), true);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn is_right_open(&self) -> bool {
        match self.0 {
            RightOpen(_, _) => true,
            UpFrom(_)       => true,
            From(_)         => true,
            Full            => true,
            _               => false,
        }
    }

    /// Returns `true` if the interval is half-open.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<Option<i32>> = Interval::left_open(Some(-3), Some(5));
    /// assert_eq!(interval.is_half_open(), true);
    ///
    /// let interval: Interval<i32> = Interval::closed(2, 4);
    /// assert_eq!(interval.is_half_open(), false);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    /// 
    /// Note that the half-unbounded intervals are considered half-open:
    /// 
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<Option<i32>> = Interval::unbounded_to(Some(4));
    /// assert_eq!(interval.is_half_open(), true);
    ///
    /// let interval: Interval<Option<i32>> = Interval::full();
    /// assert_eq!(interval.is_half_open(), true);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn is_half_open(&self) -> bool {
        match self.0 {
            Empty        => false,
            Point(_)     => false,
            Closed(_, _) => false,
            _            => true,
        }
    }

    /// Returns `true` if the interval is closed.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<i32> = Interval::closed(-3, 5);
    /// assert_eq!(interval.is_closed(), true);
    ///
    /// let interval: Interval<Option<i32>> = Interval::left_open(Some(-2), Some(4));
    /// assert_eq!(interval.is_closed(), false);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    /// 
    /// Note that the empty and full intervals are closed:
    /// 
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<i32> = Interval::empty();
    /// assert_eq!(interval.is_closed(), true);
    ///
    /// let interval: Interval<Option<i32>> = Interval::full();
    /// assert_eq!(interval.is_closed(), true);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn is_closed(&self) -> bool {
        match self.0 {
            Empty        => true,
            Point(_)     => true,
            Closed(_, _) => true,
            Full         => true,
            _            => false,
        }
    }

    /// Returns `true` if the interval is left-closed.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<i32> = Interval::closed(-3, 5);
    /// assert_eq!(interval.is_left_closed(), true);
    ///
    /// let interval: Interval<Option<i32>> = Interval::left_open(Some(-2), Some(4));
    /// assert_eq!(interval.is_left_closed(), false);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn is_left_closed(&self) -> bool {
        match self.0 {
            Point(_)        => true,
            RightOpen(_, _) => true,
            Closed(_, _)    => true,
            From(_)         => true,
            _               => false,
        }
    }

    /// Returns `true` if the interval is right-closed.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<i32> = Interval::closed(-3, 5);
    /// assert_eq!(interval.is_right_closed(), true);
    ///
    /// let interval: Interval<Option<i32>> = Interval::right_open(Some(-2), Some(4));
    /// assert_eq!(interval.is_right_closed(), false);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn is_right_closed(&self) -> bool {
        match self.0 {
            Point(_)       => true,
            LeftOpen(_, _) => true,
            Closed(_, _)   => true,
            To(_)          => true,
            _              => false,
        }
    }

    /// Returns `true` if the interval is half-closed.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<Option<i32>> = Interval::unbounded_to(Some(-3));
    /// assert_eq!(interval.is_half_closed(), true);
    ///
    /// let interval: Interval<Option<i32>> = Interval::open(Some(-2), Some(4));
    /// assert_eq!(interval.is_half_closed(), false);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn is_half_closed(&self) -> bool {
        match self.0 {
            LeftOpen(_, _)  => true,
            RightOpen(_, _) => true,
            To(_)           => true,
            From(_)         => true,
            _               => false,
        }
    }

    /// Returns `true` if the the interval is bounded.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<Option<i32>> = Interval::open(Some(-2), Some(4));
    /// assert_eq!(interval.is_left_bounded(), true);
    ///
    /// let interval: Interval<Option<i32>> = Interval::unbounded_to(Some(-3));
    /// assert_eq!(interval.is_left_bounded(), false);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn is_bounded(&self) -> bool {
        match self.0 {
            UpTo(_)   => false,
            UpFrom(_) => false,
            To(_)     => false,
            From(_)   => false,
            Full      => false,
            _         => true,
        }
    }

    /// Returns `true` if the the interval is left-bounded.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<Option<i32>> = Interval::open(Some(-2), Some(4));
    /// assert_eq!(interval.is_left_bounded(), true);
    ///
    /// let interval: Interval<Option<i32>> = Interval::unbounded_to(Some(-3));
    /// assert_eq!(interval.is_left_bounded(), false);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn is_left_bounded(&self) -> bool {
        match self.0 {
            UpTo(_) => false,
            To(_)   => false,
            Full    => false,
            _       => true,
        }
    }

    
    /// Returns `true` if the the interval is right-bounded.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<Option<i32>> = Interval::open(Some(-2), Some(4));
    /// assert_eq!(interval.is_right_bounded(), true);
    ///
    /// let interval: Interval<Option<i32>> = Interval::unbounded_from(Some(-3));
    /// assert_eq!(interval.is_right_bounded(), false);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn is_right_bounded(&self) -> bool {
        match self.0 {
            UpFrom(_) => false,
            From(_)   => false,
            Full      => false,
            _         => true,
        }
    }

    /// Returns `true` if the the interval is helf-bounded.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<Option<i32>> = Interval::unbounded_to(Some(-2));
    /// assert_eq!(interval.is_half_bounded(), true);
    ///
    /// let interval: Interval<Option<i32>> = Interval::full();
    /// assert_eq!(interval.is_half_bounded(), false);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn is_half_bounded(&self) -> bool {
        match self.0 {
            UpTo(_)   => true,
            UpFrom(_) => true,
            To(_)     => true,
            From(_)   => true,
            _         => false,
        }
    }


    /// Returns `true` if the the interval contains the given point.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<i32> = Interval::closed(0, 20);
    /// assert_eq!(interval.contains(&2), true);
    ///
    /// assert_eq!(interval.contains(&-15), false);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn contains(&self, point: &T) -> bool {
        self.0.contains(point)
    }

    ////////////////////////////////////////////////////////////////////////////
    // Set comparisons
    ////////////////////////////////////////////////////////////////////////////
    
    /// Returns `true` if the `Interval` overlaps the given `Interval`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let a: Interval<i32> = Interval::closed(-3, 5);
    /// let b: Interval<i32> = Interval::closed(4, 15);
    /// assert_eq!(a.intersects(&b), true);
    ///
    /// let a: Interval<i32> = Interval::closed(-3, 5);
    /// let b: Interval<i32> = Interval::closed(8, 12);
    /// assert_eq!(a.intersects(&b), false);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    pub fn intersects(&self, other: &Self) -> bool {
        self.0.intersects(&other.0)
    }

    /// Returns `true` if the `Interval` shares a bound with the given 
    /// `Interval`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let a: Interval<i32> = Interval::closed(-3, 5);
    /// let b: Interval<i32> = Interval::closed(5, 15);
    /// assert_eq!(a.adjacent(&b), true);
    ///
    /// let a: Interval<i32> = Interval::closed(-3, 5);
    /// let b: Interval<i32> = Interval::closed(8, 12);
    /// assert_eq!(a.adjacent(&b), false);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    pub fn adjacent(&self, other: &Self) -> bool {
        // TODO: Consider normalization steps adjacent.
        self.0.adjacent(&other.0)
    }

    ////////////////////////////////////////////////////////////////////////////
    // Set operations
    ////////////////////////////////////////////////////////////////////////////

    /// Returns `Interval`s containing all points not contained in the 
    /// [`Interval`].
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<i32> = Interval::open(-3, 5);
    /// 
    /// assert_eq!(interval.complement().collect::<Vec<_>>(), 
    ///     [Interval::unbounded_to(-3), Interval::unbounded_from(5)]);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    pub fn complement(&self) -> impl Iterator<Item=Self> {
        self.0
            .complement()
            .map(Normalize::normalized)
            .map(Interval)
    }
    
    /// Returns the largest `Interval` whose points are all contained entirely
    /// within the `Interval` and the given `Interval`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let a: Interval<i32> = Interval::closed(-3, 7);
    /// let b: Interval<i32> = Interval::closed(4, 13);
    /// assert_eq!(a.intersect(&b), Interval::closed(4, 7));
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    pub fn intersect(&self, other: &Self) -> Self {
        self.0.intersect(&other.0).normalized().into()
    }

    /// Returns the `Interval`s containing all points in the `Interval` and the
    /// given `Interval`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let a: Interval<i32> = Interval::closed(-3, 7);
    /// let b: Interval<i32> = Interval::closed(4, 13);
    /// assert_eq!(a.union(&b).collect::<Vec<_>>(),
    ///     [Interval::closed(-3, 13)]);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    pub fn union(&self, other: &Self) -> impl Iterator<Item=Self> {
        // TODO: Fix intervals that are adjacent after normalization.
        self.0
            .union(&other.0)
            .map(Normalize::normalized)
            .map(Interval)
    }
    
    /// Returns the `Interval`s containing all points in the `Interval` which
    /// are not in the given `Interval`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let a: Interval<i32> = Interval::closed(-3, 7);
    /// let b: Interval<i32> = Interval::closed(4, 13);
    /// assert_eq!(a.minus(&b).collect::<Vec<_>>(),
    ///     [Interval::right_open(-3, 4)]);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    pub fn minus(&self, other: &Self) -> impl Iterator<Item=Self> {
        // TODO: Fix intervals that are adjacent after normalization.
        self.0
            .minus(&other.0)
            .map(Normalize::normalized)
            .map(Interval)
    }

    /// Returns the smallest `Interval` that contains all of the points
    /// contained within the `Interval` and the given `Interval`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let a: Interval<i32> = Interval::closed(-3, 5);
    /// let b: Interval<i32> = Interval::closed(9, 13);
    /// assert_eq!(a.enclose(&b), Interval::closed(-3, 13));
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    pub fn enclose(&self, other: &Self) -> Self {
        self.0.enclose(&other.0).normalized().into()
    }

    /// Returns the smallest closed `Interval` containing all of the points in 
    /// this `Interval`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<Option<i32>> = Interval::open(Some(-3), Some(7));
    /// assert_eq!(interval.closure(), Interval::closed(Some(-3), Some(7)));
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    pub fn closure(&self) -> Self {
        self.0.closure().normalized().into()
    }
}


////////////////////////////////////////////////////////////////////////////////
// Conversion traits
////////////////////////////////////////////////////////////////////////////////

impl<T> convert::From<RawInterval<T>> for Interval<T> 
    where T: PartialOrd + Ord + Clone
{
    fn from(raw_interval: RawInterval<T>) -> Self {
        Interval(raw_interval.normalized())
    }
}

// NOTE: Conflicts with From<RangeFull> convertion.
impl<T> convert::From<T> for Interval<T> 
    where T: PartialOrd + Ord + Clone
{
    fn from(point: T) -> Self {
        Interval(RawInterval::Point(point).normalized())
    }
}

////////////////////////////////////////////////////////////////////////////////
// Conversion traits
////////////////////////////////////////////////////////////////////////////////

impl<T> convert::From<Range<T>> for Interval<T>
    where T: PartialOrd + Ord + Clone
{
    fn from(r: Range<T>) -> Self {
        Interval(RawInterval::right_open(r.start, r.end).normalized())
    }
}

// TODO(Sky): Fix RangeInclusive once the accessors become available.
// impl<T> convert::From<RangeInclusive<T>> for Interval<T>
//     where T: PartialOrd + Ord + Clone
// {
//     fn from(r: RangeInclusive<T>) -> Self {
//         Interval(RawInterval::closed(r.next().unwrap(), r.next_back().unwrap()).normalized())
//     }
// }

impl<T> convert::From<RangeFrom<T>> for Interval<T>
    where T: PartialOrd + Ord + Clone
{
    fn from(r: RangeFrom<T>) -> Self {
        Interval(RawInterval::From(r.start).normalized())
    }
}

impl<T> convert::From<RangeTo<T>> for Interval<T>
    where T: PartialOrd + Ord + Clone
{
    fn from(r: RangeTo<T>) -> Self {
        Interval(RawInterval::UpTo(r.end).normalized())
    }
}

impl<T> convert::From<RangeToInclusive<T>> for Interval<T>
    where T: PartialOrd + Ord + Clone
{
    fn from(r: RangeToInclusive<T>) -> Self {
        Interval(RawInterval::To(r.end).normalized())
    }
}

// NOTE: Conflicts with From<T> convertion.
// impl<T> convert::From<RangeFull> for Interval<T>
//     where T: PartialOrd + Ord + Clone
// {
//     fn from(r: RangeFull) -> Self {
//         Interval(RawInterval::full().normalized())
//     }
// }

////////////////////////////////////////////////////////////////////////////////
// Default
////////////////////////////////////////////////////////////////////////////////
impl<T> Default for Interval<T> where T: PartialOrd + Ord + Clone {
    fn default() -> Self {
        Interval::empty()
    }
}


////////////////////////////////////////////////////////////////////////////////
// Finite iteration support
////////////////////////////////////////////////////////////////////////////////
impl<T> Interval<T> where T: PartialOrd + Ord + Clone + Finite {
    /// Returns an `Iterator` over the points in the `Interval`. Only defined
    /// for `Finite` `Interval`s.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<i32> = Interval::open(3, 7);
    /// assert_eq!(interval.iter().collect::<Vec<_>>(), [4, 5, 6]);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// The `Interval` can be iterated in both directions.
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Interval;
    /// # fn main() { example().unwrap(); }
    /// # fn example() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let interval: Interval<i32> = Interval::open(3, 7);
    /// assert_eq!(interval.iter().rev().collect::<Vec<_>>(), [6, 5, 4]);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    pub fn iter(&self) -> Iter<T> {
        Iter {
            inner: self.clone(),
        }
    }
}

/// An `Iterator` over the points in an `Interval`.
pub struct Iter<T> where T: PartialOrd + Ord + Clone {
    /// The `Interval` being iterated over.
    inner: Interval<T>,
}

impl<T> Iterator for Iter<T>
    where T: PartialOrd + Ord + Clone + Finite
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.lower_bound() {
            Some(Bound::Include(lb)) => {
                self.inner = self.inner
                    .minus(&lb.clone().into())
                    .next()
                    .unwrap_or(Interval::empty());
                Some(lb)
            },
            None => None,
            _ => unreachable!("iter for Finite interval with open lower bound"),
        }
    }
}

impl<T> DoubleEndedIterator for Iter<T>
    where T: PartialOrd + Ord + Clone + Finite
{
    fn next_back(&mut self) -> Option<Self::Item> {
        match self.inner.upper_bound() {
            Some(Bound::Include(ub)) => {
                self.inner = self.inner
                    .minus(&ub.clone().into())
                    .next()
                    .unwrap_or(Interval::empty());
                Some(ub)
            },
            None => None,
            _ => unreachable!("iter for Finite interval with open upper bound"),
        }
    }
}
