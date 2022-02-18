// Copyright 2018 Skylor R. Schermer.
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
    let mut t: TineTree<i32> = TineTree::new();

    t.union_in_place(&UpTo(0));
	t.union_in_place(&Point(1));
	t.union_in_place(&Empty);
	t.union_in_place(&Open(2, 3));
	t.union_in_place(&LeftOpen(4, 5));
	t.union_in_place(&RightOpen(6, 7));
	t.union_in_place(&Empty);
	t.union_in_place(&Closed(8, 9));
	t.union_in_place(&UpFrom(10));
	t.union_in_place(&Empty);

    assert_eq!(t.iter_intervals().collect::<Vec<_>>(), [
    	UpTo(0),
		Point(1),
		Open(2, 3),
		LeftOpen(4, 5),
		RightOpen(6, 7),
		Closed(8, 9),
		UpFrom(10)]
    );
}

#[test]
fn left_aggregation() {
    let mut t: TineTree<i32> = TineTree::new();

    t.union_in_place(&UpTo(1));
	t.union_in_place(&Point(1));
	t.union_in_place(&Empty);
	t.union_in_place(&Open(0, 3));
	t.union_in_place(&LeftOpen(2, 5));
	t.union_in_place(&RightOpen(4, 7));
	t.union_in_place(&Empty);
	t.union_in_place(&Closed(6, 9));
	t.union_in_place(&UpFrom(8));
	t.union_in_place(&Empty);

    assert_eq!(t.iter_intervals().collect::<Vec<_>>(), [
    	Full]);
}

#[test]
fn right_aggregation() {
    let mut t: TineTree<i32> = TineTree::new();

	t.union_in_place(&UpFrom(8));
	t.union_in_place(&Closed(6, 9));
	t.union_in_place(&Empty);
	t.union_in_place(&RightOpen(4, 7));
	t.union_in_place(&LeftOpen(2, 5));
	t.union_in_place(&Open(0, 3));
	t.union_in_place(&Empty);
	t.union_in_place(&Point(1));
    t.union_in_place(&UpTo(1));
    t.union_in_place(&Empty);

    assert_eq!(t.iter_intervals().collect::<Vec<_>>(), [
    	Full]);
}

#[test]
fn center_aggregation() {
    let mut t: TineTree<i32> = TineTree::new();

    t.union_in_place(&UpTo(10));
	t.union_in_place(&Point(5));
	t.union_in_place(&Empty);
	t.union_in_place(&Open(0, 7));
	t.union_in_place(&LeftOpen(2, 8));
	t.union_in_place(&RightOpen(4, 6));
	t.union_in_place(&Empty);
	t.union_in_place(&Closed(1, 9));
	t.union_in_place(&Empty);

    assert_eq!(t.iter_intervals().collect::<Vec<_>>(), [
    	UpTo(10)]);
}

#[test]
fn adjacent_aggregation() {
    let mut t: TineTree<i32> = TineTree::new();

    t.union_in_place(&UpTo(1));
	t.union_in_place(&Point(1));
	t.union_in_place(&Empty);
	t.union_in_place(&Open(1, 3));
	t.union_in_place(&LeftOpen(3, 5));
	t.union_in_place(&RightOpen(5, 7));
	t.union_in_place(&Empty);
	t.union_in_place(&Closed(7, 9));
	t.union_in_place(&UpFrom(9));
	t.union_in_place(&Empty);

    assert_eq!(t.iter_intervals().collect::<Vec<_>>(), [
    	UpTo(3),
    	UpFrom(3)]);
}

#[test]
fn full_aggregation() {
    let mut t: TineTree<i32> = TineTree::new();

	t.union_in_place(&Full);
    t.union_in_place(&UpTo(1));
	t.union_in_place(&Point(1));
	t.union_in_place(&Empty);
	t.union_in_place(&Open(1, 3));
	t.union_in_place(&LeftOpen(3, 5));
	t.union_in_place(&RightOpen(5, 7));
	t.union_in_place(&Full);
	t.union_in_place(&Closed(7, 9));
	t.union_in_place(&UpFrom(9));
	t.union_in_place(&Full);

    assert_eq!(t.iter_intervals().collect::<Vec<_>>(), [
    	Full]);
}



////////////////////////////////////////////////////////////////////////////////
// Non-mutating union tests.
////////////////////////////////////////////////////////////////////////////////

#[test]
fn empty() {
    let a: TineTree<i32> = Empty.into();

    assert_eq_i!(a.union(&TineTree::from(Empty)),             []);
    assert_eq_i!(a.union(&TineTree::from(Point(3))),          [Point(3)]);
    assert_eq_i!(a.union(&TineTree::from(Open(0, 3))),        [Open(0, 3)]);
    assert_eq_i!(a.union(&TineTree::from(LeftOpen(0, 3))),    [LeftOpen(0, 3)]);
    assert_eq_i!(a.union(&TineTree::from(RightOpen(0, 3))),   [RightOpen(0, 3)]);
    assert_eq_i!(a.union(&TineTree::from(Closed(0, 3))),      [Closed(0, 3)]);
    assert_eq_i!(a.union(&TineTree::from(UpTo(3))),           [UpTo(3)]);
    assert_eq_i!(a.union(&TineTree::from(UpFrom(3))),         [UpFrom(3)]);
    assert_eq_i!(a.union(&TineTree::from(To(3))),             [To(3)]);
    assert_eq_i!(a.union(&TineTree::from(From(3))),           [From(3)]);
    assert_eq_i!(a.union(&TineTree::from(Full)),              [Full]);
}


