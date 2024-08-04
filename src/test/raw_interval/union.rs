// Copyright 2018 Skylor R. Schermer.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
////////////////////////////////////////////////////////////////////////////////
//!
//! Testing module for [`union`] operations.
//!
//! [`union`]: struct.RawInterval.html#method.union
//!
////////////////////////////////////////////////////////////////////////////////

// Internal library imports.
use crate::raw_interval::RawInterval;

// Local enum shortcuts.
use crate::raw_interval::RawInterval::*;

////////////////////////////////////////////////////////////////////////////
// Tests
////////////////////////////////////////////////////////////////////////////

#[test]
fn empty() {
    let a: RawInterval<i32> = Empty;

    assert_eq_u!(a.union(&Empty),             []);
    assert_eq_u!(a.union(&Point(3)),          [Point(3)]);
    assert_eq_u!(a.union(&Open(0, 3)),        [Open(0, 3)]);
    assert_eq_u!(a.union(&LeftOpen(0, 3)),    [LeftOpen(0, 3)]);
    assert_eq_u!(a.union(&RightOpen(0, 3)),   [RightOpen(0, 3)]);
    assert_eq_u!(a.union(&Closed(0, 3)),      [Closed(0, 3)]);
    assert_eq_u!(a.union(&UpTo(3)),           [UpTo(3)]);
    assert_eq_u!(a.union(&UpFrom(3)),         [UpFrom(3)]);
    assert_eq_u!(a.union(&To(3)),             [To(3)]);
    assert_eq_u!(a.union(&From(3)),           [From(3)]);
    assert_eq_u!(a.union(&Full),              [Full]);
}

#[test]
fn point_center() {
    let a: RawInterval<i32> = Point(3);

    assert_eq_u!(a.union(&Empty),             [Point(3)]);
    assert_eq_u!(a.union(&Point(3)),          [Point(3)]);
    assert_eq_u!(a.union(&Open(0, 3)),        [LeftOpen(0, 3)]);
    assert_eq_u!(a.union(&LeftOpen(0, 3)),    [LeftOpen(0, 3)]);
    assert_eq_u!(a.union(&RightOpen(0, 3)),   [Closed(0, 3)]);
    assert_eq_u!(a.union(&Closed(0, 3)),      [Closed(0, 3)]);
    assert_eq_u!(a.union(&UpTo(3)),           [To(3)]);
    assert_eq_u!(a.union(&UpFrom(3)),         [From(3)]);
    assert_eq_u!(a.union(&To(3)),             [To(3)]);
    assert_eq_u!(a.union(&From(3)),           [From(3)]);
    assert_eq_u!(a.union(&Full),              [Full]);
}

#[test]
fn point_left() {
    let a: RawInterval<i32> = Point(3);

    assert_eq_u!(a.union(&Empty),             [Point(3)]);
    assert_eq_u!(a.union(&Point(-1)),         [Point(3), Point(-1)]);
    assert_eq_u!(a.union(&Open(-3, -1)),      [Point(3), Open(-3, -1)]);
    assert_eq_u!(a.union(&LeftOpen(-3, -1)),  [Point(3), LeftOpen(-3, -1)]);
    assert_eq_u!(a.union(&RightOpen(-3, -1)), [Point(3), RightOpen(-3, -1)]);
    assert_eq_u!(a.union(&Closed(-3, -1)),    [Point(3), Closed(-3, -1)]);
    assert_eq_u!(a.union(&UpTo(-3)),          [Point(3), UpTo(-3)]);
    assert_eq_u!(a.union(&UpFrom(-3)),        [UpFrom(-3)]);
    assert_eq_u!(a.union(&To(-3)),            [Point(3), To(-3)]);
    assert_eq_u!(a.union(&From(-3)),          [From(-3)]);
    assert_eq_u!(a.union(&Full),              [Full]);
}

