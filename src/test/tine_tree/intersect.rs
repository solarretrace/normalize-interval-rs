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

#[test]
fn disjoint_aggregation() {
    let mut t: TineTree<i32> = Full.into();

    t.intersect_in_place(&UpTo(0));
	t.intersect_in_place(&Point(5));
	t.intersect_in_place(&Empty);
	t.intersect_in_place(&Open(10, 15));
	t.intersect_in_place(&LeftOpen(20, 25));
	t.intersect_in_place(&RightOpen(30, 35));
	t.intersect_in_place(&Empty);
	t.intersect_in_place(&Closed(40, 45));
	t.intersect_in_place(&UpFrom(50));
	t.intersect_in_place(&Empty);

    assert_eq!(t.into_iter().collect::<Vec<_>>(), []);
}

#[test]
fn nested_aggregation() {
    let mut t: TineTree<i32> = Full.into();

    t.intersect_in_place(&UpTo(100));
    t.intersect_in_place(&Open(0, 50));
    t.intersect_in_place(&Full);
    t.intersect_in_place(&LeftOpen(5, 45));
    t.intersect_in_place(&RightOpen(10, 40));
    t.intersect_in_place(&Full);
    t.intersect_in_place(&Closed(15, 35));
    t.intersect_in_place(&UpFrom(20));
    t.intersect_in_place(&Point(25));

    assert_eq!(t.into_iter().collect::<Vec<_>>(), [Point(25)]);
}



////////////////////////////////////////////////////////////////////////////////
// Non-mutating intersect tests.
////////////////////////////////////////////////////////////////////////////////


#[test]
fn empty() {
    let a: TineTree<i32> = Empty.into();

    assert_eq_i!(a.intersect(&TineTree::from(Empty)),             []);
    assert_eq_i!(a.intersect(&TineTree::from(Point(3))),          []);
    assert_eq_i!(a.intersect(&TineTree::from(Open(0, 3))),        []);
    assert_eq_i!(a.intersect(&TineTree::from(LeftOpen(0, 3))),    []);
    assert_eq_i!(a.intersect(&TineTree::from(RightOpen(0, 3))),   []);
    assert_eq_i!(a.intersect(&TineTree::from(Closed(0, 3))),      []);
    assert_eq_i!(a.intersect(&TineTree::from(UpTo(3))),           []);
    assert_eq_i!(a.intersect(&TineTree::from(UpFrom(3))),         []);
    assert_eq_i!(a.intersect(&TineTree::from(To(3))),             []);
    assert_eq_i!(a.intersect(&TineTree::from(From(3))),           []);
    assert_eq_i!(a.intersect(&TineTree::from(Full)),              []);
}

#[test]
fn point_center() {
    let a: TineTree<i32> = Point(2).into();

    assert_eq_i!(a.intersect(&TineTree::from(Empty)),             []);
    assert_eq_i!(a.intersect(&TineTree::from(Point(2))),          [Point(2)]);
    assert_eq_i!(a.intersect(&TineTree::from(Open(0, 3))),        [Point(2)]);
    assert_eq_i!(a.intersect(&TineTree::from(LeftOpen(0, 3))),    [Point(2)]);
    assert_eq_i!(a.intersect(&TineTree::from(RightOpen(0, 3))),   [Point(2)]);
    assert_eq_i!(a.intersect(&TineTree::from(Closed(0, 3))),      [Point(2)]);
    assert_eq_i!(a.intersect(&TineTree::from(UpTo(2))),           []);
    assert_eq_i!(a.intersect(&TineTree::from(UpFrom(2))),         []);
    assert_eq_i!(a.intersect(&TineTree::from(To(2))),             [Point(2)]);
    assert_eq_i!(a.intersect(&TineTree::from(From(2))),           [Point(2)]);
    assert_eq_i!(a.intersect(&TineTree::from(Full)),              [Point(2)]);
}

#[test]
fn point_left() {
    let a: TineTree<i32> = Point(2).into();

    assert_eq_i!(a.intersect(&TineTree::from(Empty)),             []);
    assert_eq_i!(a.intersect(&TineTree::from(Point(-1))),         []);
    assert_eq_i!(a.intersect(&TineTree::from(Open(-3, -1))),      []);
    assert_eq_i!(a.intersect(&TineTree::from(LeftOpen(-3, -1))),  []);
    assert_eq_i!(a.intersect(&TineTree::from(RightOpen(-3, -1))), []);
    assert_eq_i!(a.intersect(&TineTree::from(Closed(-3, -1))),    []);
    assert_eq_i!(a.intersect(&TineTree::from(UpTo(-3))),          []);
    assert_eq_i!(a.intersect(&TineTree::from(UpFrom(-3))),        [Point(2)]);
    assert_eq_i!(a.intersect(&TineTree::from(To(-3))),            []);
    assert_eq_i!(a.intersect(&TineTree::from(From(-3))),          [Point(2)]);
    assert_eq_i!(a.intersect(&TineTree::from(Full)),              [Point(2)]);
}

