// The MIT License (MIT)
// 
// Copyright (c) 2017 Skylor R. Schermer
// 
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// 
// The above copyright notice and this permission notice shall be included in 
// all copies or substantial portions of the Software.
// 
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//
////////////////////////////////////////////////////////////////////////////////
//!
//! Provides a managed collection of intervals.
//!
////////////////////////////////////////////////////////////////////////////////

// Local imports.
use bound::Bound;
use interval::{Interval, IntervalBounds};

// Standard imports.
use std::collections::{btree_set, BTreeSet};
use std::cmp::Ordering;
use std::fmt;


////////////////////////////////////////////////////////////////////////////////
// Selection
////////////////////////////////////////////////////////////////////////////////
/// A possibly non-contiguous selection of intervals.
#[derive(Debug, PartialEq, Clone)]
pub struct Selection<T> where T: IntervalBounds {
	disjunction_map: DisjunctionMap<T>,
}

impl<T> Selection<T> where T: IntervalBounds {
	/// Returns an empty Selection.
	pub fn empty() -> Self {
		Selection {
			disjunction_map: DisjunctionMap::new(),
		}
	}

	/// Returns a Selection over the given intervals.
	pub fn new<I>(intervals: I) -> Self
		where I: IntoIterator<Item=Interval<T>>
	{
		Selection {
			disjunction_map: DisjunctionMap::from_intervals(intervals),
		}
	}

	/// Inserts all of the points in the given interval into the Selection.
	pub fn union_insert(&mut self, interval: Interval<T>) {
		self.disjunction_map.union_insert(interval);
	}
}

////////////////////////////////////////////////////////////////////////////////
// Tine
////////////////////////////////////////////////////////////////////////////////
/// A portion of an interval.
///
/// Tines are used to implement ordering over the interval bounds in such a way
/// that the `BTreeSet` will always be able to split at the appropriate place
/// for a given bound type.
#[derive(Debug, PartialEq, Clone)]
struct Tine<T> {
	pub lb: bool,
	pub ub: bool,
	pub incl: bool,
	pub point: Option<T>,
}

impl<T> Tine<T> where T: IntervalBounds {
	pub fn is_point(&self) -> bool {
		!self.lb && !self.ub && self.incl && self.point.is_some()
	}

	pub fn from_degenerate_interval(interval: Interval<T>) -> Self {
		debug_assert!(!interval.is_empty());
		debug_assert!(interval.is_degenerate());
		Tine {
			lb: false,
			ub: false,
			incl: true,
			point: interval.infimum(),
		}
	}

	pub fn from_nondegenerate_interval(interval: Interval<T>) -> (Self, Self) {
		debug_assert!(!interval.is_empty());
		debug_assert!(!interval.is_degenerate());
		let left = Tine {
			lb: true,
			ub: false,
			incl: !interval.is_left_open(),
			point: interval.infimum(),
		};
		let right = Tine {
			lb: false,
			ub: true,
			incl: !interval.is_right_open(),
			point: interval.supremum(),
		};
		println!("\nGenerating tines {:?} {}, {}", interval, &left, &right); //TODO: Remove.
		(left, right)
	}

	pub fn merge(self, other: Self) -> Option<Self> {
		debug_assert_eq!(self.point, other.point);
		debug_assert!(self.point.is_some()
			|| other.point.is_some() 
			|| (self.point.is_none() 
				&& other.point.is_none()
				&& self.lb == other.lb 
				&& self.ub == other.ub));

		let merged = Tine {
			lb: self.lb || other.lb,
			ub: self.ub || other.ub,
			incl: self.incl || other.incl,
			point: self.point,
		};

		if merged.lb && merged.ub && merged.incl {
			// New tine is merging 2 or more intervals.
			None
		} else {
			Some(merged)
		}
	}

	pub fn make_lower_bound(lb: Self) -> Option<Bound<T>> {
		debug_assert!(lb.lb);
		if lb.point.is_none() {
			return None;
		}
		match lb.incl {
			true => Some(Bound::Include(lb.point.expect("bounded tine"))),
			false => Some(Bound::Exclude(lb.point.expect("bounded tine"))),
		}
	}

	pub fn make_upper_bound(ub: Self) -> Option<Bound<T>> {
		debug_assert!(ub.ub);
		if ub.point.is_none() {
			return None;
		}
		match ub.incl {
			true => Some(Bound::Include(ub.point.expect("bounded tine"))),
			false => Some(Bound::Exclude(ub.point.expect("bounded tine"))),
		}
	}
}

