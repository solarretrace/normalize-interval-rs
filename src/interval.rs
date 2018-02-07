// The MIT License (MIT)
// 
// Copyright (c) 2017 Skylor R. Schermer
// 
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// 
// The above copyright notice and this permission notice shall be included in 
// all copies or substantial portions of the Software.
// 
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//
////////////////////////////////////////////////////////////////////////////////
//!
//! Provides an interval type for doing set selection and iteration.
//!
////////////////////////////////////////////////////////////////////////////////

// Local imports.
use bound::*;

// Standard imports.
use std::cmp::Ordering;
use std::fmt;
use std::convert::From;
use std::mem;
use std;

// Local enum shortcuts.
use Bound::*;
use self::RawInterval::*;


////////////////////////////////////////////////////////////////////////////////
// Interval<T>
////////////////////////////////////////////////////////////////////////////////
/// A contiguous interval of the type T.
#[derive(Debug, Eq, Hash, Clone, Copy)]
pub struct Interval<T>(RawInterval<T>) where T: PartialOrd + Ord + Clone;

// All mutable operations and constructors on `Interval` must ensure that the
// interval is normalized before returning.
impl<T> Interval<T> where T: PartialOrd + Ord + Clone {
	/// Constructs a new interval from the given bounds. If the right bound
	/// point is less than the left bound point, an empty interval will be 
	/// returned.
	///
	/// # Example
	///
	/// ```rust
	/// use interval::{Bound, Interval};
	/// 
	/// #[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
	/// struct Opaque(i32);
	/// 
	/// let l = Bound::Exclude(Opaque(15));
	/// let r = Bound::Exclude(Opaque(30));
	///
	/// let int = Interval::new(Some(l), Some(r));
	/// 
	/// assert_eq!(int.lower_bound(), Some(l));
	/// assert_eq!(int.upper_bound(), Some(r));
	/// ```
	///
	/// If the interval has a normalized representation, it will be
	/// normalized before being returned:
	///
	/// ```rust
	/// # use interval::{Bound, Interval};
	/// let l = Bound::Exclude(15);
	/// let r = Bound::Exclude(30);
	///
	/// let int = Interval::new(Some(l), Some(r));
	/// 
	/// assert_eq!(int.lower_bound(), Some(Bound::Include(16)));
	/// assert_eq!(int.upper_bound(), Some(Bound::Include(29)));
	/// ```
	///
	/// If the arguments are out of order, an empty interval will be returned:
	///
	/// ```rust
	/// # use interval::{Bound, Interval};
	/// let l = Bound::Include(15);
	/// let r = Bound::Include(30);
	///
	/// let int = Interval::new(Some(r), Some(l));
	/// 
	/// assert!(int.is_empty());
	/// ```
	pub fn new(left: Option<Bound<T>>, right: Option<Bound<T>>) -> Self {
		Interval::normalized(RawInterval::new(left, right))
	}

	/// Returns the normalized interval.
	fn normalized(raw: RawInterval<T>) -> Self {
		Normalize::normalized(Interval(raw))
	}
	
	/// Returns an empty interval.
	pub fn empty() -> Self {
		// Normalization not needed for empty intervals.
		Interval(Empty)
	}
	
	/// Constructs a new point interval for the given point.
	pub fn point(point: T) -> Self {
		// Normalization not needed for point intervals.
		Interval(RawInterval::Point(point))
	}
	
	/// Constructs a new open interval from the given points.
	///
	/// # Example
	///
	/// ```rust
	/// use interval::{Bound, Interval};
	/// 
	/// #[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
	/// struct Opaque(i32);
	///
	/// let int = Interval::open(Opaque(15), Opaque(30));
	/// 
	/// assert_eq!(int.lower_bound(), Some(Bound::Exclude(Opaque(15))));
	/// assert_eq!(int.upper_bound(), Some(Bound::Exclude(Opaque(30))));
	/// ```
	///
	/// If the interval has a normalized representation, it will be
	/// normalized before being returned:
	///
	/// ```rust
	/// # use interval::{Bound, Interval};
	/// let int = Interval::open(15, 30);
	/// 
	/// assert_eq!(int.lower_bound(), Some(Bound::Include(16)));
	/// assert_eq!(int.upper_bound(), Some(Bound::Include(29)));
	/// ```
	///
	/// If the arguments are out of order, an empty interval will be returned:
	///
	/// ```rust
	/// # use interval::{Bound, Interval};
	/// let int = Interval::open(30, 15);
	/// 
	/// assert!(int.is_empty());
	/// ```
	pub fn open(left: T, right: T) -> Self {
		Interval::normalized(RawInterval::open(left, right))
	}
	
	/// Constructs a new left-open interval from the given points.
	///
	/// # Example
	///
	/// ```rust
	/// use interval::{Bound, Interval};
	/// 
	/// #[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
	/// struct Opaque(i32);
	///
	/// let int = Interval::left_open(Opaque(15), Opaque(30));
	/// 
	/// assert_eq!(int.lower_bound(), Some(Bound::Exclude(Opaque(15))));
	/// assert_eq!(int.upper_bound(), Some(Bound::Include(Opaque(30))));
	/// ```
	///
	/// If the interval has a normalized representation, it will be
	/// normalized before being returned:
	///
	/// ```rust
	/// # use interval::{Bound, Interval};
	/// let int = Interval::left_open(15, 30);
	/// 
	/// assert_eq!(int.lower_bound(), Some(Bound::Include(16)));
	/// assert_eq!(int.upper_bound(), Some(Bound::Include(30)));
	/// ```
	///
	/// If the arguments are out of order, an empty interval will be returned:
	///
	/// ```rust
	/// # use interval::{Bound, Interval};
	/// let int = Interval::left_open(30, 15);
	/// 
	/// assert!(int.is_empty());
	/// ```
	pub fn left_open(left: T, right: T) -> Self {
		Interval::normalized(RawInterval::left_open(left, right))
	}
	
	/// Constructs a new right-open interval from the given points.
	///
	/// # Example
	///
	/// ```rust
	/// use interval::{Bound, Interval};
	/// 
	/// #[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
	/// struct Opaque(i32);
	///
	/// let int = Interval::right_open(Opaque(15), Opaque(30));
	/// 
	/// assert_eq!(int.lower_bound(), Some(Bound::Include(Opaque(15))));
	/// assert_eq!(int.upper_bound(), Some(Bound::Exclude(Opaque(30))));
	/// ```
	///
	/// If the interval has a normalized representation, it will be
	/// normalized before being returned:
	///
	/// ```rust
	/// # use interval::{Bound, Interval};
	/// let int = Interval::right_open(15, 30);
	/// 
	/// assert_eq!(int.lower_bound(), Some(Bound::Include(15)));
	/// assert_eq!(int.upper_bound(), Some(Bound::Include(29)));
	/// ```
	///
	/// If the arguments are out of order, an empty interval will be returned:
	///
	/// ```rust
	/// # use interval::{Bound, Interval};
	/// let int = Interval::right_open(30, 15);
	/// 
	/// assert!(int.is_empty());
	/// ```
	pub fn right_open(left: T, right: T) -> Self {
		Interval::normalized(RawInterval::right_open(left, right))
	}
	