#[test]
fn point_right() {
    let a: RawInterval<i32> = Point(3);

    assert_eq_u!(a.union(&Empty),             [Point(3)]);
    assert_eq_u!(a.union(&Point(10)),         [Point(3), Point(10)]);
    assert_eq_u!(a.union(&Open(10, 13)),      [Point(3), Open(10, 13)]);
    assert_eq_u!(a.union(&LeftOpen(10, 13)),  [Point(3), LeftOpen(10, 13)]);
    assert_eq_u!(a.union(&RightOpen(10, 13)), [Point(3), RightOpen(10, 13)]);
    assert_eq_u!(a.union(&Closed(10, 13)),    [Point(3), Closed(10, 13)]);
    assert_eq_u!(a.union(&UpTo(13)),          [UpTo(13)]);
    assert_eq_u!(a.union(&UpFrom(13)),        [Point(3), UpFrom(13)]);
    assert_eq_u!(a.union(&To(13)),            [To(13)]);
    assert_eq_u!(a.union(&From(13)),          [Point(3), From(13)]);
    assert_eq_u!(a.union(&Full),              [Full]);
}

#[test]
fn open_center() {
    let a: RawInterval<i32> = Open(0, 3);

    assert_eq_u!(a.union(&Empty),             [Open(0, 3)]);
    assert_eq_u!(a.union(&Point(3)),          [LeftOpen(0, 3)]);
    assert_eq_u!(a.union(&Open(0, 3)),        [Open(0, 3)]);
    assert_eq_u!(a.union(&LeftOpen(0, 3)),    [LeftOpen(0, 3)]);
    assert_eq_u!(a.union(&RightOpen(0, 3)),   [RightOpen(0, 3)]);
    assert_eq_u!(a.union(&Closed(0, 3)),      [Closed(0, 3)]);
    assert_eq_u!(a.union(&UpTo(3)),           [UpTo(3)]);
    assert_eq_u!(a.union(&UpFrom(3)),         [Open(0, 3), UpFrom(3)]);
    assert_eq_u!(a.union(&To(3)),             [To(3)]);
    assert_eq_u!(a.union(&From(3)),           [UpFrom(0)]);
    assert_eq_u!(a.union(&Full),              [Full]);
}

#[test]
fn open_left() {
    let a: RawInterval<i32> = Open(0, 3);

    assert_eq_u!(a.union(&Empty),             [Open(0, 3)]);
    assert_eq_u!(a.union(&Point(-3)),         [Open(0, 3), Point(-3)]);
    assert_eq_u!(a.union(&Open(-3, -1)),      [Open(0, 3), Open(-3, -1)]);
    assert_eq_u!(a.union(&LeftOpen(-3, -1)),  [Open(0, 3), LeftOpen(-3, -1)]);
    assert_eq_u!(a.union(&RightOpen(-3, -1)), [Open(0, 3), RightOpen(-3, -1)]);
    assert_eq_u!(a.union(&Closed(-3, -1)),    [Open(0, 3), Closed(-3, -1)]);
    assert_eq_u!(a.union(&UpTo(-3)),          [Open(0, 3), UpTo(-3)]);
    assert_eq_u!(a.union(&UpFrom(-3)),        [UpFrom(-3)]);
    assert_eq_u!(a.union(&To(-3)),            [Open(0, 3), To(-3)]);
    assert_eq_u!(a.union(&From(-3)),          [From(-3)]);
    assert_eq_u!(a.union(&Full),              [Full]);
}

#[test]
fn open_right() {
    let a: RawInterval<i32> = Open(0, 3);

    assert_eq_u!(a.union(&Empty),             [Open(0, 3)]);
    assert_eq_u!(a.union(&Point(13)),         [Open(0, 3), Point(13)]);
    assert_eq_u!(a.union(&Open(10, 13)),      [Open(0, 3), Open(10, 13)]);
    assert_eq_u!(a.union(&LeftOpen(10, 13)),  [Open(0, 3), LeftOpen(10, 13)]);
    assert_eq_u!(a.union(&RightOpen(10, 13)), [Open(0, 3), RightOpen(10, 13)]);
    assert_eq_u!(a.union(&Closed(10, 13)),    [Open(0, 3), Closed(10, 13)]);
    assert_eq_u!(a.union(&UpTo(13)),          [UpTo(13)]);
    assert_eq_u!(a.union(&UpFrom(13)),        [Open(0, 3), UpFrom(13)]);
    assert_eq_u!(a.union(&To(13)),            [To(13)]);
    assert_eq_u!(a.union(&From(13)),          [Open(0, 3), From(13)]);
    assert_eq_u!(a.union(&Full),              [Full]);
}

