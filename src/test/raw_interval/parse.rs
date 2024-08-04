// Copyright 2024 Skylor R. Schermer.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
////////////////////////////////////////////////////////////////////////////////
//!
//! Testing module for [`RawInterval`] parsing.
//!
//! [`RawInterval`] struct.RawInterval.html
//!
////////////////////////////////////////////////////////////////////////////////

// Internal library imports.
use crate::raw_interval::RawInterval;

// Local enum shortcuts.
use crate::raw_interval::RawInterval::*;

// Standard library imports.
use std::str::FromStr;

////////////////////////////////////////////////////////////////////////////
// Tests
////////////////////////////////////////////////////////////////////////////

#[test]
fn round_trip_empty() {
    let a: RawInterval<i32> = Empty;
    let gened = a.to_string();
    let parsed = RawInterval::<i32>::from_str(&gened)
        .expect("successful parse");
    assert_eq!(gened, "Ø");
    assert_eq!(a, parsed);
}

#[test]
fn round_trip_point() {
    let a: RawInterval<i32> = Point(3);
    let gened = a.to_string();
    let parsed = RawInterval::<i32>::from_str(&gened)
        .expect("successful parse");
    assert_eq!(gened, "3");
    assert_eq!(a, parsed);
}

#[test]
fn round_trip_open() {
    let a: RawInterval<i32> = Open(0, 3);
    let gened = a.to_string();
    let parsed = RawInterval::<i32>::from_str(&gened)
        .expect("successful parse");
    assert_eq!(gened, "(0,3)");
    assert_eq!(a, parsed);
}

#[test]
fn round_trip_left_open() {
    let a: RawInterval<i32> = LeftOpen(0, 3);
    let gened = a.to_string();
    let parsed = RawInterval::<i32>::from_str(&gened)
        .expect("successful parse");
    assert_eq!(gened, "(0,3]");
    assert_eq!(a, parsed);
}

#[test]
fn round_trip_right_open() {
    let a: RawInterval<i32> = RightOpen(0, 3);
    let gened = a.to_string();
    let parsed = RawInterval::<i32>::from_str(&gened)
        .expect("successful parse");
    assert_eq!(gened, "[0,3)");
    assert_eq!(a, parsed);
}

#[test]
fn round_trip_closed() {
    let a: RawInterval<i32> = Closed(0, 3);
    let gened = a.to_string();
    let parsed = RawInterval::<i32>::from_str(&gened)
        .expect("successful parse");
    assert_eq!(gened, "[0,3]");
    assert_eq!(a, parsed);
}


#[test]
fn round_trip_up_to() {
    let a: RawInterval<i32> = UpTo(3);
    let gened = a.to_string();
    let parsed = RawInterval::<i32>::from_str(&gened)
        .expect("successful parse");
    assert_eq!(gened, "(-∞,3)");
    assert_eq!(a, parsed);
}

#[test]
fn round_trip_up_from() {
    let a: RawInterval<i32> = UpFrom(3);
    let gened = a.to_string();
    let parsed = RawInterval::<i32>::from_str(&gened)
        .expect("successful parse");
    assert_eq!(gened, "(3,∞)");
    assert_eq!(a, parsed);
}

#[test]
fn round_trip_to() {
    let a: RawInterval<i32> = To(3);
    let gened = a.to_string();
    let parsed = RawInterval::<i32>::from_str(&gened)
        .expect("successful parse");
    assert_eq!(gened, "(-∞,3]");
    assert_eq!(a, parsed);
}

#[test]
fn round_trip_from() {
    let a: RawInterval<i32> = From(3);
    let gened = a.to_string();
    let parsed = RawInterval::<i32>::from_str(&gened)
        .expect("successful parse");
    assert_eq!(gened, "[3,∞)");
    assert_eq!(a, parsed);
}

#[test]
fn round_trip_full() {
    let a: RawInterval<i32> = Full;
    let gened = a.to_string();
    let parsed = RawInterval::<i32>::from_str(&gened)
        .expect("successful parse");
    assert_eq!(gened, "(-∞,∞)");
    assert_eq!(a, parsed);
}
