// Copyright 2018 Skylor R. Schermer.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
////////////////////////////////////////////////////////////////////////////////
//!
//! Testing module for [`intersect`] operations.
//!
//! [`intersect`]: struct.RawInterval.html#method.intersect
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

    assert_eq!(a.intersect(&Empty),             Empty);
    assert_eq!(a.intersect(&Point(3)),          Empty);
    assert_eq!(a.intersect(&Open(0, 3)),        Empty);
    assert_eq!(a.intersect(&LeftOpen(0, 3)),    Empty);
    assert_eq!(a.intersect(&RightOpen(0, 3)),   Empty);
    assert_eq!(a.intersect(&Closed(0, 3)),      Empty);
    assert_eq!(a.intersect(&UpTo(3)),           Empty);
    assert_eq!(a.intersect(&UpFrom(3)),         Empty);
    assert_eq!(a.intersect(&To(3)),             Empty);
    assert_eq!(a.intersect(&From(3)),           Empty);
    assert_eq!(a.intersect(&Full),              Empty);
}

#[test]
fn point_center() {
    let a: RawInterval<i32> = Point(2);

    assert_eq!(a.intersect(&Empty),             Empty);
    assert_eq!(a.intersect(&Point(2)),          Point(2));
    assert_eq!(a.intersect(&Open(0, 3)),        Point(2));
    assert_eq!(a.intersect(&LeftOpen(0, 3)),    Point(2));
    assert_eq!(a.intersect(&RightOpen(0, 3)),   Point(2));
    assert_eq!(a.intersect(&Closed(0, 3)),      Point(2));
    assert_eq!(a.intersect(&UpTo(2)),           Empty);
    assert_eq!(a.intersect(&UpFrom(2)),         Empty);
    assert_eq!(a.intersect(&To(2)),             Point(2));
    assert_eq!(a.intersect(&From(2)),           Point(2));
    assert_eq!(a.intersect(&Full),              Point(2));
}

#[test]
fn point_left() {
    let a: RawInterval<i32> = Point(2);

    assert_eq!(a.intersect(&Empty),             Empty);
    assert_eq!(a.intersect(&Point(-1)),         Empty);
    assert_eq!(a.intersect(&Open(-3, -1)),      Empty);
    assert_eq!(a.intersect(&LeftOpen(-3, -1)),  Empty);
    assert_eq!(a.intersect(&RightOpen(-3, -1)), Empty);
    assert_eq!(a.intersect(&Closed(-3, -1)),    Empty);
    assert_eq!(a.intersect(&UpTo(-3)),          Empty);
    assert_eq!(a.intersect(&UpFrom(-3)),        Point(2));
    assert_eq!(a.intersect(&To(-3)),            Empty);
    assert_eq!(a.intersect(&From(-3)),          Point(2));
    assert_eq!(a.intersect(&Full),              Point(2));
}

#[test]
fn point_right() {
    let a: RawInterval<i32> = Point(2);

    assert_eq!(a.intersect(&Empty),             Empty);
    assert_eq!(a.intersect(&Point(10)),         Empty);
    assert_eq!(a.intersect(&Open(10, 13)),      Empty);
    assert_eq!(a.intersect(&LeftOpen(10, 13)),  Empty);
    assert_eq!(a.intersect(&RightOpen(10, 13)), Empty);
    assert_eq!(a.intersect(&Closed(10, 13)),    Empty);
    assert_eq!(a.intersect(&UpTo(13)),          Point(2));
    assert_eq!(a.intersect(&UpFrom(13)),        Empty);
    assert_eq!(a.intersect(&To(13)),            Point(2));
    assert_eq!(a.intersect(&From(13)),          Empty);
    assert_eq!(a.intersect(&Full),              Point(2));
}