// Tine ordering is total.
impl<T> Eq for Tine<T> where T: IntervalBounds {}


impl<T> PartialOrd for Tine<T> where T: IntervalBounds {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl<T> Ord for Tine<T> where T: IntervalBounds {
	fn cmp(&self, other: &Self) -> Ordering {
		match (&self.point, &other.point) {
			(&Some(ref a), &Some(ref b))
				=> a.cmp(&b),

			(&None, &Some(_))
				=> if self.lb {Ordering::Less} else {Ordering::Greater},

			(&Some(_), &None)
				=> if other.lb {Ordering::Greater} else {Ordering::Less},

			_
				=> match (self.lb, self.ub, other.lb, other.ub) {
					(true, false, true, false) |
					(false, true, false, true) => Ordering::Equal,
					(true, false, false, true) => Ordering::Less,
					(false, true, true, false) => Ordering::Greater,
					_						   => panic!("invalid Tine"),
				}
		}
	}
}


impl<T> fmt::Display for Tine<T> where T: fmt::Debug {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}-{:?}", 
			match (self.lb, self.incl, self.ub) {
				(true, false, false) => "(",
				(true, true, false) => "[",
				(true, true, true) => "!!",
				(true, false, true) => ")(",
				(false, false, true) => ")",
				(false, true, true) => "]",
				(false, true, false) => "|",
				(false, false, false) => "??",
			},
			self.point)
	}
}


////////////////////////////////////////////////////////////////////////////////
// DisjunctionMap
////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, PartialEq, Clone)]
struct DisjunctionMap<T> where T: IntervalBounds {
	tine_map: BTreeSet<Tine<T>>,
}

impl<T> DisjunctionMap<T> where T: IntervalBounds {
	pub fn new() -> Self {
		DisjunctionMap {
			tine_map: BTreeSet::new(),
		}
	}

	pub fn from_intervals<I>(intervals: I) -> Self
		where I: IntoIterator<Item=Interval<T>>
	{
		let mut dm = DisjunctionMap::new();
		for interval in intervals.into_iter() {
			dm.union_insert(interval);
		}
		dm
	}

	pub fn union_insert(&mut self, interval: Interval<T>) {
		if interval.is_empty() {
			return
		}

		if interval.is_degenerate() {
			self.union_pt(Tine::from_degenerate_interval(interval));
		} else {
			let (lb, ub) = Tine::from_nondegenerate_interval(interval);
			self.union_normal(lb, ub);
		}
	}

	fn union_pt(&mut self, pt: Tine<T>) {
		if self.tine_map.is_empty() {
			self.tine_map.insert(pt);
			return
		}

		let mut r_map = self.tine_map.split_off(&pt);
		let before = self.tine_map.iter().next_back().cloned();
		let after = r_map.iter().next().cloned();

		match (before, after) {
			(_,			  Some(ref a)) if a.point == pt.point
				=> if let Some(merged) = pt.merge(a.clone()) {
					r_map.insert(merged);
				} else {
					r_map.remove(&a);
				},
			
			(Some(ref b), Some(ref a)) if b.ub && a.lb
				=> {r_map.insert(pt);},

			(Some(_),	  Some(_))
				=> (), // Point is subsumed; do nothing.

			(Some(ref b), None)
				=> {r_map.insert(pt); debug_assert!(!b.lb);}

			(None,		  Some(a))
				=> {r_map.insert(pt); debug_assert!(!a.ub);}

			_	=> (), // Nothing to do.
		};

		self.tine_map.extend(r_map);
	}


