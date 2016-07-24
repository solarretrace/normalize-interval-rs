// The MIT License (MIT)
// 
// Copyright (c) 2016 Skylor R. Schermer
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
//! Provides a basic bounded interval type for doing complex set selections.
//!
////////////////////////////////////////////////////////////////////////////////
#[warn(missing_docs)]
mod bound;
#[cfg(test)]
mod tests;

// Re-exports.
pub use bound::Bound;

// Module imports.
use std::ops::{Add, Sub};
use std::default::Default;
use std::mem;
use std::fmt;


////////////////////////////////////////////////////////////////////////////////
// Interval<T>
////////////////////////////////////////////////////////////////////////////////
/// A contiguous interval of the type T, which may include or exclude either 
/// boundary.
#[derive(Debug, PartialEq, Eq, Hash, Default, Clone, Copy)]
pub struct Interval<T> {
    /// The start of the interval.
    start: Bound<T>,
    /// The end of the interval.
    end: Bound<T>
}

impl<T> Interval<T> where T: PartialOrd + PartialEq + Clone  {
    /// Creates a new interval from the given boundaries.
    ///
    /// # Example
    ///
    /// ```rust
    /// use interval::{Bound, Interval};
    ///
    /// let l = Bound::Included(12);
    /// let r = Bound::Included(16);
    /// let int = Interval::new(l, Some(r));
    /// 
    /// assert_eq!(int.left_point(), 12);
    /// assert_eq!(int.right_point(), 16);
    /// ```
    ///
    /// If the arguments are out of order, they will be swapped:
    ///
    /// ```rust
    /// use interval::{Bound, Interval};
    ///
    /// let l = Bound::Included(12);
    /// let r = Bound::Included(16);
    /// let int = Interval::new(r, Some(l));
    /// 
    /// assert_eq!(int.left_point(), 12);
    /// assert_eq!(int.right_point(), 16);
    /// ```
    #[inline]
    pub fn new(start: Bound<T>, end: Option<Bound<T>>) -> Self {
        if let Some(end_bound) = end {
            Interval {
                start: start.union_or_least(&end_bound), 
                end: start.union_or_greatest(&end_bound)
            }
        } else {
            Interval {start: start.clone(), end: start}
        }
    }

    /// Creates a new open interval from the given points.
    ///
    /// # Example
    ///
    /// ```rust
    /// use interval::Interval;
    ///
    /// let int = Interval::open(0, 2);
    /// 
    /// assert_eq!(int.left_point(), 0);
    /// assert!(!int.left_bound().is_closed());
    /// assert_eq!(int.right_point(), 2);
    /// assert!(!int.right_bound().is_closed());
    /// ```
    #[inline]
    pub fn open(start: T, end: T) -> Self {
        Interval::new(
            Bound::Excluded(start),
            Some(Bound::Excluded(end))
        )
    }

    /// Creates a new closed interval from the given points.
    ///
    /// # Example
    ///
    /// ```rust
    /// use interval::Interval;
    ///
    /// let int = Interval::closed(0, 2);
    /// 
    /// assert_eq!(int.left_point(), 0);
    /// assert!(int.left_bound().is_closed());
    /// assert_eq!(int.right_point(), 2);
    /// assert!(int.right_bound().is_closed());
    /// ```
    #[inline]
    pub fn closed(start: T, end: T) -> Self {
        Interval::new(
            Bound::Included(start),
            Some(Bound::Included(end))
        )
    }

    /// Creates a new left-open interval from the given points.
    ///
    /// # Example
    ///
    /// ```rust
    /// use interval::Interval;
    ///
    /// let int = Interval::left_open(0, 2);
    /// 
    /// assert_eq!(int.left_point(), 0);
    /// assert!(!int.left_bound().is_closed());
    /// assert_eq!(int.right_point(), 2);
    /// assert!(int.right_bound().is_closed());
    /// ```
    #[inline]
    pub fn left_open(start: T, end: T) -> Self {
        Interval::new(
            Bound::Excluded(start),
            Some(Bound::Included(end))
        )
    }