#[test]
fn empty_in_place() {
    let a: TineTree<i32> = Empty.into();

    assert_eq_i!(a.union(&TineTree::from(Empty)),             []);
    assert_eq_i!(a.union(&TineTree::from(Point(3))),          [Point(3)]);
    assert_eq_i!(a.union(&TineTree::from(Open(0, 3))),        [Open(0, 3)]);
    assert_eq_i!(a.union(&TineTree::from(LeftOpen(0, 3))),    [LeftOpen(0, 3)]);
    assert_eq_i!(a.union(&TineTree::from(RightOpen(0, 3))),   [RightOpen(0, 3)]);
    assert_eq_i!(a.union(&TineTree::from(Closed(0, 3))),      [Closed(0, 3)]);
    assert_eq_i!(a.union(&TineTree::from(UpTo(3))),           [UpTo(3)]);
    assert_eq_i!(a.union(&TineTree::from(UpFrom(3))),         [UpFrom(3)]);
    assert_eq_i!(a.union(&TineTree::from(To(3))),             [To(3)]);
    assert_eq_i!(a.union(&TineTree::from(From(3))),           [From(3)]);
    assert_eq_i!(a.union(&TineTree::from(Full)),              [Full]);
}

#[test]
fn point_center() {
    let a: TineTree<i32> = Point(3).into();

    assert_eq_i!(a.union(&TineTree::from(Empty)),             [Point(3)]);
    assert_eq_i!(a.union(&TineTree::from(Point(3))),          [Point(3)]);
    assert_eq_i!(a.union(&TineTree::from(Open(0, 3))),        [LeftOpen(0, 3)]);
    assert_eq_i!(a.union(&TineTree::from(LeftOpen(0, 3))),    [LeftOpen(0, 3)]);
    assert_eq_i!(a.union(&TineTree::from(RightOpen(0, 3))),   [Closed(0, 3)]);
    assert_eq_i!(a.union(&TineTree::from(Closed(0, 3))),      [Closed(0, 3)]);
    assert_eq_i!(a.union(&TineTree::from(UpTo(3))),           [To(3)]);
    assert_eq_i!(a.union(&TineTree::from(UpFrom(3))),         [From(3)]);
    assert_eq_i!(a.union(&TineTree::from(To(3))),             [To(3)]);
    assert_eq_i!(a.union(&TineTree::from(From(3))),           [From(3)]);
    assert_eq_i!(a.union(&TineTree::from(Full)),              [Full]);
}

#[test]
fn point_left() {
    let a: TineTree<i32> = Point(3).into();

    assert_eq_i!(a.union(&TineTree::from(Empty)),             [Point(3)]);
    assert_eq_i!(a.union(&TineTree::from(Point(-1))),         [Point(3), Point(-1)]);
    assert_eq_i!(a.union(&TineTree::from(Open(-3, -1))),      [Point(3), Open(-3, -1)]);
    assert_eq_i!(a.union(&TineTree::from(LeftOpen(-3, -1))),  [Point(3), LeftOpen(-3, -1)]);
    assert_eq_i!(a.union(&TineTree::from(RightOpen(-3, -1))), [Point(3), RightOpen(-3, -1)]);
    assert_eq_i!(a.union(&TineTree::from(Closed(-3, -1))),    [Point(3), Closed(-3, -1)]);
    assert_eq_i!(a.union(&TineTree::from(UpTo(-3))),          [Point(3), UpTo(-3)]);
    assert_eq_i!(a.union(&TineTree::from(UpFrom(-3))),        [UpFrom(-3)]);
    assert_eq_i!(a.union(&TineTree::from(To(-3))),            [Point(3), To(-3)]);
    assert_eq_i!(a.union(&TineTree::from(From(-3))),          [From(-3)]);
    assert_eq_i!(a.union(&TineTree::from(Full)),              [Full]);
}

#[test]
fn point_right() {
    let a: TineTree<i32> = Point(3).into();

    assert_eq_i!(a.union(&TineTree::from(Empty)),             [Point(3)]);
    assert_eq_i!(a.union(&TineTree::from(Point(10))),         [Point(3), Point(10)]);
    assert_eq_i!(a.union(&TineTree::from(Open(10, 13))),      [Point(3), Open(10, 13)]);
    assert_eq_i!(a.union(&TineTree::from(LeftOpen(10, 13))),  [Point(3), LeftOpen(10, 13)]);
    assert_eq_i!(a.union(&TineTree::from(RightOpen(10, 13))), [Point(3), RightOpen(10, 13)]);
    assert_eq_i!(a.union(&TineTree::from(Closed(10, 13))),    [Point(3), Closed(10, 13)]);
    assert_eq_i!(a.union(&TineTree::from(UpTo(13))),          [UpTo(13)]);
    assert_eq_i!(a.union(&TineTree::from(UpFrom(13))),        [Point(3), UpFrom(13)]);
    assert_eq_i!(a.union(&TineTree::from(To(13))),            [To(13)]);
    assert_eq_i!(a.union(&TineTree::from(From(13))),          [Point(3), From(13)]);
    assert_eq_i!(a.union(&TineTree::from(Full)),              [Full]);
}