	/// Constructs a new closed interval from the given points. If the interval
	/// has a normalized representation, it will be  normalized before being
	/// returned.
	///
	/// # Example
	///
	/// ```rust
	/// use interval::{Bound, Interval};
	///
	/// let int = Interval::closed(15, 30);
	/// 
	/// assert_eq!(int.lower_bound(), Some(Bound::Include(15)));
	/// assert_eq!(int.upper_bound(), Some(Bound::Include(30)));
	/// ```
	///
	/// If the arguments are out of order, an empty interval will be returned:
	///
	/// ```rust
	/// # use interval::Interval;
	/// let int = Interval::closed(30, 15);
	/// 
	/// assert!(int.is_empty());
	/// ```
	pub fn closed(left: T, right: T) -> Self {
		// Normalization not needed for closed intervals.
		Interval::normalized(RawInterval::closed(left, right))
	}

	/// Constructs a new interval including the given point and all points
	/// greater than it.
	///
	/// # Example
	///
	/// ```rust
	/// use interval::{Bound, Interval};
	/// 
	/// #[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
	/// struct Opaque(i32);
	///
	/// let int = Interval::from(Opaque(15));
	/// 
	/// assert_eq!(int.lower_bound(), Some(Bound::Include(Opaque(15))));
	/// assert_eq!(int.upper_bound(), None);
	/// ```
	///
	/// If the interval has a normalized representation, it will be
	/// normalized before being returned:
	///
	/// ```rust
	/// # use interval::{Bound, Interval};
	/// let int = Interval::from(15);
	/// 
	/// assert_eq!(int.lower_bound(), Some(Bound::Include(15)));
	/// assert_eq!(int.upper_bound(), Some(Bound::Include(std::i32::MAX)));
	/// ```
	pub fn from(point: T) -> Self {
		// Normalization not needed for closed intervals.
		Interval::normalized(RawInterval::From(point))
	}

	/// Constructs a new interval including the given point and all points
	/// less than it.
	///
	/// # Example
	///
	/// ```rust
	/// use interval::{Bound, Interval};
	/// 
	/// #[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
	/// struct Opaque(i32);
	///
	/// let int = Interval::to(Opaque(15));
	/// 
	/// assert_eq!(int.lower_bound(), None);
	/// assert_eq!(int.upper_bound(), Some(Bound::Include(Opaque(15))));
	/// ```
	///
	/// If the interval has a normalized representation, it will be
	/// normalized before being returned:
	///
	/// ```rust
	/// # use interval::{Bound, Interval};
	/// let int = Interval::to(15);
	/// 
	/// assert_eq!(int.lower_bound(), Some(Bound::Include(std::i32::MIN)));
	/// assert_eq!(int.upper_bound(), Some(Bound::Include(15)));
	/// ```
	pub fn to(point: T) -> Self {
		// Normalization not needed for closed intervals.
		Interval::normalized(RawInterval::To(point))
	}

	/// Constructs a new interval containing all points greater than the given
	/// point.
	///
	/// # Example
	///
	/// ```rust
	/// use interval::{Bound, Interval};
	/// 
	/// #[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
	/// struct Opaque(i32);
	///
	/// let int = Interval::up_from(Opaque(15));
	/// 
	/// assert_eq!(int.lower_bound(), Some(Bound::Exclude(Opaque(15))));
	/// assert_eq!(int.upper_bound(), None);
	/// ```
	///
	/// If the interval has a normalized representation, it will be
	/// normalized before being returned:
	///
	/// ```rust
	/// # use interval::{Bound, Interval};
	/// let int = Interval::up_from(15);
	/// 
	/// assert_eq!(int.lower_bound(), Some(Bound::Include(16)));
	/// assert_eq!(int.upper_bound(), Some(Bound::Include(std::i32::MAX)));
	/// ```
	pub fn up_from(point: T) -> Self {
		// Normalization not needed for closed intervals.
		Interval::normalized(RawInterval::UpFrom(point))
	}

	/// Constructs a new interval containing all points less than the given
	/// point.
	///
	/// # Example
	///
	/// ```rust
	/// use interval::{Bound, Interval};
	/// 
	/// #[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
	/// struct Opaque(i32);
	///
	/// let int = Interval::up_to(Opaque(15));
	/// 
	/// assert_eq!(int.lower_bound(), None);
	/// assert_eq!(int.upper_bound(), Some(Bound::Exclude(Opaque(15))));
	/// ```
	///
	/// If the interval has a normalized representation, it will be
	/// normalized before being returned:
	///
	/// ```rust
	/// # use interval::{Bound, Interval};
	/// let int = Interval::up_to(15);
	/// 
	/// assert_eq!(int.lower_bound(), Some(Bound::Include(std::i32::MIN)));
	/// assert_eq!(int.upper_bound(), Some(Bound::Include(14)));
	/// ```
	pub fn up_to(point: T) -> Self {
		// Normalization not needed for closed intervals.
		Interval::normalized(RawInterval::UpTo(point))
	}

	/// Constructs a new interval containing every point.
	///
	/// # Example
	///
	/// ```rust
	/// use interval::{Bound, Interval};
	/// 
	/// #[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
	/// struct Opaque(i32);
	///
	/// let int = Interval::<Opaque>::full();
	/// 
	/// assert_eq!(int.lower_bound(), None);
	/// assert_eq!(int.upper_bound(), None);
	/// ```
	///
	/// If the interval has a normalized representation, it will be
	/// normalized before being returned:
	///
	/// ```rust
	/// # use interval::{Bound, Interval};
	/// let int = Interval::<i32>::full();
	/// 
	/// assert_eq!(int.lower_bound(), Some(Bound::Include(std::i32::MIN)));
	/// assert_eq!(int.upper_bound(), Some(Bound::Include(std::i32::MAX)));
	/// ```
	pub fn full() -> Self {
		// Normalization not needed for closed intervals.
		Interval::normalized(RawInterval::Full)
	}
	
	/// Returns whether the interval excludes any of its boundary points.
	///
	/// # Example
	///
	/// ```rust
	/// use interval::Interval;
	/// 
	/// #[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
	/// struct Opaque(i32);
	///
	/// let a = Interval::left_open(Opaque(15), Opaque(30));
	/// let b = Interval::closed(Opaque(15), Opaque(30));
	/// 
	/// assert!(a.is_open());
	/// assert!(!b.is_open())
	/// ```
	pub fn is_open(&self) -> bool {
		match self.0 {
			Point(_)	 => false,
			Closed(_, _) => false,
			_ 			 => true,
		}
	}
	
	/// Returns whether the interval includes only one of its boundary points.
	///
	/// # Example
	///
	/// ```rust
	/// use interval::Interval;
	/// 
	/// #[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
	/// struct Opaque(i32);
	///
	/// let a = Interval::left_open(Opaque(15), Opaque(30));
	/// let b = Interval::open(Opaque(15), Opaque(30));
	/// 
	/// assert!(a.is_half_open());
	/// assert!(!b.is_half_open())
	/// ```
	pub fn is_half_open(&self) -> bool {
		match self.0 {
			LeftOpen(_, _)	=> true,
			RightOpen(_, _)	=> true,
			To(_)			=> true,
			From(_)			=> true,
			_				=> false,
		}
	}


