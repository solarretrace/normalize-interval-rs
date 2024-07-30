// Copyright 2018 Skylor R. Schermer.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
////////////////////////////////////////////////////////////////////////////////
//!
//! Provides a bound type for intervals.
//!
////////////////////////////////////////////////////////////////////////////////

// Standard library imports.
use std::borrow::Borrow;
use std::default::Default;

// Local enum shortcut.
use self::Bound::*;


////////////////////////////////////////////////////////////////////////////////
// Bound
////////////////////////////////////////////////////////////////////////////////
/// Determines the type of an [`Interval`]'s boundary point.
///
/// [`Interval`]: struct.Interval.html
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Bound<T> {
    /// The bound includes the point.
    Include(T),
    /// The bound excludes the point.
    Exclude(T),
    /// The bound does not exist.
    Infinite,
}

impl<T> Bound<T> {
    // Querying the contained values
    ////////////////////////////////////////////////////////////////////////////

    /// Returns `true` if the bound is an [`Include`] or [`Exclude`] value.
    ///
    /// [`Include`]: #variant.Include
    /// [`Exclude`]: #variant.Exclude
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use normalize_interval::Bound;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// # //-------------------------------------------------------------------
    /// let x: Bound<i32> = Bound::Include(15);
    /// assert_eq!(x.is_finite(), true);
    ///
    /// let x: Bound<i32> = Bound::Infinite;
    /// assert_eq!(x.is_finite(), false);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn is_finite(&self) -> bool {
        !matches!(self, Infinite)
    }

    /// Returns `true` if the bound is an [`Include`] value.
    ///
    /// [`Include`]: #variant.Include
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use normalize_interval::Bound;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// # //-------------------------------------------------------------------
    /// let x: Bound<i32> = Bound::Include(15);
    /// assert_eq!(x.is_inclusive(), true);
    ///
    /// let x: Bound<i32> = Bound::Exclude(15);
    /// assert_eq!(x.is_inclusive(), false);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn is_inclusive(&self) -> bool {
        matches!(self, Include(_))
    }

    /// Returns `true` if the bound is an [`Exclude`] value.
    ///
    /// [`Exclude`]: #variant.Exclude
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use normalize_interval::Bound;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// # //-------------------------------------------------------------------
    /// let x: Bound<i32> = Bound::Exclude(15);
    /// assert_eq!(x.is_exclusive(), true);
    ///
    /// let x: Bound<i32> = Bound::Include(15);
    /// assert_eq!(x.is_exclusive(), false);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn is_exclusive(&self) -> bool {
        matches!(self, Exclude(_))
    }

    // Adapter for working with references
    ////////////////////////////////////////////////////////////////////////////

    /// Returns a reference to the contained point, or `None` if the bound is 
    /// [`Infinite`].
    ///
    /// [`Infinite`]: #variant.Infinite
    /// 
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use normalize_interval::Bound;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// # //-------------------------------------------------------------------
    /// let x: Bound<i32> = Bound::Exclude(34);
    ///
    /// assert_eq!(x.as_ref(), Some(&34));
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn as_ref(&self) -> Option<&T> {
        match self {
            Include(bound) |
            Exclude(bound) => Some(bound),
            Infinite       => None,
        }
    }

    /// Returns a mutable reference to the contained point, or `None` if the
    /// bound is [`Infinite`].
    ///
    /// [`Infinite`]: #variant.Infinite
    /// 
    /// 
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use normalize_interval::Bound;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// # //-------------------------------------------------------------------
    /// let mut x: Bound<i32> = Bound::Exclude(34);
    ///
    /// assert_eq!(x.as_mut(), Some(&mut 34));
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn as_mut(&mut self) -> Option<&mut T> {
        match self {
            Include(bound) |
            Exclude(bound) => Some(bound),
            Infinite       => None,
        }
    }

    // Getting to contained values
    ////////////////////////////////////////////////////////////////////////////

    /// Moves the value out of the `Bound<T>` if it is [`Include`] or
    /// [`Exclude`].
    ///
    /// In general, because this function may panic, its use is discouraged.
    /// Instead, prefer to use pattern matching and handle the [`Infinite`]
    /// case explicitly.
    ///
    /// [`Include`]: #variant.Include
    /// [`Exclude`]: #variant.Exclude
    /// [`Infinite`]: #variant.Infinite
    /// 
    /// # Panics
    ///
    /// Panics if the self value equals [`Infinite`].
    ///
    /// [`Infinite`]: #variant.Infinite
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use normalize_interval::Bound;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// # //-------------------------------------------------------------------
    /// let x: Bound<i32> = Bound::Exclude(34);
    /// assert_eq!(x.unwrap(), 34);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// ```rust,should_panic
    /// # use std::error::Error;
    /// # use normalize_interval::Bound;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// # //-------------------------------------------------------------------
    /// let x: Bound<i32> = Bound::Infinite;
    /// assert_eq!(x.unwrap(), 34); // fails
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn unwrap(self) -> T {
        match self {
            Include(x) |
            Exclude(x) => x,
            Infinite
                => panic!("called `Bound::unwrap()` on an `Infinite` value"),
        }
    }

    /// Returns the bound value or a default.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use normalize_interval::Bound;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// # //-------------------------------------------------------------------
    /// assert_eq!(Bound::Exclude(34).unwrap_or(15), 34);
    /// assert_eq!(Bound::Infinite.unwrap_or(15), 15);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn unwrap_or(self, def: T) -> T {
        match self {
            Include(x) |
            Exclude(x) => x,
            Infinite   => def,
        }
    }

    /// Returns the bound value or computes it from a closure.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use normalize_interval::Bound;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// # //-------------------------------------------------------------------
    /// let k = 10;
    /// assert_eq!(Bound::Exclude(34).unwrap_or_else(|| 2 * k), 34);
    /// assert_eq!(Bound::Infinite.unwrap_or_else(|| 2 * k), 20);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn unwrap_or_else<F: FnOnce() -> T>(self, f: F) -> T {
        match self {
            Include(x) |
            Exclude(x) => x,
            Infinite   => f(),
        }
    }

    // Transforming contained values
    ////////////////////////////////////////////////////////////////////////////

    /// Maps an `Bound<T>` to `Bound<U>` by applying a function to a contained
    /// value.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use normalize_interval::Bound;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// # //-------------------------------------------------------------------
    /// let x: Bound<u32> = Bound::Include(10);
    /// let y: Bound<usize> = x.map(|v| v as usize);
    ///
    /// assert_eq!(y, Bound::Include(10usize));
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Bound<U> {
        match self {
            Include(x) => Include(f(x)),
            Exclude(x) => Exclude(f(x)),
            Infinite   => Infinite,
        }
    }

    /// Applyies a function to a contained value (if finite) or returns a 
    /// default value (if [`Infinte`]).
    ///
    /// [`Infinite`]: #variant.Infinite
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use normalize_interval::Bound;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// # //-------------------------------------------------------------------
    /// assert_eq!(Bound::Include(10).map_or(6, |k| k * 2), 20);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn map_or<U, F>(self, def: U, f: F) -> U where F: FnOnce(T) -> U {
        match self {
            Include(x) |
            Exclude(x) => f(x),
            Infinite   => def,
        }
    }

    /// Applyies a function to a contained value (if finite) or returns a 
    /// computed value (if [`Infinte`]).
    ///
    /// [`Infinite`]: #variant.Infinite
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use normalize_interval::Bound;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// # //-------------------------------------------------------------------
    /// assert_eq!(Bound::Include(10).map_or_else(|| 6, |k| k * 2), 20);
    /// assert_eq!(Bound::Infinite.map_or_else(|| 6, |k: u32| k * 2), 6);
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn map_or_else<U, D, F>(self, def: D, f: F) -> U
        where
            D: FnOnce() -> U,
            F: FnOnce(T) -> U
    {
        match self {
            Include(x) |
            Exclude(x) => f(x),
            Infinite   => def(),
        }
    }

    // Transfering bound type
    ////////////////////////////////////////////////////////////////////////////

    /// Constructs a new `Bound` by applyting the bound type to the given value.
    /// 
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use normalize_interval::Bound;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// # //-------------------------------------------------------------------
    /// let x: Bound<i32> = Bound::transfer(Bound::Exclude(34), 18);
    ///
    /// assert_eq!(x, Bound::Exclude(18));
    /// # //-------------------------------------------------------------------
    /// #     Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn transfer<B: Borrow<Self>, O>(from: B, to: O) -> Bound<O> {
        match *from.borrow() {
            Include(_) => Include(to),
            Exclude(_) => Exclude(to),
            Infinite   => Infinite,
        }
    }
}

