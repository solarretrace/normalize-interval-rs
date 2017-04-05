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
//! Provides an interval type for doing complex set selections.
//!
////////////////////////////////////////////////////////////////////////////////

// Module imports.
use bound::*;

use std::cmp::Ordering;
use std::fmt;
use std::convert::From;

// Local enum shortcuts.
use Bound::*;
use Interval::*;


////////////////////////////////////////////////////////////////////////////////
// Interval<T>
////////////////////////////////////////////////////////////////////////////////
/// A contiguous interval of the type T.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Interval<T> {
    /// An interval containing no points.
    Empty,
    /// An interval containing only the given point.
    Point(T),
    /// An interval containing all points between two given points, exclude
    /// them both.
    Open(T, T),
    /// An interval containing all points between two given points, include
    /// the greater of the two.
    LeftOpen(T, T),
    /// An interval containing all points between two given points, include
    /// the lesser of the two.
    RightOpen(T, T),
    /// An interval containing all points between two given points, include
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
    /// let l = Bound::Include(12);
    /// let r = Bound::Include(16);
    /// let int = Interval::new(Some(l), Some(r));
    /// 
    /// assert_eq!(int.infimum(), Some(12));
    /// assert_eq!(int.supremum(), Some(16));
    /// ```
    ///
    /// If the arguments are out of order, an empty interval will be returned:
    ///
    /// ```rust
    /// use interval::{Bound, Interval};
    ///
    /// let l = Bound::Include(12);
    /// let r = Bound::Include(16);
    /// let int = Interval::new(Some(r), Some(l));
    /// 
    /// assert!(int.is_empty());
    /// ```
    pub fn new(left: Option<Bound<T>>, right: Option<Bound<T>>) -> Self {
        match (left, right) {
            (Some(lb), Some(rb)) => match (lb, rb) {
                (Exclude(l), Exclude(r)) => Interval::open(l, r),
                (Exclude(l), Include(r)) => Interval::left_open(l, r),
                (Include(l), Exclude(r)) => Interval::right_open(l, r),
                (Include(l), Include(r)) => Interval::closed(l, r),
            },
            (None, Some(rb)) => match rb {
                Exclude(r) => UpTo(r),
                Include(r) => To(r),
            },
            (Some(lb), None) => match lb {
                Exclude(l) => UpFrom(l),
                Include(l) => From(l),
            },
            (None, None) => Full,
        }
    }
    
    /// Returns an empty interval.
    pub fn empty() -> Self {
        Empty
    }
    
    /// Constructs a new point interval for the given point.
    pub fn point(point: T) -> Self {
        Interval::Point(point)
    }
    
    /// Constructs a new open interval from the given points. If the right
    /// point is less than the left point, an empty interval will be returned.
    ///
    /// # Example
    ///
    /// ```rust
    /// use interval::{Bound, Interval};
    ///
    /// let int = Interval::open(0, 2);
    /// 
    /// assert_eq!(int.infimum(), Some(0));
    /// assert_eq!(int.supremum(), Some(2));
    /// assert_eq!(int.lower_bound(), Some(Bound::Exclude(0)));
    /// assert_eq!(int.upper_bound(), Some(Bound::Exclude(2)));
    /// ```
    pub fn open(left: T, right: T) -> Self {
        match T::cmp(&left, &right) {
            Ordering::Less => Open(left, right),
            _              => Empty,
        }
    }
    
    /// Constructs a new left-open interval from the given points. If the
    /// right bound point is less than the left bound point, an empty interval
    /// will be returned.
    ///
    /// # Example
    ///
    /// ```rust
    /// use interval::{Bound, Interval};
    ///
    /// let int = Interval::left_open(0, 2);
    /// 
    /// assert_eq!(int.infimum(), Some(0));
    /// assert_eq!(int.supremum(), Some(2));
    /// assert_eq!(int.lower_bound(), Some(Bound::Exclude(0)));
    /// assert_eq!(int.upper_bound(), Some(Bound::Include(2)));
    /// ```
    pub fn left_open(left: T, right: T) -> Self {
        match T::cmp(&left, &right) {
            Ordering::Less    => LeftOpen(left, right),
            Ordering::Equal   => Point(left),
            Ordering::Greater => Empty,
        }
    }
    
    /// Constructs a new right-open interval from the given points. If the
    /// right bound point is less than the left bound point, an empty interval
    /// will be returned.
    ///
    /// # Example
    ///
    /// ```rust
    /// use interval::{Bound, Interval};
    ///
    /// let int = Interval::right_open(0, 2);
    /// 
    /// assert_eq!(int.infimum(), Some(0));
    /// assert_eq!(int.supremum(), Some(2));
    /// assert_eq!(int.lower_bound(), Some(Bound::Include(0)));
    /// assert_eq!(int.upper_bound(), Some(Bound::Exclude(2)));
    /// ```
    pub fn right_open(left: T, right: T) -> Self {
        match T::cmp(&left, &right) {
            Ordering::Less    => RightOpen(left, right),
            Ordering::Equal   => Point(left),
            Ordering::Greater => Empty,
        }
    }
    
    /// Constructs a new closed interval from the given points. If the
    /// right bound point is less than the left bound point, an empty interval
    /// will be returned.
    ///
    /// # Example
    ///
    /// ```rust
    /// use interval::{Bound, Interval};
    ///
    /// let int = Interval::closed(0, 2);
    /// 
    /// assert_eq!(int.infimum(), Some(0));
    /// assert_eq!(int.supremum(), Some(2));
    /// assert_eq!(int.lower_bound(), Some(Bound::Include(0)));
    /// assert_eq!(int.upper_bound(), Some(Bound::Include(2)));
    /// ```
    pub fn closed(left: T, right: T) -> Self {
        match T::cmp(&left, &right) {
            Ordering::Less    => Closed(left, right),
            Ordering::Equal   => Point(left),
            Ordering::Greater => Empty,
        }
    }
    
    /// Returns whether the interval excludes any of its boundary points.
    ///
    /// # Example
    ///
    /// ```rust
    /// use interval::Interval;
    ///
    /// let a = Interval::closed(0, 2);
    /// let b = Interval::open(0, 2);
    /// 
    /// assert!(!a.is_open());
    /// assert!(b.is_open());
    /// ```
    pub fn is_open(&self) -> bool {
        match *self {
            Point(_)     => false,
            Closed(_, _) => false,
            _            => true,
        }
    }
    
    /// Returns whether the interval includes only one of its boundary points.
    ///
    /// # Example
    ///
    /// ```rust
    /// use interval::Interval;
    ///
    /// let a = Interval::closed(0, 2);
    /// let b = Interval::left_open(0, 2);
    /// 
    /// assert!(!a.is_half_open());
    /// assert!(b.is_half_open());
    /// ```
    pub fn is_half_open(&self) -> bool {
        match *self {
            LeftOpen(_, _)  => true,
            RightOpen(_, _) => true,
            To(_)           => true,
            From(_)         => true,
            _               => false,
        }
    }
    
    /// Returns whether the interval includes all of its boundary points.
    ///
    /// # Example
    ///
    /// ```rust
    /// use interval::Interval;
    ///
    /// let a = Interval::closed(0, 2);
    /// let b = Interval::open(0, 2);
    /// 
    /// assert!(a.is_closed());
    /// assert!(!b.is_closed());
    /// ```
    pub fn is_closed(&self) -> bool {
        match *self {
            Empty        => true,
            Point(_)     => true,
            Closed(_, _) => true,
            Full         => true,
            _            => false,
        }
    }
    
    /// Returns whether the interval is empty (i.e., contains no points.)
    ///
    /// # Example
    ///
    /// ```rust
    /// use interval::Interval;
    ///
    /// let a = Interval::closed(0, 2);
    /// let b = Interval::open(0, 0);
    /// 
    /// assert!(!a.is_empty());
    /// assert!(b.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        match *self {
            Empty => true,
            _     => false,
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
    /// let a = Interval::left_open(0, 0);
    /// let b = Interval::open(0, 0);
    /// 
    /// assert!(a.is_degenerate());
    /// assert!(!b.is_degenerate());
    /// ```
    pub fn is_degenerate(&self) -> bool {
        match *self {
            Point(_) => true,
            _        => false,
        }
    }
    
    /// Returns whether the interval is full (i.e., contains all points.)
    ///
    /// # Example
    ///
    /// ```rust
    /// use interval::Interval;
    ///
    /// let a = Interval::<u32>::new(None, None);
    /// let b = Interval::open(-100, 100);
    /// 
    /// assert!(a.is_full());
    /// assert!(!b.is_full());
    /// ```
    pub fn is_full(&self) -> bool {
        match *self {
            Full => true,
            _    => false,
        }
    }
    
    /// Returns whether the interval is bounded on both sides.
    ///
    /// # Example
    ///
    /// ```rust
    /// use interval::{Bound, Interval};
    ///
    /// let a = Interval::new(Some(Bound::Include(0)), None);
    /// let b = Interval::open(0, 2);
    /// 
    /// assert!(!a.is_bounded());
    /// assert!(b.is_bounded());
    /// ```
    pub fn is_bounded(&self) -> bool {
        match *self {
            UpTo(_)   => false,
            UpFrom(_) => false,
            To(_)     => false,
            From(_)   => false,
            Full      => false,
            _         => true,
        }
    }
    
    /// Returns the lower bound of the interval.
    ///
    /// # Example
    ///
    /// ```rust
    /// use interval::{Bound, Interval};
    ///
    /// let int = Interval::open(0, 2);
    /// 
    /// assert_eq!(int.lower_bound(), Some(Bound::Exclude(0)));
    /// ```
    pub fn lower_bound(&self) -> Option<Bound<T>> {
        match *self {
            Empty               => None,
            Point(ref p)        => Some(Include(p.clone())),
            Open(ref l, _)      => Some(Exclude(l.clone())),
            LeftOpen(ref l, _)  => Some(Exclude(l.clone())),
            RightOpen(ref l, _) => Some(Include(l.clone())),
            Closed(ref l, _)    => Some(Include(l.clone())),
            UpTo(_)             => None,
            UpFrom(ref p)       => Some(Exclude(p.clone())),
            To(_)               => None,
            From(ref p)         => Some(Include(p.clone())),
            Full                => None,
        }
    }
    
    /// Returns the upper bound of the interval.
    ///
    /// # Example
    ///
    /// ```rust
    /// use interval::{Bound, Interval};
    ///
    /// let int = Interval::open(0, 2);
    /// 
    /// assert_eq!(int.upper_bound(), Some(Bound::Exclude(2)));
    /// ```
    pub fn upper_bound(&self) -> Option<Bound<T>> {
        match *self {
            Empty               => None,
            Point(ref p)        => Some(Include(p.clone())),
            Open(_, ref r)      => Some(Exclude(r.clone())),
            LeftOpen(_, ref r)  => Some(Include(r.clone())),
            RightOpen(_, ref r) => Some(Exclude(r.clone())),
            Closed(_, ref r)    => Some(Include(r.clone())),
            UpTo(ref p)         => Some(Exclude(p.clone())),
            UpFrom(_)           => None,
            To(ref p)           => Some(Include(p.clone())),
            From(_)             => None,
            Full                => None,
        }
    }
    
    /// Returns the greatest lower bound of the interval.
    ///
    /// # Example
    ///
    /// ```rust
    /// use interval::Interval;
    ///
    /// let int = Interval::open(0, 2);
    /// 
    /// assert_eq!(int.infimum(), Some(0));
    /// ```
    pub fn infimum(&self) -> Option<T> {
        self.lower_bound().map(|b| b.as_ref().clone())
    }
    
    /// Returns the least upper bound of the interval.
    ///
    /// # Example
    ///
    /// ```rust
    /// use interval::Interval;
    ///
    /// let int = Interval::open(0, 2);
    /// 
    /// assert_eq!(int.supremum(), Some(2));
    /// ```
    pub fn supremum(&self) -> Option<T> {
        self.upper_bound().map(|b| b.as_ref().clone())
    }
    
    /// Returns whether the interval contains the given point.
    ///
    /// # Example
    ///
    /// ```rust
    /// use interval::Interval;
    ///
    /// let int = Interval::left_open(0, 2);
    /// 
    /// assert!(!int.contains(&0));
    /// assert!(int.contains(&1));
    /// assert!(int.contains(&2));
    /// assert!(!int.contains(&3));
    /// ```
    pub fn contains(&self, point: &T) -> bool {
        match *self {
            Empty                   => false,
            Point(ref p)            => point == p,
            Open(ref l, ref r)      => point > l && point < r,
            LeftOpen(ref l, ref r)  => point > l && point <= r,
            RightOpen(ref l, ref r) => point >= l && point < r,
            Closed(ref l, ref r)    => point >= l && point <= r,
            UpTo(ref p)             => point < p,
            UpFrom(ref p)           => point > p,
            To(ref p)               => point <= p,
            From(ref p)             => point >= p,
            Full                    => true,
        }
    }
    
    /// Returns the smallest interval that contains all of the points contained
    /// within this interval and the given interval.
    ///
    /// # Example
    ///
    /// ```rust
    /// use interval::Interval;
    ///
    /// let a = Interval::right_open(0, 2);
    /// let b = Interval::closed(1, 3);
    /// 
    /// assert_eq!(a.enclose(&b), Interval::closed(0, 3));
    /// ```
    pub fn enclose(&self, other: &Self) -> Self {
        Interval::new(
            self.lower_bound().least_union(&other.lower_bound()), 
            self.upper_bound().greatest_union(&other.upper_bound()))
    }
    
    /// Returns the largest interval whose points are all contained
    /// entirely within this interval and the given interval.
    ///
    /// # Example
    ///
    /// ```rust
    /// use interval::Interval;
    ///
    /// let a = Interval::right_open(0, 2);
    /// let b = Interval::closed(1, 3);
    /// 
    /// assert_eq!(a.intersect(&b), Interval::right_open(1, 2));
    /// ```
    pub fn intersect(&self, other: &Self) -> Self {
        Interval::new(
            self.lower_bound().greatest_intersect(&other.lower_bound()), 
            self.upper_bound().least_intersect(&other.upper_bound()))
    }
    
    /// Returns a `Vec` of `Interval`s containing all of the points contained
    /// within this interval and the given interval.
    ///
    /// # Example
    ///
    /// ```rust
    /// use interval::Interval;
    ///
    /// let a = Interval::right_open(0, 2);
    /// let b = Interval::closed(1, 3);
    /// 
    /// assert_eq!(a.union(&b), vec![Interval::closed(0, 3)]);
    /// ```
    ///
    /// Disjoint intervals will remain seperate:
    ///
    /// ```rust
    /// use interval::Interval;
    ///
    /// let a = Interval::right_open(0, 2);
    /// let b = Interval::closed(3, 4);
    /// 
    /// assert_eq!(a.union(&b), vec![a, b]);
    /// ```
    pub fn union(&self, other: &Self) -> Vec<Self> {
        if !self.intersect(other).is_empty() {
            vec![self.enclose(other)]
        } else {
            vec![self.clone(), other.clone()]
        }
    }
    
    /// Returns a `Vec` of `Interval`s containing all of the points contained
    /// within this interval that are not in the given interval.
    ///
    /// # Example
    ///
    /// ```rust
    /// use interval::Interval;
    ///
    /// let a = Interval::right_open(0, 2);
    /// let b = Interval::closed(1, 3);
    /// 
    /// assert_eq!(a.minus(&b), vec![Interval::right_open(0, 1)]);
    /// ```
    ///
    /// Disjoint intervals will remain seperate:
    ///
    /// ```rust
    /// use interval::Interval;
    ///
    /// let a = Interval::closed(0, 5);
    /// let b = Interval::closed(1, 3);
    /// 
    /// assert_eq!(a.minus(&b), vec![
    ///     Interval::right_open(0, 1),
    ///     Interval::left_open(3, 5),
    /// ]);
    /// ```
    pub fn minus(&self, other: &Self) -> Vec<Self> {
        other.complement()
            .into_iter()
            .filter_map(|i| self.intersect(&i).into_non_empty())
            .collect()
    }
    
    /// Returns a `Vec` of `Interval`s containing all of the points not in the
    /// interval.
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
        match *self {
            Empty                   => vec![Full],
            Point(ref p)            => vec![UpTo(p.clone()), UpFrom(p.clone())],
            Open(ref l, ref r)      => vec![To(l.clone()), From(r.clone())],
            LeftOpen(ref l, ref r)  => vec![To(l.clone()), From(r.clone())],
            RightOpen(ref l, ref r) => vec![UpTo(l.clone()), From(r.clone())],
            Closed(ref l, ref r)    => vec![UpTo(l.clone()), UpFrom(r.clone())],
            UpTo(ref p)             => vec![From(p.clone())],
            UpFrom(ref p)           => vec![To(p.clone())],
            To(ref p)               => vec![UpFrom(p.clone())],
            From(ref p)             => vec![UpTo(p.clone())],
            Full                    => vec![], // Or vec![Empty]?
        }
    }

    /// Returns the smallest closed interval containing all of the points in 
    /// this interval.
    ///
    /// # Example
    ///
    /// ```rust
    /// use interval::Interval;
    ///
    /// let int = Interval::right_open(0, 2);
    /// 
    /// assert_eq!(int.closure(), Interval::closed(0, 2));
    /// ```
    ///
    /// 
    ///
    /// ```rust
    /// use interval::Interval;
    ///
    /// let int = Interval::right_open(0, 2);
    /// 
    /// assert_eq!(int.closure(), Interval::closed(0, 2));
    /// ```
    pub fn closure(&self) -> Self {
        match self {
            &Open(ref l, ref r)      => Closed(l.clone(), r.clone()),
            &LeftOpen(ref l, ref r)  => Closed(l.clone(), r.clone()),
            &RightOpen(ref l, ref r) => Closed(l.clone(), r.clone()),
            &UpTo(_)                 => Full,
            &UpFrom(_)               => Full,
            &To(_)                   => Full,
            &From(_)                 => Full,
            _                        => self.clone(),
        }
    }

    /// Returns the partitions the interval formed by the given point.
    ///
    /// # Example
    ///
    /// ```rust
    /// use interval::Interval;
    ///
    /// let int = Interval::right_open(0, 2);
    /// 
    /// assert_eq!(int.partition_at(&1), Some((
    ///     Interval::right_open(0, 1),
    ///     Interval::point(1),
    ///     Interval::open(1, 2)
    /// )));
    /// ```
    pub fn partition_at(&self, point: &T) -> Option<(Self, Self, Self)> {
        if self.contains(point) {
            Some((
                Interval::new(
                    self.lower_bound(),
                    Some(Exclude(point.clone()))),
                Point(point.clone()),
                Interval::new(
                    Some(Exclude(point.clone())),
                    self.upper_bound())
            ))
        } else {
            None
        }
    }

    /// Converts the interval into an `Option`, returning `None` if it is empty.
    ///
    /// # Example
    ///
    /// ```rust
    /// use interval::Interval;
    ///
    /// assert!(Interval::open(0, 0).into_non_empty().is_none());
    ///
    /// let int = Interval::open(0, 1);
    /// assert_eq!(int.into_non_empty(), Some(int));
    /// ```
    pub fn into_non_empty(self) -> Option<Self> {
        if self.is_empty() {
            None
        } else {
            Some(self)
        }
    }


    /// Returns the intersection of all of the given intervals.
    ///
    /// # Example
    ///
    /// ```rust
    /// use interval::Interval;
    ///
    /// let ints = vec![
    ///     Interval::open(0, 2),
    ///     Interval::open(-1, 1),
    ///     Interval::open(0, 1),
    ///     Interval::closed(0, 1),
    ///     Interval::open(0, 1),
    /// ];
    ///
    /// assert_eq!(Interval::intersect_all(ints), Interval::open(0, 1));
    /// ```
    pub fn intersect_all<I>(intervals: I) -> Self
        where I: IntoIterator<Item=Self>
    {
        intervals
            .into_iter()
            .fold(Full, |acc, i| acc.intersect(&i))
    }

    /// Returns the union of all of the given intervals.
    ///
    /// # Example
    ///
    /// ```rust
    /// use interval::Interval;
    ///
    /// let ints = vec![
    ///     Interval::open(0, 2),
    ///     Interval::open(-1, 1),
    ///     Interval::open(0, 1),
    ///     Interval::closed(0, 1),
    ///     Interval::open(0, 1),
    /// ];
    ///
    /// assert_eq!(Interval::union_all(ints), vec![Interval::open(-1, 2)]);
    /// ```
    pub fn union_all<I>(intervals: I) -> Vec<Self>
        where I: IntoIterator<Item=Self>
    {
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

// Display using interval notation.
impl<T> fmt::Display for Interval<T> 
    where T: fmt::Display + PartialOrd + Ord + Clone 
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Empty                   => write!(f, "Ø"),
            Point(ref p)            => write!(f, "{}", p),
            Open(ref l, ref r)      => write!(f, "({}, {})", l, r),
            LeftOpen(ref l, ref r)  => write!(f, "({}, {}]", l, r),
            RightOpen(ref l, ref r) => write!(f, "[{}, {})", l, r),
            Closed(ref l, ref r)    => write!(f, "[{}, {}]", l, r),
            UpTo(ref p)             => write!(f, "(-∞, {})", p),
            UpFrom(ref p)           => write!(f, "({}, ∞)", p),
            To(ref p)               => write!(f, "(-∞, {})", p),
            From(ref p)             => write!(f, "({}, ∞)", p),
            Full                    => write!(f, "(-∞, ∞)"),
        }
    }
}

// Interval-from-Point conversion.
impl<T> From<T> for Interval<T> where T: PartialOrd + Ord + Clone {
    #[inline]
    fn from(t: T) -> Self {
        Interval::point(t)
    }
}
