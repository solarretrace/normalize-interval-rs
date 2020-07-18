// Copyright 2018 Skylor R. Schermer.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
////////////////////////////////////////////////////////////////////////////////
//!
//! Testing module for [`enclose`] operations.
//!
//! [`enclose`]: struct.RawInterval.html#method.enclose
//! 
////////////////////////////////////////////////////////////////////////////////

// Local imports.
use crate::raw_interval::RawInterval;

// Local enum shortcuts.
use crate::raw_interval::RawInterval::*;


#[test]
fn empty() {
    let a: RawInterval<i32> = Empty;

    assert_eq!(a.enclose(&Empty),             Empty);
    assert_eq!(a.enclose(&Point(3)),          Point(3));
    assert_eq!(a.enclose(&Open(0, 3)),        Open(0, 3));
    assert_eq!(a.enclose(&LeftOpen(0, 3)),    LeftOpen(0, 3));
    assert_eq!(a.enclose(&RightOpen(0, 3)),   RightOpen(0, 3));
    assert_eq!(a.enclose(&Closed(0, 3)),      Closed(0, 3));
    assert_eq!(a.enclose(&UpTo(3)),           UpTo(3));
    assert_eq!(a.enclose(&UpFrom(3)),         UpFrom(3));
    assert_eq!(a.enclose(&To(3)),             To(3));
    assert_eq!(a.enclose(&From(3)),           From(3));
    assert_eq!(a.enclose(&Full),              Full);
}

#[test]
fn point_center() {
    let a: RawInterval<i32> = Point(3);

    assert_eq!(a.enclose(&Empty),             Point(3));
    assert_eq!(a.enclose(&Point(3)),          Point(3));
    assert_eq!(a.enclose(&Open(0, 3)),        LeftOpen(0, 3));
    assert_eq!(a.enclose(&LeftOpen(0, 3)),    LeftOpen(0, 3));
    assert_eq!(a.enclose(&RightOpen(0, 3)),   Closed(0, 3));
    assert_eq!(a.enclose(&Closed(0, 3)),      Closed(0, 3));
    assert_eq!(a.enclose(&UpTo(3)),           To(3));
    assert_eq!(a.enclose(&UpFrom(3)),         From(3));
    assert_eq!(a.enclose(&To(3)),             To(3));
    assert_eq!(a.enclose(&From(3)),           From(3));
    assert_eq!(a.enclose(&Full),              Full);
}

#[test]
fn point_left() {
    let a: RawInterval<i32> = Point(3);

    assert_eq!(a.enclose(&Empty),             Point(3));
    assert_eq!(a.enclose(&Point(-1)),         Closed(-1, 3));
    assert_eq!(a.enclose(&Open(-3, -1)),      LeftOpen(-3, 3));
    assert_eq!(a.enclose(&LeftOpen(-3, -1)),  LeftOpen(-3, 3));
    assert_eq!(a.enclose(&RightOpen(-3, -1)), Closed(-3, 3));
    assert_eq!(a.enclose(&Closed(-3, -1)),    Closed(-3, 3));
    assert_eq!(a.enclose(&UpTo(-3)),          To(3));
    assert_eq!(a.enclose(&UpFrom(-3)),        UpFrom(-3));
    assert_eq!(a.enclose(&To(-3)),            To(3));
    assert_eq!(a.enclose(&From(-3)),          From(-3));
    assert_eq!(a.enclose(&Full),              Full);
}

#[test]
fn point_right() {
    let a: RawInterval<i32> = Point(3);

    assert_eq!(a.enclose(&Empty),             Point(3));
    assert_eq!(a.enclose(&Point(10)),         Closed(3, 10));
    assert_eq!(a.enclose(&Open(10, 13)),      RightOpen(3, 13));
    assert_eq!(a.enclose(&LeftOpen(10, 13)),  Closed(3, 13));
    assert_eq!(a.enclose(&RightOpen(10, 13)), RightOpen(3, 13));
    assert_eq!(a.enclose(&Closed(10, 13)),    Closed(3, 13));
    assert_eq!(a.enclose(&UpTo(13)),          UpTo(13));
    assert_eq!(a.enclose(&UpFrom(13)),        From(3));
    assert_eq!(a.enclose(&To(13)),            To(13));
    assert_eq!(a.enclose(&From(13)),          From(3));
    assert_eq!(a.enclose(&Full),              Full);
}