impl<T> Bound<T> where T: PartialOrd {
    /// Returns `true` if the `Bound` points are considered adjacent under a
    /// union.
    pub(in crate) fn is_union_adjacent_to(&self, other: &Self) -> bool {
        matches!((self, other),
            (Include(p), Include(o))           |
            (Include(p), Exclude(o))           |
            (Exclude(p), Include(o)) if p == o )
    }
}

impl<T> Bound<T> where T: PartialOrd + Clone {
    // Union and Intersection operators
    ////////////////////////////////////////////////////////////////////////////

    /// Returns the union of the given boundaries, or the lowest one if they are
    /// not at the same point.
    #[must_use]
    pub fn least_union(&self, other: &Self) -> Self {
        match (self, other) {
            (Include(p), Include(o))
                => if p < o {Include(p.clone())} else {Include(o.clone())},

            (Include(p), Exclude(o))
                => if p <= o {Include(p.clone())} else {Exclude(o.clone())},

            (Exclude(p), Include(o))
                => if p < o {Exclude(p.clone())} else {Include(o.clone())},

            (Exclude(p), Exclude(o))
                => if p < o {Exclude(p.clone())} else {Exclude(o.clone())},
        
            _   => Infinite,
        }
    }

    /// Returns the intersect of the given boundaries, or the lowest one if they
    /// are not at the same point.
    #[must_use]
    pub fn least_intersect(&self, other: &Self) -> Self {
        match (self, other) {
            (Include(p), Include(o))
                => if p < o {Include(p.clone())} else {Include(o.clone())},

            (Include(p), Exclude(o))
                => if p < o {Include(p.clone())} else {Exclude(o.clone())},

            (Exclude(p), Include(o))
                => if p <= o {Exclude(p.clone())} else {Include(o.clone())},

            (Exclude(p), Exclude(o))
                => if p < o {Exclude(p.clone())} else {Exclude(o.clone())},

            (Include(p), Infinite) => Include(p.clone()),

            (Exclude(p), Infinite) => Exclude(p.clone()),
            
            (Infinite, Include(o)) => Include(o.clone()),
            
            (Infinite, Exclude(o)) => Exclude(o.clone()),
            
            _   => Infinite,
        }
    }

