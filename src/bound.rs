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
//! Provides a bound type for intervals.
//!
////////////////////////////////////////////////////////////////////////////////

// Module imports.
use std::default::Default;

// Local enum shortcuts.
use Bound::*;

////////////////////////////////////////////////////////////////////////////////
// Bound<T>
////////////////////////////////////////////////////////////////////////////////
/// Determines the type of an interval's boundary.
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Bound<T> {
    /// The boundary includes the point.
    Include(T),
    /// The boundary excludes the point.
    Exclude(T),
}


impl<T> Bound<T> where T: PartialOrd + PartialEq + Clone {
    /// Constructs a new `Bound` by applying the bound type of the given `from` 
    /// bound to the given value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use interval::Bound;
    ///
    /// let b1 = Bound::Exclude(35);
    /// let b2 = Bound::transfer(&b1, 13.56f32);
    ///
    /// assert_eq!(b2, Bound::Exclude(13.56f32));
    /// ```
    #[inline]
    pub fn transfer<O>(from: &Self, to: O) -> Bound<O> {
        match from {
            &Include(_) => Include(to),
            &Exclude(_) => Exclude(to),
        }
    }
}



////////////////////////////////////////////////////////////////////////////////
// BoundOps
////////////////////////////////////////////////////////////////////////////////
/// Provides operations to simplify the computation of interval operations.
pub trait BoundOps {
    /// Returns the union of the given boundaries, or the lowest one if they are
    /// not at the same point.
    ///
    /// # Example
    ///
    /// ```rust
    /// use interval::{Bound, BoundOps};
    ///
    /// let b1 = Bound::Include(0);
    /// let b2 = Bound::Exclude(0);
    /// 
    /// assert_eq!(b1.least_union(&b2), b1);
    /// ```
    fn least_union(&self, other: &Self) -> Self;

    /// Returns the intersect of the given boundaries, or the lowest one if they
    /// are not at the same point.
    ///
    /// # Example
    ///
    /// ```rust
    /// use interval::{Bound, BoundOps};
    ///
    /// let b1 = Bound::Include(0);
    /// let b2 = Bound::Exclude(0);
    /// 
    /// assert_eq!(b1.least_intersect(&b2), b2);
    /// ```
    fn least_intersect(&self, other: &Self) -> Self;

    /// Returns the union of the given boundaries, or the greatest one if they 
    /// are not at the same point.
    ///
    /// # Example
    ///
    /// ```rust
    /// use interval::{Bound, BoundOps};
    ///
    /// let b1 = Bound::Include(0);
    /// let b2 = Bound::Exclude(0);
    /// 
    /// assert_eq!(b1.greatest_union(&b2), b1);
    /// ```
    fn greatest_union(&self, other: &Self) -> Self;

    /// Returns the intersect of the given boundaries, or the greatest one if 
    /// they are not at the same point.
    ///
    /// # Example
    ///
    /// ```rust
    /// use interval::{Bound, BoundOps};
    ///
    /// let b1 = Bound::Include(0);
    /// let b2 = Bound::Exclude(0);
    /// 
    /// assert_eq!(b1.greatest_intersect(&b2), b2);
    /// ```
    fn greatest_intersect(&self, other: &Self) -> Self;
}


////////////////////////////////////////////////////////////////////////////////
// BoundOps trait impls.
////////////////////////////////////////////////////////////////////////////////
// Basic implementation.
impl<T> BoundOps for Bound<T> where T: PartialOrd + PartialEq + Clone {
    #[inline]
    fn least_union(&self, other: &Self) -> Self {
        match (self, other) {
            (&Exclude(ref p), &Exclude(ref o))
                => if p < o {Exclude(p.clone())} else {Exclude(o.clone())},

            (&Exclude(ref p), &Include(ref o))
                => if p < o {Exclude(p.clone())} else {Include(o.clone())},

            (&Include(ref p), &Exclude(ref o))
                => if p < o {Include(p.clone())} else {Include(o.clone())},

            (&Include(ref p), &Include(ref o))
                => if p < o {Include(p.clone())} else {Include(o.clone())},
        }
    }
    
