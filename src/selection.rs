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
use interval::{Interval, IntervalNormalize};

// Standard imports.
use std::collections::{btree_set, BTreeSet};
use std::cmp::Ordering;

// Local enum shortcuts.
use Bound::*;
use self::Tine::*;

////////////////////////////////////////////////////////////////////////////////
// Selection
////////////////////////////////////////////////////////////////////////////////
/// A possibly non-contiguous selection of intervals.
#[derive(Debug, PartialEq, Clone)]
pub struct Selection<T> 
	where
		T: PartialOrd + Ord + Clone, 
		Interval<T>: IntervalNormalize 
{
	disjunction_map: DisjunctionMap<T>,
}

impl<T> Selection<T>
	where
		T: PartialOrd + Ord + Clone, 
		Interval<T>: IntervalNormalize 
{
	pub fn new(intervals: Vec<Interval<T>>) -> Self {
		Selection {
			disjunction_map: DisjunctionMap::from_intervals(intervals),
		}
	}
}

////////////////////////////////////////////////////////////////////////////////
// Tine
////////////////////////////////////////////////////////////////////////////////
/// A portion of an interval.
#[derive(Debug, PartialEq, Clone)]
enum Tine<T> where T: PartialOrd + Ord + Clone {
	/// A lower bound.
	Lo(Option<Bound<T>>),
	/// A point interval.
	Pt(Interval<T>),
	/// An upper bound.
	Hi(Option<Bound<T>>),
}

impl<T> Tine<T> where T: PartialOrd + Ord + Clone {

	pub fn is_lo(&self) -> bool {
		match self {
			&Lo(_) => true,
			_	   => false,
		}
	}

	pub fn is_hi(&self) -> bool {
		match self {
			&Hi(_) => true,
			_	   => false,
		}
	}

	pub fn val(&self) -> Option<T> {
		match *self {
			Pt(ref i)		=> i.infimum(),
			Lo(Some(ref l)) => Some(l.as_ref().clone()),
			Hi(Some(ref r)) => Some(r.as_ref().clone()),
			_				=> None,
		}
	}

	pub fn close(self) -> Self {
		match self {
			Lo(Some(Exclude(l))) => Lo(Some(Include(l))),
			Hi(Some(Exclude(r))) => Hi(Some(Include(r))),
			_					 => self,
		}

	}
	
	pub fn cmp_tine_types(&self, other: &Self) -> Ordering {
		match (self, other) {
			(&Lo(_), &Pt(_)) => Ordering::Less,
			(&Lo(_), &Hi(_)) => Ordering::Less,
			(&Pt(_), &Lo(_)) => Ordering::Greater,
			(&Pt(_), &Hi(_)) => Ordering::Less,
			(&Hi(_), &Lo(_)) => Ordering::Greater,
			(&Hi(_), &Pt(_)) => Ordering::Greater,
			_				 => Ordering::Equal,
		}
	}
	
	pub fn cmp_bound_types(&self, other: &Self) -> Ordering {
		match (self, other) {
			(&Lo(Some(ref a)), &Lo(Some(ref b))) => match (a, b) {
				(&Exclude(_), &Include(_)) => Ordering::Greater,
				(&Include(_), &Exclude(_)) => Ordering::Less,
				_						   => Ordering::Equal,
			},
			(&Hi(Some(ref a)), &Hi(Some(ref b))) => match (a, b) {
				(&Exclude(_), &Include(_)) => Ordering::Less,
				(&Include(_), &Exclude(_)) => Ordering::Greater,
				_						   => Ordering::Equal,
			},
			_									 => Ordering::Equal,
		}
	}
}


impl<T> Eq for Tine<T> where T: PartialOrd + Ord + Clone {}


// Tine comparison:
// First check the points.
// if any are null
//     compare Lo < Pt < Hi
// else
//     compare values
//     then compare Hi < Pt < Lo
//     then if both are Lo
//         compare Include < Exclude
//     else if both are Hi
//         compare Exclude < Include

impl<T> PartialOrd for Tine<T> where T: PartialOrd + Ord + Clone {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl<T> Ord for Tine<T> where T: PartialOrd + Ord + Clone {
	fn cmp(&self, other: &Self) -> Ordering {
		match (self.val(), other.val()) {
			(Some(ref a), Some(ref b)) => {
				let res = a.cmp(b);
				let res = match res {
					Ordering::Equal => self.cmp_tine_types(other).reverse(),
					_				=> return res,
				};
				match res {
					Ordering::Equal => self.cmp_bound_types(other),
					_				=> res,
				}
			},
			_						   => self.cmp_tine_types(other),
		}
	}
}



////////////////////////////////////////////////////////////////////////////////
// DisjunctionMap
////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, PartialEq, Clone)]
struct DisjunctionMap<T> where T: PartialOrd + Ord + Clone {
	tine_map: BTreeSet<Tine<T>>,
}

impl<T> DisjunctionMap<T> where T: PartialOrd + Ord + Clone {
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