	/// Returns whether the interval excludes its left boundary point.
	///
	/// # Example
	///
	/// ```rust
	/// use interval::Interval;
	/// 
	/// #[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
	/// struct Opaque(i32);
	///
	/// let a = Interval::left_open(Opaque(15), Opaque(30));
	/// let b = Interval::open(Opaque(15), Opaque(30));
	/// let c = Interval::right_open(Opaque(15), Opaque(30));
	/// 
	/// assert!(a.is_left_open());
	/// assert!(b.is_left_open());
	/// assert!(!c.is_left_open());
	/// ```
	pub fn is_left_open(&self) -> bool {
		match self.0 {
			Open(_, _)		=> true,
			LeftOpen(_, _)	=> true,
			UpFrom(_)		=> true,
			To(_)			=> true,
			Full			=> true,
			_				=> false,
		}
	}


	/// Returns whether the interval excludes its right boundary point.
	///
	/// # Example
	///
	/// ```rust
	/// use interval::Interval;
	/// 
	/// #[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
	/// struct Opaque(i32);
	///
	/// let a = Interval::left_open(Opaque(15), Opaque(30));
	/// let b = Interval::open(Opaque(15), Opaque(30));
	/// let c = Interval::right_open(Opaque(15), Opaque(30));
	/// 
	/// assert!(!a.is_right_open());
	/// assert!(b.is_right_open());
	/// assert!(c.is_right_open());
	/// ```
	pub fn is_right_open(&self) -> bool {
		match self.0 {
			Open(_, _)		=> true,
			RightOpen(_, _)	=> true,
			UpTo(_)			=> true,
			From(_)			=> true,
			Full			=> true,
			_				=> false,
		}
	}

	
	/// Returns whether the interval includes all of its boundary points.
	///
	/// # Example
	///
	/// ```rust
	/// use interval::Interval;
	/// 
	/// #[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
	/// struct Opaque(i32);
	///
	/// let a = Interval::closed(Opaque(15), Opaque(30));
	/// let b = Interval::left_open(Opaque(15), Opaque(30));
	/// 
	/// assert!(a.is_closed());
	/// assert!(!b.is_closed())
	/// ```
	pub fn is_closed(&self) -> bool {
		match self.0 {
			Empty		 => true,
			Point(_)	 => true,
			Closed(_, _) => true,
			Full		 => true,
			_			 => false,
		}
	}
	
	/// Returns whether the interval is empty (i.e., contains no points.)
	///
	/// # Example
	///
	/// ```rust
	/// use interval::Interval;
	///
	/// let a = Interval::<u32>::empty();
	/// let b = Interval::closed(15, 30);
	/// 
	/// assert!(a.is_empty());
	/// assert!(!b.is_empty())
	/// ```
	pub fn is_empty(&self) -> bool {
		match self.0 {
			Empty => true,
			_	  => false,
		}
	}
	
	/// Returns whether the interval is degenerate (i.e., contains only a
	/// single point.)
	///
	/// # Example
	///
	/// ```rust
	/// use interval::Interval;
	///
	/// let a = Interval::closed(1, 1);
	/// let b = Interval::closed(15, 30);
	/// 
	/// assert!(a.is_degenerate());
	/// assert!(!b.is_degenerate());
	/// ```
	pub fn is_degenerate(&self) -> bool {
		match self.0 {
			Point(_) => true,
			_		 => false,
		}
	}
	
	/// Returns whether the interval is full (i.e., contains all points.)
	///
	/// # Example
	///
	/// ```rust
	/// use interval::Interval;
	///
	/// let a = Interval::<i32>::new(None, None);
	/// let b = Interval::closed(15, 30);
	/// 
	/// assert!(a.is_full());
	/// assert!(!b.is_full());
	/// ```
	pub fn is_full(&self) -> bool {
		*self == Interval::normalized(Full)
	}
	
	/// Returns whether the `Interval` is bounded on both sides.
	///
	/// # Example
	///
	/// ```rust
	/// use interval::{Bound, Interval};
	///
	/// let int = Interval::closed(15, 30);
	/// 
	/// assert!(int.is_bounded());
	/// ```
	pub fn is_bounded(&self) -> bool {
		match self.0 {
			UpTo(_)	  => false,
			UpFrom(_) => false,
			To(_)	  => false,
			From(_)	  => false,
			Full	  => false,
			_		  => true,
		}
	}
	
	/// Returns the lower bound of the interval.
	///
	/// # Example
	///
	/// ```rust
	/// use interval::{Bound, Interval};
	///
	/// let int = Interval::closed(15, 30);
	/// 
	/// assert_eq!(int.lower_bound(), Some(Bound::Include(15)));
	/// ```
	pub fn lower_bound(&self) -> Option<Bound<T>> {
		self.0.lower_bound()
	}
	
	/// Returns the upper bound of the interval.
	///
	/// # Example
	///
	/// ```rust
	/// use interval::{Bound, Interval};
	///
	/// let int = Interval::closed(15, 30);
	/// 
	/// assert_eq!(int.upper_bound(), Some(Bound::Include(30)));
	/// ```
	pub fn upper_bound(&self) -> Option<Bound<T>> {
		self.0.upper_bound()
	}
	
	/// Returns the greatest lower bound of the `Interval`.
	///
	/// # Example
	///
	/// ```rust
	/// use interval::Interval;
	///
	/// let int = Interval::closed(15, 30);
	/// 
	/// assert_eq!(int.infimum(), Some(15));
	/// ```
	///
	/// The infimum need not lie within the interval. This will occur if the 
	/// lower bound of the normalized interval is open:
	///
	/// ```rust
	/// use interval::{Bound, Interval};
	/// 
	/// #[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
	/// struct Opaque(i32);
	///
	/// let a = Interval::left_open(Opaque(15), Opaque(30));
	/// 
	/// assert_eq!(a.infimum(), Some(Opaque(15)));
	/// ```
	pub fn infimum(&self) -> Option<T> {
		self.0.infimum()
	}
	
	/// Returns the least upper bound of the `Interval`.
	///
	/// # Example
	///
	/// ```rust
	/// use interval::Interval;
	///
	/// let int = Interval::closed(15, 30);
	/// 
	/// assert_eq!(int.supremum(), Some(30));
	/// ```
	///
	/// The supremum need not lie within the interval. This will occur if the 
	/// lower bound of the normalized interval is open:
	///
	/// ```rust
	/// use interval::{Bound, Interval};
	/// 
	/// #[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
	/// struct Opaque(i32);
	///
	/// let a = Interval::right_open(Opaque(15), Opaque(30));
	/// 
	/// assert_eq!(a.supremum(), Some(Opaque(30)));
	/// ```
	pub fn supremum(&self) -> Option<T> {
		self.0.supremum()
	}
	
	/// Returns whether the `Interval` contains the given point.
	///
	/// # Example
	///
	/// ```rust
	/// use interval::Interval;
	///
	/// let int = Interval::closed(1, 6);
	/// 
	/// assert!(!int.contains(&0));
	/// assert!(int.contains(&1));
	/// assert!(int.contains(&6));
	/// assert!(!int.contains(&10));
	/// ```
	pub fn contains(&self, point: &T) -> bool {
		self.0.contains(point)
	}
	
	/// Returns the smallest `Interval` that contains all of the points
	/// contained within this `Interval` and the given `Interval`.
	///
	/// # Example
	///
	/// ```rust
	/// use interval::Interval;
	///
	/// let a = Interval::closed(0, 4);
	/// let b = Interval::closed(7, 9);
	/// 
	/// assert_eq!(a.enclose(&b), Interval::closed(0, 9));
	/// ```
	pub fn enclose(&self, other: &Self) -> Self {
		Interval::normalized(self.0.enclose(&other.0))
	}
	