#[test]
fn point_right() {
    let a: TineTree<i32> = Point(2).into();

    assert_eq_i!(a.intersect(&TineTree::from(Empty)),             []);
    assert_eq_i!(a.intersect(&TineTree::from(Point(10))),         []);
    assert_eq_i!(a.intersect(&TineTree::from(Open(10, 13))),      []);
    assert_eq_i!(a.intersect(&TineTree::from(LeftOpen(10, 13))),  []);
    assert_eq_i!(a.intersect(&TineTree::from(RightOpen(10, 13))), []);
    assert_eq_i!(a.intersect(&TineTree::from(Closed(10, 13))),    []);
    assert_eq_i!(a.intersect(&TineTree::from(UpTo(13))),          [Point(2)]);
    assert_eq_i!(a.intersect(&TineTree::from(UpFrom(13))),        []);
    assert_eq_i!(a.intersect(&TineTree::from(To(13))),            [Point(2)]);
    assert_eq_i!(a.intersect(&TineTree::from(From(13))),          []);
    assert_eq_i!(a.intersect(&TineTree::from(Full)),              [Point(2)]);
}

#[test]
fn open_center() {
    let a: TineTree<i32> = Open(0, 3).into();

    assert_eq_i!(a.intersect(&TineTree::from(Empty)),             []);
    assert_eq_i!(a.intersect(&TineTree::from(Point(2))),          [Point(2)]);
    assert_eq_i!(a.intersect(&TineTree::from(Open(0, 3))),        [Open(0, 3)]);
    assert_eq_i!(a.intersect(&TineTree::from(LeftOpen(0, 3))),    [Open(0, 3)]);
    assert_eq_i!(a.intersect(&TineTree::from(RightOpen(0, 3))),   [Open(0, 3)]);
    assert_eq_i!(a.intersect(&TineTree::from(Closed(0, 3))),      [Open(0, 3)]);
    assert_eq_i!(a.intersect(&TineTree::from(UpTo(2))),           [Open(0, 2)]);
    assert_eq_i!(a.intersect(&TineTree::from(UpFrom(2))),         [Open(2, 3)]);
    assert_eq_i!(a.intersect(&TineTree::from(To(2))),             [LeftOpen(0, 2)]);
    assert_eq_i!(a.intersect(&TineTree::from(From(2))),           [RightOpen(2, 3)]);
    assert_eq_i!(a.intersect(&TineTree::from(Full)),              [Open(0, 3)]);
}

#[test]
fn open_left() {
    let a: TineTree<i32> = Open(0, 3).into();

    assert_eq_i!(a.intersect(&TineTree::from(Empty)),             []);
    assert_eq_i!(a.intersect(&TineTree::from(Point(-3))),         []);
    assert_eq_i!(a.intersect(&TineTree::from(Open(-3, -1))),      []);
    assert_eq_i!(a.intersect(&TineTree::from(LeftOpen(-3, -1))),  []);
    assert_eq_i!(a.intersect(&TineTree::from(RightOpen(-3, -1))), []);
    assert_eq_i!(a.intersect(&TineTree::from(Closed(-3, -1))),    []);
    assert_eq_i!(a.intersect(&TineTree::from(UpTo(-3))),          []);
    assert_eq_i!(a.intersect(&TineTree::from(UpFrom(-3))),        [Open(0, 3)]);
    assert_eq_i!(a.intersect(&TineTree::from(To(-3))),            []);
    assert_eq_i!(a.intersect(&TineTree::from(From(-3))),          [Open(0, 3)]);
    assert_eq_i!(a.intersect(&TineTree::from(Full)),              [Open(0, 3)]);
}

#[test]
fn open_right() {
    let a: TineTree<i32> = Open(0, 3).into();

    assert_eq_i!(a.intersect(&TineTree::from(Empty)),             []);
    assert_eq_i!(a.intersect(&TineTree::from(Point(13))),         []);
    assert_eq_i!(a.intersect(&TineTree::from(Open(10, 13))),      []);
    assert_eq_i!(a.intersect(&TineTree::from(LeftOpen(10, 13))),  []);
    assert_eq_i!(a.intersect(&TineTree::from(RightOpen(10, 13))), []);
    assert_eq_i!(a.intersect(&TineTree::from(Closed(10, 13))),    []);
    assert_eq_i!(a.intersect(&TineTree::from(UpTo(13))),          [Open(0, 3)]);
    assert_eq_i!(a.intersect(&TineTree::from(UpFrom(13))),        []);
    assert_eq_i!(a.intersect(&TineTree::from(To(13))),            [Open(0, 3)]);
    assert_eq_i!(a.intersect(&TineTree::from(From(13))),          []);
    assert_eq_i!(a.intersect(&TineTree::from(Full)),              [Open(0, 3)]);
}

