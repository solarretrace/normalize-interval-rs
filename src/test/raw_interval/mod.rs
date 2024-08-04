// Copyright 2018 Skylor R. Schermer.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
////////////////////////////////////////////////////////////////////////////////
//!
//! Testing module for [`RawInterval`].
//!
//! [`RawInterval`] struct.RawInterval.html
//!
////////////////////////////////////////////////////////////////////////////////

// Module declarations.
mod enclose;
mod intersect;
mod minus;
mod union;
mod parse;

// Internal library imports.
use crate::raw_interval::RawInterval;
use crate::bound::Bound;

// Local enum shortcuts.
use crate::raw_interval::RawInterval::*;

////////////////////////////////////////////////////////////////////////////
// Constructor tests
////////////////////////////////////////////////////////////////////////////

#[test]
fn new_reordering() {
    let e2: Bound<i32> = Bound::Exclude(2);
    let e3: Bound<i32> = Bound::Exclude(3);
    let e4: Bound<i32> = Bound::Exclude(4);

    let i2: Bound<i32> = Bound::Include(2);
    let i3: Bound<i32> = Bound::Include(3);
    let i4: Bound<i32> = Bound::Include(4);

    assert_eq!(RawInterval::new(i2, i4), Closed(2, 4));
    assert_eq!(RawInterval::new(i2, e4), RightOpen(2, 4));
    assert_eq!(RawInterval::new(e2, i4), LeftOpen(2, 4));
    assert_eq!(RawInterval::new(e2, e4), Open(2, 4));

    assert_eq!(RawInterval::new(i3, i3), Point(3));
    assert_eq!(RawInterval::new(i3, e3), Point(3));
    assert_eq!(RawInterval::new(e3, i3), Point(3));
    assert_eq!(RawInterval::new(e3, e3), Empty);

    assert_eq!(RawInterval::new(i4, i2), Empty);
    assert_eq!(RawInterval::new(e4, i2), Empty);
    assert_eq!(RawInterval::new(i4, e2), Empty);
    assert_eq!(RawInterval::new(e4, e2), Empty);
}

#[test]
fn open_reordering() {
    assert_eq!(RawInterval::open(2, 4), Open(2, 4));
    assert_eq!(RawInterval::open(3, 3), Empty);
    assert_eq!(RawInterval::open(4, 2), Empty);
}

#[test]
fn left_open_reordering() {
    assert_eq!(RawInterval::left_open(2, 4), LeftOpen(2, 4));
    assert_eq!(RawInterval::left_open(3, 3), Point(3));
    assert_eq!(RawInterval::left_open(4, 2), Empty);
}

#[test]
fn right_open_reordering() {
    assert_eq!(RawInterval::right_open(2, 4), RightOpen(2, 4));
    assert_eq!(RawInterval::right_open(3, 3), Point(3));
    assert_eq!(RawInterval::right_open(4, 2), Empty);
}

#[test]
fn closed_reordering() {
    assert_eq!(RawInterval::closed(2, 4), Closed(2, 4));
    assert_eq!(RawInterval::closed(3, 3), Point(3));
    assert_eq!(RawInterval::closed(4, 2), Empty);
}

////////////////////////////////////////////////////////////////////////////
// Bound accessor tests
////////////////////////////////////////////////////////////////////////////

#[test]
fn lower_bound() {
    let a: RawInterval<i32> = Empty;
    assert_eq!(a.lower_bound(), None);

    let a: RawInterval<i32> = Point(3);
    assert_eq!(a.lower_bound(), Some(Bound::Include(3)));

    let a: RawInterval<i32> = Open(0, 3);
    assert_eq!(a.lower_bound(), Some(Bound::Exclude(0)));

    let a: RawInterval<i32> = LeftOpen(0, 3);
    assert_eq!(a.lower_bound(), Some(Bound::Exclude(0)));

    let a: RawInterval<i32> = RightOpen(0, 3);
    assert_eq!(a.lower_bound(), Some(Bound::Include(0)));

    let a: RawInterval<i32> = Closed(0, 3);
    assert_eq!(a.lower_bound(), Some(Bound::Include(0)));

    let a: RawInterval<i32> = UpTo(3);
    assert_eq!(a.lower_bound(), Some(Bound::Infinite));

    let a: RawInterval<i32> = UpFrom(3);
    assert_eq!(a.lower_bound(), Some(Bound::Exclude(3)));

    let a: RawInterval<i32> = To(3);
    assert_eq!(a.lower_bound(), Some(Bound::Infinite));

    let a: RawInterval<i32> = From(3);
    assert_eq!(a.lower_bound(), Some(Bound::Include(3)));

    let a: RawInterval<i32> = Full;
    assert_eq!(a.lower_bound(), Some(Bound::Infinite));
}