#[test]
fn open_center() {
    let a: RawInterval<i32> = Open(0, 3);

    assert_eq!(a.enclose(&Empty),             Open(0, 3));
    assert_eq!(a.enclose(&Point(3)),          LeftOpen(0, 3));
    assert_eq!(a.enclose(&Open(0, 3)),        Open(0, 3));
    assert_eq!(a.enclose(&LeftOpen(0, 3)),    LeftOpen(0, 3));
    assert_eq!(a.enclose(&RightOpen(0, 3)),   RightOpen(0, 3));
    assert_eq!(a.enclose(&Closed(0, 3)),      Closed(0, 3));
    assert_eq!(a.enclose(&UpTo(3)),           UpTo(3));
    assert_eq!(a.enclose(&UpFrom(3)),         UpFrom(0));
    assert_eq!(a.enclose(&To(3)),             To(3));
    assert_eq!(a.enclose(&From(3)),           UpFrom(0));
    assert_eq!(a.enclose(&Full),              Full);
}

#[test]
fn open_left() {
    let a: RawInterval<i32> = Open(0, 3);

    assert_eq!(a.enclose(&Empty),             Open(0, 3));
    assert_eq!(a.enclose(&Point(-3)),         RightOpen(-3, 3));
    assert_eq!(a.enclose(&Open(-3, -1)),      Open(-3, 3));
    assert_eq!(a.enclose(&LeftOpen(-3, -1)),  Open(-3, 3));
    assert_eq!(a.enclose(&RightOpen(-3, -1)), RightOpen(-3, 3));
    assert_eq!(a.enclose(&Closed(-3, -1)),    RightOpen(-3, 3));
    assert_eq!(a.enclose(&UpTo(-3)),          UpTo(3));
    assert_eq!(a.enclose(&UpFrom(-3)),        UpFrom(-3));
    assert_eq!(a.enclose(&To(-3)),            UpTo(3));
    assert_eq!(a.enclose(&From(-3)),          From(-3));
    assert_eq!(a.enclose(&Full),              Full);
}

#[test]
fn open_right() {
    let a: RawInterval<i32> = Open(0, 3);

    assert_eq!(a.enclose(&Empty),             Open(0, 3));
    assert_eq!(a.enclose(&Point(13)),         LeftOpen(0, 13));
    assert_eq!(a.enclose(&Open(10, 13)),      Open(0, 13));
    assert_eq!(a.enclose(&LeftOpen(10, 13)),  LeftOpen(0, 13));
    assert_eq!(a.enclose(&RightOpen(10, 13)), Open(0, 13));
    assert_eq!(a.enclose(&Closed(10, 13)),    LeftOpen(0, 13));
    assert_eq!(a.enclose(&UpTo(13)),          UpTo(13));
    assert_eq!(a.enclose(&UpFrom(13)),        UpFrom(0));
    assert_eq!(a.enclose(&To(13)),            To(13));
    assert_eq!(a.enclose(&From(13)),          UpFrom(0));
    assert_eq!(a.enclose(&Full),              Full);
}

#[test]
fn left_open_center() {
    let a: RawInterval<i32> = LeftOpen(0, 3);

    assert_eq!(a.enclose(&Empty),             LeftOpen(0, 3));
    assert_eq!(a.enclose(&Point(3)),          LeftOpen(0, 3));
    assert_eq!(a.enclose(&Open(0, 3)),        LeftOpen(0, 3));
    assert_eq!(a.enclose(&LeftOpen(0, 3)),    LeftOpen(0, 3));
    assert_eq!(a.enclose(&RightOpen(0, 3)),   Closed(0, 3));
    assert_eq!(a.enclose(&Closed(0, 3)),      Closed(0, 3));
    assert_eq!(a.enclose(&UpTo(3)),           To(3));
    assert_eq!(a.enclose(&UpFrom(3)),         UpFrom(0));
    assert_eq!(a.enclose(&To(3)),             To(3));
    assert_eq!(a.enclose(&From(3)),           UpFrom(0));
    assert_eq!(a.enclose(&Full),              Full);
}