#[test]
fn left_open_center() {
    let a: TineTree<i32> = LeftOpen(0, 3).into();

    assert_eq_i!(a.intersect(&TineTree::from(Empty)),             []);
    assert_eq_i!(a.intersect(&TineTree::from(Point(2))),          [Point(2)]);
    assert_eq_i!(a.intersect(&TineTree::from(Open(0, 3))),        [Open(0, 3)]);
    assert_eq_i!(a.intersect(&TineTree::from(LeftOpen(0, 3))),    [LeftOpen(0, 3)]);
    assert_eq_i!(a.intersect(&TineTree::from(RightOpen(0, 3))),   [Open(0, 3)]);
    assert_eq_i!(a.intersect(&TineTree::from(Closed(0, 3))),      [LeftOpen(0, 3)]);
    assert_eq_i!(a.intersect(&TineTree::from(UpTo(2))),           [Open(0, 2)]);
    assert_eq_i!(a.intersect(&TineTree::from(UpFrom(2))),         [LeftOpen(2, 3)]);
    assert_eq_i!(a.intersect(&TineTree::from(To(2))),             [LeftOpen(0, 2)]);
    assert_eq_i!(a.intersect(&TineTree::from(From(2))),           [Closed(2, 3)]);
    assert_eq_i!(a.intersect(&TineTree::from(Full)),              [LeftOpen(0, 3)]);
}

#[test]
fn left_open_left() {
    let a: TineTree<i32> = LeftOpen(0, 3).into();

    assert_eq_i!(a.intersect(&TineTree::from(Empty)),             []);
    assert_eq_i!(a.intersect(&TineTree::from(Point(-3))),         []);
    assert_eq_i!(a.intersect(&TineTree::from(Open(-3, -1))),      []);
    assert_eq_i!(a.intersect(&TineTree::from(LeftOpen(-3, -1))),  []);
    assert_eq_i!(a.intersect(&TineTree::from(RightOpen(-3, -1))), []);
    assert_eq_i!(a.intersect(&TineTree::from(Closed(-3, -1))),    []);
    assert_eq_i!(a.intersect(&TineTree::from(UpTo(-3))),          []);
    assert_eq_i!(a.intersect(&TineTree::from(UpFrom(-3))),        [LeftOpen(0, 3)]);
    assert_eq_i!(a.intersect(&TineTree::from(To(-3))),            []);
    assert_eq_i!(a.intersect(&TineTree::from(From(-3))),          [LeftOpen(0, 3)]);
    assert_eq_i!(a.intersect(&TineTree::from(Full)),              [LeftOpen(0, 3)]);
}

#[test]
fn left_open_right() {
    let a: TineTree<i32> = LeftOpen(0, 3).into();

    assert_eq_i!(a.intersect(&TineTree::from(Empty)),             []);
    assert_eq_i!(a.intersect(&TineTree::from(Point(13))),         []);
    assert_eq_i!(a.intersect(&TineTree::from(Open(10, 13))),      []);
    assert_eq_i!(a.intersect(&TineTree::from(LeftOpen(10, 13))),  []);
    assert_eq_i!(a.intersect(&TineTree::from(RightOpen(10, 13))), []);
    assert_eq_i!(a.intersect(&TineTree::from(Closed(10, 13))),    []);
    assert_eq_i!(a.intersect(&TineTree::from(UpTo(13))),          [LeftOpen(0, 3)]);
    assert_eq_i!(a.intersect(&TineTree::from(UpFrom(13))),        []);
    assert_eq_i!(a.intersect(&TineTree::from(To(13))),            [LeftOpen(0, 3)]);
    assert_eq_i!(a.intersect(&TineTree::from(From(13))),          []);
    assert_eq_i!(a.intersect(&TineTree::from(Full)),              [LeftOpen(0, 3)]);
}