#[test]
fn upper_bound() {
    let a: RawInterval<i32> = Empty;
    assert_eq!(a.upper_bound(), None);

    let a: RawInterval<i32> = Point(3);
    assert_eq!(a.upper_bound(), Some(Bound::Include(3)));

    let a: RawInterval<i32> = Open(0, 3);
    assert_eq!(a.upper_bound(), Some(Bound::Exclude(3)));

    let a: RawInterval<i32> = LeftOpen(0, 3);
    assert_eq!(a.upper_bound(), Some(Bound::Include(3)));

    let a: RawInterval<i32> = RightOpen(0, 3);
    assert_eq!(a.upper_bound(), Some(Bound::Exclude(3)));

    let a: RawInterval<i32> = Closed(0, 3);
    assert_eq!(a.upper_bound(), Some(Bound::Include(3)));

    let a: RawInterval<i32> = UpTo(3);
    assert_eq!(a.upper_bound(), Some(Bound::Exclude(3)));

    let a: RawInterval<i32> = UpFrom(3);
    assert_eq!(a.upper_bound(), Some(Bound::Infinite));

    let a: RawInterval<i32> = To(3);
    assert_eq!(a.upper_bound(), Some(Bound::Include(3)));

    let a: RawInterval<i32> = From(3);
    assert_eq!(a.upper_bound(), Some(Bound::Infinite));

    let a: RawInterval<i32> = Full;
    assert_eq!(a.upper_bound(), Some(Bound::Infinite));
}

#[test]
fn infimum() {
    let a: RawInterval<i32> = Empty;
    assert_eq!(a.infimum(), None);

    let a: RawInterval<i32> = Point(3);
    assert_eq!(a.infimum(), Some(3));

    let a: RawInterval<i32> = Open(0, 3);
    assert_eq!(a.infimum(), Some(0));

    let a: RawInterval<i32> = LeftOpen(0, 3);
    assert_eq!(a.infimum(), Some(0));

    let a: RawInterval<i32> = RightOpen(0, 3);
    assert_eq!(a.infimum(), Some(0));

    let a: RawInterval<i32> = Closed(0, 3);
    assert_eq!(a.infimum(), Some(0));

    let a: RawInterval<i32> = UpTo(3);
    assert_eq!(a.infimum(), None);

    let a: RawInterval<i32> = UpFrom(3);
    assert_eq!(a.infimum(), Some(3));

    let a: RawInterval<i32> = To(3);
    assert_eq!(a.infimum(), None);

    let a: RawInterval<i32> = From(3);
    assert_eq!(a.infimum(), Some(3));

    let a: RawInterval<i32> = Full;
    assert_eq!(a.infimum(), None);
}

#[test]
fn supremum() {
    let a: RawInterval<i32> = Empty;
    assert_eq!(a.supremum(), None);

    let a: RawInterval<i32> = Point(3);
    assert_eq!(a.supremum(), Some(3));

    let a: RawInterval<i32> = Open(0, 3);
    assert_eq!(a.supremum(), Some(3));

    let a: RawInterval<i32> = LeftOpen(0, 3);
    assert_eq!(a.supremum(), Some(3));

    let a: RawInterval<i32> = RightOpen(0, 3);
    assert_eq!(a.supremum(), Some(3));

    let a: RawInterval<i32> = Closed(0, 3);
    assert_eq!(a.supremum(), Some(3));

    let a: RawInterval<i32> = UpTo(3);
    assert_eq!(a.supremum(), Some(3));

    let a: RawInterval<i32> = UpFrom(3);
    assert_eq!(a.supremum(), None);

    let a: RawInterval<i32> = To(3);
    assert_eq!(a.supremum(), Some(3));

    let a: RawInterval<i32> = From(3);
    assert_eq!(a.supremum(), None);

    let a: RawInterval<i32> = Full;
    assert_eq!(a.supremum(), None);
}


////////////////////////////////////////////////////////////////////////////
// Query operation tests
////////////////////////////////////////////////////////////////////////////

#[test]
fn is_empty() {
    let a: RawInterval<i32> = Empty;
    assert!(a.is_empty());

    let a: RawInterval<i32> = Point(3);
    assert!(!a.is_empty());

    let a: RawInterval<i32> = Open(0, 3);
    assert!(!a.is_empty());

    let a: RawInterval<i32> = LeftOpen(0, 3);
    assert!(!a.is_empty());

    let a: RawInterval<i32> = RightOpen(0, 3);
    assert!(!a.is_empty());

    let a: RawInterval<i32> = Closed(0, 3);
    assert!(!a.is_empty());

    let a: RawInterval<i32> = UpTo(3);
    assert!(!a.is_empty());

    let a: RawInterval<i32> = UpFrom(3);
    assert!(!a.is_empty());

    let a: RawInterval<i32> = To(3);
    assert!(!a.is_empty());

    let a: RawInterval<i32> = From(3);
    assert!(!a.is_empty());

    let a: RawInterval<i32> = Full;
    assert!(!a.is_empty());
}