#[test]
fn left_open_center() {
    let a: RawInterval<i32> = LeftOpen(0, 3);

    assert_eq_u!(a.union(&Empty),             [LeftOpen(0, 3)]);
    assert_eq_u!(a.union(&Point(3)),          [LeftOpen(0, 3)]);
    assert_eq_u!(a.union(&Open(0, 3)),        [LeftOpen(0, 3)]);
    assert_eq_u!(a.union(&LeftOpen(0, 3)),    [LeftOpen(0, 3)]);
    assert_eq_u!(a.union(&RightOpen(0, 3)),   [Closed(0, 3)]);
    assert_eq_u!(a.union(&Closed(0, 3)),      [Closed(0, 3)]);
    assert_eq_u!(a.union(&UpTo(3)),           [To(3)]);
    assert_eq_u!(a.union(&UpFrom(3)),         [UpFrom(0)]);
    assert_eq_u!(a.union(&To(3)),             [To(3)]);
    assert_eq_u!(a.union(&From(3)),           [UpFrom(0)]);
    assert_eq_u!(a.union(&Full),              [Full]);
}

#[test]
fn left_open_left() {
    let a: RawInterval<i32> = LeftOpen(0, 3);

    assert_eq_u!(a.union(&Empty),             [LeftOpen(0, 3)]);
    assert_eq_u!(a.union(&Point(-3)),         [LeftOpen(0, 3), Point(-3)]);
    assert_eq_u!(a.union(&Open(-3, -1)),      [LeftOpen(0, 3), Open(-3, -1)]);
    assert_eq_u!(a.union(&LeftOpen(-3, -1)),  [LeftOpen(0, 3), LeftOpen(-3, -1)]);
    assert_eq_u!(a.union(&RightOpen(-3, -1)), [LeftOpen(0, 3), RightOpen(-3, -1)]);
    assert_eq_u!(a.union(&Closed(-3, -1)),    [LeftOpen(0, 3), Closed(-3, -1)]);
    assert_eq_u!(a.union(&UpTo(-3)),          [LeftOpen(0, 3), UpTo(-3)]);
    assert_eq_u!(a.union(&UpFrom(-3)),        [UpFrom(-3)]);
    assert_eq_u!(a.union(&To(-3)),            [LeftOpen(0, 3), To(-3)]);
    assert_eq_u!(a.union(&From(-3)),          [From(-3)]);
    assert_eq_u!(a.union(&Full),              [Full]);
}

#[test]
fn left_open_right() {
    let a: RawInterval<i32> = LeftOpen(0, 3);

    assert_eq_u!(a.union(&Empty),             [LeftOpen(0, 3)]);
    assert_eq_u!(a.union(&Point(13)),         [LeftOpen(0, 3), Point(13)]);
    assert_eq_u!(a.union(&Open(10, 13)),      [LeftOpen(0, 3), Open(10, 13)]);
    assert_eq_u!(a.union(&LeftOpen(10, 13)),  [LeftOpen(0, 3), LeftOpen(10, 13)]);
    assert_eq_u!(a.union(&RightOpen(10, 13)), [LeftOpen(0, 3), RightOpen(10, 13)]);
    assert_eq_u!(a.union(&Closed(10, 13)),    [LeftOpen(0, 3), Closed(10, 13)]);
    assert_eq_u!(a.union(&UpTo(13)),          [UpTo(13)]);
    assert_eq_u!(a.union(&UpFrom(13)),        [LeftOpen(0, 3), UpFrom(13)]);
    assert_eq_u!(a.union(&To(13)),            [To(13)]);
    assert_eq_u!(a.union(&From(13)),          [LeftOpen(0, 3), From(13)]);
    assert_eq_u!(a.union(&Full),              [Full]);
}