#[test]
fn left_open_left() {
    let a: RawInterval<i32> = LeftOpen(0, 3);

    assert_eq!(a.enclose(&Empty),             LeftOpen(0, 3));
    assert_eq!(a.enclose(&Point(-3)),         Closed(-3, 3));
    assert_eq!(a.enclose(&Open(-3, -1)),      LeftOpen(-3, 3));
    assert_eq!(a.enclose(&LeftOpen(-3, -1)),  LeftOpen(-3, 3));
    assert_eq!(a.enclose(&RightOpen(-3, -1)), Closed(-3, 3));
    assert_eq!(a.enclose(&Closed(-3, -1)),    Closed(-3, 3));
    assert_eq!(a.enclose(&UpTo(-3)),          To(3));
    assert_eq!(a.enclose(&UpFrom(-3)),        UpFrom(-3));
    assert_eq!(a.enclose(&To(-3)),            To(3));
    assert_eq!(a.enclose(&From(-3)),          From(-3));
    assert_eq!(a.enclose(&Full),              Full);
}

#[test]
fn left_open_right() {
    let a: RawInterval<i32> = LeftOpen(0, 3);

    assert_eq!(a.enclose(&Empty),             LeftOpen(0, 3));
    assert_eq!(a.enclose(&Point(13)),         LeftOpen(0, 13));
    assert_eq!(a.enclose(&Open(10, 13)),      Open(0, 13));
    assert_eq!(a.enclose(&LeftOpen(10, 13)),  LeftOpen(0, 13));
    assert_eq!(a.enclose(&RightOpen(10, 13)), Open(0, 13));
    assert_eq!(a.enclose(&Closed(10, 13)),    LeftOpen(0, 13));
    assert_eq!(a.enclose(&UpTo(13)),          UpTo(13));
    assert_eq!(a.enclose(&UpFrom(13)),        UpFrom(0));
    assert_eq!(a.enclose(&To(13)),            To(13));
    assert_eq!(a.enclose(&From(13)),          UpFrom(0));
    assert_eq!(a.enclose(&Full),              Full);
}

#[test]
fn right_open_center() {
    let a: RawInterval<i32> = RightOpen(0, 3);

    assert_eq!(a.enclose(&Empty),             RightOpen(0, 3));
    assert_eq!(a.enclose(&Point(3)),          Closed(0, 3));
    assert_eq!(a.enclose(&Open(0, 3)),        RightOpen(0, 3));
    assert_eq!(a.enclose(&LeftOpen(0, 3)),    Closed(0, 3));
    assert_eq!(a.enclose(&RightOpen(0, 3)),   RightOpen(0, 3));
    assert_eq!(a.enclose(&Closed(0, 3)),      Closed(0, 3));
    assert_eq!(a.enclose(&UpTo(3)),           UpTo(3));
    assert_eq!(a.enclose(&UpFrom(3)),         From(0));
    assert_eq!(a.enclose(&To(3)),             To(3));
    assert_eq!(a.enclose(&From(3)),           From(0));
    assert_eq!(a.enclose(&Full),              Full);
}

#[test]
fn right_open_left() {
    let a: RawInterval<i32> = RightOpen(0, 3);

    assert_eq!(a.enclose(&Empty),             RightOpen(0, 3));
    assert_eq!(a.enclose(&Point(-3)),         RightOpen(-3, 3));
    assert_eq!(a.enclose(&Open(-3, -1)),      Open(-3, 3));
    assert_eq!(a.enclose(&LeftOpen(-3, -1)),  Open(-3, 3));
    assert_eq!(a.enclose(&RightOpen(-3, -1)), RightOpen(-3, 3));
    assert_eq!(a.enclose(&Closed(-3, -1)),    RightOpen(-3, 3));
    assert_eq!(a.enclose(&UpTo(-3)),          UpTo(3));
    assert_eq!(a.enclose(&UpFrom(-3)),        UpFrom(-3));
    assert_eq!(a.enclose(&To(-3)),            UpTo(3));
    assert_eq!(a.enclose(&From(-3)),          From(-3));
    assert_eq!(a.enclose(&Full),              Full);
}