#[test]
fn open_center() {
    let a: TineTree<i32> = Open(0, 3).into();

    assert_eq_i!(a.union(&TineTree::from(Empty)),             [Open(0, 3)]);
    assert_eq_i!(a.union(&TineTree::from(Point(3))),          [LeftOpen(0, 3)]);
    assert_eq_i!(a.union(&TineTree::from(Open(0, 3))),        [Open(0, 3)]);
    assert_eq_i!(a.union(&TineTree::from(LeftOpen(0, 3))),    [LeftOpen(0, 3)]);
    assert_eq_i!(a.union(&TineTree::from(RightOpen(0, 3))),   [RightOpen(0, 3)]);
    assert_eq_i!(a.union(&TineTree::from(Closed(0, 3))),      [Closed(0, 3)]);
    assert_eq_i!(a.union(&TineTree::from(UpTo(3))),           [UpTo(3)]);
    assert_eq_i!(a.union(&TineTree::from(UpFrom(3))),         [Open(0, 3), UpFrom(3)]);
    assert_eq_i!(a.union(&TineTree::from(To(3))),             [To(3)]);
    assert_eq_i!(a.union(&TineTree::from(From(3))),           [UpFrom(0)]);
    assert_eq_i!(a.union(&TineTree::from(Full)),              [Full]);
}

#[test]
fn open_left() {
    let a: TineTree<i32> = Open(0, 3).into();

    assert_eq_i!(a.union(&TineTree::from(Empty)),             [Open(0, 3)]);
    assert_eq_i!(a.union(&TineTree::from(Point(-3))),         [Open(0, 3), Point(-3)]);
    assert_eq_i!(a.union(&TineTree::from(Open(-3, -1))),      [Open(0, 3), Open(-3, -1)]);
    assert_eq_i!(a.union(&TineTree::from(LeftOpen(-3, -1))),  [Open(0, 3), LeftOpen(-3, -1)]);
    assert_eq_i!(a.union(&TineTree::from(RightOpen(-3, -1))), [Open(0, 3), RightOpen(-3, -1)]);
    assert_eq_i!(a.union(&TineTree::from(Closed(-3, -1))),    [Open(0, 3), Closed(-3, -1)]);
    assert_eq_i!(a.union(&TineTree::from(UpTo(-3))),          [Open(0, 3), UpTo(-3)]);
    assert_eq_i!(a.union(&TineTree::from(UpFrom(-3))),        [UpFrom(-3)]);
    assert_eq_i!(a.union(&TineTree::from(To(-3))),            [Open(0, 3), To(-3)]);
    assert_eq_i!(a.union(&TineTree::from(From(-3))),          [From(-3)]);
    assert_eq_i!(a.union(&TineTree::from(Full)),              [Full]);
}

#[test]
fn open_right() {
    let a: TineTree<i32> = Open(0, 3).into();

    assert_eq_i!(a.union(&TineTree::from(Empty)),             [Open(0, 3)]);
    assert_eq_i!(a.union(&TineTree::from(Point(13))),         [Open(0, 3), Point(13)]);
    assert_eq_i!(a.union(&TineTree::from(Open(10, 13))),      [Open(0, 3), Open(10, 13)]);
    assert_eq_i!(a.union(&TineTree::from(LeftOpen(10, 13))),  [Open(0, 3), LeftOpen(10, 13)]);
    assert_eq_i!(a.union(&TineTree::from(RightOpen(10, 13))), [Open(0, 3), RightOpen(10, 13)]);
    assert_eq_i!(a.union(&TineTree::from(Closed(10, 13))),    [Open(0, 3), Closed(10, 13)]);
    assert_eq_i!(a.union(&TineTree::from(UpTo(13))),          [UpTo(13)]);
    assert_eq_i!(a.union(&TineTree::from(UpFrom(13))),        [Open(0, 3), UpFrom(13)]);
    assert_eq_i!(a.union(&TineTree::from(To(13))),            [To(13)]);
    assert_eq_i!(a.union(&TineTree::from(From(13))),          [Open(0, 3), From(13)]);
    assert_eq_i!(a.union(&TineTree::from(Full)),              [Full]);
}

#[test]
fn left_open_center() {
    let a: TineTree<i32> = LeftOpen(0, 3).into();

    assert_eq_i!(a.union(&TineTree::from(Empty)),             [LeftOpen(0, 3)]);
    assert_eq_i!(a.union(&TineTree::from(Point(3))),          [LeftOpen(0, 3)]);
    assert_eq_i!(a.union(&TineTree::from(Open(0, 3))),        [LeftOpen(0, 3)]);
    assert_eq_i!(a.union(&TineTree::from(LeftOpen(0, 3))),    [LeftOpen(0, 3)]);
    assert_eq_i!(a.union(&TineTree::from(RightOpen(0, 3))),   [Closed(0, 3)]);
    assert_eq_i!(a.union(&TineTree::from(Closed(0, 3))),      [Closed(0, 3)]);
    assert_eq_i!(a.union(&TineTree::from(UpTo(3))),           [To(3)]);
    assert_eq_i!(a.union(&TineTree::from(UpFrom(3))),         [UpFrom(0)]);
    assert_eq_i!(a.union(&TineTree::from(To(3))),             [To(3)]);
    assert_eq_i!(a.union(&TineTree::from(From(3))),           [UpFrom(0)]);
    assert_eq_i!(a.union(&TineTree::from(Full)),              [Full]);
}