#[test]
fn right_open_center() {
    let a: RawInterval<i32> = RightOpen(0, 3);

    assert_eq_u!(a.union(&Empty),             [RightOpen(0, 3)]);
    assert_eq_u!(a.union(&Point(3)),          [Closed(0, 3)]);
    assert_eq_u!(a.union(&Open(0, 3)),        [RightOpen(0, 3)]);
    assert_eq_u!(a.union(&LeftOpen(0, 3)),    [Closed(0, 3)]);
    assert_eq_u!(a.union(&RightOpen(0, 3)),   [RightOpen(0, 3)]);
    assert_eq_u!(a.union(&Closed(0, 3)),      [Closed(0, 3)]);
    assert_eq_u!(a.union(&UpTo(3)),           [UpTo(3)]);
    assert_eq_u!(a.union(&UpFrom(3)),         [RightOpen(0, 3), UpFrom(3)]);
    assert_eq_u!(a.union(&To(3)),             [To(3)]);
    assert_eq_u!(a.union(&From(3)),           [From(0)]);
    assert_eq_u!(a.union(&Full),              [Full]);
}

#[test]
fn right_open_left() {
    let a: RawInterval<i32> = RightOpen(0, 3);

    assert_eq_u!(a.union(&Empty),             [RightOpen(0, 3)]);
    assert_eq_u!(a.union(&Point(-3)),         [RightOpen(0, 3), Point(-3)]);
    assert_eq_u!(a.union(&Open(-3, -1)),      [RightOpen(0, 3), Open(-3, -1)]);
    assert_eq_u!(a.union(&LeftOpen(-3, -1)),  [RightOpen(0, 3), LeftOpen(-3, -1)]);
    assert_eq_u!(a.union(&RightOpen(-3, -1)), [RightOpen(0, 3), RightOpen(-3, -1)]);
    assert_eq_u!(a.union(&Closed(-3, -1)),    [RightOpen(0, 3), Closed(-3, -1)]);
    assert_eq_u!(a.union(&UpTo(-3)),          [RightOpen(0, 3), UpTo(-3)]);
    assert_eq_u!(a.union(&UpFrom(-3)),        [UpFrom(-3)]);
    assert_eq_u!(a.union(&To(-3)),            [RightOpen(0, 3), To(-3)]);
    assert_eq_u!(a.union(&From(-3)),          [From(-3)]);
    assert_eq_u!(a.union(&Full),              [Full]);
}

#[test]
fn right_open_right() {
    let a: RawInterval<i32> = RightOpen(0, 3);

    assert_eq_u!(a.union(&Empty),             [RightOpen(0, 3)]);
    assert_eq_u!(a.union(&Point(13)),         [RightOpen(0, 3), Point(13)]);
    assert_eq_u!(a.union(&Open(10, 13)),      [RightOpen(0, 3), Open(10, 13)]);
    assert_eq_u!(a.union(&LeftOpen(10, 13)),  [RightOpen(0, 3), LeftOpen(10, 13)]);
    assert_eq_u!(a.union(&RightOpen(10, 13)), [RightOpen(0, 3), RightOpen(10, 13)]);
    assert_eq_u!(a.union(&Closed(10, 13)),    [RightOpen(0, 3), Closed(10, 13)]);
    assert_eq_u!(a.union(&UpTo(13)),          [UpTo(13)]);
    assert_eq_u!(a.union(&UpFrom(13)),        [RightOpen(0, 3), UpFrom(13)]);
    assert_eq_u!(a.union(&To(13)),            [To(13)]);
    assert_eq_u!(a.union(&From(13)),          [RightOpen(0, 3), From(13)]);
    assert_eq_u!(a.union(&Full),              [Full]);
}

#[test]
fn closed_center() {
    let a: RawInterval<i32> = Closed(0, 3);

    assert_eq_u!(a.union(&Empty),             [Closed(0, 3)]);
    assert_eq_u!(a.union(&Point(3)),          [Closed(0, 3)]);
    assert_eq_u!(a.union(&Open(0, 3)),        [Closed(0, 3)]);
    assert_eq_u!(a.union(&LeftOpen(0, 3)),    [Closed(0, 3)]);
    assert_eq_u!(a.union(&RightOpen(0, 3)),   [Closed(0, 3)]);
    assert_eq_u!(a.union(&Closed(0, 3)),      [Closed(0, 3)]);
    assert_eq_u!(a.union(&UpTo(3)),           [To(3)]);
    assert_eq_u!(a.union(&UpFrom(3)),         [From(0)]);
    assert_eq_u!(a.union(&To(3)),             [To(3)]);
    assert_eq_u!(a.union(&From(3)),           [From(0)]);
    assert_eq_u!(a.union(&Full),              [Full]);
}