#[test]
fn open_center() {
    let a: RawInterval<i32> = Open(0, 3);

    assert_eq!(a.intersect(&Empty),             Empty);
    assert_eq!(a.intersect(&Point(2)),          Point(2));
    assert_eq!(a.intersect(&Open(0, 3)),        Open(0, 3));
    assert_eq!(a.intersect(&LeftOpen(0, 3)),    Open(0, 3));
    assert_eq!(a.intersect(&RightOpen(0, 3)),   Open(0, 3));
    assert_eq!(a.intersect(&Closed(0, 3)),      Open(0, 3));
    assert_eq!(a.intersect(&UpTo(2)),           Open(0, 2));
    assert_eq!(a.intersect(&UpFrom(2)),         Open(2, 3));
    assert_eq!(a.intersect(&To(2)),             LeftOpen(0, 2));
    assert_eq!(a.intersect(&From(2)),           RightOpen(2, 3));
    assert_eq!(a.intersect(&Full),              Open(0, 3));
}

#[test]
fn open_left() {
    let a: RawInterval<i32> = Open(0, 3);

    assert_eq!(a.intersect(&Empty),             Empty);
    assert_eq!(a.intersect(&Point(-3)),         Empty);
    assert_eq!(a.intersect(&Open(-3, -1)),      Empty);
    assert_eq!(a.intersect(&LeftOpen(-3, -1)),  Empty);
    assert_eq!(a.intersect(&RightOpen(-3, -1)), Empty);
    assert_eq!(a.intersect(&Closed(-3, -1)),    Empty);
    assert_eq!(a.intersect(&UpTo(-3)),          Empty);
    assert_eq!(a.intersect(&UpFrom(-3)),        Open(0, 3));
    assert_eq!(a.intersect(&To(-3)),            Empty);
    assert_eq!(a.intersect(&From(-3)),          Open(0, 3));
    assert_eq!(a.intersect(&Full),              Open(0, 3));
}

#[test]
fn open_right() {
    let a: RawInterval<i32> = Open(0, 3);

    assert_eq!(a.intersect(&Empty),             Empty);
    assert_eq!(a.intersect(&Point(13)),         Empty);
    assert_eq!(a.intersect(&Open(10, 13)),      Empty);
    assert_eq!(a.intersect(&LeftOpen(10, 13)),  Empty);
    assert_eq!(a.intersect(&RightOpen(10, 13)), Empty);
    assert_eq!(a.intersect(&Closed(10, 13)),    Empty);
    assert_eq!(a.intersect(&UpTo(13)),          Open(0, 3));
    assert_eq!(a.intersect(&UpFrom(13)),        Empty);
    assert_eq!(a.intersect(&To(13)),            Open(0, 3));
    assert_eq!(a.intersect(&From(13)),          Empty);
    assert_eq!(a.intersect(&Full),              Open(0, 3));
}

#[test]
fn left_open_center() {
    let a: RawInterval<i32> = LeftOpen(0, 3);

    assert_eq!(a.intersect(&Empty),             Empty);
    assert_eq!(a.intersect(&Point(2)),          Point(2));
    assert_eq!(a.intersect(&Open(0, 3)),        Open(0, 3));
    assert_eq!(a.intersect(&LeftOpen(0, 3)),    LeftOpen(0, 3));
    assert_eq!(a.intersect(&RightOpen(0, 3)),   Open(0, 3));
    assert_eq!(a.intersect(&Closed(0, 3)),      LeftOpen(0, 3));
    assert_eq!(a.intersect(&UpTo(2)),           Open(0, 2));
    assert_eq!(a.intersect(&UpFrom(2)),         LeftOpen(2, 3));
    assert_eq!(a.intersect(&To(2)),             LeftOpen(0, 2));
    assert_eq!(a.intersect(&From(2)),           Closed(2, 3));
    assert_eq!(a.intersect(&Full),              LeftOpen(0, 3));
}