#[test]
fn left_open_left() {
    let a: TineTree<i32> = LeftOpen(0, 3).into();

    assert_eq_i!(a.union(&TineTree::from(Empty)),             [LeftOpen(0, 3)]);
    assert_eq_i!(a.union(&TineTree::from(Point(-3))),         [LeftOpen(0, 3), Point(-3)]);
    assert_eq_i!(a.union(&TineTree::from(Open(-3, -1))),      [LeftOpen(0, 3), Open(-3, -1)]);
    assert_eq_i!(a.union(&TineTree::from(LeftOpen(-3, -1))),  [LeftOpen(0, 3), LeftOpen(-3, -1)]);
    assert_eq_i!(a.union(&TineTree::from(RightOpen(-3, -1))), [LeftOpen(0, 3), RightOpen(-3, -1)]);
    assert_eq_i!(a.union(&TineTree::from(Closed(-3, -1))),    [LeftOpen(0, 3), Closed(-3, -1)]);
    assert_eq_i!(a.union(&TineTree::from(UpTo(-3))),          [LeftOpen(0, 3), UpTo(-3)]);
    assert_eq_i!(a.union(&TineTree::from(UpFrom(-3))),        [UpFrom(-3)]);
    assert_eq_i!(a.union(&TineTree::from(To(-3))),            [LeftOpen(0, 3), To(-3)]);
    assert_eq_i!(a.union(&TineTree::from(From(-3))),          [From(-3)]);
    assert_eq_i!(a.union(&TineTree::from(Full)),              [Full]);
}

#[test]
fn left_open_right() {
    let a: TineTree<i32> = LeftOpen(0, 3).into();

    assert_eq_i!(a.union(&TineTree::from(Empty)),             [LeftOpen(0, 3)]);
    assert_eq_i!(a.union(&TineTree::from(Point(13))),         [LeftOpen(0, 3), Point(13)]);
    assert_eq_i!(a.union(&TineTree::from(Open(10, 13))),      [LeftOpen(0, 3), Open(10, 13)]);
    assert_eq_i!(a.union(&TineTree::from(LeftOpen(10, 13))),  [LeftOpen(0, 3), LeftOpen(10, 13)]);
    assert_eq_i!(a.union(&TineTree::from(RightOpen(10, 13))), [LeftOpen(0, 3), RightOpen(10, 13)]);
    assert_eq_i!(a.union(&TineTree::from(Closed(10, 13))),    [LeftOpen(0, 3), Closed(10, 13)]);
    assert_eq_i!(a.union(&TineTree::from(UpTo(13))),          [UpTo(13)]);
    assert_eq_i!(a.union(&TineTree::from(UpFrom(13))),        [LeftOpen(0, 3), UpFrom(13)]);
    assert_eq_i!(a.union(&TineTree::from(To(13))),            [To(13)]);
    assert_eq_i!(a.union(&TineTree::from(From(13))),          [LeftOpen(0, 3), From(13)]);
    assert_eq_i!(a.union(&TineTree::from(Full)),              [Full]);
}

#[test]
fn right_open_center() {
    let a: TineTree<i32> = RightOpen(0, 3).into();

    assert_eq_i!(a.union(&TineTree::from(Empty)),             [RightOpen(0, 3)]);
    assert_eq_i!(a.union(&TineTree::from(Point(3))),          [Closed(0, 3)]);
    assert_eq_i!(a.union(&TineTree::from(Open(0, 3))),        [RightOpen(0, 3)]);
    assert_eq_i!(a.union(&TineTree::from(LeftOpen(0, 3))),    [Closed(0, 3)]);
    assert_eq_i!(a.union(&TineTree::from(RightOpen(0, 3))),   [RightOpen(0, 3)]);
    assert_eq_i!(a.union(&TineTree::from(Closed(0, 3))),      [Closed(0, 3)]);
    assert_eq_i!(a.union(&TineTree::from(UpTo(3))),           [UpTo(3)]);
    assert_eq_i!(a.union(&TineTree::from(UpFrom(3))),         [RightOpen(0, 3), UpFrom(3)]);
    assert_eq_i!(a.union(&TineTree::from(To(3))),             [To(3)]);
    assert_eq_i!(a.union(&TineTree::from(From(3))),           [From(0)]);
    assert_eq_i!(a.union(&TineTree::from(Full)),              [Full]);
}

#[test]
fn right_open_left() {
    let a: TineTree<i32> = RightOpen(0, 3).into();

    assert_eq_i!(a.union(&TineTree::from(Empty)),             [RightOpen(0, 3)]);
    assert_eq_i!(a.union(&TineTree::from(Point(-3))),         [RightOpen(0, 3), Point(-3)]);
    assert_eq_i!(a.union(&TineTree::from(Open(-3, -1))),      [RightOpen(0, 3), Open(-3, -1)]);
    assert_eq_i!(a.union(&TineTree::from(LeftOpen(-3, -1))),  [RightOpen(0, 3), LeftOpen(-3, -1)]);
    assert_eq_i!(a.union(&TineTree::from(RightOpen(-3, -1))), [RightOpen(0, 3), RightOpen(-3, -1)]);
    assert_eq_i!(a.union(&TineTree::from(Closed(-3, -1))),    [RightOpen(0, 3), Closed(-3, -1)]);
    assert_eq_i!(a.union(&TineTree::from(UpTo(-3))),          [RightOpen(0, 3), UpTo(-3)]);
    assert_eq_i!(a.union(&TineTree::from(UpFrom(-3))),        [UpFrom(-3)]);
    assert_eq_i!(a.union(&TineTree::from(To(-3))),            [RightOpen(0, 3), To(-3)]);
    assert_eq_i!(a.union(&TineTree::from(From(-3))),          [From(-3)]);
    assert_eq_i!(a.union(&TineTree::from(Full)),              [Full]);
}