#[test]
fn closed_left() {
    let a: RawInterval<i32> = Closed(0, 3);

    assert_eq_u!(a.union(&Empty),             [Closed(0, 3)]);
    assert_eq_u!(a.union(&Point(-3)),         [Closed(0, 3), Point(-3)]);
    assert_eq_u!(a.union(&Open(-3, -1)),      [Closed(0, 3), Open(-3, -1)]);
    assert_eq_u!(a.union(&LeftOpen(-3, -1)),  [Closed(0, 3), LeftOpen(-3, -1)]);
    assert_eq_u!(a.union(&RightOpen(-3, -1)), [Closed(0, 3), RightOpen(-3, -1)]);
    assert_eq_u!(a.union(&Closed(-3, -1)),    [Closed(0, 3), Closed(-3, -1)]);
    assert_eq_u!(a.union(&UpTo(-3)),          [Closed(0, 3), UpTo(-3)]);
    assert_eq_u!(a.union(&UpFrom(-3)),        [UpFrom(-3)]);
    assert_eq_u!(a.union(&To(-3)),            [Closed(0, 3), To(-3)]);
    assert_eq_u!(a.union(&From(-3)),          [From(-3)]);
    assert_eq_u!(a.union(&Full),              [Full]);
}

#[test]
fn closed_right() {
    let a: RawInterval<i32> = Closed(0, 3);

    assert_eq_u!(a.union(&Empty),             [Closed(0, 3)]);
    assert_eq_u!(a.union(&Point(13)),         [Closed(0, 3), Point(13)]);
    assert_eq_u!(a.union(&Open(10, 13)),      [Closed(0, 3), Open(10, 13)]);
    assert_eq_u!(a.union(&LeftOpen(10, 13)),  [Closed(0, 3), LeftOpen(10, 13)]);
    assert_eq_u!(a.union(&RightOpen(10, 13)), [Closed(0, 3), RightOpen(10, 13)]);
    assert_eq_u!(a.union(&Closed(10, 13)),    [Closed(0, 3), Closed(10, 13)]);
    assert_eq_u!(a.union(&UpTo(13)),          [UpTo(13)]);
    assert_eq_u!(a.union(&UpFrom(13)),        [Closed(0, 3), UpFrom(13)]);
    assert_eq_u!(a.union(&To(13)),            [To(13)]);
    assert_eq_u!(a.union(&From(13)),          [Closed(0, 3), From(13)]);
    assert_eq_u!(a.union(&Full),              [Full]);
}

#[test]
fn up_to_center() {
    let a: RawInterval<i32> = UpTo(3);

    assert_eq_u!(a.union(&Empty),             [UpTo(3)]);
    assert_eq_u!(a.union(&Point(3)),          [To(3)]);
    assert_eq_u!(a.union(&Open(0, 3)),        [UpTo(3)]);
    assert_eq_u!(a.union(&LeftOpen(0, 3)),    [To(3)]);
    assert_eq_u!(a.union(&RightOpen(0, 3)),   [UpTo(3)]);
    assert_eq_u!(a.union(&Closed(0, 3)),      [To(3)]);
    assert_eq_u!(a.union(&UpTo(3)),           [UpTo(3)]);
    assert_eq_u!(a.union(&UpFrom(3)),         [UpTo(3), UpFrom(3)]);
    assert_eq_u!(a.union(&To(3)),             [To(3)]);
    assert_eq_u!(a.union(&From(3)),           [Full]);
    assert_eq_u!(a.union(&Full),              [Full]);
}