#[test]
fn right_open_center() {
    let a: TineTree<i32> = RightOpen(0, 3).into();

    assert_eq_i!(a.intersect(&TineTree::from(Empty)),             []);
    assert_eq_i!(a.intersect(&TineTree::from(Point(2))),          [Point(2)]);
    assert_eq_i!(a.intersect(&TineTree::from(Open(0, 3))),        [Open(0, 3)]);
    assert_eq_i!(a.intersect(&TineTree::from(LeftOpen(0, 3))),    [Open(0, 3)]);
    assert_eq_i!(a.intersect(&TineTree::from(RightOpen(0, 3))),   [RightOpen(0, 3)]);
    assert_eq_i!(a.intersect(&TineTree::from(Closed(0, 3))),      [RightOpen(0, 3)]);
    assert_eq_i!(a.intersect(&TineTree::from(UpTo(2))),           [RightOpen(0, 2)]);
    assert_eq_i!(a.intersect(&TineTree::from(UpFrom(2))),         [Open(2, 3)]);
    assert_eq_i!(a.intersect(&TineTree::from(To(2))),             [Closed(0, 2)]);
    assert_eq_i!(a.intersect(&TineTree::from(From(2))),           [RightOpen(2, 3)]);
    assert_eq_i!(a.intersect(&TineTree::from(Full)),              [RightOpen(0, 3)]);
}

#[test]
fn right_open_left() {
    let a: TineTree<i32> = RightOpen(0, 3).into();

    assert_eq_i!(a.intersect(&TineTree::from(Empty)),             []);
    assert_eq_i!(a.intersect(&TineTree::from(Point(-3))),         []);
    assert_eq_i!(a.intersect(&TineTree::from(Open(-3, -1))),      []);
    assert_eq_i!(a.intersect(&TineTree::from(LeftOpen(-3, -1))),  []);
    assert_eq_i!(a.intersect(&TineTree::from(RightOpen(-3, -1))), []);
    assert_eq_i!(a.intersect(&TineTree::from(Closed(-3, -1))),    []);
    assert_eq_i!(a.intersect(&TineTree::from(UpTo(-3))),          []);
    assert_eq_i!(a.intersect(&TineTree::from(UpFrom(-3))),        [RightOpen(0, 3)]);
    assert_eq_i!(a.intersect(&TineTree::from(To(-3))),            []);
    assert_eq_i!(a.intersect(&TineTree::from(From(-3))),          [RightOpen(0, 3)]);
    assert_eq_i!(a.intersect(&TineTree::from(Full)),              [RightOpen(0, 3)]);
}

#[test]
fn right_open_right() {
    let a: TineTree<i32> = RightOpen(0, 3).into();

    assert_eq_i!(a.intersect(&TineTree::from(Empty)),             []);
    assert_eq_i!(a.intersect(&TineTree::from(Point(13))),         []);
    assert_eq_i!(a.intersect(&TineTree::from(Open(10, 13))),      []);
    assert_eq_i!(a.intersect(&TineTree::from(LeftOpen(10, 13))),  []);
    assert_eq_i!(a.intersect(&TineTree::from(RightOpen(10, 13))), []);
    assert_eq_i!(a.intersect(&TineTree::from(Closed(10, 13))),    []);
    assert_eq_i!(a.intersect(&TineTree::from(UpTo(13))),          [RightOpen(0, 3)]);
    assert_eq_i!(a.intersect(&TineTree::from(UpFrom(13))),        []);
    assert_eq_i!(a.intersect(&TineTree::from(To(13))),            [RightOpen(0, 3)]);
    assert_eq_i!(a.intersect(&TineTree::from(From(13))),          []);
    assert_eq_i!(a.intersect(&TineTree::from(Full)),              [RightOpen(0, 3)]);
}

#[test]
fn closed_center() {
    let a: TineTree<i32> = Closed(0, 3).into();

    assert_eq_i!(a.intersect(&TineTree::from(Empty)),             []);
    assert_eq_i!(a.intersect(&TineTree::from(Point(2))),          [Point(2)]);
    assert_eq_i!(a.intersect(&TineTree::from(Open(0, 3))),        [Open(0, 3)]);
    assert_eq_i!(a.intersect(&TineTree::from(LeftOpen(0, 3))),    [LeftOpen(0, 3)]);
    assert_eq_i!(a.intersect(&TineTree::from(RightOpen(0, 3))),   [RightOpen(0, 3)]);
    assert_eq_i!(a.intersect(&TineTree::from(Closed(0, 3))),      [Closed(0, 3)]);
    assert_eq_i!(a.intersect(&TineTree::from(UpTo(2))),           [RightOpen(0, 2)]);
    assert_eq_i!(a.intersect(&TineTree::from(UpFrom(2))),         [LeftOpen(2, 3)]);
    assert_eq_i!(a.intersect(&TineTree::from(To(2))),             [Closed(0, 2)]);
    assert_eq_i!(a.intersect(&TineTree::from(From(2))),           [Closed(2, 3)]);
    assert_eq_i!(a.intersect(&TineTree::from(Full)),              [Closed(0, 3)]);
}