#[test]
fn right_open_right() {
    let a: TineTree<i32> = RightOpen(0, 3).into();

    assert_eq_i!(a.union(&TineTree::from(Empty)),             [RightOpen(0, 3)]);
    assert_eq_i!(a.union(&TineTree::from(Point(13))),         [RightOpen(0, 3), Point(13)]);
    assert_eq_i!(a.union(&TineTree::from(Open(10, 13))),      [RightOpen(0, 3), Open(10, 13)]);
    assert_eq_i!(a.union(&TineTree::from(LeftOpen(10, 13))),  [RightOpen(0, 3), LeftOpen(10, 13)]);
    assert_eq_i!(a.union(&TineTree::from(RightOpen(10, 13))), [RightOpen(0, 3), RightOpen(10, 13)]);
    assert_eq_i!(a.union(&TineTree::from(Closed(10, 13))),    [RightOpen(0, 3), Closed(10, 13)]);
    assert_eq_i!(a.union(&TineTree::from(UpTo(13))),          [UpTo(13)]);
    assert_eq_i!(a.union(&TineTree::from(UpFrom(13))),        [RightOpen(0, 3), UpFrom(13)]);
    assert_eq_i!(a.union(&TineTree::from(To(13))),            [To(13)]);
    assert_eq_i!(a.union(&TineTree::from(From(13))),          [RightOpen(0, 3), From(13)]);
    assert_eq_i!(a.union(&TineTree::from(Full)),              [Full]);
}

#[test]
fn closed_center() {
    let a: TineTree<i32> = Closed(0, 3).into();

    assert_eq_i!(a.union(&TineTree::from(Empty)),             [Closed(0, 3)]);
    assert_eq_i!(a.union(&TineTree::from(Point(3))),          [Closed(0, 3)]);
    assert_eq_i!(a.union(&TineTree::from(Open(0, 3))),        [Closed(0, 3)]);
    assert_eq_i!(a.union(&TineTree::from(LeftOpen(0, 3))),    [Closed(0, 3)]);
    assert_eq_i!(a.union(&TineTree::from(RightOpen(0, 3))),   [Closed(0, 3)]);
    assert_eq_i!(a.union(&TineTree::from(Closed(0, 3))),      [Closed(0, 3)]);
    assert_eq_i!(a.union(&TineTree::from(UpTo(3))),           [To(3)]);
    assert_eq_i!(a.union(&TineTree::from(UpFrom(3))),         [From(0)]);
    assert_eq_i!(a.union(&TineTree::from(To(3))),             [To(3)]);
    assert_eq_i!(a.union(&TineTree::from(From(3))),           [From(0)]);
    assert_eq_i!(a.union(&TineTree::from(Full)),              [Full]);
}

#[test]
fn closed_left() {
    let a: TineTree<i32> = Closed(0, 3).into();

    assert_eq_i!(a.union(&TineTree::from(Empty)),             [Closed(0, 3)]);
    assert_eq_i!(a.union(&TineTree::from(Point(-3))),         [Closed(0, 3), Point(-3)]);
    assert_eq_i!(a.union(&TineTree::from(Open(-3, -1))),      [Closed(0, 3), Open(-3, -1)]);
    assert_eq_i!(a.union(&TineTree::from(LeftOpen(-3, -1))),  [Closed(0, 3), LeftOpen(-3, -1)]);
    assert_eq_i!(a.union(&TineTree::from(RightOpen(-3, -1))), [Closed(0, 3), RightOpen(-3, -1)]);
    assert_eq_i!(a.union(&TineTree::from(Closed(-3, -1))),    [Closed(0, 3), Closed(-3, -1)]);
    assert_eq_i!(a.union(&TineTree::from(UpTo(-3))),          [Closed(0, 3), UpTo(-3)]);
    assert_eq_i!(a.union(&TineTree::from(UpFrom(-3))),        [UpFrom(-3)]);
    assert_eq_i!(a.union(&TineTree::from(To(-3))),            [Closed(0, 3), To(-3)]);
    assert_eq_i!(a.union(&TineTree::from(From(-3))),          [From(-3)]);
    assert_eq_i!(a.union(&TineTree::from(Full)),              [Full]);
}

#[test]
fn closed_right() {
    let a: TineTree<i32> = Closed(0, 3).into();

    assert_eq_i!(a.union(&TineTree::from(Empty)),             [Closed(0, 3)]);
    assert_eq_i!(a.union(&TineTree::from(Point(13))),         [Closed(0, 3), Point(13)]);
    assert_eq_i!(a.union(&TineTree::from(Open(10, 13))),      [Closed(0, 3), Open(10, 13)]);
    assert_eq_i!(a.union(&TineTree::from(LeftOpen(10, 13))),  [Closed(0, 3), LeftOpen(10, 13)]);
    assert_eq_i!(a.union(&TineTree::from(RightOpen(10, 13))), [Closed(0, 3), RightOpen(10, 13)]);
    assert_eq_i!(a.union(&TineTree::from(Closed(10, 13))),    [Closed(0, 3), Closed(10, 13)]);
    assert_eq_i!(a.union(&TineTree::from(UpTo(13))),          [UpTo(13)]);
    assert_eq_i!(a.union(&TineTree::from(UpFrom(13))),        [Closed(0, 3), UpFrom(13)]);
    assert_eq_i!(a.union(&TineTree::from(To(13))),            [To(13)]);
    assert_eq_i!(a.union(&TineTree::from(From(13))),          [Closed(0, 3), From(13)]);
    assert_eq_i!(a.union(&TineTree::from(Full)),              [Full]);
}