	#[allow(dead_code)]
	pub fn union_insert(&mut self, interval: Interval<T>) {
		if interval.is_degenerate() {
			self.union_pt(Pt(interval));
		} else {
			self.union_normal(
				Lo(interval.lower_bound()),
				Hi(interval.upper_bound()),
			);
		}
	}

	fn union_pt(&mut self, pt: Tine<T>) {
		let mut r_map = self.tine_map.split_off(&pt);
		
		let before = self.tine_map.iter().next_back().cloned();
		let after = r_map.iter().next().cloned();
		
		match (&before.as_ref().map(Tine::is_lo), 
			   &after.as_ref().map(Tine::is_hi))
		{
			(&Some(true), &Some(true)) => {
				let before = before.unwrap();
				let after = after.unwrap();
				
				if before.val() == pt.val() {
					self.tine_map.remove(&before);
					r_map.insert(before.close());
				} else if after.val() == pt.val() {
					r_map.remove(&after);
					r_map.insert(after.close());
				};
				// Else do nothing: point is subsumed.
			},
			_						   => {r_map.insert(pt);},
		}
		
		self.tine_map.extend(r_map);
	}


	fn union_normal(&mut self, lb: Tine<T>, ub: Tine<T>) {
		let mut r_map = self.tine_map.split_off(&lb);
		r_map = r_map.split_off(&ub);
		
		let before = self.tine_map.iter().next_back().cloned();
		let after = r_map.iter().next().cloned();
		
		match (&before.as_ref().map(Tine::is_lo), 
			   &after.as_ref().map(Tine::is_hi))
		{
			(&Some(true), &Some(true)) => (), // Do nothing: interval subsumed.
			(_,			  &Some(true)) => {r_map.insert(lb);},
			(&Some(true), _)		   => {r_map.insert(ub);},
			_						   => {r_map.insert(lb); r_map.insert(ub);},
		}
		
		self.tine_map.extend(r_map);
	}
}

impl<T> IntoIterator for DisjunctionMap<T> where T: PartialOrd + Ord + Clone {
	type Item = Interval<T>;
    type IntoIter = DisjunctionMapIter<T>;
    fn into_iter(self) -> Self::IntoIter {
    	DisjunctionMapIter {
    		tine_iter: self.tine_map.into_iter(),
    	}
    }
}

////////////////////////////////////////////////////////////////////////////////
// DisjunctionMapIter
////////////////////////////////////////////////////////////////////////////////
pub struct DisjunctionMapIter<T> where T: PartialOrd + Ord + Clone {
	tine_iter: btree_set::IntoIter<Tine<T>>,
}

impl<T> Iterator for DisjunctionMapIter<T> where T: PartialOrd + Ord + Clone {
	type Item = Interval<T>;
	fn next(&mut self) -> Option<Self::Item> {
		match self.tine_iter.next() {
			Some(Pt(int)) => Some(int),
			Some(Lo(l))	  => if let Some(Hi(r)) = self.tine_iter.next() {
				Some(Interval::new(l, r))
			} else {
				panic!("disjunction map in invalid order")
			},
			None		  => None,
			_	  => panic!("disjunction map in invalid order"),
		}
	}
}


#[cfg(test)]
mod tests {
	use ::interval::Interval;
	use super::*;

	#[test]
	fn disjunction_map_insert_1() {
		let mut dm: DisjunctionMap<u32> = DisjunctionMap::new();

		let a = Interval::closed(0, 15);
		let b = Interval::closed(20, 25);
		dm.union_insert(b.clone());
		dm.union_insert(a.clone());

		let dm_res: Vec<Interval<u32>> = dm.into_iter().collect();
		assert_eq!(dm_res, vec![a, b])
	}

	#[test]
	fn disjunction_map_insert_2() {
		let mut dm: DisjunctionMap<u32> = DisjunctionMap::new();

		let a = Interval::closed(0, 15);
		let b = Interval::closed(20, 25);
		let c = Interval::closed(14, 26);
		dm.union_insert(b.clone());
		dm.union_insert(a.clone());
		dm.union_insert(c.clone());

		let dm_res: Vec<Interval<u32>> = dm.into_iter().collect();
		assert_eq!(dm_res, vec![Interval::closed(0, 26)])
	}
}