#[test]
fn left_open_left() {
    let a: RawInterval<i32> = LeftOpen(0, 3);

    assert_eq!(a.intersect(&Empty),             Empty);
    assert_eq!(a.intersect(&Point(-3)),         Empty);
    assert_eq!(a.intersect(&Open(-3, -1)),      Empty);
    assert_eq!(a.intersect(&LeftOpen(-3, -1)),  Empty);
    assert_eq!(a.intersect(&RightOpen(-3, -1)), Empty);
    assert_eq!(a.intersect(&Closed(-3, -1)),    Empty);
    assert_eq!(a.intersect(&UpTo(-3)),          Empty);
    assert_eq!(a.intersect(&UpFrom(-3)),        LeftOpen(0, 3));
    assert_eq!(a.intersect(&To(-3)),            Empty);
    assert_eq!(a.intersect(&From(-3)),          LeftOpen(0, 3));
    assert_eq!(a.intersect(&Full),              LeftOpen(0, 3));
}

#[test]
fn left_open_right() {
    let a: RawInterval<i32> = LeftOpen(0, 3);

    assert_eq!(a.intersect(&Empty),             Empty);
    assert_eq!(a.intersect(&Point(13)),         Empty);
    assert_eq!(a.intersect(&Open(10, 13)),      Empty);
    assert_eq!(a.intersect(&LeftOpen(10, 13)),  Empty);
    assert_eq!(a.intersect(&RightOpen(10, 13)), Empty);
    assert_eq!(a.intersect(&Closed(10, 13)),    Empty);
    assert_eq!(a.intersect(&UpTo(13)),          LeftOpen(0, 3));
    assert_eq!(a.intersect(&UpFrom(13)),        Empty);
    assert_eq!(a.intersect(&To(13)),            LeftOpen(0, 3));
    assert_eq!(a.intersect(&From(13)),          Empty);
    assert_eq!(a.intersect(&Full),              LeftOpen(0, 3));
}

#[test]
fn right_open_center() {
    let a: RawInterval<i32> = RightOpen(0, 3);

    assert_eq!(a.intersect(&Empty),             Empty);
    assert_eq!(a.intersect(&Point(2)),          Point(2));
    assert_eq!(a.intersect(&Open(0, 3)),        Open(0, 3));
    assert_eq!(a.intersect(&LeftOpen(0, 3)),    Open(0, 3));
    assert_eq!(a.intersect(&RightOpen(0, 3)),   RightOpen(0, 3));
    assert_eq!(a.intersect(&Closed(0, 3)),      RightOpen(0, 3));
    assert_eq!(a.intersect(&UpTo(2)),           RightOpen(0, 2));
    assert_eq!(a.intersect(&UpFrom(2)),         Open(2, 3));
    assert_eq!(a.intersect(&To(2)),             Closed(0, 2));
    assert_eq!(a.intersect(&From(2)),           RightOpen(2, 3));
    assert_eq!(a.intersect(&Full),              RightOpen(0, 3));
}

#[test]
fn right_open_left() {
    let a: RawInterval<i32> = RightOpen(0, 3);

    assert_eq!(a.intersect(&Empty),             Empty);
    assert_eq!(a.intersect(&Point(-3)),         Empty);
    assert_eq!(a.intersect(&Open(-3, -1)),      Empty);
    assert_eq!(a.intersect(&LeftOpen(-3, -1)),  Empty);
    assert_eq!(a.intersect(&RightOpen(-3, -1)), Empty);
    assert_eq!(a.intersect(&Closed(-3, -1)),    Empty);
    assert_eq!(a.intersect(&UpTo(-3)),          Empty);
    assert_eq!(a.intersect(&UpFrom(-3)),        RightOpen(0, 3));
    assert_eq!(a.intersect(&To(-3)),            Empty);
    assert_eq!(a.intersect(&From(-3)),          RightOpen(0, 3));
    assert_eq!(a.intersect(&Full),              RightOpen(0, 3));
}