#[test]
fn up_to_center() {
    let a: TineTree<i32> = UpTo(3).into();

    assert_eq_i!(a.union(&TineTree::from(Empty)),             [UpTo(3)]);
    assert_eq_i!(a.union(&TineTree::from(Point(3))),          [To(3)]);
    assert_eq_i!(a.union(&TineTree::from(Open(0, 3))),        [UpTo(3)]);
    assert_eq_i!(a.union(&TineTree::from(LeftOpen(0, 3))),    [To(3)]);
    assert_eq_i!(a.union(&TineTree::from(RightOpen(0, 3))),   [UpTo(3)]);
    assert_eq_i!(a.union(&TineTree::from(Closed(0, 3))),      [To(3)]);
    assert_eq_i!(a.union(&TineTree::from(UpTo(3))),           [UpTo(3)]);
    assert_eq_i!(a.union(&TineTree::from(UpFrom(3))),         [UpTo(3), UpFrom(3)]);
    assert_eq_i!(a.union(&TineTree::from(To(3))),             [To(3)]);
    assert_eq_i!(a.union(&TineTree::from(From(3))),           [Full]);
    assert_eq_i!(a.union(&TineTree::from(Full)),              [Full]);
}

#[test]
fn up_to_left() {
    let a: TineTree<i32> = UpTo(3).into();

    assert_eq_i!(a.union(&TineTree::from(Empty)),             [UpTo(3)]);
    assert_eq_i!(a.union(&TineTree::from(Point(-3))),         [UpTo(3)]);
    assert_eq_i!(a.union(&TineTree::from(Open(-3, -1))),      [UpTo(3)]);
    assert_eq_i!(a.union(&TineTree::from(LeftOpen(-3, -1))),  [UpTo(3)]);
    assert_eq_i!(a.union(&TineTree::from(RightOpen(-3, -1))), [UpTo(3)]);
    assert_eq_i!(a.union(&TineTree::from(Closed(-3, -1))),    [UpTo(3)]);
    assert_eq_i!(a.union(&TineTree::from(UpTo(-3))),          [UpTo(3)]);
    assert_eq_i!(a.union(&TineTree::from(UpFrom(-3))),        [Full]);
    assert_eq_i!(a.union(&TineTree::from(To(-3))),            [UpTo(3)]);
    assert_eq_i!(a.union(&TineTree::from(From(-3))),          [Full]);
    assert_eq_i!(a.union(&TineTree::from(Full)),              [Full]);
}

#[test]
fn up_to_right() {
    let a: TineTree<i32> = UpTo(3).into();

    assert_eq_i!(a.union(&TineTree::from(Empty)),             [UpTo(3)]);
    assert_eq_i!(a.union(&TineTree::from(Point(13))),         [UpTo(3), Point(13)]);
    assert_eq_i!(a.union(&TineTree::from(Open(10, 13))),      [UpTo(3), Open(10, 13)]);
    assert_eq_i!(a.union(&TineTree::from(LeftOpen(10, 13))),  [UpTo(3), LeftOpen(10, 13)]);
    assert_eq_i!(a.union(&TineTree::from(RightOpen(10, 13))), [UpTo(3), RightOpen(10, 13)]);
    assert_eq_i!(a.union(&TineTree::from(Closed(10, 13))),    [UpTo(3), Closed(10, 13)]);
    assert_eq_i!(a.union(&TineTree::from(UpTo(13))),          [UpTo(13)]);
    assert_eq_i!(a.union(&TineTree::from(UpFrom(13))),        [UpTo(3), UpFrom(13)]);
    assert_eq_i!(a.union(&TineTree::from(To(13))),            [To(13)]);
    assert_eq_i!(a.union(&TineTree::from(From(13))),          [UpTo(3), From(13)]);
    assert_eq_i!(a.union(&TineTree::from(Full)),              [Full]);
}

#[test]
fn up_from_center() {
    let a: TineTree<i32> = UpFrom(0).into();

    assert_eq_i!(a.union(&TineTree::from(Empty)),             [UpFrom(0)]);
    assert_eq_i!(a.union(&TineTree::from(Point(0))),          [From(0)]);
    assert_eq_i!(a.union(&TineTree::from(Open(0, 3))),        [UpFrom(0)]);
    assert_eq_i!(a.union(&TineTree::from(LeftOpen(0, 3))),    [UpFrom(0)]);
    assert_eq_i!(a.union(&TineTree::from(RightOpen(0, 3))),   [From(0)]);
    assert_eq_i!(a.union(&TineTree::from(Closed(0, 3))),      [From(0)]);
    assert_eq_i!(a.union(&TineTree::from(UpTo(0))),           [UpTo(0), UpFrom(0)]);
    assert_eq_i!(a.union(&TineTree::from(UpFrom(0))),         [UpFrom(0)]);
    assert_eq_i!(a.union(&TineTree::from(To(0))),             [Full]);
    assert_eq_i!(a.union(&TineTree::from(From(0))),           [From(0)]);
    assert_eq_i!(a.union(&TineTree::from(Full)),              [Full]);
}