	/// Returns the largest `Interval` whose points are all contained
	/// entirely within this `Interval` and the given `Interval`.
	///
	/// # Example
	///
	/// ```rust
	/// use interval::Interval;
	///
	/// let a = Interval::closed(0, 4);
	/// let b = Interval::closed(2, 9);
	/// 
	/// assert_eq!(a.intersect(&b), Interval::closed(2, 4));
	/// ```
	pub fn intersect(&self, other: &Self) -> Self {
		Interval::normalized(self.0.intersect(&other.0))
	}
	
	/// Returns a `Vec` of `Interval`s containing all of the points contained
	/// within this `Interval` and the given `Interval`.
	///
	/// # Example
	///
	/// ```rust
	/// use interval::Interval;
	///
	/// let a = Interval::closed(0, 4);
	/// let b = Interval::closed(3, 7);
	/// 
	/// assert_eq!(a.union(&b), vec![Interval::closed(0, 7)]);
	/// ```
	///
	/// Disjoint intervals will remain seperate:
	///
	/// ```rust
	/// use interval::Interval;
	///
	/// let a = Interval::closed(0, 4);
	/// let b = Interval::closed(7, 9);
	/// 
	/// assert_eq!(a.union(&b), vec![a, b]);
	/// ```
	pub fn union(&self, other: &Self) -> Vec<Self> {
		self.0
			.union(&other.0)
			.into_iter()
			.map(Interval::normalized)
			.collect()
	}
	
	/// Returns a `Vec` of `Interval`s containing all of the points contained
	/// within this `Interval` that are not in the given `Interval`.
	///
	/// # Example
	///
	/// ```rust
	/// use interval::Interval;
	///
	/// let a = Interval::closed(0, 4);
	/// let b = Interval::closed(2, 7);
	/// 
	/// assert_eq!(a.minus(&b), vec![Interval::closed(0, 1)]);
	/// ```
	///
	/// Disjoint intervals will remain seperate:
	///
	/// ```rust
	/// use interval::Interval;
	///
	/// let a = Interval::closed(0, 5);
	/// let b = Interval::closed(2, 3);
	/// 
	/// assert_eq!(a.minus(&b), vec![
	///     Interval::closed(0, 1),
	///     Interval::closed(4, 5),
	/// ]);
	/// ```
	pub fn minus(&self, other: &Self) -> Vec<Self> {
		self.0
			.minus(&other.0)
			.into_iter()
			.map(Interval::normalized)
			.collect()
	}
	
	/// Returns a `Vec` of `Interval`s containing all of the points not in the
	/// `Interval`.
	///
	/// # Example
	///
	/// ```rust
	/// use interval::{Bound, Interval};
	///
	/// let int = Interval::right_open(0, 2);
	/// 
	/// assert_eq!(int.complement(), vec![
	///     Interval::new(None, Some(Bound::Exclude(0))),
	///     Interval::new(Some(Bound::Include(2)), None),
	/// ]);
	/// ```
	pub fn complement(&self) -> Vec<Self> {
		self.0
			.complement()
			.into_iter()
			.map(Interval::normalized)
			.collect()
	}

	/// Returns the smallest closed `Interval` containing all of the points in 
	/// this `Interval`.
	pub fn closure(&self) -> Self {
		Interval::normalized(match self.0 {
			Open(ref l, ref r)		=> Closed(l.clone(), r.clone()),
			LeftOpen(ref l, ref r)	=> Closed(l.clone(), r.clone()),
			RightOpen(ref l, ref r)	=> Closed(l.clone(), r.clone()),
			UpTo(ref r)				=> To(r.clone()),
			UpFrom(ref l)			=> From(l.clone()),
			To(_)					=> Full,
			From(_)					=> Full,
			_ 						=> self.0.clone(),
		})
	}

	/// Returns the partitions the `Interval` formed by the given point.
	///
	/// # Example
	///
	/// ```rust
	/// use interval::Interval;
	///
	/// let int = Interval::closed(0, 6);
	/// 
	/// assert_eq!(int.partition_at(&3), Some((
	///     Interval::closed(0, 2),
	///     Interval::point(3),
	///     Interval::closed(4, 6)
	/// )));
	/// ```
	pub fn partition_at(&self, point: &T) -> Option<(Self, Self, Self)> {
		if self.contains(point) {
			Some((
				Interval::new(
					self.lower_bound(),
					Some(Exclude(point.clone()))),
				Interval::point(point.clone()),
				Interval::new(
					Some(Exclude(point.clone())),
					self.upper_bound())
			))
		} else {
			None
		}
	}

	/// Converts the `Interval` into an `Option`, returning `None` if it is 
	/// empty.
	///
	/// # Example
	///
	/// ```rust
	/// use interval::Interval;
	///
	/// assert!(Interval::open(0, 0).into_non_empty().is_none());
	///
	/// let int = Interval::open(0, 2);
	/// assert_eq!(int.into_non_empty(), Some(int));
	/// ```
	pub fn into_non_empty(self) -> Option<Self> {
		if self.is_empty() {
			None
		} else {
			Some(self)
		}
	}


	/// Returns the intersection of all of the given `Interval`s.
	///
	/// # Example
	///
	/// ```rust
	/// use interval::Interval;
	///
	/// let ints = vec![
	///     Interval::closed(0, 2),
	///     Interval::closed(-1, 1),
	///     Interval::closed(0, 1),
	///     Interval::closed(0, 1),
	///     Interval::closed(0, 1),
	/// ];
	///
	/// assert_eq!(Interval::intersect_all(ints), Interval::closed(0, 1));
	/// ```
	pub fn intersect_all<I>(intervals: I) -> Self
		where I: IntoIterator<Item=Self>
	{
		Interval::normalized(
			RawInterval::intersect_all(intervals.into_iter().map(|i| i.0))
		)
	}

	/// Returns the union of all of the given `Interval`s.
	///
	/// # Example
	///
	/// ```rust
	/// use interval::Interval;
	///
	/// let ints = vec![
	///     Interval::closed(0, 2),
	///     Interval::closed(-1, 1),
	///     Interval::closed(0, 1),
	///     Interval::closed(0, 1),
	/// ];
	///
	/// assert_eq!(Interval::union_all(ints), vec![Interval::closed(-1, 2)]);
	/// ```
	pub fn union_all<I>(intervals: I) -> Vec<Self>
		where I: IntoIterator<Item=Self>
	{
		RawInterval::union_all(intervals.into_iter().map(|i| i.0))
			.into_iter()
			.map(Interval::normalized)
			.collect()
	}
}



////////////////////////////////////////////////////////////////////////////////
// Basic Trait impls
////////////////////////////////////////////////////////////////////////////////
// Interval-from-Point conversion.
impl<T> From<T> for Interval<T> where T: PartialOrd + Ord + Clone {
	#[inline]
	fn from(t: T) -> Self {
		Interval(Point(t))
	}
}

impl<T> PartialEq for Interval<T> where T: PartialOrd + Ord + Clone {
	fn eq(&self, other: &Self) -> bool {
		self.0 == other.0
	}
}