#[test]
fn right_open_right() {
    let a: RawInterval<i32> = RightOpen(0, 3);

    assert_eq!(a.intersect(&Empty),             Empty);
    assert_eq!(a.intersect(&Point(13)),         Empty);
    assert_eq!(a.intersect(&Open(10, 13)),      Empty);
    assert_eq!(a.intersect(&LeftOpen(10, 13)),  Empty);
    assert_eq!(a.intersect(&RightOpen(10, 13)), Empty);
    assert_eq!(a.intersect(&Closed(10, 13)),    Empty);
    assert_eq!(a.intersect(&UpTo(13)),          RightOpen(0, 3));
    assert_eq!(a.intersect(&UpFrom(13)),        Empty);
    assert_eq!(a.intersect(&To(13)),            RightOpen(0, 3));
    assert_eq!(a.intersect(&From(13)),          Empty);
    assert_eq!(a.intersect(&Full),              RightOpen(0, 3));
}

#[test]
fn closed_center() {
    let a: RawInterval<i32> = Closed(0, 3);

    assert_eq!(a.intersect(&Empty),             Empty);
    assert_eq!(a.intersect(&Point(2)),          Point(2));
    assert_eq!(a.intersect(&Open(0, 3)),        Open(0, 3));
    assert_eq!(a.intersect(&LeftOpen(0, 3)),    LeftOpen(0, 3));
    assert_eq!(a.intersect(&RightOpen(0, 3)),   RightOpen(0, 3));
    assert_eq!(a.intersect(&Closed(0, 3)),      Closed(0, 3));
    assert_eq!(a.intersect(&UpTo(2)),           RightOpen(0, 2));
    assert_eq!(a.intersect(&UpFrom(2)),         LeftOpen(2, 3));
    assert_eq!(a.intersect(&To(2)),             Closed(0, 2));
    assert_eq!(a.intersect(&From(2)),           Closed(2, 3));
    assert_eq!(a.intersect(&Full),              Closed(0, 3));
}

#[test]
fn closed_left() {
    let a: RawInterval<i32> = Closed(0, 3);

    assert_eq!(a.intersect(&Empty),             Empty);
    assert_eq!(a.intersect(&Point(-3)),         Empty);
    assert_eq!(a.intersect(&Open(-3, -1)),      Empty);
    assert_eq!(a.intersect(&LeftOpen(-3, -1)),  Empty);
    assert_eq!(a.intersect(&RightOpen(-3, -1)), Empty);
    assert_eq!(a.intersect(&Closed(-3, -1)),    Empty);
    assert_eq!(a.intersect(&UpTo(-3)),          Empty);
    assert_eq!(a.intersect(&UpFrom(-3)),        Closed(0, 3));
    assert_eq!(a.intersect(&To(-3)),            Empty);
    assert_eq!(a.intersect(&From(-3)),          Closed(0, 3));
    assert_eq!(a.intersect(&Full),              Closed(0, 3));
}

#[test]
fn closed_right() {
    let a: RawInterval<i32> = Closed(0, 3);

    assert_eq!(a.intersect(&Empty),             Empty);
    assert_eq!(a.intersect(&Point(13)),         Empty);
    assert_eq!(a.intersect(&Open(10, 13)),      Empty);
    assert_eq!(a.intersect(&LeftOpen(10, 13)),  Empty);
    assert_eq!(a.intersect(&RightOpen(10, 13)), Empty);
    assert_eq!(a.intersect(&Closed(10, 13)),    Empty);
    assert_eq!(a.intersect(&UpTo(13)),          Closed(0, 3));
    assert_eq!(a.intersect(&UpFrom(13)),        Empty);
    assert_eq!(a.intersect(&To(13)),            Closed(0, 3));
    assert_eq!(a.intersect(&From(13)),          Empty);
    assert_eq!(a.intersect(&Full),              Closed(0, 3));
}

#[test]
fn up_to_center() {
    let a: RawInterval<i32> = UpTo(3);

    assert_eq!(a.intersect(&Empty),             Empty);
    assert_eq!(a.intersect(&Point(2)),          Point(2));
    assert_eq!(a.intersect(&Open(0, 3)),        Open(0, 3));
    assert_eq!(a.intersect(&LeftOpen(0, 3)),    Open(0, 3));
    assert_eq!(a.intersect(&RightOpen(0, 3)),   RightOpen(0, 3));
    assert_eq!(a.intersect(&Closed(0, 3)),      RightOpen(0, 3));
    assert_eq!(a.intersect(&UpTo(2)),           UpTo(2));
    assert_eq!(a.intersect(&UpFrom(2)),         Open(2, 3));
    assert_eq!(a.intersect(&To(2)),             To(2));
    assert_eq!(a.intersect(&From(2)),           RightOpen(2, 3));
    assert_eq!(a.intersect(&Full),              UpTo(3));
}