	fn union_normal(&mut self, lb: Tine<T>, ub: Tine<T>) {
		// Should have proper bounds set...
		debug_assert!(lb.lb);
		debug_assert!(ub.ub);

		if self.tine_map.is_empty() 
			|| (lb.point.is_none() && ub.point.is_none()) 
		{
			self.tine_map = BTreeSet::new();
			self.tine_map.insert(lb);
			self.tine_map.insert(ub);
			return
		}

		let mut r_map = self.tine_map.split_off(&lb);
		let before = self.tine_map.iter().next_back().cloned();

		let lb = if let Some(a) = r_map.take(&lb) {
			lb.merge(a)
		} else {
			Some(lb)
		};

		let mut r_map = r_map.split_off(&ub);

		let ub = if let Some(a) = r_map.take(&ub) {
			ub.merge(a)
		} else {
			Some(ub)
		};
		let after = r_map.iter().next().cloned();

		println!("Before...");//TODO: Remove.
		for tine in self.tine_map.iter() { //TODO: Remove.
    		println!("\t{}", tine); //TODO: Remove.
    	} //TODO: Remove.

    	println!("After...");//TODO: Remove.
		for tine in r_map.iter() { //TODO: Remove.
    		println!("\t{}", tine); //TODO: Remove.
    	} //TODO: Remove.

		match (lb, ub) {
			(None, None) => {
				println!("Double annhilation");
				debug_assert!(before.expect("upper bound annhilation").lb);
				debug_assert!(after.expect("lower bound annhilation").ub);
			},
			(Some(lb), None) => {
				println!("Inserting lb\t{}", lb); //TODO: Remove.
				r_map.insert(lb);
				debug_assert!(after.expect("lower bound annhilation").ub);	
			},

			(None, Some(ub)) => {
				println!("Inserting ub\t{}", ub); //TODO: Remove.
				r_map.insert(ub);
				debug_assert!(before.expect("upper bound annhilation").lb);
			},

			(Some(lb), Some(ub)) => {
				println!("No annhilation");
				if before.is_none() 
					|| before.as_ref().map(|b| b.ub) == Some(true)
					|| (before.map(|b| b.lb) == Some(true) && lb.ub)

				{
					println!("Inserting lb/ub\t{}", lb); //TODO: Remove.
					r_map.insert(lb);
				}
				if after.is_none() 
					|| after.as_ref().map(|a| a.lb) == Some(true) 
					|| (after.map(|a| a.ub) == Some(true)  && ub.lb)
				{
					println!("Inserting lb/ub\t{}", ub); //TODO: Remove.
					r_map.insert(ub);
				}

			}
		}
		println!("Done."); //TODO: Remove.

		self.tine_map.extend(r_map);
		for tine in self.tine_map.iter() { //TODO: Remove.
    		println!("\t{}", tine); //TODO: Remove.
    	} //TODO: Remove.
	}
}

impl<T> IntoIterator for DisjunctionMap<T> where T: IntervalBounds {
	type Item = Interval<T>;
    type IntoIter = SelectionIter<T>;
    fn into_iter(self) -> Self::IntoIter {
    	SelectionIter {
    		tine_iter: self.tine_map.into_iter(),
    		saved: None,
    	}
    }
}

////////////////////////////////////////////////////////////////////////////////
// SelectionIter
////////////////////////////////////////////////////////////////////////////////
/// An iterator over the intervals in 
pub struct SelectionIter<T> where T: IntervalBounds {
	tine_iter: btree_set::IntoIter<Tine<T>>,
	saved: Option<Tine<T>>,
}

impl<T> Iterator for SelectionIter<T> where T: IntervalBounds {
	type Item = Interval<T>;

	fn next(&mut self) -> Option<Self::Item> {
		if let Some(saved) = self.saved.take() {
			debug_assert!(saved.lb);
			let second = self.tine_iter.next().expect("");
			debug_assert!(second.ub);
			if second.lb {self.saved = Some(second.clone());}

			return Some(Interval::new(
				Tine::make_lower_bound(saved),
				Tine::make_upper_bound(second)));

		}
		
		if let Some(first) = self.tine_iter.next() {
			debug_assert!(!first.ub);

			if first.is_point() {
				return Some(Interval::point(first.point.expect("point tine")));
			}
			let second = self.tine_iter.next().expect("upper bound");
			if second.lb {self.saved = Some(second.clone());}

			Some(Interval::new(
				Tine::make_lower_bound(first),
				Tine::make_upper_bound(second)))
		} else {
			None
		}
	}
}


#[cfg(test)]
mod tests {
	use ::interval::Interval;
	use super::*;


	#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
	struct Opaque(i32);

	#[test]
	fn disjunction_map_insert_disjoint() {
		let mut dm: DisjunctionMap<Opaque> = DisjunctionMap::new();

		let a = Interval::open(Opaque(0), Opaque(15));
		let b = Interval::open(Opaque(20), Opaque(25));
		let c = Interval::open(Opaque(30), Opaque(35));
		dm.union_insert(b.clone());
		dm.union_insert(a.clone());
		dm.union_insert(c.clone());

		let dm_res: Vec<Interval<Opaque>> = dm.into_iter().collect();
		assert_eq!(dm_res, vec![a, b, c]);
	}

	#[test]
	fn disjunction_map_insert_overlap() {
		let mut dm: DisjunctionMap<Opaque> = DisjunctionMap::new();

		let a = Interval::open(Opaque(0), Opaque(15));
		let b = Interval::open(Opaque(20), Opaque(25));
		let c = Interval::open(Opaque(14), Opaque(26));
		dm.union_insert(b.clone());
		dm.union_insert(a.clone());
		dm.union_insert(c.clone());

		let dm_res: Vec<Interval<Opaque>> = dm.into_iter().collect();
		assert_eq!(dm_res, vec![Interval::open(Opaque(0), Opaque(26))]);
	}