    /// Creates a new right-open interval from the given points.
    ///
    /// # Example
    ///
    /// ```rust
    /// use interval::Interval;
    ///
    /// let int = Interval::right_open(0, 2);
    /// 
    /// assert_eq!(int.left_point(), 0);
    /// assert!(int.left_bound().is_closed());
    /// assert_eq!(int.right_point(), 2);
    /// assert!(!int.right_bound().is_closed());
    /// ```
    #[inline]
    pub fn right_open(start: T, end: T) -> Self {
        Interval::new(
            Bound::Included(start),
            Some(Bound::Excluded(end))
        )
    }

    /// Returns the leftmost (least) boundary point of the interval. Note that 
    /// this point may not be in the interval if the interval is left-open.
    ///
    /// # Example
    ///
    /// ```rust
    /// use interval::Interval;
    ///
    /// let int = Interval::open(0, 2);
    /// 
    /// assert_eq!(int.left_point(), 0);
    /// ```
    #[inline]
    pub fn left_point(&self) -> T {
        self.start.as_ref().clone()
    }

    /// Returns the rightmost (greatest) boundary point of the interval. Note 
    /// that this point may not be in the interval if the interval is 
    /// right-open.
    ///
    /// # Example
    ///
    /// ```rust
    /// use interval::Interval;
    ///
    /// let int = Interval::open(0, 2);
    /// 
    /// assert_eq!(int.right_point(), 2);
    /// ```
    #[inline]
    pub fn right_point(&self) -> T {
        self.end.as_ref().clone()
    }

    /// Returns the left (least) boundary of the interval.
    ///
    /// # Example
    ///
    /// ```rust
    /// use interval::{Interval, Bound};
    ///
    /// let int = Interval::open(0, 2);
    /// 
    /// assert_eq!(int.left_bound(), Bound::Excluded(0));
    /// ```
    #[inline]
    pub fn left_bound(&self) -> Bound<T> {
        self.start.clone()
    }

    /// Returns the right (greatest) boundary of the interval.
    ///
    /// # Example
    ///
    /// ```rust
    /// use interval::{Interval, Bound};
    ///
    /// let int = Interval::open(0, 2);
    /// 
    /// assert_eq!(int.right_bound(), Bound::Excluded(2));
    /// ```
    #[inline]
    pub fn right_bound(&self) -> Bound<T> {
        self.end.clone()
    }

    /// Returns whether the interval contains any points.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use interval::{Interval, Bound};
    ///
    /// let int = Interval::right_open(0, 2);
    /// assert!(!int.is_empty());
    /// ```
    ///
    /// An open interval with two of the same points is empty:
    ///
    /// ```rust
    /// # use interval::{Interval, Bound};
    /// let int = Interval::open(0, 0);
    /// assert!(int.is_empty());
    /// ```
    ///
    /// A half-open interval with two of the same points is not:
    ///
    /// ```rust
    /// # use interval::{Interval, Bound};
    /// let int = Interval::left_open(0, 0);
    /// assert!(!int.is_empty());
    /// ```
    ///
    /// A single-point interval is empty only if that point is excluded:
    ///
    /// ```rust
    /// # use interval::{Interval, Bound};
    /// let int_a = Interval::new(Bound::Excluded(0), None);
    /// let int_b = Interval::new(Bound::Included(0), None);
    /// assert!(int_a.is_empty());
    /// assert!(!int_b.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.left_bound() == self.right_bound() && self.left_bound().is_open()
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
    #[inline]
    pub fn into_non_empty(self) -> Option<Self> {
        if self.is_empty() {
            None
        } else {
            Some(self)
        }
    }