#[test]
fn up_to_left() {
    let a: RawInterval<i32> = UpTo(3);

    assert_eq!(a.intersect(&Empty),             Empty);
    assert_eq!(a.intersect(&Point(-3)),         Point(-3));
    assert_eq!(a.intersect(&Open(-3, -1)),      Open(-3, -1));
    assert_eq!(a.intersect(&LeftOpen(-3, -1)),  LeftOpen(-3, -1));
    assert_eq!(a.intersect(&RightOpen(-3, -1)), RightOpen(-3, -1));
    assert_eq!(a.intersect(&Closed(-3, -1)),    Closed(-3, -1));
    assert_eq!(a.intersect(&UpTo(-3)),          UpTo(-3));
    assert_eq!(a.intersect(&UpFrom(-3)),        Open(-3, 3));
    assert_eq!(a.intersect(&To(-3)),            To(-3));
    assert_eq!(a.intersect(&From(-3)),          RightOpen(-3, 3));
    assert_eq!(a.intersect(&Full),              UpTo(3));
}

#[test]
fn up_to_right() {
    let a: RawInterval<i32> = UpTo(3);

    assert_eq!(a.intersect(&Empty),             Empty);
    assert_eq!(a.intersect(&Point(13)),         Empty);
    assert_eq!(a.intersect(&Open(10, 13)),      Empty);
    assert_eq!(a.intersect(&LeftOpen(10, 13)),  Empty);
    assert_eq!(a.intersect(&RightOpen(10, 13)), Empty);
    assert_eq!(a.intersect(&Closed(10, 13)),    Empty);
    assert_eq!(a.intersect(&UpTo(13)),          UpTo(3));
    assert_eq!(a.intersect(&UpFrom(13)),        Empty);
    assert_eq!(a.intersect(&To(13)),            UpTo(3));
    assert_eq!(a.intersect(&From(13)),          Empty);
    assert_eq!(a.intersect(&Full),              UpTo(3));
}

#[test]
fn up_from_center() {
    let a: RawInterval<i32> = UpFrom(0);

    assert_eq!(a.intersect(&Empty),             Empty);
    assert_eq!(a.intersect(&Point(2)),          Point(2));
    assert_eq!(a.intersect(&Open(0, 3)),        Open(0, 3));
    assert_eq!(a.intersect(&LeftOpen(0, 3)),    LeftOpen(0, 3));
    assert_eq!(a.intersect(&RightOpen(0, 3)),   Open(0, 3));
    assert_eq!(a.intersect(&Closed(0, 3)),      LeftOpen(0, 3));
    assert_eq!(a.intersect(&UpTo(2)),           Open(0, 2));
    assert_eq!(a.intersect(&UpFrom(2)),         UpFrom(2));
    assert_eq!(a.intersect(&To(2)),             LeftOpen(0, 2));
    assert_eq!(a.intersect(&From(2)),           From(2));
    assert_eq!(a.intersect(&Full),              UpFrom(0));
}

#[test]
fn up_from_left() {
    let a: RawInterval<i32> = UpFrom(0);

    assert_eq!(a.intersect(&Empty),             Empty);
    assert_eq!(a.intersect(&Point(-3)),         Empty);
    assert_eq!(a.intersect(&Open(-3, -1)),      Empty);
    assert_eq!(a.intersect(&LeftOpen(-3, -1)),  Empty);
    assert_eq!(a.intersect(&RightOpen(-3, -1)), Empty);
    assert_eq!(a.intersect(&Closed(-3, -1)),    Empty);
    assert_eq!(a.intersect(&UpTo(-3)),          Empty);
    assert_eq!(a.intersect(&UpFrom(-3)),        UpFrom(0));
    assert_eq!(a.intersect(&To(-3)),            Empty);
    assert_eq!(a.intersect(&From(-3)),          UpFrom(0));
    assert_eq!(a.intersect(&Full),              UpFrom(0));
}

