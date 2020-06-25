// Copyright 2018 Skylor R. Schermer.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
////////////////////////////////////////////////////////////////////////////////
//!
//! Provides a set of possibly noncontiguous intervals.
//!
////////////////////////////////////////////////////////////////////////////////

// Local imports.
use crate::bound::Bound;
use crate::interval::Interval;
use crate::normalize::Normalize;
use crate::raw_interval::RawInterval;
use crate::tine_tree::RawIntervalIter;
use crate::tine_tree::TineTree;
use crate::tine_tree;

// Standard library imports.
use std::iter::FromIterator;


////////////////////////////////////////////////////////////////////////////////
// Selection<T>
////////////////////////////////////////////////////////////////////////////////
/// A possibly noncontiguous collection of `Interval`s of the type `T`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Selection<T>(TineTree<T>) where T: PartialOrd + Ord + Clone;

// All intervals in the `TineTree` must be denormalized before insert and
// normalized before return. This ensures proper merging of adjacent normalized
// intervals.
impl<T> Selection<T> 
    where 
        T: PartialOrd + Ord + Clone,
        RawInterval<T>: Normalize 
{
    ////////////////////////////////////////////////////////////////////////////
    // Constructors
    ////////////////////////////////////////////////////////////////////////////
    
    /// Constructs a new empty `Selection`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Selection;
    /// # fn main() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let sel: Selection<i32> = Selection::new();
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn new() -> Self {
        Selection(TineTree::new())
    }

    /// Constructs a new empty `Selection`.
    #[inline]
    pub fn empty() -> Self {
        Selection::new()
    }

    /// Constructs a new full `Selection`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Selection;
    /// # fn main() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let sel: Selection<i32> = Selection::full();
    ///
    /// 
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn full() -> Self {
        Interval::full().into()
    }

    ////////////////////////////////////////////////////////////////////////////
    // Bound accessors
    ////////////////////////////////////////////////////////////////////////////

    /// Returns the lower [`Bound`] of the `Selection`, or `None` if the 
    /// `Selection` is empty.
    ///
    /// [`Bound`]: ../bound/enum.Bound.html
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Bound::*;
    /// # use interval::Interval;
    /// # use interval::Selection;
    /// # fn main() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let sel: Selection<i32> = Selection::from(Interval::closed(-3, 5));
    /// assert_eq!(sel.lower_bound(), Some(Include(-3)));
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
    /// # use interval::Selection;
    /// # fn main() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let sel: Selection<i32> = Selection::from(Interval::open(-3, 5));
    /// 
    /// assert_eq!(sel.lower_bound(), Some(Include(-2)));
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn lower_bound(&self) -> Option<Bound<T>> {
        self.iter().next().and_then(|i| i.lower_bound())
    }
    
    /// Returns the upper [`Bound`] of the `Selection`, or `None` if the 
    /// `Selection` is [`empty`].
    ///
    /// [`Bound`]: ../bound/enum.Bound.html
    /// [`empty`]: #method.empty
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Bound::*;
    /// # use interval::Interval;
    /// # use interval::Selection;
    /// # fn main() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let sel: Selection<i32> = Selection::from(Interval::closed(-3, 5));
    /// assert_eq!(sel.upper_bound(), Some(Include(5)));
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
    /// # use interval::Selection;
    /// # fn main() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let sel: Selection<i32> = Selection::from(Interval::open(-3, 5));
    /// 
    /// assert_eq!(sel.upper_bound(), Some(Include(4)));
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn upper_bound(&self) -> Option<Bound<T>> {
        self.iter().next_back().and_then(|i| i.upper_bound())
    }
    
    /// Returns the greatest lower bound of the `Selection`, or `None` if the
    /// `Selection` is [`empty`].
    ///
    /// [`empty`]: #method.empty
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Interval;
    /// # use interval::Selection;
    /// # fn main() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let sel: Selection<i32> = Selection::from(Interval::open(-3, 5));
    /// assert_eq!(sel.infimum(), Some(-3));
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
    /// # use interval::Selection;
    /// # fn main() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let sel: Selection<i32> = Selection::from(Interval::open(-3, 5));
    /// 
    /// assert_eq!(sel.infimum(), Some(-3));
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn infimum(&self) -> Option<T> {
        self.0.lower_bound().and_then(|b| b.as_ref().cloned())
    }
    
    
    /// Returns the least upper bound of the `Interval`, or `None` if the
    /// `Interval` is [`empty`].
    ///
    /// [`empty`]: #method.empty
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Interval;
    /// # use interval::Selection;
    /// # fn main() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let sel: Selection<i32> = Selection::from(Interval::open(-3, 5));
    /// assert_eq!(sel.supremum(), Some(5));
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
    /// # use interval::Selection;
    /// # fn main() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let sel: Selection<i32> = Selection::from(Interval::open(-3, 5));
    /// 
    /// assert_eq!(sel.supremum(), Some(5));
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn supremum(&self) -> Option<T> {
        self.0.upper_bound().and_then(|b| b.as_ref().cloned())
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
    /// # use interval::Selection;
    /// # fn main() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let sel: Selection<i32> = Selection::from(Interval::closed(-3, 5));
    /// assert_eq!(sel.is_empty(), false);
    ///
    /// let sel: Selection<i32> = Selection::from(Interval::empty());
    /// assert_eq!(sel.is_empty(), true);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns `true` if the interval contains all points.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Interval;
    /// # use interval::Selection;
    /// # fn main() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let sel: Selection<i32> = Selection::from(Interval::closed(-3, 5));
    /// assert_eq!(sel.is_full(), false);
    ///
    /// let sel: Selection<i32> = Selection::from(Interval::full());
    /// assert_eq!(sel.is_full(), true);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn is_full(&self) -> bool {
        self.0.is_full()
    }

    /// Returns `true` if the the interval is bounded.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Interval;
    /// # use interval::Selection;
    /// # fn main() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let sel: Interval<Option<i32>> = Interval::open(Some(-2), Some(4));
    /// assert_eq!(sel.is_left_bounded(), true);
    ///
    /// let sel: Interval<Option<i32>> = Interval::unbounded_to(Some(-3));
    /// assert_eq!(sel.is_left_bounded(), false);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn is_bounded(&self) -> bool {
        self.lower_bound().is_some() || self.upper_bound().is_some()
    }

    /// Returns `true` if the the `Selection` is left-bounded.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Interval;
    /// # use interval::Selection;
    /// # fn main() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let sel: Selection<Option<i32>> = Interval::open(Some(-2), Some(4)).into();
    /// assert_eq!(sel.is_left_bounded(), true);
    ///
    /// let sel: Selection<Option<i32>> = Interval::unbounded_to(Some(-3)).into();
    /// assert_eq!(sel.is_left_bounded(), false);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn is_left_bounded(&self) -> bool {
        match self.lower_bound() {
            Some(Bound::Infinite) => false,
            _                     => true,
        }
    }

    
    /// Returns `true` if the the `Selection` is right-bounded.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Interval;
    /// # use interval::Selection;
    /// # fn main() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let sel: Selection<Option<i32>> = Interval::open(Some(-2), Some(4)).into();
    /// assert_eq!(sel.is_right_bounded(), true);
    ///
    /// let sel: Selection<Option<i32>> = Interval::unbounded_from(Some(-3)).into();
    /// assert_eq!(sel.is_right_bounded(), false);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn is_right_bounded(&self) -> bool {
        match self.upper_bound() {
            Some(Bound::Infinite) => false,
            _                     => true,
        }
    }

    /// Returns `true` if the the `Selection` is helf-bounded.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Interval;
    /// # use interval::Selection;
    /// # fn main() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let sel: Selection<Option<i32>> = Interval::unbounded_to(Some(-2)).into();
    /// assert_eq!(sel.is_half_bounded(), true);
    ///
    /// let sel: Selection<Option<i32>> = Interval::full().into();
    /// assert_eq!(sel.is_half_bounded(), false);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn is_half_bounded(&self) -> bool {
        let l = self.is_left_bounded();
        let r = self.is_right_bounded();
        (l && !r) || (!l && r)
    }

    /// Returns `true` if the the `Selection` contains the given point.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Interval;
    /// # use interval::Selection;
    /// # fn main() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let sel: Selection<i32> = Selection::from(Interval::closed(0, 20));
    /// assert_eq!(sel.contains(&2), true);
    ///
    /// assert_eq!(sel.contains(&-15), false);
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
    
    /// Returns `true` if the `Selection` overlaps the given `Selection`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Interval;
    /// # use interval::Selection;
    /// # fn main() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let a: Selection<i32> = Selection::from(Interval::closed(-3, 5));
    /// let b: Selection<i32> = Selection::from(Interval::closed(4, 15));
    /// assert_eq!(a.intersects(&b), true);
    ///
    /// let a: Selection<i32> = Selection::from(Interval::closed(-3, 5));
    /// let b: Selection<i32> = Selection::from(Interval::closed(8, 12));
    /// assert_eq!(a.intersects(&b), false);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    pub fn intersects(&self, other: &Self) -> bool {
        // TODO: Make generic?
        !self.0.intersect(&other.0).is_empty()
    }

    ////////////////////////////////////////////////////////////////////////////
    // Symmetric set operations
    ////////////////////////////////////////////////////////////////////////////

    /// Returns the `Selection` containing all points in not contained in the
    /// `Selection`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Interval;
    /// # use interval::Selection;
    /// # fn main() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    // /// let sel: Selection<i32> = Selection::from(Interval::open(-3, 5));
    // /// 
    // /// assert_eq!(sel.complement().collect::<Vec<_>>(), 
    // ///     vec![Interval::unbounded_to(-3), Interval::unbounded_from(5)]);
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
    /// # use std::i32;
    /// # use interval::Interval;
    /// # use interval::Selection;
    /// # fn main() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let sel: Selection<i32> = Selection::from(Interval::closed(-3, 5));
    /// 
    /// assert_eq!(sel.complement().iter().collect::<Vec<_>>(), vec![
    ///     Interval::closed(i32::MIN, -4),
    ///     Interval::closed(6, i32::MAX),
    /// ]);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    pub fn complement(&self) -> Self {
        Selection(self.0.complement())
    }

    /// Returns the `Selection` containing all points in both the given
    /// `Selection`s.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Interval;
    /// # use interval::Selection;
    /// # fn main() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let a: Selection<i32> = Selection::from(Interval::closed(-3, 7));
    /// let b: Selection<i32> = Selection::from(Interval::closed(4, 13));
    /// assert_eq!(a.intersect(&b).iter().collect::<Vec<_>>(),
    ///     vec![Interval::closed(4, 7)]);
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
    /// # use interval::Selection;
    /// # fn main() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let a: Selection<i32> = Selection::from(Interval::open(-3, 7));
    /// let b: Selection<i32> = Selection::from(Interval::open(4, 13));
    /// assert_eq!(a.intersect(&b).iter().collect::<Vec<_>>(),
    ///     vec![Interval::closed(5, 6)]);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    pub fn intersect(&self, other: &Self) -> Self {
        Selection(self.0.intersect(&other.0))
    }

    /// Returns the `Selection` containing all points in either of the given
    /// `Selection`s.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Interval;
    /// # use interval::Selection;
    /// # fn main() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let a: Selection<i32> = Selection::from(Interval::closed(-3, 7));
    /// let b: Selection<i32> = Selection::from(Interval::closed(4, 13));
    /// assert_eq!(a.union(&b).iter().collect::<Vec<_>>(),
    ///     vec![Interval::closed(-3, 13)]);
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
    /// # use interval::Selection;
    /// # fn main() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let a: Selection<i32> = Selection::from(Interval::open(-3, 7));
    /// let b: Selection<i32> = Selection::from(Interval::open(4, 13));
    /// assert_eq!(a.union(&b).iter().collect::<Vec<_>>(),
    ///     vec![Interval::closed(-2, 12)]);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    pub fn union(&self, other: &Self) -> Self {
        Selection(self.0.union(&other.0))
    }

    /// Returns the `Selection` containing all points in the `Selection` which
    /// are not in the given `Selection`s.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Interval;
    /// # use interval::Selection;
    /// # fn main() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let a: Selection<i32> = Selection::from(Interval::closed(-3, 7));
    /// let b: Selection<i32> = Selection::from(Interval::closed(4, 13));
    /// assert_eq!(a.minus(&b).iter().collect::<Vec<_>>(),
    ///     vec![Interval::right_open(-3, 4)]);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    pub fn minus(&self, other: &Self) -> Self {
        Selection(self.0.minus(&other.0))
    }

    /// Returns the smallest `Interval` containing all of the points in the 
    /// `Selection`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Interval;
    /// # use interval::Selection;
    /// # fn main() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let a: Selection<i32> = Selection::from(Interval::open(-3, 5));
    /// let b: Selection<i32> = Selection::from(Interval::closed(9, 13));
    /// let sel = a.union(&b);
    ///
    /// assert_eq!(sel.enclose(), Interval::left_open(-3, 13));
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    pub fn enclose(&self) -> Interval<T> {
        Interval(self.0.enclose().normalized())
    }

    /// Returns the smallest closed `Interval` containing all of the points
    /// in the `Selection`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Interval;
    /// # use interval::Selection;
    /// # fn main() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let a: Selection<i32> = Selection::from(Interval::open(-3, 5));
    /// let b: Selection<i32> = Selection::from(Interval::closed(9, 13));
    /// let sel = a.union(&b);
    ///
    /// assert_eq!(sel.enclose(), Interval::closed(-2, 13));
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    pub fn closure(&self) -> Interval<T> {
        Interval(self.0.closure().normalized())
    }

    ////////////////////////////////////////////////////////////////////////////
    // In-place operations
    ////////////////////////////////////////////////////////////////////////////

    /// Reduces the `Selection` to only those points contained in the given
    /// `Interval`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Interval;
    /// # use interval::Selection;
    /// # fn main() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let mut sel: Selection<i32> = Selection::from(Interval::closed(-3, 7));
    /// sel.intersect_in_place(Interval::open(2, 5));
    ///
    /// assert_eq!(sel.iter().collect::<Vec<_>>(),
    ///     [Interval::open(2, 5)]);
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
    /// # use interval::Selection;
    /// # fn main() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let mut sel: Selection<i32> = Selection::from(Interval::closed(-3, 7));
    /// sel.intersect_in_place(Interval::open(2, 5));
    ///
    /// assert_eq!(sel.iter().collect::<Vec<_>>(),
    ///     [Interval::closed(3, 4)]);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    pub fn intersect_in_place(&mut self, interval: Interval<T>) {
        self.0.intersect_in_place(&interval.0.denormalized());
    }

    /// Adds all of the points in the given `Interval` to the `Selection`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Interval;
    /// # use interval::Selection;
    /// # fn main() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let mut sel: Selection<i32> = Selection::from(Interval::closed(-3, 7));
    /// sel.union_in_place(Interval::open(12, 15));
    ///
    /// assert_eq!(sel.iter().collect::<Vec<_>>(),
    ///     [Interval::closed(-3, 7), Interval::open(12, 15)]);
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
    /// # use interval::Selection;
    /// # fn main() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let mut sel: Selection<i32> = Selection::from(Interval::open(-3, 8));
    /// sel.union_in_place(Interval::open(7, 10));
    ///
    /// assert_eq!(sel.iter().collect::<Vec<_>>(),
    ///     [Interval::closed(-2, 9)]);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    pub fn union_in_place(&mut self, interval: Interval<T>) {
        self.0.union_in_place(&interval.0.denormalized());
    }

    /// Removes all of the points in the given `Interval` from the `Selection`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use interval::Interval;
    /// # use interval::Selection;
    /// # fn main() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let mut sel: Selection<i32> = Selection::from(Interval::closed(-3, 7));
    /// sel.minus_in_place(Interval::open(2, 5));
    ///
    /// assert_eq!(sel.iter().collect::<Vec<_>>(),
    ///     [Interval::closed(-3, 2), Interval::closed(5, 7)]);
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
    /// # use interval::Selection;
    /// # fn main() -> Result<(), Box<Error>> {
    /// # //-------------------------------------------------------------------
    /// let mut sel: Selection<i32> = Selection::from(Interval::closed(-3, 7));
    /// sel.minus_in_place(Interval::closed(2, 5));
    ///
    /// assert_eq!(sel.iter().collect::<Vec<_>>(),
    ///     [Interval::closed(-3, 1), Interval::closed(6, 7)]);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    pub fn minus_in_place(&mut self, interval: Interval<T>) {
        self.0.minus_in_place(&interval.0.denormalized());
    }

    ////////////////////////////////////////////////////////////////////////////
    // Iterator conversions
    ////////////////////////////////////////////////////////////////////////////

    /// Returns an iterator over each of the `Interval`s in the `Selection`.
    pub fn iter(&self) -> IntervalIter<'_, T> {
        IntervalIter(self.0.iter_intervals())
    }
}