#[test]
fn right_open_right() {
    let a: RawInterval<i32> = RightOpen(0, 3);

    assert_eq!(a.enclose(&Empty),             RightOpen(0, 3));
    assert_eq!(a.enclose(&Point(13)),         Closed(0, 13));
    assert_eq!(a.enclose(&Open(10, 13)),      RightOpen(0, 13));
    assert_eq!(a.enclose(&LeftOpen(10, 13)),  Closed(0, 13));
    assert_eq!(a.enclose(&RightOpen(10, 13)), RightOpen(0, 13));
    assert_eq!(a.enclose(&Closed(10, 13)),    Closed(0, 13));
    assert_eq!(a.enclose(&UpTo(13)),          UpTo(13));
    assert_eq!(a.enclose(&UpFrom(13)),        From(0));
    assert_eq!(a.enclose(&To(13)),            To(13));
    assert_eq!(a.enclose(&From(13)),          From(0));
    assert_eq!(a.enclose(&Full),              Full);
}

#[test]
fn closed_center() {
    let a: RawInterval<i32> = Closed(0, 3);

    assert_eq!(a.enclose(&Empty),             Closed(0, 3));
    assert_eq!(a.enclose(&Point(3)),          Closed(0, 3));
    assert_eq!(a.enclose(&Open(0, 3)),        Closed(0, 3));
    assert_eq!(a.enclose(&LeftOpen(0, 3)),    Closed(0, 3));
    assert_eq!(a.enclose(&RightOpen(0, 3)),   Closed(0, 3));
    assert_eq!(a.enclose(&Closed(0, 3)),      Closed(0, 3));
    assert_eq!(a.enclose(&UpTo(3)),           To(3));
    assert_eq!(a.enclose(&UpFrom(3)),         From(0));
    assert_eq!(a.enclose(&To(3)),             To(3));
    assert_eq!(a.enclose(&From(3)),           From(0));
    assert_eq!(a.enclose(&Full),              Full);
}

#[test]
fn closed_left() {
    let a: RawInterval<i32> = Closed(0, 3);

    assert_eq!(a.enclose(&Empty),             Closed(0, 3));
    assert_eq!(a.enclose(&Point(-3)),         Closed(-3, 3));
    assert_eq!(a.enclose(&Open(-3, -1)),      LeftOpen(-3, 3));
    assert_eq!(a.enclose(&LeftOpen(-3, -1)),  LeftOpen(-3, 3));
    assert_eq!(a.enclose(&RightOpen(-3, -1)), Closed(-3, 3));
    assert_eq!(a.enclose(&Closed(-3, -1)),    Closed(-3, 3));
    assert_eq!(a.enclose(&UpTo(-3)),          To(3));
    assert_eq!(a.enclose(&UpFrom(-3)),        UpFrom(-3));
    assert_eq!(a.enclose(&To(-3)),            To(3));
    assert_eq!(a.enclose(&From(-3)),          From(-3));
    assert_eq!(a.enclose(&Full),              Full);
}

#[test]
fn closed_right() {
    let a: RawInterval<i32> = Closed(0, 3);

    assert_eq!(a.enclose(&Empty),             Closed(0, 3));
    assert_eq!(a.enclose(&Point(13)),         Closed(0, 13));
    assert_eq!(a.enclose(&Open(10, 13)),      RightOpen(0, 13));
    assert_eq!(a.enclose(&LeftOpen(10, 13)),  Closed(0, 13));
    assert_eq!(a.enclose(&RightOpen(10, 13)), RightOpen(0, 13));
    assert_eq!(a.enclose(&Closed(10, 13)),    Closed(0, 13));
    assert_eq!(a.enclose(&UpTo(13)),          UpTo(13));
    assert_eq!(a.enclose(&UpFrom(13)),        From(0));
    assert_eq!(a.enclose(&To(13)),            To(13));
    assert_eq!(a.enclose(&From(13)),          From(0));
    assert_eq!(a.enclose(&Full),              Full);
}