#[test]
fn closed_left() {
    let a: TineTree<i32> = Closed(0, 3).into();

    assert_eq_i!(a.intersect(&TineTree::from(Empty)),             []);
    assert_eq_i!(a.intersect(&TineTree::from(Point(-3))),         []);
    assert_eq_i!(a.intersect(&TineTree::from(Open(-3, -1))),      []);
    assert_eq_i!(a.intersect(&TineTree::from(LeftOpen(-3, -1))),  []);
    assert_eq_i!(a.intersect(&TineTree::from(RightOpen(-3, -1))), []);
    assert_eq_i!(a.intersect(&TineTree::from(Closed(-3, -1))),    []);
    assert_eq_i!(a.intersect(&TineTree::from(UpTo(-3))),          []);
    assert_eq_i!(a.intersect(&TineTree::from(UpFrom(-3))),        [Closed(0, 3)]);
    assert_eq_i!(a.intersect(&TineTree::from(To(-3))),            []);
    assert_eq_i!(a.intersect(&TineTree::from(From(-3))),          [Closed(0, 3)]);
    assert_eq_i!(a.intersect(&TineTree::from(Full)),              [Closed(0, 3)]);
}

#[test]
fn closed_right() {
    let a: TineTree<i32> = Closed(0, 3).into();

    assert_eq_i!(a.intersect(&TineTree::from(Empty)),             []);
    assert_eq_i!(a.intersect(&TineTree::from(Point(13))),         []);
    assert_eq_i!(a.intersect(&TineTree::from(Open(10, 13))),      []);
    assert_eq_i!(a.intersect(&TineTree::from(LeftOpen(10, 13))),  []);
    assert_eq_i!(a.intersect(&TineTree::from(RightOpen(10, 13))), []);
    assert_eq_i!(a.intersect(&TineTree::from(Closed(10, 13))),    []);
    assert_eq_i!(a.intersect(&TineTree::from(UpTo(13))),          [Closed(0, 3)]);
    assert_eq_i!(a.intersect(&TineTree::from(UpFrom(13))),        []);
    assert_eq_i!(a.intersect(&TineTree::from(To(13))),            [Closed(0, 3)]);
    assert_eq_i!(a.intersect(&TineTree::from(From(13))),          []);
    assert_eq_i!(a.intersect(&TineTree::from(Full)),              [Closed(0, 3)]);
}

#[test]
fn up_to_center() {
    let a: TineTree<i32> = UpTo(3).into();

    assert_eq_i!(a.intersect(&TineTree::from(Empty)),             []);
    assert_eq_i!(a.intersect(&TineTree::from(Point(2))),          [Point(2)]);
    assert_eq_i!(a.intersect(&TineTree::from(Open(0, 3))),        [Open(0, 3)]);
    assert_eq_i!(a.intersect(&TineTree::from(LeftOpen(0, 3))),    [Open(0, 3)]);
    assert_eq_i!(a.intersect(&TineTree::from(RightOpen(0, 3))),   [RightOpen(0, 3)]);
    assert_eq_i!(a.intersect(&TineTree::from(Closed(0, 3))),      [RightOpen(0, 3)]);
    assert_eq_i!(a.intersect(&TineTree::from(UpTo(2))),           [UpTo(2)]);
    assert_eq_i!(a.intersect(&TineTree::from(UpFrom(2))),         [Open(2, 3)]);
    assert_eq_i!(a.intersect(&TineTree::from(To(2))),             [To(2)]);
    assert_eq_i!(a.intersect(&TineTree::from(From(2))),           [RightOpen(2, 3)]);
    assert_eq_i!(a.intersect(&TineTree::from(Full)),              [UpTo(3)]);
}

#[test]
fn up_to_left() {
    let a: TineTree<i32> = UpTo(3).into();

    assert_eq_i!(a.intersect(&TineTree::from(Empty)),             []);
    assert_eq_i!(a.intersect(&TineTree::from(Point(-3))),         [Point(-3)]);
    assert_eq_i!(a.intersect(&TineTree::from(Open(-3, -1))),      [Open(-3, -1)]);
    assert_eq_i!(a.intersect(&TineTree::from(LeftOpen(-3, -1))),  [LeftOpen(-3, -1)]);
    assert_eq_i!(a.intersect(&TineTree::from(RightOpen(-3, -1))), [RightOpen(-3, -1)]);
    assert_eq_i!(a.intersect(&TineTree::from(Closed(-3, -1))),    [Closed(-3, -1)]);
    assert_eq_i!(a.intersect(&TineTree::from(UpTo(-3))),          [UpTo(-3)]);
    assert_eq_i!(a.intersect(&TineTree::from(UpFrom(-3))),        [Open(-3, 3)]);
    assert_eq_i!(a.intersect(&TineTree::from(To(-3))),            [To(-3)]);
    assert_eq_i!(a.intersect(&TineTree::from(From(-3))),          [RightOpen(-3, 3)]);
    assert_eq_i!(a.intersect(&TineTree::from(Full)),              [UpTo(3)]);
}