    /// Returns whether the given point is included in the interval.
    ///
    /// # Example
    ///
    /// ```rust
    /// use interval::Interval;
    ///
    /// let int = Interval::right_open(0.0, 2.0);
    /// assert!(int.contains(&0.0));
    /// assert!(int.contains(&1.0));
    /// assert!(!int.contains(&2.0));
    /// ```
    #[inline]
    pub fn contains(&self, point: &T) -> bool {
        *point > self.left_point() && *point < self.right_point()
            || *point == self.left_point() && self.left_bound().is_closed()
            || *point == self.right_point() && self.right_bound().is_closed()
    }

    /// Returns the set intersection of the interval with the given interval,
    /// or `None` if the intervals do not overlap.
    ///
    /// # Example
    ///
    /// ```rust
    /// use interval::Interval;
    ///
    /// let a = Interval::right_open(0.0, 2.0);
    /// let b = Interval::closed(1.0, 3.0);
    /// 
    /// assert_eq!(a.intersect(&b), Some(Interval::right_open(1.0, 2.0)));
    /// ```
    #[inline]
    pub fn intersect(&self, other: &Self) -> Option<Self> {
        // Check if either one is empty.
        if self.is_empty() || other.is_empty() {
            return None;
        }

        // Choose orientation for intervals.
        let (a, b) = if self.left_point() <= other.left_point() {
            (self, other)
        } else {
            (other, self)
        };
        
        if a.right_point() < b.left_point() ||
            (a.right_point() == b.left_point() &&
            (a.right_bound().is_open() || 
            b.left_bound().is_open()))
        {
            // Not overlapping, or overlapping at one non-closed point.
            None
        } else {
            // Overlapping.
            Some(Interval::new(
                 a.left_bound().intersect_or_greatest(&b.left_bound()),
                 Some(a.right_bound().intersect_or_least(&b.right_bound()))
            ))
        }
    }

    /// Returns the set union of the interval with the given interval. Note that
    /// since an interval requires contiguous points, a union of disjoint 
    /// intervals will fail to produce an interval and `None` will be returned.
    ///
    /// # Example
    ///
    /// ```rust
    /// use interval::Interval;
    ///
    /// let a = Interval::left_open(0.0, 2.0);
    /// let b = Interval::closed(1.0, 3.0);
    /// 
    /// assert_eq!(a.union(&b), Some(Interval::left_open(0.0, 3.0)));
    /// ```
    #[inline]
    pub fn union(&self, other: &Self) -> Option<Self> {
        // Check for empty unions.
        if self.is_empty() && other.is_empty() {
            return None;
        } else if self.is_empty() {
            return Some(other.clone())
        } else if other.is_empty() {
            return Some(self.clone())
        }

        // Choose orientation for intervals.
        let (a, b) = if self.left_point() <= other.left_point() {
            (self, other)
        } else {
            (other, self)
        };
        
        if a.right_point() < b.left_point() ||
            (a.right_point() == b.left_point() &&
            a.right_bound().is_open() && 
            b.left_bound().is_open())
        {
            // Not overlapping, or overlapping at one open point.
            None
        } else {
            // Overlapping.
            Some(Interval {
                start: a.left_bound().union_or_least(&b.left_bound()),
                end: a.right_bound().union_or_greatest(&b.right_bound()),
            })
        }
    }

    /// Returns the smallest interval containing all of the points in the given
    /// intervals, or `None` if all the intervals are empty.
    ///
    /// # Example
    ///
    /// ```rust
    /// use interval::Interval;
    ///
    /// let res = Interval::enclose(vec![
    ///     Interval::open(1.0, 2.0),
    ///     Interval::open(2.0, 3.0),
    ///     Interval::open(2.5, 3.5),
    ///     Interval::closed(3.0, 3.0),
    ///     Interval::open(0.0, 1.5),
    ///     Interval::open(6.0, 6.0),
    /// ].into_iter());
    /// 
    /// assert_eq!(
    ///     res, 
    ///     Some(Interval::open(0.0, 3.5))
    /// );
    #[inline]
    pub fn enclose<I>(intervals: I) -> Option<Interval<T>>
        where I: IntoIterator<Item=Interval<T>>
    {
        // Find first non-empty interval.
        let mut ints = intervals.into_iter().skip_while(|i| i.is_empty());
        let first_non_empty = ints.next();

        if let Some(first) = first_non_empty {
            Some(ints.fold(first, |acc, next_interval| {
                if next_interval.is_empty() {
                    acc
                } else {
                    Interval::new(
                        acc.left_bound()
                            .union_or_least(&next_interval.left_bound()), 
                        Some(acc.right_bound()
                            .union_or_greatest(&next_interval.right_bound()))
                    )
                }
            }))
        } else {
            None
        }
    }