#[test]
fn up_to_center() {
    let a: RawInterval<i32> = UpTo(3);

    assert_eq!(a.enclose(&Empty),             UpTo(3));
    assert_eq!(a.enclose(&Point(3)),          To(3));
    assert_eq!(a.enclose(&Open(0, 3)),         UpTo(3));
    assert_eq!(a.enclose(&LeftOpen(0, 3)),    To(3));
    assert_eq!(a.enclose(&RightOpen(0, 3)),   UpTo(3));
    assert_eq!(a.enclose(&Closed(0, 3)),      To(3));
    assert_eq!(a.enclose(&UpTo(3)),           UpTo(3));
    assert_eq!(a.enclose(&UpFrom(3)),         Full);
    assert_eq!(a.enclose(&To(3)),             To(3));
    assert_eq!(a.enclose(&From(3)),           Full);
    assert_eq!(a.enclose(&Full),              Full);
}

#[test]
fn up_to_left() {
    let a: RawInterval<i32> = UpTo(3);

    assert_eq!(a.enclose(&Empty),             UpTo(3));
    assert_eq!(a.enclose(&Point(-3)),         UpTo(3));
    assert_eq!(a.enclose(&Open(-3, -1)),      UpTo(3));
    assert_eq!(a.enclose(&LeftOpen(-3, -1)),  UpTo(3));
    assert_eq!(a.enclose(&RightOpen(-3, -1)), UpTo(3));
    assert_eq!(a.enclose(&Closed(-3, -1)),    UpTo(3));
    assert_eq!(a.enclose(&UpTo(-3)),          UpTo(3));
    assert_eq!(a.enclose(&UpFrom(-3)),        Full);
    assert_eq!(a.enclose(&To(-3)),            UpTo(3));
    assert_eq!(a.enclose(&From(-3)),          Full);
    assert_eq!(a.enclose(&Full),              Full);
}

#[test]
fn up_to_right() {
    let a: RawInterval<i32> = UpTo(3);

    assert_eq!(a.enclose(&Empty),             UpTo(3));
    assert_eq!(a.enclose(&Point(13)),         To(13));
    assert_eq!(a.enclose(&Open(10, 13)),      UpTo(13));
    assert_eq!(a.enclose(&LeftOpen(10, 13)),  To(13));
    assert_eq!(a.enclose(&RightOpen(10, 13)), UpTo(13));
    assert_eq!(a.enclose(&Closed(10, 13)),    To(13));
    assert_eq!(a.enclose(&UpTo(13)),          UpTo(13));
    assert_eq!(a.enclose(&UpFrom(13)),        Full);
    assert_eq!(a.enclose(&To(13)),            To(13));
    assert_eq!(a.enclose(&From(13)),          Full);
    assert_eq!(a.enclose(&Full),              Full);
}


#[test]
fn up_from_center() {
    let a: RawInterval<i32> = UpFrom(0);

    assert_eq!(a.enclose(&Empty),             UpFrom(0));
    assert_eq!(a.enclose(&Point(0)),          From(0));
    assert_eq!(a.enclose(&Open(0, 3)),        UpFrom(0));
    assert_eq!(a.enclose(&LeftOpen(0, 3)),    UpFrom(0));
    assert_eq!(a.enclose(&RightOpen(0, 3)),   From(0));
    assert_eq!(a.enclose(&Closed(0, 3)),      From(0));
    assert_eq!(a.enclose(&UpTo(0)),           Full);
    assert_eq!(a.enclose(&UpFrom(0)),         UpFrom(0));
    assert_eq!(a.enclose(&To(0)),             Full);
    assert_eq!(a.enclose(&From(0)),           From(0));
    assert_eq!(a.enclose(&Full),              Full);
}