#[test]
fn up_to_right() {
    let a: TineTree<i32> = UpTo(3).into();

    assert_eq_i!(a.intersect(&TineTree::from(Empty)),             []);
    assert_eq_i!(a.intersect(&TineTree::from(Point(13))),         []);
    assert_eq_i!(a.intersect(&TineTree::from(Open(10, 13))),      []);
    assert_eq_i!(a.intersect(&TineTree::from(LeftOpen(10, 13))),  []);
    assert_eq_i!(a.intersect(&TineTree::from(RightOpen(10, 13))), []);
    assert_eq_i!(a.intersect(&TineTree::from(Closed(10, 13))),    []);
    assert_eq_i!(a.intersect(&TineTree::from(UpTo(13))),          [UpTo(3)]);
    assert_eq_i!(a.intersect(&TineTree::from(UpFrom(13))),        []);
    assert_eq_i!(a.intersect(&TineTree::from(To(13))),            [UpTo(3)]);
    assert_eq_i!(a.intersect(&TineTree::from(From(13))),          []);
    assert_eq_i!(a.intersect(&TineTree::from(Full)),              [UpTo(3)]);
}

#[test]
fn up_from_center() {
    let a: TineTree<i32> = UpFrom(0).into();

    assert_eq_i!(a.intersect(&TineTree::from(Empty)),             []);
    assert_eq_i!(a.intersect(&TineTree::from(Point(2))),          [Point(2)]);
    assert_eq_i!(a.intersect(&TineTree::from(Open(0, 3))),        [Open(0, 3)]);
    assert_eq_i!(a.intersect(&TineTree::from(LeftOpen(0, 3))),    [LeftOpen(0, 3)]);
    assert_eq_i!(a.intersect(&TineTree::from(RightOpen(0, 3))),   [Open(0, 3)]);
    assert_eq_i!(a.intersect(&TineTree::from(Closed(0, 3))),      [LeftOpen(0, 3)]);
    assert_eq_i!(a.intersect(&TineTree::from(UpTo(2))),           [Open(0, 2)]);
    assert_eq_i!(a.intersect(&TineTree::from(UpFrom(2))),         [UpFrom(2)]);
    assert_eq_i!(a.intersect(&TineTree::from(To(2))),             [LeftOpen(0, 2)]);
    assert_eq_i!(a.intersect(&TineTree::from(From(2))),           [From(2)]);
    assert_eq_i!(a.intersect(&TineTree::from(Full)),              [UpFrom(0)]);
}

#[test]
fn up_from_left() {
    let a: TineTree<i32> = UpFrom(0).into();

    assert_eq_i!(a.intersect(&TineTree::from(Empty)),             []);
    assert_eq_i!(a.intersect(&TineTree::from(Point(-3))),         []);
    assert_eq_i!(a.intersect(&TineTree::from(Open(-3, -1))),      []);
    assert_eq_i!(a.intersect(&TineTree::from(LeftOpen(-3, -1))),  []);
    assert_eq_i!(a.intersect(&TineTree::from(RightOpen(-3, -1))), []);
    assert_eq_i!(a.intersect(&TineTree::from(Closed(-3, -1))),    []);
    assert_eq_i!(a.intersect(&TineTree::from(UpTo(-3))),          []);
    assert_eq_i!(a.intersect(&TineTree::from(UpFrom(-3))),        [UpFrom(0)]);
    assert_eq_i!(a.intersect(&TineTree::from(To(-3))),            []);
    assert_eq_i!(a.intersect(&TineTree::from(From(-3))),          [UpFrom(0)]);
    assert_eq_i!(a.intersect(&TineTree::from(Full)),              [UpFrom(0)]);
}

#[test]
fn up_from_right() {
    let a: TineTree<i32> = UpFrom(0).into();

    assert_eq_i!(a.intersect(&TineTree::from(Empty)),             []);
    assert_eq_i!(a.intersect(&TineTree::from(Point(13))),         [Point(13)]);
    assert_eq_i!(a.intersect(&TineTree::from(Open(10, 13))),      [Open(10, 13)]);
    assert_eq_i!(a.intersect(&TineTree::from(LeftOpen(10, 13))),  [LeftOpen(10, 13)]);
    assert_eq_i!(a.intersect(&TineTree::from(RightOpen(10, 13))), [RightOpen(10, 13)]);
    assert_eq_i!(a.intersect(&TineTree::from(Closed(10, 13))),    [Closed(10, 13)]);
    assert_eq_i!(a.intersect(&TineTree::from(UpTo(13))),          [Open(0, 13)]);
    assert_eq_i!(a.intersect(&TineTree::from(UpFrom(13))),        [UpFrom(13)]);
    assert_eq_i!(a.intersect(&TineTree::from(To(13))),            [LeftOpen(0, 13)]);
    assert_eq_i!(a.intersect(&TineTree::from(From(13))),          [From(13)]);
    assert_eq_i!(a.intersect(&TineTree::from(Full)),              [UpFrom(0)]);
}