    /// Returns the union of the given boundaries, or the greatest one if they 
    /// are not at the same point.
    #[must_use]
    pub fn greatest_union(&self, other: &Self) -> Self {
        match (self, other) {
            (Include(p), Include(o))
                => if p > o {Include(p.clone())} else {Include(o.clone())},

            (Include(p), Exclude(o))
                => if p >= o {Include(p.clone())} else {Exclude(o.clone())},

            (Exclude(p), Include(o))
                => if p > o {Exclude(p.clone())} else {Include(o.clone())},

            (Exclude(p), Exclude(o))
                => if p > o {Exclude(p.clone())} else {Exclude(o.clone())},

            _   => Infinite,
        }
    }

    /// Returns the intersect of the given boundaries, or the greatest one if 
    /// they are not at the same point.
    #[must_use]
    pub fn greatest_intersect(&self, other: &Self) -> Self {
        match (self, other) {
            (Include(p), Include(o))
                => if p > o {Include(p.clone())} else {Include(o.clone())},

            (Include(p), Exclude(o))
                => if p > o {Include(p.clone())} else {Exclude(o.clone())},

            (Exclude(p), Include(o))
                => if p >= o {Exclude(p.clone())} else {Include(o.clone())},

            (Exclude(p), Exclude(o))
                => if p > o {Exclude(p.clone())} else {Exclude(o.clone())},

            (Include(p), Infinite) => Include(p.clone()),

            (Exclude(p), Infinite) => Exclude(p.clone()),

            (Infinite, Include(o)) => Include(o.clone()),

            (Infinite, Exclude(o)) => Exclude(o.clone()),

            _   => Infinite,
        }
    }
}

// Default `Bound` is closed.
impl<T> Default for Bound<T> where T: Default {
    #[inline]
    fn default() -> Self {
        Include(Default::default())
    }
}

// `Bound`-from-point conversion.
impl<T> From<T> for Bound<T> {
    #[inline]
    fn from(t: T) -> Self {
        Include(t)
    }
}