#[test]
fn up_from_left() {
    let a: TineTree<i32> = UpFrom(0).into();

    assert_eq_i!(a.union(&TineTree::from(Empty)),             [UpFrom(0)]);
    assert_eq_i!(a.union(&TineTree::from(Point(-3))),         [UpFrom(0), Point(-3)]);
    assert_eq_i!(a.union(&TineTree::from(Open(-3, -1))),      [UpFrom(0), Open(-3, -1)]);
    assert_eq_i!(a.union(&TineTree::from(LeftOpen(-3, -1))),  [UpFrom(0), LeftOpen(-3, -1)]);
    assert_eq_i!(a.union(&TineTree::from(RightOpen(-3, -1))), [UpFrom(0), RightOpen(-3, -1)]);
    assert_eq_i!(a.union(&TineTree::from(Closed(-3, -1))),    [UpFrom(0), Closed(-3, -1)]);
    assert_eq_i!(a.union(&TineTree::from(UpTo(-3))),          [UpFrom(0), UpTo(-3)]);
    assert_eq_i!(a.union(&TineTree::from(UpFrom(-3))),        [UpFrom(-3)]);
    assert_eq_i!(a.union(&TineTree::from(To(-3))),            [UpFrom(0), To(-3)]);
    assert_eq_i!(a.union(&TineTree::from(From(-3))),          [From(-3)]);
    assert_eq_i!(a.union(&TineTree::from(Full)),              [Full]);
}

#[test]
fn up_from_right() {
    let a: TineTree<i32> = UpFrom(0).into();

    assert_eq_i!(a.union(&TineTree::from(Empty)),             [UpFrom(0)]);
    assert_eq_i!(a.union(&TineTree::from(Point(13))),         [UpFrom(0)]);
    assert_eq_i!(a.union(&TineTree::from(Open(10, 13))),      [UpFrom(0)]);
    assert_eq_i!(a.union(&TineTree::from(LeftOpen(10, 13))),  [UpFrom(0)]);
    assert_eq_i!(a.union(&TineTree::from(RightOpen(10, 13))), [UpFrom(0)]);
    assert_eq_i!(a.union(&TineTree::from(Closed(10, 13))),    [UpFrom(0)]);
    assert_eq_i!(a.union(&TineTree::from(UpTo(13))),          [Full]);
    assert_eq_i!(a.union(&TineTree::from(UpFrom(13))),        [UpFrom(0)]);
    assert_eq_i!(a.union(&TineTree::from(To(13))),            [Full]);
    assert_eq_i!(a.union(&TineTree::from(From(13))),          [UpFrom(0)]);
    assert_eq_i!(a.union(&TineTree::from(Full)),              [Full]);
}

#[test]
fn to_center() {
    let a: TineTree<i32> = To(3).into();

    assert_eq_i!(a.union(&TineTree::from(Empty)),             [To(3)]);
    assert_eq_i!(a.union(&TineTree::from(Point(3))),          [To(3)]);
    assert_eq_i!(a.union(&TineTree::from(Open(0, 3))),        [To(3)]);
    assert_eq_i!(a.union(&TineTree::from(LeftOpen(0, 3))),    [To(3)]);
    assert_eq_i!(a.union(&TineTree::from(RightOpen(0, 3))),   [To(3)]);
    assert_eq_i!(a.union(&TineTree::from(Closed(0, 3))),      [To(3)]);
    assert_eq_i!(a.union(&TineTree::from(UpTo(3))),           [To(3)]);
    assert_eq_i!(a.union(&TineTree::from(UpFrom(3))),         [Full]);
    assert_eq_i!(a.union(&TineTree::from(To(3))),             [To(3)]);
    assert_eq_i!(a.union(&TineTree::from(From(3))),           [Full]);
    assert_eq_i!(a.union(&TineTree::from(Full)),              [Full]);
}

#[test]
fn to_left() {
    let a: TineTree<i32> = To(3).into();

    assert_eq_i!(a.union(&TineTree::from(Empty)),             [To(3)]);
    assert_eq_i!(a.union(&TineTree::from(Point(-3))),         [To(3)]);
    assert_eq_i!(a.union(&TineTree::from(Open(-3, -1))),      [To(3)]);
    assert_eq_i!(a.union(&TineTree::from(LeftOpen(-3, -1))),  [To(3)]);
    assert_eq_i!(a.union(&TineTree::from(RightOpen(-3, -1))), [To(3)]);
    assert_eq_i!(a.union(&TineTree::from(Closed(-3, -1))),    [To(3)]);
    assert_eq_i!(a.union(&TineTree::from(UpTo(-3))),          [To(3)]);
    assert_eq_i!(a.union(&TineTree::from(UpFrom(-3))),        [Full]);
    assert_eq_i!(a.union(&TineTree::from(To(-3))),            [To(3)]);
    assert_eq_i!(a.union(&TineTree::from(From(-3))),          [Full]);
    assert_eq_i!(a.union(&TineTree::from(Full)),              [Full]);
}