#[test]
fn to_center() {
    let a: TineTree<i32> = To(3).into();

    assert_eq_i!(a.intersect(&TineTree::from(Empty)),             []);
    assert_eq_i!(a.intersect(&TineTree::from(Point(2))),          [Point(2)]);
    assert_eq_i!(a.intersect(&TineTree::from(Open(0, 3))),        [Open(0, 3)]);
    assert_eq_i!(a.intersect(&TineTree::from(LeftOpen(0, 3))),    [LeftOpen(0, 3)]);
    assert_eq_i!(a.intersect(&TineTree::from(RightOpen(0, 3))),   [RightOpen(0, 3)]);
    assert_eq_i!(a.intersect(&TineTree::from(Closed(0, 3))),      [Closed(0, 3)]);
    assert_eq_i!(a.intersect(&TineTree::from(UpTo(2))),           [UpTo(2)]);
    assert_eq_i!(a.intersect(&TineTree::from(UpFrom(2))),         [LeftOpen(2, 3)]);
    assert_eq_i!(a.intersect(&TineTree::from(To(2))),             [To(2)]);
    assert_eq_i!(a.intersect(&TineTree::from(From(2))),           [Closed(2, 3)]);
    assert_eq_i!(a.intersect(&TineTree::from(Full)),              [To(3)]);
}

#[test]
fn to_left() {
    let a: TineTree<i32> = To(3).into();

    assert_eq_i!(a.intersect(&TineTree::from(Empty)),             []);
    assert_eq_i!(a.intersect(&TineTree::from(Point(-3))),         [Point(-3)]);
    assert_eq_i!(a.intersect(&TineTree::from(Open(-3, -1))),      [Open(-3, -1)]);
    assert_eq_i!(a.intersect(&TineTree::from(LeftOpen(-3, -1))),  [LeftOpen(-3, -1)]);
    assert_eq_i!(a.intersect(&TineTree::from(RightOpen(-3, -1))), [RightOpen(-3, -1)]);
    assert_eq_i!(a.intersect(&TineTree::from(Closed(-3, -1))),    [Closed(-3, -1)]);
    assert_eq_i!(a.intersect(&TineTree::from(UpTo(-3))),          [UpTo(-3)]);
    assert_eq_i!(a.intersect(&TineTree::from(UpFrom(-3))),        [LeftOpen(-3, 3)]);
    assert_eq_i!(a.intersect(&TineTree::from(To(-3))),            [To(-3)]);
    assert_eq_i!(a.intersect(&TineTree::from(From(-3))),          [Closed(-3, 3)]);
    assert_eq_i!(a.intersect(&TineTree::from(Full)),              [To(3)]);
}

#[test]
fn to_right() {
    let a: TineTree<i32> = To(3).into();

    assert_eq_i!(a.intersect(&TineTree::from(Empty)),             []);
    assert_eq_i!(a.intersect(&TineTree::from(Point(13))),         []);
    assert_eq_i!(a.intersect(&TineTree::from(Open(10, 13))),      []);
    assert_eq_i!(a.intersect(&TineTree::from(LeftOpen(10, 13))),  []);
    assert_eq_i!(a.intersect(&TineTree::from(RightOpen(10, 13))), []);
    assert_eq_i!(a.intersect(&TineTree::from(Closed(10, 13))),    []);
    assert_eq_i!(a.intersect(&TineTree::from(UpTo(13))),          [To(3)]);
    assert_eq_i!(a.intersect(&TineTree::from(UpFrom(13))),        []);
    assert_eq_i!(a.intersect(&TineTree::from(To(13))),            [To(3)]);
    assert_eq_i!(a.intersect(&TineTree::from(From(13))),          []);
    assert_eq_i!(a.intersect(&TineTree::from(Full)),              [To(3)]);
}