#[test]
fn up_to_left() {
    let a: RawInterval<i32> = UpTo(3);

    assert_eq_u!(a.union(&Empty),             [UpTo(3)]);
    assert_eq_u!(a.union(&Point(-3)),         [UpTo(3)]);
    assert_eq_u!(a.union(&Open(-3, -1)),      [UpTo(3)]);
    assert_eq_u!(a.union(&LeftOpen(-3, -1)),  [UpTo(3)]);
    assert_eq_u!(a.union(&RightOpen(-3, -1)), [UpTo(3)]);
    assert_eq_u!(a.union(&Closed(-3, -1)),    [UpTo(3)]);
    assert_eq_u!(a.union(&UpTo(-3)),          [UpTo(3)]);
    assert_eq_u!(a.union(&UpFrom(-3)),        [Full]);
    assert_eq_u!(a.union(&To(-3)),            [UpTo(3)]);
    assert_eq_u!(a.union(&From(-3)),          [Full]);
    assert_eq_u!(a.union(&Full),              [Full]);
}

#[test]
fn up_to_right() {
    let a: RawInterval<i32> = UpTo(3);

    assert_eq_u!(a.union(&Empty),             [UpTo(3)]);
    assert_eq_u!(a.union(&Point(13)),         [UpTo(3), Point(13)]);
    assert_eq_u!(a.union(&Open(10, 13)),      [UpTo(3), Open(10, 13)]);
    assert_eq_u!(a.union(&LeftOpen(10, 13)),  [UpTo(3), LeftOpen(10, 13)]);
    assert_eq_u!(a.union(&RightOpen(10, 13)), [UpTo(3), RightOpen(10, 13)]);
    assert_eq_u!(a.union(&Closed(10, 13)),    [UpTo(3), Closed(10, 13)]);
    assert_eq_u!(a.union(&UpTo(13)),          [UpTo(13)]);
    assert_eq_u!(a.union(&UpFrom(13)),        [UpTo(3), UpFrom(13)]);
    assert_eq_u!(a.union(&To(13)),            [To(13)]);
    assert_eq_u!(a.union(&From(13)),          [UpTo(3), From(13)]);
    assert_eq_u!(a.union(&Full),              [Full]);
}

#[test]
fn up_from_center() {
    let a: RawInterval<i32> = UpFrom(0);

    assert_eq_u!(a.union(&Empty),             [UpFrom(0)]);
    assert_eq_u!(a.union(&Point(0)),          [From(0)]);
    assert_eq_u!(a.union(&Open(0, 3)),        [UpFrom(0)]);
    assert_eq_u!(a.union(&LeftOpen(0, 3)),    [UpFrom(0)]);
    assert_eq_u!(a.union(&RightOpen(0, 3)),   [From(0)]);
    assert_eq_u!(a.union(&Closed(0, 3)),      [From(0)]);
    assert_eq_u!(a.union(&UpTo(0)),           [UpTo(0), UpFrom(0)]);
    assert_eq_u!(a.union(&UpFrom(0)),         [UpFrom(0)]);
    assert_eq_u!(a.union(&To(0)),             [Full]);
    assert_eq_u!(a.union(&From(0)),           [From(0)]);
    assert_eq_u!(a.union(&Full),              [Full]);
}

#[test]
fn up_from_left() {
    let a: RawInterval<i32> = UpFrom(0);

    assert_eq_u!(a.union(&Empty),             [UpFrom(0)]);
    assert_eq_u!(a.union(&Point(-3)),         [UpFrom(0), Point(-3)]);
    assert_eq_u!(a.union(&Open(-3, -1)),      [UpFrom(0), Open(-3, -1)]);
    assert_eq_u!(a.union(&LeftOpen(-3, -1)),  [UpFrom(0), LeftOpen(-3, -1)]);
    assert_eq_u!(a.union(&RightOpen(-3, -1)), [UpFrom(0), RightOpen(-3, -1)]);
    assert_eq_u!(a.union(&Closed(-3, -1)),    [UpFrom(0), Closed(-3, -1)]);
    assert_eq_u!(a.union(&UpTo(-3)),          [UpFrom(0), UpTo(-3)]);
    assert_eq_u!(a.union(&UpFrom(-3)),        [UpFrom(-3)]);
    assert_eq_u!(a.union(&To(-3)),            [UpFrom(0), To(-3)]);
    assert_eq_u!(a.union(&From(-3)),          [From(-3)]);
    assert_eq_u!(a.union(&Full),              [Full]);
}