#[test]
fn to_right() {
    let a: TineTree<i32> = To(3).into();

    assert_eq_i!(a.union(&TineTree::from(Empty)),             [To(3)]);
    assert_eq_i!(a.union(&TineTree::from(Point(13))),         [To(3), Point(13)]);
    assert_eq_i!(a.union(&TineTree::from(Open(10, 13))),      [To(3), Open(10, 13)]);
    assert_eq_i!(a.union(&TineTree::from(LeftOpen(10, 13))),  [To(3), LeftOpen(10, 13)]);
    assert_eq_i!(a.union(&TineTree::from(RightOpen(10, 13))), [To(3), RightOpen(10, 13)]);
    assert_eq_i!(a.union(&TineTree::from(Closed(10, 13))),    [To(3), Closed(10, 13)]);
    assert_eq_i!(a.union(&TineTree::from(UpTo(13))),          [UpTo(13)]);
    assert_eq_i!(a.union(&TineTree::from(UpFrom(13))),        [To(3), UpFrom(13)]);
    assert_eq_i!(a.union(&TineTree::from(To(13))),            [To(13)]);
    assert_eq_i!(a.union(&TineTree::from(From(13))),          [To(3), From(13)]);
    assert_eq_i!(a.union(&TineTree::from(Full)),              [Full]);
}

#[test]
fn from_center() {
    let a: TineTree<i32> = From(0).into();

    assert_eq_i!(a.union(&TineTree::from(Empty)),             [From(0)]);
    assert_eq_i!(a.union(&TineTree::from(Point(0))),          [From(0)]);
    assert_eq_i!(a.union(&TineTree::from(Open(0, 3))),        [From(0)]);
    assert_eq_i!(a.union(&TineTree::from(LeftOpen(0, 3))),    [From(0)]);
    assert_eq_i!(a.union(&TineTree::from(RightOpen(0, 3))),   [From(0)]);

    assert_eq_i!(a.union(&TineTree::from(Closed(0, 3))),      [From(0)]);
    assert_eq_i!(a.union(&TineTree::from(UpTo(0))),           [Full]);
    assert_eq_i!(a.union(&TineTree::from(UpFrom(0))),         [From(0)]);
    assert_eq_i!(a.union(&TineTree::from(To(0))),             [Full]);
    assert_eq_i!(a.union(&TineTree::from(From(0))),           [From(0)]);
    assert_eq_i!(a.union(&TineTree::from(Full)),              [Full]);
}

#[test]
fn from_left() {
    let a: TineTree<i32> = From(0).into();

    assert_eq_i!(a.union(&TineTree::from(Empty)),             [From(0)]);
    assert_eq_i!(a.union(&TineTree::from(Point(-3))),         [From(0), Point(-3)]);
    assert_eq_i!(a.union(&TineTree::from(Open(-3, -1))),      [From(0), Open(-3, -1)]);
    assert_eq_i!(a.union(&TineTree::from(LeftOpen(-3, -1))),  [From(0), LeftOpen(-3, -1)]);
    assert_eq_i!(a.union(&TineTree::from(RightOpen(-3, -1))), [From(0), RightOpen(-3, -1)]);
    assert_eq_i!(a.union(&TineTree::from(Closed(-3, -1))),    [From(0), Closed(-3, -1)]);
    assert_eq_i!(a.union(&TineTree::from(UpTo(-3))),          [From(0), UpTo(-3)]);
    assert_eq_i!(a.union(&TineTree::from(UpFrom(-3))),        [UpFrom(-3)]);
    assert_eq_i!(a.union(&TineTree::from(To(-3))),            [From(0), To(-3)]);
    assert_eq_i!(a.union(&TineTree::from(From(-3))),          [From(-3)]);
    assert_eq_i!(a.union(&TineTree::from(Full)),              [Full]);
}

#[test]
fn from_right() {
    let a: TineTree<i32> = From(0).into();

    assert_eq_i!(a.union(&TineTree::from(Empty)),             [From(0)]);
    assert_eq_i!(a.union(&TineTree::from(Point(13))),         [From(0)]);
    assert_eq_i!(a.union(&TineTree::from(Open(10, 13))),      [From(0)]);
    assert_eq_i!(a.union(&TineTree::from(LeftOpen(10, 13))),  [From(0)]);
    assert_eq_i!(a.union(&TineTree::from(RightOpen(10, 13))), [From(0)]);
    assert_eq_i!(a.union(&TineTree::from(Closed(10, 13))),    [From(0)]);
    assert_eq_i!(a.union(&TineTree::from(UpTo(13))),          [Full]);
    assert_eq_i!(a.union(&TineTree::from(UpFrom(13))),        [From(0)]);
    assert_eq_i!(a.union(&TineTree::from(To(13))),            [Full]);
    assert_eq_i!(a.union(&TineTree::from(From(13))),          [From(0)]);
    assert_eq_i!(a.union(&TineTree::from(Full)),              [Full]);
}

#[test]
fn full() {
    let a: TineTree<i32> = Full.into();

    assert_eq_i!(a.union(&TineTree::from(Empty)),             [Full]);
    assert_eq_i!(a.union(&TineTree::from(Point(0))),          [Full]);
    assert_eq_i!(a.union(&TineTree::from(Open(0, 3))),        [Full]);
    assert_eq_i!(a.union(&TineTree::from(LeftOpen(0, 3))),    [Full]);
    assert_eq_i!(a.union(&TineTree::from(RightOpen(0, 3))),   [Full]);
    assert_eq_i!(a.union(&TineTree::from(Closed(0, 3))),      [Full]);
    assert_eq_i!(a.union(&TineTree::from(UpTo(0))),           [Full]);
    assert_eq_i!(a.union(&TineTree::from(UpFrom(0))),         [Full]);
    assert_eq_i!(a.union(&TineTree::from(To(0))),             [Full]);
    assert_eq_i!(a.union(&TineTree::from(From(0))),           [Full]);
    assert_eq_i!(a.union(&TineTree::from(Full)),              [Full]);
}