////////////////////////////////////////////////////////////////////////////////
// RawInterval<T>
////////////////////////////////////////////////////////////////////////////////
/// A contiguous interval of the type T. Used to implement the internal state of
/// `Interval`.
///
/// This module provides the `Interval` and `RawInterval` types. `Interval` is
/// a wrapper around `RawInterval` that enforces any normalization rules that
/// may be applicable for intervals of the given type. For instance, all of
/// the std integral types are finite so we can easily convert unbounded
/// intervals to bounded intervals and open intervals to closed intervals,
/// since we can guarantee that the same values will remain in the interval
/// in either case. `Interval` does this conversion, while `RawInteral` 
/// maintains seperate representations for bounded/unbounded and open/closed
/// intervals.
///
/// For types that are not bounded or iterable, `Interval` operations are
/// essentially equivalent to `RawInterval` operations, though they may expose
/// different interfaces.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum RawInterval<T> {
	/// An interval containing no points.
	Empty,
	/// An interval containing only the given point.
	Point(T),
	/// An interval containing all points between two given points, excluding
	/// them both.
	Open(T, T),
	/// An interval containing all points between two given points, including
	/// the greater of the two.
	LeftOpen(T, T),
	/// An interval containing all points between two given points, including
	/// the lesser of the two.
	RightOpen(T, T),
	/// An interval containing all points between two given points, including
	/// them both.
	Closed(T, T),
	/// An interval containing all points less than the given point.
	UpTo(T),
	/// An interval containing all points greater than the given point.
	UpFrom(T),
	/// An interval containing the given point and all points less than it.
	To(T),
	/// An interval containing the given point and all points greater than it.
	From(T),
	/// An interval containing all points.
	Full,
}

impl<T> RawInterval<T> where T: PartialOrd + Ord + Clone {
	/// Constructs a new interval from the given bounds. If the right bound
	/// point is less than the left bound point, an empty interval will be 
	/// returned.
	pub fn new(left: Option<Bound<T>>, right: Option<Bound<T>>) -> Self {
		match (left, right) {
			(Some(lb), Some(rb)) => match (lb, rb) {
				(Exclude(l), Exclude(r)) => RawInterval::open(l, r),
				(Exclude(l), Include(r)) => RawInterval::left_open(l, r),
				(Include(l), Exclude(r)) => RawInterval::right_open(l, r),
				(Include(l), Include(r)) => RawInterval::closed(l, r),
			},
			(None,	   Some(rb)) => match rb {
				Exclude(r) => UpTo(r),
				Include(r) => To(r),
			},
			(Some(lb), None)	 => match lb {
				Exclude(l) => UpFrom(l),
				Include(l) => From(l),
			},
			(None,	   None)	 => Full,
		}
	}

	/// Returns the normalized interval.
	pub fn normalize(&mut self) {
		*self = match self.clone() {
			Open(l, r)		=> match T::cmp(&l, &r) {
				Ordering::Less => Open(l, r),
				_			   => Empty,
			},
			LeftOpen(l, r)	=> match T::cmp(&l, &r) {
				Ordering::Less	  => LeftOpen(l, r),
				Ordering::Equal	  => Point(l),
				Ordering::Greater => Empty,
			},
			RightOpen(l, r)	=> match T::cmp(&l, &r) {
				Ordering::Less	  => RightOpen(l, r),
				Ordering::Equal	  => Point(l),
				Ordering::Greater => Empty,
			},
			Closed(l, r)	=> match T::cmp(&l, &r) {
				Ordering::Less	  => Closed(l, r),
				Ordering::Equal	  => Point(l),
				Ordering::Greater => Empty,
			},
			_				=> self.clone(),
		};
	}
	
	/// Constructs a new open interval from the given points. If the right
	/// point is less than the left point, an empty interval will be returned.
	pub fn open(left: T, right: T) -> Self {
		match T::cmp(&left, &right) {
			Ordering::Less => Open(left, right),
			_			   => Empty,
		}
	}
	
	/// Constructs a new left-open interval from the given points. If the
	/// right bound point is less than the left bound point, an empty interval
	/// will be returned.
	pub fn left_open(left: T, right: T) -> Self {
		match T::cmp(&left, &right) {
			Ordering::Less	  => LeftOpen(left, right),
			Ordering::Equal	  => Point(left),
			Ordering::Greater => Empty,
		}
	}
	
	/// Constructs a new right-open interval from the given points. If the
	/// right bound point is less than the left bound point, an empty interval
	/// will be returned.
	pub fn right_open(left: T, right: T) -> Self {
		match T::cmp(&left, &right) {
			Ordering::Less	  => RightOpen(left, right),
			Ordering::Equal	  => Point(left),
			Ordering::Greater => Empty,
		}
	}
	
	/// Constructs a new closed interval from the given points. If the
	/// right bound point is less than the left bound point, an empty interval
	/// will be returned.
	pub fn closed(left: T, right: T) -> Self {
		match T::cmp(&left, &right) {
			Ordering::Less	  => Closed(left, right),
			Ordering::Equal	  => Point(left),
			Ordering::Greater => Empty,
		}
	}

	/// Returns whether the interval is empty (i.e., contains no points.)
	pub fn is_empty(&self) -> bool {
		match *self {
			Empty => true,
			_	  => false,
		}
	}

	/// Returns the lower bound of the interval.
	pub fn lower_bound(&self) -> Option<Bound<T>> {
		match *self {
			Empty				=> None,
			Point(ref p)		=> Some(Include(p.clone())),
			Open(ref l, _)		=> Some(Exclude(l.clone())),
			LeftOpen(ref l, _)	=> Some(Exclude(l.clone())),
			RightOpen(ref l, _)	=> Some(Include(l.clone())),
			Closed(ref l, _)	=> Some(Include(l.clone())),
			UpTo(_)				=> None,
			UpFrom(ref p)		=> Some(Exclude(p.clone())),
			To(_)				=> None,
			From(ref p)			=> Some(Include(p.clone())),
			Full				=> None,
		}
	}
	
	/// Returns the upper bound of the interval.
	pub fn upper_bound(&self) -> Option<Bound<T>> {
		match *self {
			Empty				=> None,
			Point(ref p)		=> Some(Include(p.clone())),
			Open(_, ref r)		=> Some(Exclude(r.clone())),
			LeftOpen(_, ref r)	=> Some(Include(r.clone())),
			RightOpen(_, ref r)	=> Some(Exclude(r.clone())),
			Closed(_, ref r)	=> Some(Include(r.clone())),
			UpTo(ref p)			=> Some(Exclude(p.clone())),
			UpFrom(_)			=> None,
			To(ref p)			=> Some(Include(p.clone())),
			From(_)				=> None,
			Full				=> None,
		}
	}
	
	/// Returns the greatest lower bound of the interval.
	pub fn infimum(&self) -> Option<T> {
		self.lower_bound().map(|b| b.as_ref().clone())
	}
	
	/// Returns the least upper bound of the interval.
	pub fn supremum(&self) -> Option<T> {
		self.upper_bound().map(|b| b.as_ref().clone())
	}
	
	/// Returns whether the interval contains the given point.
	pub fn contains(&self, point: &T) -> bool {
		match *self {
			Empty					=> false,
			Point(ref p)			=> point == p,
			Open(ref l, ref r)		=> point > l && point < r,
			LeftOpen(ref l, ref r)	=> point > l && point <= r,
			RightOpen(ref l, ref r)	=> point >= l && point < r,
			Closed(ref l, ref r)	=> point >= l && point <= r,
			UpTo(ref p)				=> point < p,
			UpFrom(ref p)			=> point > p,
			To(ref p)				=> point <= p,
			From(ref p)				=> point >= p,
			Full					=> true,
		}
	}
	
