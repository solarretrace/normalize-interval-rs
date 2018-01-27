

use ::{
	Selection,
	Interval,
};

use std::i32;

#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
struct Opaque(i32);






#[test] 
fn selection_from_intervals() {
	let s = Selection::from_intervals(vec![
		Interval::up_to(15),
		Interval::open(38, 45),
		Interval::left_open(20, 25),
		Interval::point(30),
		Interval::open(35, 40),
		Interval::point(40),
		Interval::point(45),
		Interval::point(45),
		Interval::from(50),
	]);

	assert_eq!(s.iter().collect::<Vec<_>>(), vec![
		Interval::up_to(15),
		Interval::left_open(20, 25),
		Interval::point(30),
		Interval::left_open(35, 45),
		Interval::from(50),
	]);
}

#[test] 
fn selection_intersect() {

}

#[test] 
fn selection_union() {

}

#[test] 
fn selection_complement() {

}

#[test] 
fn selection_minus() {

}

#[test] 
fn selection_closure() {

}



#[test] 
fn selection_opaque_from_intervals() {
	let s = Selection::from_intervals(vec![
		Interval::up_to(Opaque(15)),
		Interval::open(Opaque(38), Opaque(45)),
		Interval::left_open(Opaque(20), Opaque(25)),
		Interval::point(Opaque(30)),
		Interval::open(Opaque(35), Opaque(40)),
		Interval::point(Opaque(40)),
		Interval::point(Opaque(45)),
		Interval::point(Opaque(45)),
		Interval::from(Opaque(50)),
	]);

	assert_eq!(s.iter().collect::<Vec<_>>(), vec![
		Interval::closed(Opaque(i32::MIN), Opaque(15)),
		Interval::closed(Opaque(21), Opaque(25)),
		Interval::point(Opaque(30)),
		Interval::closed(Opaque(36), Opaque(45)),
		Interval::closed(Opaque(51), Opaque(i32::MAX)),
	]);

}

#[test] 
fn selection_opaque_intersect() {

}

#[test] 
fn selection_opaque_union() {

}

#[test] 
fn selection_opaque_complement() {

}

#[test] 
fn selection_opaque_minus() {

}

#[test] 
fn selection_opaque_closure() {

}