////////////////////////////////////////////////////////////////////////////////
// Default
////////////////////////////////////////////////////////////////////////////////
impl<T> Default for Selection<T> 
    where T: PartialOrd + Ord + Clone
{
    fn default() -> Self {
        Selection::new()
    }
}


////////////////////////////////////////////////////////////////////////////////
// Special iterator traits
////////////////////////////////////////////////////////////////////////////////
impl<T> Extend<Interval<T>> for Selection<T>
    where T: PartialOrd + Ord + Clone
{
    fn extend<I>(&mut self, iter: I) where I: IntoIterator<Item=Interval<T>> {
        for interval in iter.into_iter() {
            let raw = interval.0.denormalized();
            self.0.union_in_place(&raw);
        }
    }
}


////////////////////////////////////////////////////////////////////////////////
// Conversion traits
////////////////////////////////////////////////////////////////////////////////
impl<T> From<Interval<T>> for Selection<T>
    where
        T: PartialOrd + Ord + Clone,
        RawInterval<T>: Normalize 
{
    fn from(interval: Interval<T>) -> Self {
        let raw = interval.0.denormalized();
        Selection(TineTree::from_raw_interval(raw))
    }
}

impl<T> FromIterator<Interval<T>> for Selection<T>
    where T: PartialOrd + Ord + Clone
{
    fn from_iter<I>(iter: I) -> Self where I: IntoIterator<Item=Interval<T>> {
        let mut selection = Selection::new();
        for interval in iter.into_iter() {
            let raw = interval.0.denormalized();
            selection.0.union_in_place(&raw);
        }
        selection
    }
}