#[test]
fn up_from_left() {
    let a: RawInterval<i32> = UpFrom(0);

    assert_eq!(a.enclose(&Empty),             UpFrom(0));
    assert_eq!(a.enclose(&Point(-3)),         From(-3));
    assert_eq!(a.enclose(&Open(-3, -1)),      UpFrom(-3));
    assert_eq!(a.enclose(&LeftOpen(-3, -1)),  UpFrom(-3));
    assert_eq!(a.enclose(&RightOpen(-3, -1)), From(-3));
    assert_eq!(a.enclose(&Closed(-3, -1)),    From(-3));
    assert_eq!(a.enclose(&UpTo(-3)),          Full);
    assert_eq!(a.enclose(&UpFrom(-3)),        UpFrom(-3));
    assert_eq!(a.enclose(&To(-3)),            Full);
    assert_eq!(a.enclose(&From(-3)),          From(-3));
    assert_eq!(a.enclose(&Full),              Full);
}

#[test]
fn up_from_right() {
    let a: RawInterval<i32> = UpFrom(0);

    assert_eq!(a.enclose(&Empty),             UpFrom(0));
    assert_eq!(a.enclose(&Point(13)),         UpFrom(0));
    assert_eq!(a.enclose(&Open(10, 13)),      UpFrom(0));
    assert_eq!(a.enclose(&LeftOpen(10, 13)),  UpFrom(0));
    assert_eq!(a.enclose(&RightOpen(10, 13)), UpFrom(0));
    assert_eq!(a.enclose(&Closed(10, 13)),    UpFrom(0));
    assert_eq!(a.enclose(&UpTo(13)),          Full);
    assert_eq!(a.enclose(&UpFrom(13)),        UpFrom(0));
    assert_eq!(a.enclose(&To(13)),            Full);
    assert_eq!(a.enclose(&From(13)),          UpFrom(0));
    assert_eq!(a.enclose(&Full),              Full);
}

#[test]
fn to_center() {
    let a: RawInterval<i32> = To(3);

    assert_eq!(a.enclose(&Empty),             To(3));
    assert_eq!(a.enclose(&Point(3)),          To(3));
    assert_eq!(a.enclose(&Open(0, 3)),        To(3));
    assert_eq!(a.enclose(&LeftOpen(0, 3)),    To(3));
    assert_eq!(a.enclose(&RightOpen(0, 3)),   To(3));
    assert_eq!(a.enclose(&Closed(0, 3)),      To(3));
    assert_eq!(a.enclose(&UpTo(3)),           To(3));
    assert_eq!(a.enclose(&UpFrom(3)),         Full);
    assert_eq!(a.enclose(&To(3)),             To(3));
    assert_eq!(a.enclose(&From(3)),           Full);
    assert_eq!(a.enclose(&Full),              Full);
}

#[test]
fn to_left() {
    let a: RawInterval<i32> = To(3);

    assert_eq!(a.enclose(&Empty),             To(3));
    assert_eq!(a.enclose(&Point(-3)),         To(3));
    assert_eq!(a.enclose(&Open(-3, -1)),      To(3));
    assert_eq!(a.enclose(&LeftOpen(-3, -1)),  To(3));
    assert_eq!(a.enclose(&RightOpen(-3, -1)), To(3));
    assert_eq!(a.enclose(&Closed(-3, -1)),    To(3));
    assert_eq!(a.enclose(&UpTo(-3)),          To(3));
    assert_eq!(a.enclose(&UpFrom(-3)),        Full);
    assert_eq!(a.enclose(&To(-3)),            To(3));
    assert_eq!(a.enclose(&From(-3)),          Full);
    assert_eq!(a.enclose(&Full),              Full);
}