	/// Returns the smallest interval that contains all of the points contained
	/// within this interval and the given interval.
	pub fn enclose(&self, other: &Self) -> Self {
		RawInterval::new(
			self.lower_bound().least_union(&other.lower_bound()), 
			self.upper_bound().greatest_union(&other.upper_bound()))
	}
	
	/// Returns the largest interval whose points are all contained
	/// entirely within this interval and the given interval.
	pub fn intersect(&self, other: &Self) -> Self {
		RawInterval::new(
			self.lower_bound().greatest_intersect(&other.lower_bound()), 
			self.upper_bound().least_intersect(&other.upper_bound()))
	}
	
	/// Returns a `Vec` of `RawInterval`s containing all of the points 
	/// contained within this interval and the given interval., vec![a, b]);
	/// ```
	pub fn union(&self, other: &Self) -> Vec<Self> {
		if !self.intersect(other).is_empty() {
			vec![self.enclose(other)]
		} else {
			vec![self.clone(), other.clone()]
		}
	}
	
	/// Returns a `Vec` of `RawInterval`s containing all of the points
	/// contained within this interval that are not in the given interval.
	pub fn minus(&self, other: &Self) -> Vec<Self> {
		other.complement()
			.into_iter()
			.map(|i| self.intersect(&i))
			.filter(|i| !i.is_empty())
			.collect()
	}
	
	/// Returns a `Vec` of `RawInterval`s containing all of the points not in
	/// the interval.
	pub fn complement(&self) -> Vec<Self> {
		match *self {
			Empty					=> vec![Full],
			Point(ref p)			=> vec![UpTo(p.clone()), UpFrom(p.clone())],
			Open(ref l, ref r)		=> vec![To(l.clone()), From(r.clone())],
			LeftOpen(ref l, ref r)	=> vec![To(l.clone()), From(r.clone())],
			RightOpen(ref l, ref r)	=> vec![UpTo(l.clone()), From(r.clone())],
			Closed(ref l, ref r)	=> vec![UpTo(l.clone()), UpFrom(r.clone())],
			UpTo(ref p)				=> vec![From(p.clone())],
			UpFrom(ref p)			=> vec![To(p.clone())],
			To(ref p)				=> vec![UpFrom(p.clone())],
			From(ref p)				=> vec![UpTo(p.clone())],
			Full					=> vec![], // Or vec![Empty]?
		}
	}

	/// Returns the intersection of all of the given intervals.
	pub fn intersect_all<I>(intervals: I) -> Self
		where I: IntoIterator<Item=Self>
	{
		intervals
			.into_iter()
			.fold(Full, |acc, i| acc.intersect(&i))
	}

	/// Returns the union of all of the given intervals.
	pub fn union_all<I>(intervals: I) -> Vec<Self>
		where I: IntoIterator<Item=Self>
	{
		// TODO: Consider using selection/disjunction map. It may be faster.
		let mut it = intervals.into_iter().filter(|i| !i.is_empty());

		// Get first interval.
		if let Some(start) = it.next() {
			// Fold over remaining intervals.
			it.fold(vec![start], |mut prev, next_interval| {
				// Early exit for full interval.
				if next_interval == Full {
					return vec![Full];
				}

				let mut append = true;
				for item in prev.iter_mut() {
					if !item.intersect(&next_interval).is_empty() {
						*item = item.enclose(&next_interval);
						append = false;
						break;
					}
				}
				if append {prev.push(next_interval);}
				prev
			})
		} else {
			Vec::new()
		}
	}
}


impl<T> RawInterval<T>
	where
		T: PartialOrd + Ord + Clone + LeftIterable,
		Interval<T>: Normalize
{
	/// Removes and returns the lowest point in the interval.
	pub fn pop_lower(&mut self) -> Option<T> {
		let left = match *self {
			Empty				=> None,
			Point(ref p)		=> Some(p.clone()),
			Closed(ref l, _)	=> Some(l.clone()),
			LeftOpen(ref l, _)	=> l.succ(),
			RightOpen(ref l, _)	=> Some(l.clone()),
			Open(ref l, _)		=> l.succ(),
			To(_)				=> Some(T::minimum()),
			UpTo(_)				=> Some(T::minimum()),
			From(ref l)			=> Some(l.clone()),
			UpFrom(ref l)		=> l.succ(),
			Full				=> Some(T::minimum()),
		};
		
		if let Some(nl) = left {
			*self = match *self { 
				Point(_) => Empty,
				Empty	 => unreachable!(),
				_		 => RawInterval::new(
					Some(Exclude(nl.clone())), 
					self.upper_bound()
				),
			};
			Some(nl)
		} else {
			*self = Empty;
			None
		}
	}
}

impl<T> RawInterval<T>
	where
		T: PartialOrd + Ord + Clone + RightIterable,
		Interval<T>: Normalize
{
	/// Removes and returns the greatest point in the interval.
	pub fn pop_upper(&mut self) -> Option<T> {
		let right = match *self {
			Empty				=> None,
			Point(ref p)		=> Some(p.clone()),
			Closed(_, ref r)	=> Some(r.clone()),
			LeftOpen(_, ref r)	=> Some(r.clone()),
			RightOpen(_, ref r) => r.pred(),
			Open(_, ref r)		=> r.pred(),
			To(ref r)			=> Some(r.clone()),
			UpTo(ref r)			=> r.pred(),
			From(_)				=> Some(T::maximum()),
			UpFrom(_)			=> Some(T::maximum()),
			Full				=> Some(T::maximum()),
		};
		
		if let Some(nr) = right {
			*self = match *self { 
				Point(_) => Empty,
				Empty    => unreachable!(),
				_		 => RawInterval::new(
					self.lower_bound(),
					Some(Exclude(nr.clone()))
				),
			};
			Some(nr)
		} else {
			*self = Empty;
			None
		}
	}
}



////////////////////////////////////////////////////////////////////////////////
// Basic Trait impls
////////////////////////////////////////////////////////////////////////////////
// Display using interval notation.
impl<T> fmt::Display for RawInterval<T>
	where T: PartialOrd + Ord + Clone + fmt::Display
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Empty					=> write!(f, "Ø"),
			Point(ref p)			=> write!(f, "{}", p),
			Open(ref l, ref r)		=> write!(f, "({}, {})", l, r),
			LeftOpen(ref l, ref r)	=> write!(f, "({}, {}]", l, r),
			RightOpen(ref l, ref r) => write!(f, "[{}, {})", l, r),
			Closed(ref l, ref r)	=> write!(f, "[{}, {}]", l, r),
			UpTo(ref p)				=> write!(f, "(-∞, {})", p),
			UpFrom(ref p)			=> write!(f, "({}, ∞)", p),
			To(ref p)				=> write!(f, "(-∞, {})", p),
			From(ref p)				=> write!(f, "({}, ∞)", p),
			Full					=> write!(f, "(-∞, ∞)"),
		}
	}
}



////////////////////////////////////////////////////////////////////////////////
// Iterator impls for Interval
////////////////////////////////////////////////////////////////////////////////
impl<T> Iterator for Interval<T>
	where T: PartialOrd + Ord + Clone + LeftIterable, Interval<T>: Normalize
{
	type Item = T;
	fn next(&mut self) -> Option<T> {
		let next = self.0.next();
		self.normalize();
		next
	}
}