#[test]
fn up_from_right() {
    let a: RawInterval<i32> = UpFrom(0);

    assert_eq_u!(a.union(&Empty),             [UpFrom(0)]);
    assert_eq_u!(a.union(&Point(13)),         [UpFrom(0)]);
    assert_eq_u!(a.union(&Open(10, 13)),      [UpFrom(0)]);
    assert_eq_u!(a.union(&LeftOpen(10, 13)),  [UpFrom(0)]);
    assert_eq_u!(a.union(&RightOpen(10, 13)), [UpFrom(0)]);
    assert_eq_u!(a.union(&Closed(10, 13)),    [UpFrom(0)]);
    assert_eq_u!(a.union(&UpTo(13)),          [Full]);
    assert_eq_u!(a.union(&UpFrom(13)),        [UpFrom(0)]);
    assert_eq_u!(a.union(&To(13)),            [Full]);
    assert_eq_u!(a.union(&From(13)),          [UpFrom(0)]);
    assert_eq_u!(a.union(&Full),              [Full]);
}

#[test]
fn to_center() {
    let a: RawInterval<i32> = To(3);

    assert_eq_u!(a.union(&Empty),             [To(3)]);
    assert_eq_u!(a.union(&Point(3)),          [To(3)]);
    assert_eq_u!(a.union(&Open(0, 3)),        [To(3)]);
    assert_eq_u!(a.union(&LeftOpen(0, 3)),    [To(3)]);
    assert_eq_u!(a.union(&RightOpen(0, 3)),   [To(3)]);
    assert_eq_u!(a.union(&Closed(0, 3)),      [To(3)]);
    assert_eq_u!(a.union(&UpTo(3)),           [To(3)]);
    assert_eq_u!(a.union(&UpFrom(3)),         [Full]);
    assert_eq_u!(a.union(&To(3)),             [To(3)]);
    assert_eq_u!(a.union(&From(3)),           [Full]);
    assert_eq_u!(a.union(&Full),              [Full]);
}

#[test]
fn to_left() {
    let a: RawInterval<i32> = To(3);

    assert_eq_u!(a.union(&Empty),             [To(3)]);
    assert_eq_u!(a.union(&Point(-3)),         [To(3)]);
    assert_eq_u!(a.union(&Open(-3, -1)),      [To(3)]);
    assert_eq_u!(a.union(&LeftOpen(-3, -1)),  [To(3)]);
    assert_eq_u!(a.union(&RightOpen(-3, -1)), [To(3)]);
    assert_eq_u!(a.union(&Closed(-3, -1)),    [To(3)]);
    assert_eq_u!(a.union(&UpTo(-3)),          [To(3)]);
    assert_eq_u!(a.union(&UpFrom(-3)),        [Full]);
    assert_eq_u!(a.union(&To(-3)),            [To(3)]);
    assert_eq_u!(a.union(&From(-3)),          [Full]);
    assert_eq_u!(a.union(&Full),              [Full]);
}

#[test]
fn to_right() {
    let a: RawInterval<i32> = To(3);

    assert_eq_u!(a.union(&Empty),             [To(3)]);
    assert_eq_u!(a.union(&Point(13)),         [To(3), Point(13)]);
    assert_eq_u!(a.union(&Open(10, 13)),      [To(3), Open(10, 13)]);
    assert_eq_u!(a.union(&LeftOpen(10, 13)),  [To(3), LeftOpen(10, 13)]);
    assert_eq_u!(a.union(&RightOpen(10, 13)), [To(3), RightOpen(10, 13)]);
    assert_eq_u!(a.union(&Closed(10, 13)),    [To(3), Closed(10, 13)]);
    assert_eq_u!(a.union(&UpTo(13)),          [UpTo(13)]);
    assert_eq_u!(a.union(&UpFrom(13)),        [To(3), UpFrom(13)]);
    assert_eq_u!(a.union(&To(13)),            [To(13)]);
    assert_eq_u!(a.union(&From(13)),          [To(3), From(13)]);
    assert_eq_u!(a.union(&Full),              [Full]);
}

