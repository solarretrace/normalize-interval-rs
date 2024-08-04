// Copyright 2024 Skylor R. Schermer.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
////////////////////////////////////////////////////////////////////////////////

// Internal library imports.
use crate::tine_tree::TineTree;

// Local enum shortcuts.
use crate::raw_interval::RawInterval::*;


////////////////////////////////////////////////////////////////////////////////
// Aggregation tests
////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////
// Non-mutating minus tests.
////////////////////////////////////////////////////////////////////////////////


#[test]
fn empty() {
    let a: TineTree<i32> = Empty.into();

    assert_eq_i!(a.minus(&TineTree::from(Empty)),             []);
    assert_eq_i!(a.minus(&TineTree::from(Point(3))),          []);
    assert_eq_i!(a.minus(&TineTree::from(Open(0, 3))),        []);
    assert_eq_i!(a.minus(&TineTree::from(LeftOpen(0, 3))),    []);
    assert_eq_i!(a.minus(&TineTree::from(RightOpen(0, 3))),   []);
    assert_eq_i!(a.minus(&TineTree::from(Closed(0, 3))),      []);
    assert_eq_i!(a.minus(&TineTree::from(UpTo(3))),           []);
    assert_eq_i!(a.minus(&TineTree::from(UpFrom(3))),         []);
    assert_eq_i!(a.minus(&TineTree::from(To(3))),             []);
    assert_eq_i!(a.minus(&TineTree::from(From(3))),           []);
    assert_eq_i!(a.minus(&TineTree::from(Full)),              []);
}

#[test]
fn point_center() {
    let a: TineTree<i32> = Point(3).into();

    assert_eq_i!(a.minus(&TineTree::from(Empty)),             [Point(3)]);
    assert_eq_i!(a.minus(&TineTree::from(Point(3))),          []);
    assert_eq_i!(a.minus(&TineTree::from(Open(0, 3))),        [Point(3)]);
    assert_eq_i!(a.minus(&TineTree::from(LeftOpen(0, 3))),    []);
    assert_eq_i!(a.minus(&TineTree::from(RightOpen(0, 3))),   [Point(3)]);
    assert_eq_i!(a.minus(&TineTree::from(Closed(0, 3))),      []);
    assert_eq_i!(a.minus(&TineTree::from(UpTo(3))),           [Point(3)]);
    assert_eq_i!(a.minus(&TineTree::from(UpFrom(3))),         [Point(3)]);
    assert_eq_i!(a.minus(&TineTree::from(To(3))),             []);
    assert_eq_i!(a.minus(&TineTree::from(From(3))),           []);
    assert_eq_i!(a.minus(&TineTree::from(Full)),              []);
}

#[test]
fn point_left() {
    let a: TineTree<i32> = Point(3).into();

    assert_eq_i!(a.minus(&TineTree::from(Empty)),             [Point(3)]);
    assert_eq_i!(a.minus(&TineTree::from(Point(-1))),         [Point(3)]);
    assert_eq_i!(a.minus(&TineTree::from(Open(-3, -1))),      [Point(3)]);
    assert_eq_i!(a.minus(&TineTree::from(LeftOpen(-3, -1))),  [Point(3)]);
    assert_eq_i!(a.minus(&TineTree::from(RightOpen(-3, -1))), [Point(3)]);
    assert_eq_i!(a.minus(&TineTree::from(Closed(-3, -1))),    [Point(3)]);
    assert_eq_i!(a.minus(&TineTree::from(UpTo(-3))),          [Point(3)]);
    assert_eq_i!(a.minus(&TineTree::from(UpFrom(-3))),        []);
    assert_eq_i!(a.minus(&TineTree::from(To(-3))),            [Point(3)]);
    assert_eq_i!(a.minus(&TineTree::from(From(-3))),          []);
    assert_eq_i!(a.minus(&TineTree::from(Full)),              []);
}