#[test]
fn up_from_right() {
    let a: RawInterval<i32> = UpFrom(0);

    assert_eq!(a.intersect(&Empty),             Empty);
    assert_eq!(a.intersect(&Point(13)),         Point(13));
    assert_eq!(a.intersect(&Open(10, 13)),      Open(10, 13));
    assert_eq!(a.intersect(&LeftOpen(10, 13)),  LeftOpen(10, 13));
    assert_eq!(a.intersect(&RightOpen(10, 13)), RightOpen(10, 13));
    assert_eq!(a.intersect(&Closed(10, 13)),    Closed(10, 13));
    assert_eq!(a.intersect(&UpTo(13)),          Open(0, 13));
    assert_eq!(a.intersect(&UpFrom(13)),        UpFrom(13));
    assert_eq!(a.intersect(&To(13)),            LeftOpen(0, 13));
    assert_eq!(a.intersect(&From(13)),          From(13));
    assert_eq!(a.intersect(&Full),              UpFrom(0));
}

#[test]
fn to_center() {
    let a: RawInterval<i32> = To(3);

    assert_eq!(a.intersect(&Empty),             Empty);
    assert_eq!(a.intersect(&Point(2)),          Point(2));
    assert_eq!(a.intersect(&Open(0, 3)),        Open(0, 3));
    assert_eq!(a.intersect(&LeftOpen(0, 3)),    LeftOpen(0, 3));
    assert_eq!(a.intersect(&RightOpen(0, 3)),   RightOpen(0, 3));
    assert_eq!(a.intersect(&Closed(0, 3)),      Closed(0, 3));
    assert_eq!(a.intersect(&UpTo(2)),           UpTo(2));
    assert_eq!(a.intersect(&UpFrom(2)),         LeftOpen(2, 3));
    assert_eq!(a.intersect(&To(2)),             To(2));
    assert_eq!(a.intersect(&From(2)),           Closed(2, 3));
    assert_eq!(a.intersect(&Full),              To(3));
}

#[test]
fn to_left() {
    let a: RawInterval<i32> = To(3);

    assert_eq!(a.intersect(&Empty),             Empty);
    assert_eq!(a.intersect(&Point(-3)),         Point(-3));
    assert_eq!(a.intersect(&Open(-3, -1)),      Open(-3, -1));
    assert_eq!(a.intersect(&LeftOpen(-3, -1)),  LeftOpen(-3, -1));
    assert_eq!(a.intersect(&RightOpen(-3, -1)), RightOpen(-3, -1));
    assert_eq!(a.intersect(&Closed(-3, -1)),    Closed(-3, -1));
    assert_eq!(a.intersect(&UpTo(-3)),          UpTo(-3));
    assert_eq!(a.intersect(&UpFrom(-3)),        LeftOpen(-3, 3));
    assert_eq!(a.intersect(&To(-3)),            To(-3));
    assert_eq!(a.intersect(&From(-3)),          Closed(-3, 3));
    assert_eq!(a.intersect(&Full),              To(3));
}

#[test]
fn to_right() {
    let a: RawInterval<i32> = To(3);

    assert_eq!(a.intersect(&Empty),             Empty);
    assert_eq!(a.intersect(&Point(13)),         Empty);
    assert_eq!(a.intersect(&Open(10, 13)),      Empty);
    assert_eq!(a.intersect(&LeftOpen(10, 13)),  Empty);
    assert_eq!(a.intersect(&RightOpen(10, 13)), Empty);
    assert_eq!(a.intersect(&Closed(10, 13)),    Empty);
    assert_eq!(a.intersect(&UpTo(13)),          To(3));
    assert_eq!(a.intersect(&UpFrom(13)),        Empty);
    assert_eq!(a.intersect(&To(13)),            To(3));
    assert_eq!(a.intersect(&From(13)),          Empty);
    assert_eq!(a.intersect(&Full),              To(3));
}