impl<T> DoubleEndedIterator for Interval<T>
	where 
		T: PartialOrd + Ord + Clone + LeftIterable + RightIterable, 
		Interval<T>: Normalize
{
	fn next_back(&mut self) -> Option<T> {
		let next = self.0.next_back();
		self.normalize();
		next
	}
}

// TODO: ExactSizeIterator

////////////////////////////////////////////////////////////////////////////////
// Iterator impls for RawInterval
////////////////////////////////////////////////////////////////////////////////
impl<T> Iterator for RawInterval<T>
	where T: PartialOrd + Ord + Clone + LeftIterable
{
	type Item = T;
	fn next(&mut self) -> Option<T> {
		self.normalize();
		self.pop_lower()
	}
}

impl<T> DoubleEndedIterator for RawInterval<T>
	where T: PartialOrd + Ord + Clone + LeftIterable + RightIterable, 
		Interval<T>: Normalize
{
	fn next_back(&mut self) -> Option<T> {
		self.normalize();
		self.pop_upper()
	}
}



////////////////////////////////////////////////////////////////////////////////
// LeftIterable
////////////////////////////////////////////////////////////////////////////////
/// Provides methods needed to iterate over an interval from the left.
pub trait LeftIterable: Clone {
	/// Returns the next element after the given one.
	fn succ(&self) -> Option<Self>;

	/// Returns the minimum element. 
	fn minimum() -> Self;
}



////////////////////////////////////////////////////////////////////////////////
// RightIterable
////////////////////////////////////////////////////////////////////////////////
/// Provides methods needed to iterate over an interval from the right.
pub trait RightIterable: Clone {
	/// Returns the previous element before the given one.
	fn pred(&self) -> Option<Self>;

	/// Returns the maximum element.
	fn maximum() -> Self;
}



////////////////////////////////////////////////////////////////////////////////
// Normalize
////////////////////////////////////////////////////////////////////////////////
/// Provides normalization capabilities for an interval.
pub trait Normalize: Sized {
	/// The interval's point type.
	type Point: PartialOrd + Ord + Clone;

	/// Normalizes the interval.
	fn normalize(&mut self);

	/// Returns the normalized interval.
	fn normalized(self) -> Self where Self: Clone {
		let mut n = self.clone();
		n.normalize();
		n
	}
}

// Blanket implementation for all `Interval`s.
impl<T> Normalize for Interval<T> where T: PartialOrd + Ord + Clone {
	type Point = T;
    default fn normalize(&mut self) {/* Do nothing. */}
}

// Blanket implementation for all `Interval`s.
impl<T> Normalize for Interval<T>
	where T: PartialOrd + Ord + Clone + LeftIterable + RightIterable
{
    fn normalize(&mut self) {
    	let (min, max) = (T::minimum(), T::maximum());

    	let old = mem::replace(&mut self.0, Empty);
    	(*self).0 = match old {
			Open(l, r)		=> match (l.succ(), r.pred()) {
				(Some(l), Some(r)) => Closed(l, r),
				_				   => Empty,
			},
			LeftOpen(l, r)	=> l.succ().map_or(Empty, |l| Closed(l, r)),
			RightOpen(l, r) => r.pred().map_or(Empty, |r| Closed(l, r)),
			UpTo(r)			=> r.pred().map_or(Empty, |r| Closed(min, r)),
			UpFrom(l)		=> l.succ().map_or(Empty, |l| Closed(l, max)),
			To(p)			=> Closed(min, p),
			From(p)			=> Closed(p, max),
			Full			=> Closed(min, max),
			_				=> old,
    	}
    }
}


// Recursively matches 'expr = expr' statements until it finds the `succ =` and 
// `minimum =` statements.
macro_rules! left_iterable_impl {
	// Finished search, provide LeftIterable implementation.
	($t:ty ; found $min:expr ; found $succ:expr) => {
		impl LeftIterable for $t {
			fn succ(&self) -> Option<Self> {
				($succ)(self.clone())
			}
			fn minimum() -> Self {
				($min)
			}
		}
	};

	// We've found succ and now minimum. Put them in order and finish.
	($t:ty ; hold ; found $succ:expr ; minimum = $min:expr , $($rest:tt)*) => {
		left_iterable_impl!($t; found $min; found $succ);
	};

	// We've found minimum and now succ. Put them in order and finish.
	($t:ty ; found $min:expr ; hold ; succ = $succ:expr , $($rest:tt)* ) => {
		left_iterable_impl!($t; found $min; found $succ);
	};

	// We've found succ, but not minimum. Continue search.
	($t:ty ; hold ; found $succ:expr ; $i:ident = $e:expr , $($rest:tt)*) => {
		left_iterable_impl!($t; hold; found $succ; $($rest)*);
	};

	// We've found minimum, but not succ. Continue search.
	($t:ty ; found $min:expr ; hold ; $i:ident = $e:expr , $($rest:tt)* ) => {
		left_iterable_impl!($t; found $min; hold; $($rest)*);
	};

	// We've just found minimum, mark it found and continue search for succ.
	($t:ty ; minimum = $min:expr , $($rest:tt)*) => {
		left_iterable_impl!($t; found $min; hold; $($rest)*);
	};

	// We've just found succ, mark it found and continue search for minimum.
	($t:ty ; succ = $succ:expr , $($rest:tt)*) => {
		left_iterable_impl!($t; hold; $succ; $($rest)*);
	};

	// We've found nothing. Continue search.
	($t:ty ; $i:ident = $e:expr , $($rest:tt)*) => {
		left_iterable_impl!($t; $($rest)*);
	};
}


// Recursively matches 'expr = expr' statements until it finds the `pred =` and 
// `maximum =` statements.
macro_rules! right_iterable_impl {
	// Finished search, provide RightIterable implementation.
	($t:ty ; found $max:expr ; found $pred:expr) => {
		impl RightIterable for $t {
			fn pred(&self) -> Option<Self> {
				($pred)(self.clone())
			}
			fn maximum() -> Self {
				($max)
			}
		}
	};

	// We've found pred and now maximum. Put them in order and finish.
	($t:ty ; hold ; found $pred:expr ; maximum = $max:expr , $($rest:tt)*) => {
		right_iterable_impl!($t; found $max; found $pred);
	};

	// We've found maximum and now pred. Put them in order and finish.
	($t:ty ; found $max:expr ; hold ; pred = $pred:expr , $($rest:tt)* ) => {
		right_iterable_impl!($t; found $max; found $pred);
	};

	// We've found pred, but not maximum. Continue search.
	($t:ty ; hold ; found $pred:expr ; $i:ident = $e:expr , $($rest:tt)*) => {
		right_iterable_impl!($t; hold; found $pred; $($rest)*);
	};

	// We've found maximum, but not pred. Continue search.
	($t:ty ; found $max:expr ; hold ; $i:ident = $e:expr , $($rest:tt)* ) => {
		right_iterable_impl!($t; found $max; hold; $($rest)*);
	};

	// We've just found maximum, mark it found and continue search for pred.
	($t:ty ; maximum = $max:expr , $($rest:tt)*) => {
		right_iterable_impl!($t; found $max; hold; $($rest)*);
	};

	// We've just found pred, mark it found and continue search for maximum.
	($t:ty ; pred = $pred:expr , $($rest:tt)*) => {
		right_iterable_impl!($t; hold; $pred; $($rest)*);
	};

	// We've found nothing. Continue search.
	($t:ty ; $i:ident = $e:expr , $($rest:tt)*) => {
		right_iterable_impl!($t; $($rest)*);
	};

}