    /// Reduces a collection of intervals to a smaller set by removing redundant
    /// intervals by unioning them together.
    ///
    /// # Example
    ///
    /// ```rust
    /// use interval::Interval;
    ///
    /// let ints = Interval::normalize(vec![
    ///     Interval::open(1.0, 2.0),
    ///     Interval::open(2.0, 3.0),
    ///     Interval::open(2.5, 3.5),
    ///     Interval::closed(3.0, 3.0),
    ///     Interval::open(0.0, 1.5),
    ///     Interval::open(6.0, 6.0),
    /// ].into_iter());
    /// 
    /// assert_eq!(
    ///     &ints[..], 
    ///     &[Interval::open(0.0, 2.0), Interval::open(2.0, 3.5)]
    /// );
    /// ```
    #[inline]
    pub fn normalize<I>(intervals: I) -> Vec<Interval<T>> 
        where I: IntoIterator<Item=Interval<T>>
    {   
        // Remove empty intervals.
        let mut it = intervals
            .into_iter()
            .filter(|interval| !interval.is_empty());

        // Get first interval.
        if let Some(start) = it.next() {
            // Fold over remaining intervals.
            it.fold(vec![start], |mut prev, next_interval| {
                let mut append = true;
                for item in prev.iter_mut() {
                    if let Some(val) = item.union(&next_interval) {
                        // Union with next_interval succeeded.
                        mem::replace(item, val);
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

    /// Returns the width of the interval.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use interval::{Interval, Bound};
    /// let int = Interval::open(0.0, 2.2);
    ///
    /// assert_eq!(int.width(), 2.2);
    /// ```
    ///
    /// If the interval is empty, a default point is returned:
    ///
    /// ```rust
    /// # use interval::{Interval, Bound};
    /// let int = Interval::open(0.0, 0.0);
    ///
    /// assert_eq!(int.width(), 0.0);
    /// ```
    #[inline]
    pub fn width<'a>(&'a self) -> <&'a T as Sub>::Output 
        where 
            T: PartialOrd + PartialEq + Clone + 'a, 
            &'a T: Sub,
            <&'a T as Sub>::Output: Default 
    {
        self.end.as_ref() - self.start.as_ref()
    }

    /// Shifts the interval to the left by the given amount.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use interval::Interval;
    /// let mut int = Interval::open(0, 20);
    /// int.left_shift(3);
    ///
    /// assert_eq!(int, Interval::open(-3, 17));
    /// ```
    #[inline]
    pub fn left_shift(&mut self, amount: T) where T: Sub<Output=T> {
        let s = self.start.as_ref().clone();
        let e = self.end.as_ref().clone();
        mem::replace(self.start.as_mut(), s - amount.clone());
        mem::replace(self.end.as_mut(), e - amount);
    }

    /// Shifts the interval to the right by the given amount.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use interval::Interval;
    /// let mut int = Interval::open(0, 20);
    /// int.right_shift(3);
    ///
    /// assert_eq!(int, Interval::open(3, 23));
    /// ```
    #[inline]
    pub fn right_shift(&mut self, amount: T) where T: Add<Output=T> {
        let s = self.start.as_ref().clone();
        let e = self.end.as_ref().clone();
        mem::replace(self.start.as_mut(), s + amount.clone());
        mem::replace(self.end.as_mut(), e + amount);
    }

    /// Shortens the interval by croping from the left by the given amount.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use interval::Interval;
    /// let mut int = Interval::open(0, 20);
    /// int.left_crop(3);
    ///
    /// assert_eq!(int, Interval::open(3, 20));
    /// ```
    ///
    /// Not that if the amount is wider than the interval, it will become empty:
    ///
    /// ```rust
    /// use interval::Interval;
    /// let mut int = Interval::open(0, 20);
    /// int.left_crop(30);
    ///
    /// assert!(int.is_empty());
    /// ```
    #[inline]
    pub fn left_crop(&mut self, amount: T)
        where T: PartialOrd + PartialEq + Clone + Add<Output=T>,
    {
        let new_start = self.start.as_ref().clone() + amount;
        let end = self.end.as_ref().clone();
        if &new_start < self.end.as_ref() {
            mem::replace(self.start.as_mut(), new_start);
        } else if &new_start == self.end.as_ref() {
            let int = Interval::new(
                Bound::apply(&self.start, end.clone()),
                Some(Bound::apply(&self.end, end))
            );
            mem::replace(self, int);
        } else {
            mem::replace(self, Interval::open(end.clone(), end));
        }
    }

    /// Shortens the interval by croping from the right by the given amount.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use interval::Interval;
    /// let mut int = Interval::open(0, 20);
    /// int.right_crop(3);
    ///
    /// assert_eq!(int, Interval::open(0, 17));
    /// ```
    ///
    /// Not that if the amount is wider than the interval, it will become empty:
    ///
    /// ```rust
    /// use interval::Interval;
    /// let mut int = Interval::open(0, 20);
    /// int.right_crop(30);
    ///
    /// assert!(int.is_empty());
    /// ```
    #[inline]
    pub fn right_crop(&mut self, amount: T)
        where T: PartialOrd + PartialEq + Clone + Sub<Output=T>,
    {
        let new_end = self.end.as_ref().clone() - amount;
        let start = self.start.as_ref().clone();
        if &new_end > self.start.as_ref() {
            mem::replace(self.end.as_mut(), new_end);
        } else if &new_end == self.start.as_ref() {
            let int = Interval::new(
                Bound::apply(&self.end, start.clone()),
                Some(Bound::apply(&self.start, start))
            );
            mem::replace(self, int);
        } else {
            mem::replace(self, Interval::open(start.clone(), start));
        }
    }

    /// Lengthens the interval by extending the left by the given amount.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use interval::Interval;
    /// let mut int = Interval::open(0, 20);
    /// int.left_extend(3);
    ///
    /// assert_eq!(int, Interval::open(-3, 20));
    /// ```
    #[inline]
    pub fn left_extend(&mut self, amount: T)
        where T: PartialOrd + PartialEq + Clone + Sub<Output=T>,
    {
        let s = self.start.as_ref().clone() - amount;
        mem::replace(self.start.as_mut(), s);
    }

    /// Lengthens the interval by extending the right by the given amount.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use interval::Interval;
    /// let mut int = Interval::open(0, 20);
    /// int.right_extend(3);
    ///
    /// assert_eq!(int, Interval::open(0, 23));
    /// ```
    #[inline]
    pub fn right_extend(&mut self, amount: T)
        where T: PartialOrd + PartialEq + Clone + Add<Output=T>,
    {
        let e = self.end.as_ref().clone() + amount;
        mem::replace(self.end.as_mut(), e);
    }
}

// Display using interval notation.
impl<T> fmt::Display for Interval<T> 
    where T: fmt::Display + PartialOrd + Clone 
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}, {}{}",
            if self.left_bound().is_open() {"("} else {"["},
            self.left_point(), 
            self.right_point(),
            if self.left_bound().is_open() {")"} else {"]"},
        )
    }
}

// Interval-from-Point conversion.
impl<T> From<T> for Interval<T> where T: PartialOrd + PartialEq + Clone {
    #[inline]
    fn from(t: T) -> Self {
        Interval::closed(t.clone(), t)
    }
}