    #[inline]
    fn least_intersect(&self, other: &Self) -> Self {
        match (self, other) {
            (&Exclude(ref p), &Exclude(ref o))
                => if p < o {Exclude(p.clone())} else {Exclude(o.clone())},

            (&Exclude(ref p), &Include(ref o))
                => if p < o {Exclude(p.clone())} else {Exclude(o.clone())},

            (&Include(ref p), &Exclude(ref o))
                => if p < o {Include(p.clone())} else {Exclude(o.clone())},

            (&Include(ref p), &Include(ref o))
                => if p < o {Include(p.clone())} else {Include(o.clone())},
        }
    }

    #[inline]
    fn greatest_union(&self, other: &Self) -> Self {
        match (self, other) {
            (&Exclude(ref p), &Exclude(ref o))
                => if p > o {Exclude(p.clone())} else {Exclude(o.clone())},

            (&Exclude(ref p), &Include(ref o))
                => if p > o {Exclude(p.clone())} else {Include(o.clone())},

            (&Include(ref p), &Exclude(ref o))
                => if p > o {Include(p.clone())} else {Include(o.clone())},

            (&Include(ref p), &Include(ref o))
                => if p > o {Include(p.clone())} else {Include(o.clone())},
        }
    }
    
    #[inline]
    fn greatest_intersect(&self, other: &Self) -> Self {
        match (self, other) {
            (&Exclude(ref p), &Exclude(ref o))
                => if p > o {Exclude(p.clone())} else {Exclude(o.clone())},

            (&Exclude(ref p), &Include(ref o))
                => if p > o {Exclude(p.clone())} else {Exclude(o.clone())},

            (&Include(ref p), &Exclude(ref o))
                => if p > o {Include(p.clone())} else {Exclude(o.clone())},

            (&Include(ref p), &Include(ref o))
                => if p > o {Include(p.clone())} else {Include(o.clone())},
        }
    }
}


// For simplifying unbounded interval handling.
impl<T> BoundOps for Option<Bound<T>> where T: PartialOrd + PartialEq + Clone {
    #[inline]
    fn least_union(&self, other: &Self) -> Self {
        match (self, other) {
            (&Some(ref lb), &Some(ref ub)) => Some(lb.least_union(ub)),
            _                              => None,
        }
    }
    
    #[inline]
    fn least_intersect(&self, other: &Self) -> Self {
        match (self, other) {
            (&Some(ref lb), &Some(ref ub)) => Some(lb.least_intersect(ub)),
            (&Some(ref lb), &None)         => Some(lb.clone()),
            (&None, &Some(ref ub))         => Some(ub.clone()),
            _                              => None,
        }
    }
    
    #[inline]
    fn greatest_union(&self, other: &Self) -> Self {
        match (self, other) {
            (&Some(ref lb), &Some(ref ub)) => Some(lb.greatest_union(ub)),
            _                              => None,
        }
    }
    
    #[inline]
    fn greatest_intersect(&self, other: &Self) -> Self {
        match (self, other) {
            (&Some(ref lb), &Some(ref ub)) => Some(lb.greatest_intersect(ub)),
            (&Some(ref lb), &None)         => Some(lb.clone()),
            (&None, &Some(ref ub))         => Some(ub.clone()),
            _                              => None,
        }
    }
}


////////////////////////////////////////////////////////////////////////////////
// Miscellaneous trait impls.
////////////////////////////////////////////////////////////////////////////////
// Default `Bound` is closed.
impl<T> Default for Bound<T> where T: Default {
    #[inline]
    fn default() -> Self {
        Include(Default::default())
    }
}

// `Bound`-from-`Point` conversion.
impl<T> From<T> for Bound<T> {
    #[inline]
    fn from(t: T) -> Self {
        Include(t)
    }
}

// Access to inner point.
impl<T> AsRef<T> for Bound<T> {
    #[inline]
    fn as_ref(&self) -> &T {
        match self {
            &Include(ref bound) => bound,
            &Exclude(ref bound) => bound,
        }
    }
}

// Mutable access to inner point.
impl<T> AsMut<T> for Bound<T> {
    #[inline]
    fn as_mut(&mut self) -> &mut T {
        match self {
            &mut Include(ref mut bound) => bound,
            &mut Exclude(ref mut bound) => bound,
        }
    }
}