#[test]
fn contains() {
    let a: RawInterval<i32> = Empty;
    assert!(!a.contains(&-1));
    assert!(!a.contains(&0));
    assert!(!a.contains(&2));
    assert!(!a.contains(&3));
    assert!(!a.contains(&4));

    let a: RawInterval<i32> = Point(3);
    assert!(!a.contains(&-1));
    assert!(!a.contains(&0));
    assert!(!a.contains(&2));
    assert!(a.contains(&3));
    assert!(!a.contains(&4));

    let a: RawInterval<i32> = Open(0, 3);
    assert!(!a.contains(&-1));
    assert!(!a.contains(&0));
    assert!(a.contains(&2));
    assert!(!a.contains(&3));
    assert!(!a.contains(&4));

    let a: RawInterval<i32> = LeftOpen(0, 3);
    assert!(!a.contains(&-1));
    assert!(!a.contains(&0));
    assert!(a.contains(&2));
    assert!(a.contains(&3));
    assert!(!a.contains(&4));

    let a: RawInterval<i32> = RightOpen(0, 3);
    assert!(!a.contains(&-1));
    assert!(a.contains(&0));
    assert!(a.contains(&2));
    assert!(!a.contains(&3));
    assert!(!a.contains(&4));

    let a: RawInterval<i32> = Closed(0, 3);
    assert!(!a.contains(&-1));
    assert!(a.contains(&0));
    assert!(a.contains(&2));
    assert!(a.contains(&3));
    assert!(!a.contains(&4));

    let a: RawInterval<i32> = UpTo(3);
    assert!(a.contains(&-1));
    assert!(a.contains(&0));
    assert!(a.contains(&2));
    assert!(!a.contains(&3));
    assert!(!a.contains(&4));

    let a: RawInterval<i32> = UpFrom(3);
    assert!(!a.contains(&-1));
    assert!(!a.contains(&0));
    assert!(!a.contains(&2));
    assert!(!a.contains(&3));
    assert!(a.contains(&4));

    let a: RawInterval<i32> = To(3);
    assert!(a.contains(&-1));
    assert!(a.contains(&0));
    assert!(a.contains(&2));
    assert!(a.contains(&3));
    assert!(!a.contains(&4));

    let a: RawInterval<i32> = From(3);
    assert!(!a.contains(&-1));
    assert!(!a.contains(&0));
    assert!(!a.contains(&2));
    assert!(a.contains(&3));
    assert!(a.contains(&4));

    let a: RawInterval<i32> = Full;
    assert!(a.contains(&-1));
    assert!(a.contains(&0));
    assert!(a.contains(&2));
    assert!(a.contains(&3));
    assert!(a.contains(&4));
}

////////////////////////////////////////////////////////////////////////////
// Set law tests
////////////////////////////////////////////////////////////////////////////

#[test]
fn complement_as_full_minus() {
    let a: RawInterval<i32> = Full;

    assert_eq_u!(a.minus(&Empty),           Empty.complement().collect::<Vec<_>>());
    assert_eq_u!(a.minus(&Point(0)),        Point(0).complement().collect::<Vec<_>>());
    assert_eq_u!(a.minus(&Open(0, 3)),      Open(0, 3).complement().collect::<Vec<_>>());
    assert_eq_u!(a.minus(&LeftOpen(0, 3)),  LeftOpen(0, 3).complement().collect::<Vec<_>>());
    assert_eq_u!(a.minus(&RightOpen(0, 3)), RightOpen(0, 3).complement().collect::<Vec<_>>());
    assert_eq_u!(a.minus(&Closed(0, 3)),    Closed(0, 3).complement().collect::<Vec<_>>());
    assert_eq_u!(a.minus(&UpTo(0)),         UpTo(0).complement().collect::<Vec<_>>());
    assert_eq_u!(a.minus(&UpFrom(0)),       UpFrom(0).complement().collect::<Vec<_>>());
    assert_eq_u!(a.minus(&To(0)),           To(0).complement().collect::<Vec<_>>());
    assert_eq_u!(a.minus(&From(0)),         From(0).complement().collect::<Vec<_>>());
    assert_eq_u!(a.minus(&Full),            Full.complement().collect::<Vec<_>>());
}