// Implements the LeftIterable and RightIterable traits.
macro_rules! iterable_impls {
	// For the given type, we pass a sequence of mapping tokens to each of the
	// trait implementing macros.
    ($t:ty ; $($rest:tt)*) => {
        // LeftIterable
        left_iterable_impl!($t; $($rest)*);
		
		// RightIterable        
        right_iterable_impl!($t; $($rest)*);
    }
}

// Implements basic normalization for a single builtin integer type.
macro_rules! std_integer_normalization_impls {
	// For each given type...
	($($t:ident),*) => {$(
		iterable_impls! { $t;
		    minimum = std::$t::MIN,
		    maximum = std::$t::MAX,
		    succ = |n| if n < std::$t::MAX {Some(n + 1)} else {None},
		    pred = |n| if n > std::$t::MIN {Some(n - 1)} else {None},
		}
	)*};
}


// Provide implementations of basic normalization for builtin integer types.
std_integer_normalization_impls![
	u8, u16, u32, u64, usize,
	i8, i16, i32, i64, isize
];




#[cfg(test)]
mod test {

	use interval::Bound;
	use super::*;

	#[test]
	fn raw_interval_new_1() {
		let l = Bound::Include(12);
		let r = Bound::Include(16);
		let int = RawInterval::new(Some(l), Some(r));
		
		assert_eq!(int.infimum(), Some(12));
		assert_eq!(int.supremum(), Some(16));
	}


	#[test]
	fn raw_interval_new_2() {
		let l = Bound::Include(12);
		let r = Bound::Include(16);
		let int = RawInterval::new(Some(r), Some(l));
		
		assert!(int.is_empty());
	}


	#[test]
	fn raw_interval_open() {
		let int = RawInterval::open(0, 2);
		
		assert_eq!(int.infimum(), Some(0));
		assert_eq!(int.supremum(), Some(2));
		assert_eq!(int.lower_bound(), Some(Bound::Exclude(0)));
		assert_eq!(int.upper_bound(), Some(Bound::Exclude(2)));
	}

	#[test]
	fn raw_interval_left_open() {
		let int = RawInterval::left_open(0, 2);
		
		assert_eq!(int.infimum(), Some(0));
		assert_eq!(int.supremum(), Some(2));
		assert_eq!(int.lower_bound(), Some(Bound::Exclude(0)));
		assert_eq!(int.upper_bound(), Some(Bound::Include(2)));
	}

	#[test]
	fn raw_interval_right_open() {
		let int = RawInterval::right_open(0, 2);
		
		assert_eq!(int.infimum(), Some(0));
		assert_eq!(int.supremum(), Some(2));
		assert_eq!(int.lower_bound(), Some(Bound::Include(0)));
		assert_eq!(int.upper_bound(), Some(Bound::Exclude(2)));
	}

	#[test]
	fn raw_interval_closed() {
		let int = RawInterval::closed(0, 2);
		
		assert_eq!(int.infimum(), Some(0));
		assert_eq!(int.supremum(), Some(2));
		assert_eq!(int.lower_bound(), Some(Bound::Include(0)));
		assert_eq!(int.upper_bound(), Some(Bound::Include(2)));
	}

	#[test]
	fn raw_interval_is_empty() {
		let a = RawInterval::closed(0, 2);
		let b = RawInterval::open(0, 0);
		
		assert!(!a.is_empty());
		assert!(b.is_empty());
	}

	#[test]
	fn raw_interval_lower_bound() {
		let int = RawInterval::open(0, 2);
		
		assert_eq!(int.lower_bound(), Some(Bound::Exclude(0)));
	}

	#[test]
	fn raw_interval_upper_bound() {
		let int = RawInterval::open(0, 2);
		
		assert_eq!(int.upper_bound(), Some(Bound::Exclude(2)));
	}

	#[test]
	fn raw_interval_infimum() {
		let int = RawInterval::open(0, 2);
		
		assert_eq!(int.infimum(), Some(0));
	}

	#[test]
	fn raw_interval_supremum() {
		let int = RawInterval::open(0, 2);
		
		assert_eq!(int.supremum(), Some(2));
	}

	#[test]
	fn raw_interval_contains() {
		let int = RawInterval::left_open(0, 2);
		
		assert!(!int.contains(&0));
		assert!(int.contains(&1));
		assert!(int.contains(&2));
		assert!(!int.contains(&3));
	}

	#[test]
	fn raw_interval_enclose() {
		let a = RawInterval::right_open(0, 2);
		let b = RawInterval::closed(1, 3);
		
		assert_eq!(a.enclose(&b), RawInterval::closed(0, 3));
	}

	#[test]
	fn raw_interval_intersect() {
		let a = RawInterval::right_open(0, 2);
		let b = RawInterval::closed(1, 3);
		
		assert_eq!(a.intersect(&b), RawInterval::right_open(1, 2));
	}

	#[test]
	fn raw_interval_union_1() {
		let a = RawInterval::right_open(0, 2);
		let b = RawInterval::closed(1, 3);
		
		assert_eq!(a.union(&b), vec![RawInterval::closed(0, 3)]);
	}

	#[test]
	fn raw_interval_union_2() {
		let a = RawInterval::right_open(0, 2);
		let b = RawInterval::closed(3, 4);
		
		assert_eq!(a.union(&b), vec![a, b]);
	}

	#[test]
	fn raw_interval_minus_1() {
		let a = RawInterval::right_open(0, 2);
		let b = RawInterval::closed(1, 3);
		
		assert_eq!(a.minus(&b), vec![RawInterval::right_open(0, 1)]);
	}

	#[test]
	fn raw_interval_minus_2() {
		let a = RawInterval::closed(0, 5);
		let b = RawInterval::closed(1, 3);
		
		assert_eq!(a.minus(&b), vec![
		    RawInterval::right_open(0, 1),
		    RawInterval::left_open(3, 5),
		]);
	}

	#[test]
	fn raw_interval_complement() {
		let int = RawInterval::right_open(0, 2);
		
		assert_eq!(int.complement(), vec![
		    RawInterval::new(None, Some(Bound::Exclude(0))),
		    RawInterval::new(Some(Bound::Include(2)), None),
		]);
	}

	#[test]
	fn raw_interval_intersect_all() {
		let ints = vec![
		    RawInterval::open(0, 2),
		    RawInterval::open(-1, 1),
		    RawInterval::open(0, 1),
		    RawInterval::closed(0, 1),
		    RawInterval::open(0, 1),
		];
		assert_eq!(RawInterval::intersect_all(ints), RawInterval::open(0, 1));
	}

	#[test]
	fn raw_interval_union_all() {
		let ints = vec![
		    RawInterval::open(0, 2),
		    RawInterval::open(-1, 1),
		    RawInterval::open(0, 1),
		    RawInterval::closed(0, 1),
		    RawInterval::open(0, 1),
		];
		assert_eq!(
			RawInterval::union_all(ints), 
			vec![RawInterval::open(-1, 2)]
		);
	}


	#[test]
	fn raw_interval_iterate_1() {
		let int = Interval::closed(8, 12);

		assert_eq!(int.collect::<Vec<_>>(), vec![8, 9, 10, 11, 12]);
	}

	#[test]
	fn raw_interval_iterate_2() {
		let int = Interval::closed(8, 12);

		assert_eq!(int.rev().collect::<Vec<_>>(), vec![12, 11, 10, 9, 8]);
	}

}