#[test]
fn from_center() {
    let a: RawInterval<i32> = From(0);

    assert_eq_u!(a.union(&Empty),             [From(0)]);
    assert_eq_u!(a.union(&Point(0)),          [From(0)]);
    assert_eq_u!(a.union(&Open(0, 3)),        [From(0)]);
    assert_eq_u!(a.union(&LeftOpen(0, 3)),    [From(0)]);
    assert_eq_u!(a.union(&RightOpen(0, 3)),   [From(0)]);
    assert_eq_u!(a.union(&Closed(0, 3)),      [From(0)]);
    assert_eq_u!(a.union(&UpTo(0)),           [Full]);
    assert_eq_u!(a.union(&UpFrom(0)),         [From(0)]);
    assert_eq_u!(a.union(&To(0)),             [Full]);
    assert_eq_u!(a.union(&From(0)),           [From(0)]);
    assert_eq_u!(a.union(&Full),              [Full]);
}

#[test]
fn from_left() {
    let a: RawInterval<i32> = From(0);

    assert_eq_u!(a.union(&Empty),             [From(0)]);
    assert_eq_u!(a.union(&Point(-3)),         [From(0), Point(-3)]);
    assert_eq_u!(a.union(&Open(-3, -1)),      [From(0), Open(-3, -1)]);
    assert_eq_u!(a.union(&LeftOpen(-3, -1)),  [From(0), LeftOpen(-3, -1)]);
    assert_eq_u!(a.union(&RightOpen(-3, -1)), [From(0), RightOpen(-3, -1)]);
    assert_eq_u!(a.union(&Closed(-3, -1)),    [From(0), Closed(-3, -1)]);
    assert_eq_u!(a.union(&UpTo(-3)),          [From(0), UpTo(-3)]);
    assert_eq_u!(a.union(&UpFrom(-3)),        [UpFrom(-3)]);
    assert_eq_u!(a.union(&To(-3)),            [From(0), To(-3)]);
    assert_eq_u!(a.union(&From(-3)),          [From(-3)]);
    assert_eq_u!(a.union(&Full),              [Full]);
}

#[test]
fn from_right() {
    let a: RawInterval<i32> = From(0);

    assert_eq_u!(a.union(&Empty),             [From(0)]);
    assert_eq_u!(a.union(&Point(13)),         [From(0)]);
    assert_eq_u!(a.union(&Open(10, 13)),      [From(0)]);
    assert_eq_u!(a.union(&LeftOpen(10, 13)),  [From(0)]);
    assert_eq_u!(a.union(&RightOpen(10, 13)), [From(0)]);
    assert_eq_u!(a.union(&Closed(10, 13)),    [From(0)]);
    assert_eq_u!(a.union(&UpTo(13)),          [Full]);
    assert_eq_u!(a.union(&UpFrom(13)),        [From(0)]);
    assert_eq_u!(a.union(&To(13)),            [Full]);
    assert_eq_u!(a.union(&From(13)),          [From(0)]);
    assert_eq_u!(a.union(&Full),              [Full]);
}

#[test]
fn full() {
    let a: RawInterval<i32> = Full;

    assert_eq_u!(a.union(&Empty),             [Full]);
    assert_eq_u!(a.union(&Point(0)),          [Full]);
    assert_eq_u!(a.union(&Open(0, 3)),        [Full]);
    assert_eq_u!(a.union(&LeftOpen(0, 3)),    [Full]);
    assert_eq_u!(a.union(&RightOpen(0, 3)),   [Full]);
    assert_eq_u!(a.union(&Closed(0, 3)),      [Full]);
    assert_eq_u!(a.union(&UpTo(0)),           [Full]);
    assert_eq_u!(a.union(&UpFrom(0)),         [Full]);
    assert_eq_u!(a.union(&To(0)),             [Full]);
    assert_eq_u!(a.union(&From(0)),           [Full]);
    assert_eq_u!(a.union(&Full),              [Full]);
}