#[test]
fn point_right() {
    let a: TineTree<i32> = Point(3).into();

    assert_eq_i!(a.minus(&TineTree::from(Empty)),             [Point(3)]);
    assert_eq_i!(a.minus(&TineTree::from(Point(10))),         [Point(3)]);
    assert_eq_i!(a.minus(&TineTree::from(Open(10, 13))),      [Point(3)]);
    assert_eq_i!(a.minus(&TineTree::from(LeftOpen(10, 13))),  [Point(3)]);
    assert_eq_i!(a.minus(&TineTree::from(RightOpen(10, 13))), [Point(3)]);
    assert_eq_i!(a.minus(&TineTree::from(Closed(10, 13))),    [Point(3)]);
    assert_eq_i!(a.minus(&TineTree::from(UpTo(13))),          []);
    assert_eq_i!(a.minus(&TineTree::from(UpFrom(13))),        [Point(3)]);
    assert_eq_i!(a.minus(&TineTree::from(To(13))),            []);
    assert_eq_i!(a.minus(&TineTree::from(From(13))),          [Point(3)]);
    assert_eq_i!(a.minus(&TineTree::from(Full)),              []);
}

#[test]
fn open_center() {
    let a: TineTree<i32> = Open(0, 3).into();

    assert_eq_i!(a.minus(&TineTree::from(Empty)),             [Open(0, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(Point(2))),          [Open(0, 2), Open(2, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(Open(0, 3))),        []);
    assert_eq_i!(a.minus(&TineTree::from(LeftOpen(0, 3))),    []);
    assert_eq_i!(a.minus(&TineTree::from(RightOpen(0, 3))),   []);
    assert_eq_i!(a.minus(&TineTree::from(Closed(0, 3))),      []);
    assert_eq_i!(a.minus(&TineTree::from(UpTo(2))),           [RightOpen(2, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(UpFrom(2))),         [LeftOpen(0, 2)]);
    assert_eq_i!(a.minus(&TineTree::from(To(2))),             [Open(2, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(From(2))),           [Open(0, 2)]);
    assert_eq_i!(a.minus(&TineTree::from(Full)),              []);
}

#[test]
fn open_left() {
    let a: TineTree<i32> = Open(0, 3).into();

    assert_eq_i!(a.minus(&TineTree::from(Empty)),             [Open(0, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(Point(-3))),         [Open(0, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(Open(-3, -1))),      [Open(0, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(LeftOpen(-3, -1))),  [Open(0, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(RightOpen(-3, -1))), [Open(0, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(Closed(-3, -1))),    [Open(0, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(UpTo(-3))),          [Open(0, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(UpFrom(-3))),        []);
    assert_eq_i!(a.minus(&TineTree::from(To(-3))),            [Open(0, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(From(-3))),          []);
    assert_eq_i!(a.minus(&TineTree::from(Full)),              []);
}

#[test]
fn open_right() {
    let a: TineTree<i32> = Open(0, 3).into();

    assert_eq_i!(a.minus(&TineTree::from(Empty)),             [Open(0, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(Point(13))),         [Open(0, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(Open(10, 13))),      [Open(0, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(LeftOpen(10, 13))),  [Open(0, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(RightOpen(10, 13))), [Open(0, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(Closed(10, 13))),    [Open(0, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(UpTo(13))),          []);
    assert_eq_i!(a.minus(&TineTree::from(UpFrom(13))),        [Open(0, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(To(13))),            []);
    assert_eq_i!(a.minus(&TineTree::from(From(13))),          [Open(0, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(Full)),              []);
}

#[test]
fn left_open_center() {
    let a: TineTree<i32> = LeftOpen(0, 3).into();

    assert_eq_i!(a.minus(&TineTree::from(Empty)),             [LeftOpen(0, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(Point(2))),          [Open(0, 2), LeftOpen(2, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(Open(0, 3))),        [Point(3)]);
    assert_eq_i!(a.minus(&TineTree::from(LeftOpen(0, 3))),    []);
    assert_eq_i!(a.minus(&TineTree::from(RightOpen(0, 3))),   [Point(3)]);
    assert_eq_i!(a.minus(&TineTree::from(Closed(0, 3))),      []);
    assert_eq_i!(a.minus(&TineTree::from(UpTo(2))),           [Closed(2, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(UpFrom(2))),         [LeftOpen(0, 2)]);
    assert_eq_i!(a.minus(&TineTree::from(To(2))),             [LeftOpen(2, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(From(2))),           [Open(0, 2)]);
    assert_eq_i!(a.minus(&TineTree::from(Full)),              []);
}

#[test]
fn left_open_left() {
    let a: TineTree<i32> = LeftOpen(0, 3).into();

    assert_eq_i!(a.minus(&TineTree::from(Empty)),             [LeftOpen(0, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(Point(-3))),         [LeftOpen(0, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(Open(-3, -1))),      [LeftOpen(0, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(LeftOpen(-3, -1))),  [LeftOpen(0, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(RightOpen(-3, -1))), [LeftOpen(0, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(Closed(-3, -1))),    [LeftOpen(0, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(UpTo(-3))),          [LeftOpen(0, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(UpFrom(-3))),        []);
    assert_eq_i!(a.minus(&TineTree::from(To(-3))),            [LeftOpen(0, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(From(-3))),          []);
    assert_eq_i!(a.minus(&TineTree::from(Full)),              []);
}

#[test]
fn left_open_right() {
    let a: TineTree<i32> = LeftOpen(0, 3).into();

    assert_eq_i!(a.minus(&TineTree::from(Empty)),             [LeftOpen(0, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(Point(13))),         [LeftOpen(0, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(Open(10, 13))),      [LeftOpen(0, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(LeftOpen(10, 13))),  [LeftOpen(0, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(RightOpen(10, 13))), [LeftOpen(0, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(Closed(10, 13))),    [LeftOpen(0, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(UpTo(13))),          []);
    assert_eq_i!(a.minus(&TineTree::from(UpFrom(13))),        [LeftOpen(0, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(To(13))),            []);
    assert_eq_i!(a.minus(&TineTree::from(From(13))),          [LeftOpen(0, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(Full)),              []);
}

#[test]
fn right_open_center() {
    let a: TineTree<i32> = RightOpen(0, 3).into();

    assert_eq_i!(a.minus(&TineTree::from(Empty)),             [RightOpen(0, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(Point(2))),          [RightOpen(0, 2), Open(2, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(Open(0, 3))),        [Point(0)]);
    assert_eq_i!(a.minus(&TineTree::from(LeftOpen(0, 3))),    [Point(0)]);
    assert_eq_i!(a.minus(&TineTree::from(RightOpen(0, 3))),   []);
    assert_eq_i!(a.minus(&TineTree::from(Closed(0, 3))),      []);
    assert_eq_i!(a.minus(&TineTree::from(UpTo(2))),           [RightOpen(2, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(UpFrom(2))),         [Closed(0, 2)]);
    assert_eq_i!(a.minus(&TineTree::from(To(2))),             [Open(2, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(From(2))),           [RightOpen(0, 2)]);
    assert_eq_i!(a.minus(&TineTree::from(Full)),              []);
}

#[test]
fn right_open_left() {
    let a: TineTree<i32> = RightOpen(0, 3).into();

    assert_eq_i!(a.minus(&TineTree::from(Empty)),             [RightOpen(0, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(Point(-3))),         [RightOpen(0, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(Open(-3, -1))),      [RightOpen(0, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(LeftOpen(-3, -1))),  [RightOpen(0, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(RightOpen(-3, -1))), [RightOpen(0, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(Closed(-3, -1))),    [RightOpen(0, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(UpTo(-3))),          [RightOpen(0, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(UpFrom(-3))),        []);
    assert_eq_i!(a.minus(&TineTree::from(To(-3))),            [RightOpen(0, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(From(-3))),          []);
    assert_eq_i!(a.minus(&TineTree::from(Full)),              []);
}

#[test]
fn right_open_right() {
    let a: TineTree<i32> = RightOpen(0, 3).into();

    assert_eq_i!(a.minus(&TineTree::from(Empty)),             [RightOpen(0, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(Point(13))),         [RightOpen(0, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(Open(10, 13))),      [RightOpen(0, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(LeftOpen(10, 13))),  [RightOpen(0, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(RightOpen(10, 13))), [RightOpen(0, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(Closed(10, 13))),    [RightOpen(0, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(UpTo(13))),          []);
    assert_eq_i!(a.minus(&TineTree::from(UpFrom(13))),        [RightOpen(0, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(To(13))),            []);
    assert_eq_i!(a.minus(&TineTree::from(From(13))),          [RightOpen(0, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(Full)),              []);
}

#[test]
fn closed_center() {
    let a: TineTree<i32> = Closed(0, 3).into();

    assert_eq_i!(a.minus(&TineTree::from(Empty)),             [Closed(0, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(Point(2))),          [RightOpen(0, 2), LeftOpen(2, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(Open(0, 3))),        [Point(0), Point(3)]);
    assert_eq_i!(a.minus(&TineTree::from(LeftOpen(0, 3))),    [Point(0)]);
    assert_eq_i!(a.minus(&TineTree::from(RightOpen(0, 3))),   [Point(3)]);
    assert_eq_i!(a.minus(&TineTree::from(Closed(0, 3))),      []);
    assert_eq_i!(a.minus(&TineTree::from(UpTo(2))),           [Closed(2, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(UpFrom(2))),         [Closed(0, 2)]);
    assert_eq_i!(a.minus(&TineTree::from(To(2))),             [LeftOpen(2, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(From(2))),           [RightOpen(0, 2)]);
    assert_eq_i!(a.minus(&TineTree::from(Full)),              []);
}

#[test]
fn closed_left() {
    let a: TineTree<i32> = Closed(0, 3).into();

    assert_eq_i!(a.minus(&TineTree::from(Empty)),             [Closed(0, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(Point(-3))),         [Closed(0, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(Open(-3, -1))),      [Closed(0, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(LeftOpen(-3, -1))),  [Closed(0, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(RightOpen(-3, -1))), [Closed(0, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(Closed(-3, -1))),    [Closed(0, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(UpTo(-3))),          [Closed(0, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(UpFrom(-3))),        []);
    assert_eq_i!(a.minus(&TineTree::from(To(-3))),            [Closed(0, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(From(-3))),          []);
    assert_eq_i!(a.minus(&TineTree::from(Full)),              []);
}

#[test]
fn closed_right() {
    let a: TineTree<i32> = Closed(0, 3).into();

    assert_eq_i!(a.minus(&TineTree::from(Empty)),             [Closed(0, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(Point(13))),         [Closed(0, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(Open(10, 13))),      [Closed(0, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(LeftOpen(10, 13))),  [Closed(0, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(RightOpen(10, 13))), [Closed(0, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(Closed(10, 13))),    [Closed(0, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(UpTo(13))),          []);
    assert_eq_i!(a.minus(&TineTree::from(UpFrom(13))),        [Closed(0, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(To(13))),            []);
    assert_eq_i!(a.minus(&TineTree::from(From(13))),          [Closed(0, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(Full)),              []);
}

#[test]
fn up_to_center() {
    let a: TineTree<i32> = UpTo(3).into();

    assert_eq_i!(a.minus(&TineTree::from(Empty)),             [UpTo(3)]);
    assert_eq_i!(a.minus(&TineTree::from(Point(2))),          [UpTo(2), Open(2, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(Open(0, 3))),        [To(0)]);
    assert_eq_i!(a.minus(&TineTree::from(LeftOpen(0, 3))),    [To(0)]);
    assert_eq_i!(a.minus(&TineTree::from(RightOpen(0, 3))),   [UpTo(0)]);
    assert_eq_i!(a.minus(&TineTree::from(Closed(0, 3))),      [UpTo(0)]);
    assert_eq_i!(a.minus(&TineTree::from(UpTo(2))),           [RightOpen(2, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(UpFrom(2))),         [To(2)]);
    assert_eq_i!(a.minus(&TineTree::from(To(2))),             [Open(2, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(From(2))),           [UpTo(2)]);
    assert_eq_i!(a.minus(&TineTree::from(Full)),              []);
}

#[test]
fn up_to_left() {
    let a: TineTree<i32> = UpTo(3).into();

    assert_eq_i!(a.minus(&TineTree::from(Empty)),             [UpTo(3)]);
    assert_eq_i!(a.minus(&TineTree::from(Point(-3))),         [UpTo(-3), Open(-3, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(Open(-3, -1))),      [To(-3), RightOpen(-1, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(LeftOpen(-3, -1))),  [To(-3), Open(-1, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(RightOpen(-3, -1))), [UpTo(-3), RightOpen(-1, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(Closed(-3, -1))),    [UpTo(-3), Open(-1, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(UpTo(-3))),          [RightOpen(-3, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(UpFrom(-3))),        [To(-3)]);
    assert_eq_i!(a.minus(&TineTree::from(To(-3))),            [Open(-3, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(From(-3))),          [UpTo(-3)]);
    assert_eq_i!(a.minus(&TineTree::from(Full)),              []);
}

#[test]
fn up_to_right() {
    let a: TineTree<i32> = UpTo(3).into();

    assert_eq_i!(a.minus(&TineTree::from(Empty)),             [UpTo(3)]);
    assert_eq_i!(a.minus(&TineTree::from(Point(13))),         [UpTo(3)]);
    assert_eq_i!(a.minus(&TineTree::from(Open(10, 13))),      [UpTo(3)]);
    assert_eq_i!(a.minus(&TineTree::from(LeftOpen(10, 13))),  [UpTo(3)]);
    assert_eq_i!(a.minus(&TineTree::from(RightOpen(10, 13))), [UpTo(3)]);
    assert_eq_i!(a.minus(&TineTree::from(Closed(10, 13))),    [UpTo(3)]);
    assert_eq_i!(a.minus(&TineTree::from(UpTo(13))),          []);
    assert_eq_i!(a.minus(&TineTree::from(UpFrom(13))),        [UpTo(3)]);
    assert_eq_i!(a.minus(&TineTree::from(To(13))),            []);
    assert_eq_i!(a.minus(&TineTree::from(From(13))),          [UpTo(3)]);
    assert_eq_i!(a.minus(&TineTree::from(Full)),              []);
}

#[test]
fn up_from_center() {
    let a: TineTree<i32> = UpFrom(0).into();

    assert_eq_i!(a.minus(&TineTree::from(Empty)),             [UpFrom(0)]);
    assert_eq_i!(a.minus(&TineTree::from(Point(2))),          [Open(0, 2), UpFrom(2)]);
    assert_eq_i!(a.minus(&TineTree::from(Open(0, 3))),        [From(3)]);
    assert_eq_i!(a.minus(&TineTree::from(LeftOpen(0, 3))),    [UpFrom(3)]);
    assert_eq_i!(a.minus(&TineTree::from(RightOpen(0, 3))),   [From(3)]);
    assert_eq_i!(a.minus(&TineTree::from(Closed(0, 3))),      [UpFrom(3)]);
    assert_eq_i!(a.minus(&TineTree::from(UpTo(2))),           [From(2)]);
    assert_eq_i!(a.minus(&TineTree::from(UpFrom(2))),         [LeftOpen(0, 2)]);
    assert_eq_i!(a.minus(&TineTree::from(To(2))),             [UpFrom(2)]);
    assert_eq_i!(a.minus(&TineTree::from(From(2))),           [Open(0, 2)]);
    assert_eq_i!(a.minus(&TineTree::from(Full)),              []);
}

#[test]
fn up_from_left() {
    let a: TineTree<i32> = UpFrom(0).into();

    assert_eq_i!(a.minus(&TineTree::from(Empty)),             [UpFrom(0)]);
    assert_eq_i!(a.minus(&TineTree::from(Point(-3))),         [UpFrom(0)]);
    assert_eq_i!(a.minus(&TineTree::from(Open(-3, -1))),      [UpFrom(0)]);
    assert_eq_i!(a.minus(&TineTree::from(LeftOpen(-3, -1))),  [UpFrom(0)]);
    assert_eq_i!(a.minus(&TineTree::from(RightOpen(-3, -1))), [UpFrom(0)]);
    assert_eq_i!(a.minus(&TineTree::from(Closed(-3, -1))),    [UpFrom(0)]);
    assert_eq_i!(a.minus(&TineTree::from(UpTo(-3))),          [UpFrom(0)]);
    assert_eq_i!(a.minus(&TineTree::from(UpFrom(-3))),        []);
    assert_eq_i!(a.minus(&TineTree::from(To(-3))),            [UpFrom(0)]);
    assert_eq_i!(a.minus(&TineTree::from(From(-3))),          []);
    assert_eq_i!(a.minus(&TineTree::from(Full)),              []);
}

#[test]
fn up_from_right() {
    let a: TineTree<i32> = UpFrom(0).into();

    assert_eq_i!(a.minus(&TineTree::from(Empty)),             [UpFrom(0)]);
    assert_eq_i!(a.minus(&TineTree::from(Point(13))),         [Open(0, 13), UpFrom(13)]);
    assert_eq_i!(a.minus(&TineTree::from(Open(10, 13))),      [LeftOpen(0, 10), From(13)]);
    assert_eq_i!(a.minus(&TineTree::from(LeftOpen(10, 13))),  [LeftOpen(0, 10), UpFrom(13)]);
    assert_eq_i!(a.minus(&TineTree::from(RightOpen(10, 13))), [Open(0, 10), From(13)]);
    assert_eq_i!(a.minus(&TineTree::from(Closed(10, 13))),    [Open(0, 10), UpFrom(13)]);
    assert_eq_i!(a.minus(&TineTree::from(UpTo(13))),          [From(13)]);
    assert_eq_i!(a.minus(&TineTree::from(UpFrom(13))),        [LeftOpen(0, 13)]);
    assert_eq_i!(a.minus(&TineTree::from(To(13))),            [UpFrom(13)]);
    assert_eq_i!(a.minus(&TineTree::from(From(13))),          [Open(0, 13)]);
    assert_eq_i!(a.minus(&TineTree::from(Full)),              []);
}

#[test]
fn to_center() {
    let a: TineTree<i32> = To(3).into();

    assert_eq_i!(a.minus(&TineTree::from(Empty)),             [To(3)]);
    assert_eq_i!(a.minus(&TineTree::from(Point(2))),          [UpTo(2), LeftOpen(2, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(Open(0, 3))),        [To(0), Point(3)]);
    assert_eq_i!(a.minus(&TineTree::from(LeftOpen(0, 3))),    [To(0)]);
    assert_eq_i!(a.minus(&TineTree::from(RightOpen(0, 3))),   [UpTo(0), Point(3)]);
    assert_eq_i!(a.minus(&TineTree::from(Closed(0, 3))),      [UpTo(0)]);
    assert_eq_i!(a.minus(&TineTree::from(UpTo(2))),           [Closed(2, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(UpFrom(2))),         [To(2)]);
    assert_eq_i!(a.minus(&TineTree::from(To(2))),             [LeftOpen(2, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(From(2))),           [UpTo(2)]);
    assert_eq_i!(a.minus(&TineTree::from(Full)),              []);
}

#[test]
fn to_left() {
    let a: TineTree<i32> = To(3).into();

    assert_eq_i!(a.minus(&TineTree::from(Empty)),             [To(3)]);
    assert_eq_i!(a.minus(&TineTree::from(Point(-3))),         [UpTo(-3), LeftOpen(-3, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(Open(-3, -1))),      [To(-3), Closed(-1, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(LeftOpen(-3, -1))),  [To(-3), LeftOpen(-1, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(RightOpen(-3, -1))), [UpTo(-3), Closed(-1, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(Closed(-3, -1))),    [UpTo(-3), LeftOpen(-1, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(UpTo(-3))),          [Closed(-3, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(UpFrom(-3))),        [To(-3)]);
    assert_eq_i!(a.minus(&TineTree::from(To(-3))),            [LeftOpen(-3, 3)]);
    assert_eq_i!(a.minus(&TineTree::from(From(-3))),          [UpTo(-3)]);
    assert_eq_i!(a.minus(&TineTree::from(Full)),              []);
}

#[test]
fn to_right() {
    let a: TineTree<i32> = To(3).into();

    assert_eq_i!(a.minus(&TineTree::from(Empty)),             [To(3)]);
    assert_eq_i!(a.minus(&TineTree::from(Point(13))),         [To(3)]);
    assert_eq_i!(a.minus(&TineTree::from(Open(10, 13))),      [To(3)]);
    assert_eq_i!(a.minus(&TineTree::from(LeftOpen(10, 13))),  [To(3)]);
    assert_eq_i!(a.minus(&TineTree::from(RightOpen(10, 13))), [To(3)]);
    assert_eq_i!(a.minus(&TineTree::from(Closed(10, 13))),    [To(3)]);
    assert_eq_i!(a.minus(&TineTree::from(UpTo(13))),          []);
    assert_eq_i!(a.minus(&TineTree::from(UpFrom(13))),        [To(3)]);
    assert_eq_i!(a.minus(&TineTree::from(To(13))),            []);
    assert_eq_i!(a.minus(&TineTree::from(From(13))),          [To(3)]);
    assert_eq_i!(a.minus(&TineTree::from(Full)),              []);
}

#[test]
fn from_center() {
    let a: TineTree<i32> = From(0).into();

    assert_eq_i!(a.minus(&TineTree::from(Empty)),             [From(0)]);
    assert_eq_i!(a.minus(&TineTree::from(Point(2))),          [RightOpen(0, 2), UpFrom(2)]);
    assert_eq_i!(a.minus(&TineTree::from(Open(0, 3))),        [Point(0), From(3)]);
    assert_eq_i!(a.minus(&TineTree::from(LeftOpen(0, 3))),    [Point(0), UpFrom(3)]);
    assert_eq_i!(a.minus(&TineTree::from(RightOpen(0, 3))),   [From(3)]);
    assert_eq_i!(a.minus(&TineTree::from(Closed(0, 3))),      [UpFrom(3)]);
    assert_eq_i!(a.minus(&TineTree::from(UpTo(2))),           [From(2)]);
    assert_eq_i!(a.minus(&TineTree::from(UpFrom(2))),         [Closed(0, 2)]);
    assert_eq_i!(a.minus(&TineTree::from(To(2))),             [UpFrom(2)]);
    assert_eq_i!(a.minus(&TineTree::from(From(2))),           [RightOpen(0, 2)]);
    assert_eq_i!(a.minus(&TineTree::from(Full)),              []);
}

#[test]
fn from_left() {
    let a: TineTree<i32> = From(0).into();

    assert_eq_i!(a.minus(&TineTree::from(Empty)),             [From(0)]);
    assert_eq_i!(a.minus(&TineTree::from(Point(-3))),         [From(0)]);
    assert_eq_i!(a.minus(&TineTree::from(Open(-3, -1))),      [From(0)]);
    assert_eq_i!(a.minus(&TineTree::from(LeftOpen(-3, -1))),  [From(0)]);
    assert_eq_i!(a.minus(&TineTree::from(RightOpen(-3, -1))), [From(0)]);
    assert_eq_i!(a.minus(&TineTree::from(Closed(-3, -1))),    [From(0)]);
    assert_eq_i!(a.minus(&TineTree::from(UpTo(-3))),          [From(0)]);
    assert_eq_i!(a.minus(&TineTree::from(UpFrom(-3))),        []);
    assert_eq_i!(a.minus(&TineTree::from(To(-3))),            [From(0)]);
    assert_eq_i!(a.minus(&TineTree::from(From(-3))),          []);
    assert_eq_i!(a.minus(&TineTree::from(Full)),              []);
}

#[test]
fn from_right() {
    let a: TineTree<i32> = From(0).into();

    assert_eq_i!(a.minus(&TineTree::from(Empty)),             [From(0)]);
    assert_eq_i!(a.minus(&TineTree::from(Point(13))),         [RightOpen(0, 13), UpFrom(13)]);
    assert_eq_i!(a.minus(&TineTree::from(Open(10, 13))),      [Closed(0, 10), From(13)]);
    assert_eq_i!(a.minus(&TineTree::from(LeftOpen(10, 13))),  [Closed(0, 10), UpFrom(13)]);
    assert_eq_i!(a.minus(&TineTree::from(RightOpen(10, 13))), [RightOpen(0, 10), From(13)]);
    assert_eq_i!(a.minus(&TineTree::from(Closed(10, 13))),    [RightOpen(0, 10), UpFrom(13)]);
    assert_eq_i!(a.minus(&TineTree::from(UpTo(13))),          [From(13)]);
    assert_eq_i!(a.minus(&TineTree::from(UpFrom(13))),        [Closed(0, 13)]);
    assert_eq_i!(a.minus(&TineTree::from(To(13))),            [UpFrom(13)]);
    assert_eq_i!(a.minus(&TineTree::from(From(13))),          [RightOpen(0, 13)]);
    assert_eq_i!(a.minus(&TineTree::from(Full)),              []);
}

#[test]
fn full() {
    let a: TineTree<i32> = Full.into();

    assert_eq_i!(a.minus(&TineTree::from(Empty)),             [Full]);
    assert_eq_i!(a.minus(&TineTree::from(Point(0))),          [UpTo(0), UpFrom(0)]);
    assert_eq_i!(a.minus(&TineTree::from(Open(0, 3))),        [To(0), From(3)]);
    assert_eq_i!(a.minus(&TineTree::from(LeftOpen(0, 3))),    [To(0), UpFrom(3)]);
    assert_eq_i!(a.minus(&TineTree::from(RightOpen(0, 3))),   [UpTo(0), From(3)]);
    assert_eq_i!(a.minus(&TineTree::from(Closed(0, 3))),      [UpTo(0), UpFrom(3)]);
    assert_eq_i!(a.minus(&TineTree::from(UpTo(0))),           [From(0)]);
    assert_eq_i!(a.minus(&TineTree::from(UpFrom(0))),         [To(0)]);
    assert_eq_i!(a.minus(&TineTree::from(To(0))),             [UpFrom(0)]);
    assert_eq_i!(a.minus(&TineTree::from(From(0))),           [UpTo(0)]);
    assert_eq_i!(a.minus(&TineTree::from(Full)),              []);
}