#[test]
fn from_center() {
    let a: RawInterval<i32> = From(0);

    assert_eq!(a.intersect(&Empty),             Empty);
    assert_eq!(a.intersect(&Point(2)),          Point(2));
    assert_eq!(a.intersect(&Open(0, 3)),        Open(0, 3));
    assert_eq!(a.intersect(&LeftOpen(0, 3)),    LeftOpen(0, 3));
    assert_eq!(a.intersect(&RightOpen(0, 3)),   RightOpen(0, 3));
    assert_eq!(a.intersect(&Closed(0, 3)),      Closed(0, 3));
    assert_eq!(a.intersect(&UpTo(2)),           RightOpen(0, 2));
    assert_eq!(a.intersect(&UpFrom(2)),         UpFrom(2));
    assert_eq!(a.intersect(&To(2)),             Closed(0, 2));
    assert_eq!(a.intersect(&From(2)),           From(2));
    assert_eq!(a.intersect(&Full),              From(0));
}

#[test]
fn from_left() {
    let a: RawInterval<i32> = From(0);

    assert_eq!(a.intersect(&Empty),             Empty);
    assert_eq!(a.intersect(&Point(-3)),         Empty);
    assert_eq!(a.intersect(&Open(-3, -1)),      Empty);
    assert_eq!(a.intersect(&LeftOpen(-3, -1)),  Empty);
    assert_eq!(a.intersect(&RightOpen(-3, -1)), Empty);
    assert_eq!(a.intersect(&Closed(-3, -1)),    Empty);
    assert_eq!(a.intersect(&UpTo(-3)),          Empty);
    assert_eq!(a.intersect(&UpFrom(-3)),        From(0));
    assert_eq!(a.intersect(&To(-3)),            Empty);
    assert_eq!(a.intersect(&From(-3)),          From(0));
    assert_eq!(a.intersect(&Full),              From(0));
}

#[test]
fn from_right() {
    let a: RawInterval<i32> = From(0);

    assert_eq!(a.intersect(&Empty),             Empty);
    assert_eq!(a.intersect(&Point(13)),         Point(13));
    assert_eq!(a.intersect(&Open(10, 13)),      Open(10, 13));
    assert_eq!(a.intersect(&LeftOpen(10, 13)),  LeftOpen(10, 13));
    assert_eq!(a.intersect(&RightOpen(10, 13)), RightOpen(10, 13));
    assert_eq!(a.intersect(&Closed(10, 13)),    Closed(10, 13));
    assert_eq!(a.intersect(&UpTo(13)),          RightOpen(0, 13));
    assert_eq!(a.intersect(&UpFrom(13)),        UpFrom(13));
    assert_eq!(a.intersect(&To(13)),            Closed(0, 13));
    assert_eq!(a.intersect(&From(13)),          From(13));
    assert_eq!(a.intersect(&Full),              From(0));
}

#[test]
fn full() {
    let a: RawInterval<i32> = Full;

    assert_eq!(a.intersect(&Empty),             Empty);
    assert_eq!(a.intersect(&Point(0)),          Point(0));
    assert_eq!(a.intersect(&Open(0, 3)),        Open(0, 3));
    assert_eq!(a.intersect(&LeftOpen(0, 3)),    LeftOpen(0, 3));
    assert_eq!(a.intersect(&RightOpen(0, 3)),   RightOpen(0, 3));
    assert_eq!(a.intersect(&Closed(0, 3)),      Closed(0, 3));
    assert_eq!(a.intersect(&UpTo(0)),           UpTo(0));
    assert_eq!(a.intersect(&UpFrom(0)),         UpFrom(0));
    assert_eq!(a.intersect(&To(0)),             To(0));
    assert_eq!(a.intersect(&From(0)),           From(0));
    assert_eq!(a.intersect(&Full),              Full);
}