impl<T> IntoIterator for Selection<T>
    where T: PartialOrd + Ord + Clone 
{
    type Item = Interval<T>;
    type IntoIter = IntoIter<T>;
    
    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self.0.into_iter())
    }
}


////////////////////////////////////////////////////////////////////////////////
// IntoIter
////////////////////////////////////////////////////////////////////////////////
/// An owning `Iterator` over the `Interval`s of a `Selection`.
#[derive(Debug)]
pub struct IntoIter<T>(tine_tree::IntoIter<T>);

impl<T> Iterator for IntoIter<T>
    where T: PartialOrd + Ord + Clone
{
    type Item = Interval<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0
            .next()
            .map(Normalize::normalized)
            .map(Interval::from)
    }
}

impl<T> DoubleEndedIterator for IntoIter<T>
    where T: PartialOrd + Ord + Clone
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0
            .next_back()
            .map(Normalize::normalized)
            .map(Interval::from)
    }
}


////////////////////////////////////////////////////////////////////////////////
// IntervalIter
////////////////////////////////////////////////////////////////////////////////
/// An `Iterator` over the `Interval`s of a `Selection`.
#[derive(Debug)]
pub struct IntervalIter<'t, T>(RawIntervalIter<'t, T>)
    where T: PartialOrd + Ord + Clone;

impl<'t, T> Iterator for IntervalIter<'t, T> 
    where T: PartialOrd + Ord + Clone
{
    type Item = Interval<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0
            .next()
            .map(Normalize::normalized)
            .map(Interval::from)
    }
}


impl<'t, T> DoubleEndedIterator for IntervalIter<'t, T> 
    where T: PartialOrd + Ord + Clone
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0
            .next_back()
            .map(Normalize::normalized)
            .map(Interval::from)
    }
}