#[test]
fn from_center() {
    let a: TineTree<i32> = From(0).into();

    assert_eq_i!(a.intersect(&TineTree::from(Empty)),             []);
    assert_eq_i!(a.intersect(&TineTree::from(Point(2))),          [Point(2)]);
    assert_eq_i!(a.intersect(&TineTree::from(Open(0, 3))),        [Open(0, 3)]);
    assert_eq_i!(a.intersect(&TineTree::from(LeftOpen(0, 3))),    [LeftOpen(0, 3)]);
    assert_eq_i!(a.intersect(&TineTree::from(RightOpen(0, 3))),   [RightOpen(0, 3)]);
    assert_eq_i!(a.intersect(&TineTree::from(Closed(0, 3))),      [Closed(0, 3)]);
    assert_eq_i!(a.intersect(&TineTree::from(UpTo(2))),           [RightOpen(0, 2)]);
    assert_eq_i!(a.intersect(&TineTree::from(UpFrom(2))),         [UpFrom(2)]);
    assert_eq_i!(a.intersect(&TineTree::from(To(2))),             [Closed(0, 2)]);
    assert_eq_i!(a.intersect(&TineTree::from(From(2))),           [From(2)]);
    assert_eq_i!(a.intersect(&TineTree::from(Full)),              [From(0)]);
}

#[test]
fn from_left() {
    let a: TineTree<i32> = From(0).into();

    assert_eq_i!(a.intersect(&TineTree::from(Empty)),             []);
    assert_eq_i!(a.intersect(&TineTree::from(Point(-3))),         []);
    assert_eq_i!(a.intersect(&TineTree::from(Open(-3, -1))),      []);
    assert_eq_i!(a.intersect(&TineTree::from(LeftOpen(-3, -1))),  []);
    assert_eq_i!(a.intersect(&TineTree::from(RightOpen(-3, -1))), []);
    assert_eq_i!(a.intersect(&TineTree::from(Closed(-3, -1))),    []);
    assert_eq_i!(a.intersect(&TineTree::from(UpTo(-3))),          []);
    assert_eq_i!(a.intersect(&TineTree::from(UpFrom(-3))),        [From(0)]);
    assert_eq_i!(a.intersect(&TineTree::from(To(-3))),            []);
    assert_eq_i!(a.intersect(&TineTree::from(From(-3))),          [From(0)]);
    assert_eq_i!(a.intersect(&TineTree::from(Full)),              [From(0)]);
}

#[test]
fn from_right() {
    let a: TineTree<i32> = From(0).into();

    assert_eq_i!(a.intersect(&TineTree::from(Empty)),             []);
    assert_eq_i!(a.intersect(&TineTree::from(Point(13))),         [Point(13)]);
    assert_eq_i!(a.intersect(&TineTree::from(Open(10, 13))),      [Open(10, 13)]);
    assert_eq_i!(a.intersect(&TineTree::from(LeftOpen(10, 13))),  [LeftOpen(10, 13)]);
    assert_eq_i!(a.intersect(&TineTree::from(RightOpen(10, 13))), [RightOpen(10, 13)]);
    assert_eq_i!(a.intersect(&TineTree::from(Closed(10, 13))),    [Closed(10, 13)]);
    assert_eq_i!(a.intersect(&TineTree::from(UpTo(13))),          [RightOpen(0, 13)]);
    assert_eq_i!(a.intersect(&TineTree::from(UpFrom(13))),        [UpFrom(13)]);
    assert_eq_i!(a.intersect(&TineTree::from(To(13))),            [Closed(0, 13)]);
    assert_eq_i!(a.intersect(&TineTree::from(From(13))),          [From(13)]);
    assert_eq_i!(a.intersect(&TineTree::from(Full)),              [From(0)]);
}

#[test]
fn full() {
    let a: TineTree<i32> = Full.into();

    assert_eq_i!(a.intersect(&TineTree::from(Empty)),             []);
    assert_eq_i!(a.intersect(&TineTree::from(Point(0))),          [Point(0)]);
    assert_eq_i!(a.intersect(&TineTree::from(Open(0, 3))),        [Open(0, 3)]);
    assert_eq_i!(a.intersect(&TineTree::from(LeftOpen(0, 3))),    [LeftOpen(0, 3)]);
    assert_eq_i!(a.intersect(&TineTree::from(RightOpen(0, 3))),   [RightOpen(0, 3)]);
    assert_eq_i!(a.intersect(&TineTree::from(Closed(0, 3))),      [Closed(0, 3)]);
    assert_eq_i!(a.intersect(&TineTree::from(UpTo(0))),           [UpTo(0)]);
    assert_eq_i!(a.intersect(&TineTree::from(UpFrom(0))),         [UpFrom(0)]);
    assert_eq_i!(a.intersect(&TineTree::from(To(0))),             [To(0)]);
    assert_eq_i!(a.intersect(&TineTree::from(From(0))),           [From(0)]);
    assert_eq_i!(a.intersect(&TineTree::from(Full)),              [Full]);
}