	#[test]
	fn disjunction_map_insert_disjoint_close() {
		let mut dm: DisjunctionMap<Opaque> = DisjunctionMap::new();

		let a = Interval::open(Opaque(0), Opaque(15));
		let b = Interval::open(Opaque(15), Opaque(25));
		let c = Interval::open(Opaque(25), Opaque(30));
		dm.union_insert(c.clone());
		dm.union_insert(a.clone());
		dm.union_insert(b.clone());

		let dm_res: Vec<Interval<Opaque>> = dm.into_iter().collect();
		assert_eq!(dm_res, vec![a, b, c]);
	}

	#[test]
	fn disjunction_map_insert_overlap_close() {
		let mut dm: DisjunctionMap<Opaque> = DisjunctionMap::new();

		let a = Interval::open(Opaque(0), Opaque(15));
		let b = Interval::closed(Opaque(15), Opaque(25));
		let c = Interval::open(Opaque(25), Opaque(30));
		dm.union_insert(c.clone());
		dm.union_insert(a.clone());
		dm.union_insert(b.clone());

		let dm_res: Vec<Interval<Opaque>> = dm.into_iter().collect();
		assert_eq!(dm_res, vec![Interval::open(Opaque(0), Opaque(30))]);
	}

	#[test]
	fn disjunction_map_insert_disjoint_point() {
		let mut dm: DisjunctionMap<Opaque> = DisjunctionMap::new();

		let a = Interval::open(Opaque(0), Opaque(15));
		let b = Interval::open(Opaque(15), Opaque(25));
		let c = Interval::point(Opaque(15));
		dm.union_insert(c.clone());
		dm.union_insert(a.clone());
		dm.union_insert(b.clone());

		let dm_res: Vec<Interval<Opaque>> = dm.into_iter().collect();
		assert_eq!(dm_res, vec![Interval::open(Opaque(0), Opaque(25))]);
	}

	#[test]
	fn disjunction_map_insert_overlap_exact() {
		let mut dm: DisjunctionMap<Opaque> = DisjunctionMap::new();

		let a = Interval::open(Opaque(0), Opaque(15));
		let b = Interval::open(Opaque(15), Opaque(25));
		let c = Interval::open(Opaque(0), Opaque(25));
		dm.union_insert(a.clone());
		dm.union_insert(b.clone());
		dm.union_insert(c.clone());

		let dm_res: Vec<Interval<Opaque>> = dm.into_iter().collect();
		assert_eq!(dm_res, vec![Interval::open(Opaque(0), Opaque(25))]);
	}

	#[test]
	fn disjunction_map_insert_overlap_widen() {
		let mut dm: DisjunctionMap<Opaque> = DisjunctionMap::new();

		let a = Interval::open(Opaque(0), Opaque(15));
		let b = Interval::open(Opaque(15), Opaque(25));
		let c = Interval::closed(Opaque(0), Opaque(25));
		dm.union_insert(a.clone());
		dm.union_insert(b.clone());
		dm.union_insert(c.clone());

		let dm_res: Vec<Interval<Opaque>> = dm.into_iter().collect();
		assert_eq!(dm_res, vec![Interval::closed(Opaque(0), Opaque(25))]);
	}

	#[test]
	fn disjunction_map_insert_overlap_point() {
		let mut dm: DisjunctionMap<Opaque> = DisjunctionMap::new();

		let a = Interval::open(Opaque(0), Opaque(30));
		let b = Interval::point(Opaque(15));
		dm.union_insert(a.clone());
		dm.union_insert(b.clone());

		let dm_res: Vec<Interval<Opaque>> = dm.into_iter().collect();
		assert_eq!(dm_res, vec![Interval::open(Opaque(0), Opaque(30))]);
	}
	#[test]
	fn disjunction_map_insert_overlap_unbounded() {
		let mut dm: DisjunctionMap<Opaque> = DisjunctionMap::new();

		let a = Interval::from(Opaque(10));
		let b = Interval::open(Opaque(15), Opaque(30));
		dm.union_insert(a.clone());
		dm.union_insert(b.clone());

		let dm_res: Vec<Interval<Opaque>> = dm.into_iter().collect();
		assert_eq!(dm_res, vec![Interval::from(Opaque(10))]);
	}
}