#[test]
fn to_right() {
    let a: RawInterval<i32> = To(3);

    assert_eq!(a.enclose(&Empty),             To(3));
    assert_eq!(a.enclose(&Point(13)),         To(13));
    assert_eq!(a.enclose(&Open(10, 13)),      UpTo(13));
    assert_eq!(a.enclose(&LeftOpen(10, 13)),  To(13));
    assert_eq!(a.enclose(&RightOpen(10, 13)), UpTo(13));
    assert_eq!(a.enclose(&Closed(10, 13)),    To(13));
    assert_eq!(a.enclose(&UpTo(13)),          UpTo(13));
    assert_eq!(a.enclose(&UpFrom(13)),        Full);
    assert_eq!(a.enclose(&To(13)),            To(13));
    assert_eq!(a.enclose(&From(13)),          Full);
    assert_eq!(a.enclose(&Full),              Full);
}

#[test]
fn from_center() {
    let a: RawInterval<i32> = From(0);

    assert_eq!(a.enclose(&Empty),             From(0));
    assert_eq!(a.enclose(&Point(0)),          From(0));
    assert_eq!(a.enclose(&Open(0, 3)),        From(0));
    assert_eq!(a.enclose(&LeftOpen(0, 3)),    From(0));
    assert_eq!(a.enclose(&RightOpen(0, 3)),   From(0));
    assert_eq!(a.enclose(&Closed(0, 3)),      From(0));
    assert_eq!(a.enclose(&UpTo(0)),           Full);
    assert_eq!(a.enclose(&UpFrom(0)),         From(0));
    assert_eq!(a.enclose(&To(0)),             Full);
    assert_eq!(a.enclose(&From(0)),           From(0));
    assert_eq!(a.enclose(&Full),              Full);
}

#[test]
fn from_left() {
    let a: RawInterval<i32> = From(0);

    assert_eq!(a.enclose(&Empty),             From(0));
    assert_eq!(a.enclose(&Point(-3)),         From(-3));
    assert_eq!(a.enclose(&Open(-3, -1)),      UpFrom(-3));
    assert_eq!(a.enclose(&LeftOpen(-3, -1)),  UpFrom(-3));
    assert_eq!(a.enclose(&RightOpen(-3, -1)), From(-3));
    assert_eq!(a.enclose(&Closed(-3, -1)),    From(-3));
    assert_eq!(a.enclose(&UpTo(-3)),          Full);
    assert_eq!(a.enclose(&UpFrom(-3)),        UpFrom(-3));
    assert_eq!(a.enclose(&To(-3)),            Full);
    assert_eq!(a.enclose(&From(-3)),          From(-3));
    assert_eq!(a.enclose(&Full),              Full);
}

#[test]
fn from_right() {
    let a: RawInterval<i32> = From(0);

    assert_eq!(a.enclose(&Empty),             From(0));
    assert_eq!(a.enclose(&Point(13)),         From(0));
    assert_eq!(a.enclose(&Open(10, 13)),      From(0));
    assert_eq!(a.enclose(&LeftOpen(10, 13)),  From(0));
    assert_eq!(a.enclose(&RightOpen(10, 13)), From(0));
    assert_eq!(a.enclose(&Closed(10, 13)),    From(0));
    assert_eq!(a.enclose(&UpTo(13)),          Full);
    assert_eq!(a.enclose(&UpFrom(13)),        From(0));
    assert_eq!(a.enclose(&To(13)),            Full);
    assert_eq!(a.enclose(&From(13)),          From(0));
    assert_eq!(a.enclose(&Full),              Full);
}

#[test]
fn full() {
    let a: RawInterval<i32> = Full;

    assert_eq!(a.enclose(&Empty),             Full);
    assert_eq!(a.enclose(&Point(0)),          Full);
    assert_eq!(a.enclose(&Open(0, 3)),        Full);
    assert_eq!(a.enclose(&LeftOpen(0, 3)),    Full);
    assert_eq!(a.enclose(&RightOpen(0, 3)),   Full);
    assert_eq!(a.enclose(&Closed(0, 3)),      Full);
    assert_eq!(a.enclose(&UpTo(0)),           Full);
    assert_eq!(a.enclose(&UpFrom(0)),         Full);
    assert_eq!(a.enclose(&To(0)),             Full);
    assert_eq!(a.enclose(&From(0)),           Full);
    assert_eq!(a.enclose(&Full),              Full);
}
