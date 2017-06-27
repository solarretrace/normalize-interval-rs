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
use interval::{
	Interval,
	Normalize,
	LeftIterable,
	RightIterable,
};

// Standard imports.
use std::collections::btree_set;
use std::collections::BTreeSet;
use std::cmp::Ordering;
use std::fmt;


////////////////////////////////////////////////////////////////////////////////
// Selection
////////////////////////////////////////////////////////////////////////////////
/// A possibly non-contiguous selection of intervals.
#[derive(Debug, PartialEq, Clone)]
pub struct Selection<T> where T: PartialOrd + Ord + Clone {
	/// Internal map of ordered interval endpoints.
	disjunction_map: DisjunctionMap<T>,
}

impl<T> Selection<T> where T: PartialOrd + Ord + Clone {
	/// Returns an empty Selection.
	pub fn empty() -> Self {
		Selection {
			disjunction_map: DisjunctionMap::new(),
		}
	}

	/// Returns a Selection over the given intervals.
	pub fn from_intervals<I>(intervals: I) -> Self
		where I: IntoIterator<Item=Interval<T>>
	{
		Selection {
			disjunction_map: DisjunctionMap::from_intervals(intervals),
		}
	}

	/// Inserts all of the points in the given interval into the Selection.
	pub fn union(&mut self, interval: Interval<T>) {
		self.disjunction_map.union_insert(interval);
	}
}


impl<T> IntoIterator for Selection<T> where T: PartialOrd + Ord + Clone {
	type Item = Interval<T>;
    type IntoIter = SelectionIter<T>;
    fn into_iter(self) -> Self::IntoIter {
    	SelectionIter {
    		tine_iter: self.disjunction_map.tine_map.into_iter(),
    		saved: None,
    	}
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
///
/// Tines representing point intervals are neither upper bounds nor lower
/// bounds.
#[derive(Debug, PartialEq, Clone)]
struct Tine<T> {
	/// The point of the `Tine`.
	pub point: Option<T>,
	/// Whether the `Tine` represents the upper bound of an interval.
	pub lb: bool,
	/// Whether the `Tine` represents the lower bound of an interval.
	pub ub: bool,
	/// Whether the `Tine` point is included in the interval.
	pub incl: bool,
}


impl<T> Tine<T> where T: PartialOrd + Ord + Clone {
	/// Returns whether the `Tine` represents a point interval.
	pub fn is_point(&self) -> bool {
		!self.lb && !self.ub && self.incl && self.point.is_some()
	}

	/// Returns the set of `Tine`s representing the given interval.
	pub fn from_interval(interval: Interval<T>) -> OptionSplit<Self> {
		if interval.is_empty() {
			OptionSplit::None
		} else if interval.is_degenerate() {
			OptionSplit::One(Self::from_degenerate_interval(interval))
		} else {
			let (lb, ub) = Self::from_nondegenerate_interval(interval);
			OptionSplit::Two(lb, ub)
		}
	}

	/// Returns the `Tine` representing a point interval.
	fn from_degenerate_interval(mut interval: Interval<T>) -> Self {
		interval.normalize();
		debug_assert!(!interval.is_empty());
		debug_assert!(interval.is_degenerate());
		Tine {
			point: interval.infimum(),
			lb: false,
			ub: false,
			incl: true,
		}
	}

	/// Returns the `Tine` representing a non-empty, non-point interval.
	fn from_nondegenerate_interval(mut interval: Interval<T>)
		-> (Self, Self)
	{
		interval.normalize();
		debug_assert!(!interval.is_empty());
		debug_assert!(!interval.is_degenerate());
		let left = Tine {
			point: interval.infimum(),
			lb: true,
			ub: false,
			incl: !interval.is_left_open(),
		};
		let right = Tine {
			point: interval.supremum(),
			lb: false,
			ub: true,
			incl: !interval.is_right_open(),
		};
		(left, right)
	}

	/// Combines two `Tine`s by 'or'ing their flags.
	pub fn merge(self, other: Self) -> Option<Self> {
		debug_assert!(self.point == other.point);
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

	/// Converts a `Tine` into a lower `Bound`.
	pub fn into_bound_lower(lb: Self) -> Option<Bound<T>> {
		debug_assert!(lb.lb);
		if lb.point.is_none() {
			return None;
		}
		match lb.incl {
			true => Some(Bound::Include(lb.point.expect("bounded tine"))),
			false => Some(Bound::Exclude(lb.point.expect("bounded tine"))),
		}
	}

	/// Converts a `Tine` into an upper `Bound`.
	pub fn into_bound_upper(ub: Self) -> Option<Bound<T>> {
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


impl<T> Eq for Tine<T> where T: PartialOrd + Ord + Clone {}

impl<T> PartialOrd for Tine<T> where T: PartialOrd + Ord + Clone {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

// Tine ordering is total.
impl<T> Ord for Tine<T> where T: PartialOrd + Ord + Clone {
	fn cmp(&self, other: &Self) -> Ordering {
		match (&self.point, &other.point) {
			// Tine points will compare directly.
			(&Some(ref a), &Some(ref b))
				=> a.cmp(&b),

			// Unbounded tines compare to bounded depending on the direction.
			(&None, &Some(_))
				=> if self.lb {Ordering::Less} else {Ordering::Greater},

			(&Some(_), &None)
				=> if other.lb {Ordering::Greater} else {Ordering::Less},

			// Unbounded tines compare to unbounded depending on the direction.
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
				(true, false, false)  => "(",  // Lower bound exclusive.
				(true, true, false)	  => "[",  // Lower bound inclusive.
				(true, true, true)	  => "[]", // Overlapping inclusive bounds.
				(true, false, true)	  => ")(", // Overlapping exclusive bounds.
				(false, false, true)  => ")",  // Upper bound exclusive.
				(false, true, true)	  => "]",  // Upper bound inclusive.
				(false, true, false)  => "|",  // Point bound.
				(false, false, false) => unreachable!(), // Invalid tine.
			},
			self.point)
	}
}


////////////////////////////////////////////////////////////////////////////////
// Unification traits
////////////////////////////////////////////////////////////////////////////////
/// Provides a method for getting the next lower point from the given point.
pub trait NextLower: Sized {
	/// Returns the next point less than the given point.
	fn next_lower(&self) -> Option<Self>;
}

/// Provides a method for getting the next upper point from the given point.
pub trait NextUpper: Sized {
	/// Returns the next point greater than the given point.
	fn next_upper(&self) -> Option<Self>;
}

// Default implementation to be overridden by normalizeable types.
impl<T> NextLower for T {
	default fn next_lower(&self) -> Option<Self> {None}
}

// Default implementation to be overridden by normalizeable types.
impl<T> NextUpper for T {
	default fn next_upper(&self) -> Option<Self> {None}
}

// Override of default for RightIterable types.
impl<T> NextLower for T where T: RightIterable {
	 fn next_lower(&self) -> Option<Self> {
	 	self.pred()
	 }
}

// Override of default for LeftIterable types.
impl<T> NextUpper for T where T: LeftIterable {
	 fn next_upper(&self) -> Option<Self> {
	 	self.succ()
	 }
}



////////////////////////////////////////////////////////////////////////////////
// OptionSplit
////////////////////////////////////////////////////////////////////////////////
/// A type which may contain zero, one, or two of a value.
#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
pub enum OptionSplit<T> {
	/// No value present.
	None,
	/// One value present.
	One(T),
	/// Two values present.
	Two(T, T),
}


////////////////////////////////////////////////////////////////////////////////
// DisjunctionMap
////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, PartialEq, Clone)]
struct DisjunctionMap<T> where T: PartialOrd + Ord + Clone {
	// TODO: Consider using specialized balanced tree.
	/// Tine tree.
	tine_map: BTreeSet<Tine<T>>,
}

impl<T> DisjunctionMap<T>
	where T: PartialOrd + Ord + Clone + NextLower + NextUpper
{
	/// Constructs an empty `DisjunctionMap`.
	pub fn new() -> Self {
		DisjunctionMap {
			tine_map: BTreeSet::new(),
		}
	}

	/// Constructs a `DisjunctionMap` containing the given `Intervals` unioned
	/// together.
	pub fn from_intervals<I>(intervals: I) -> Self
		where I: IntoIterator<Item=Interval<T>>
	{
		let mut dm = DisjunctionMap::new();
		for interval in intervals.into_iter() {
			dm.union_insert(interval);
		}
		dm
	}

	/// Inserts the given `Interval` by unioning with the current contents.
	pub fn union_insert(&mut self, interval: Interval<T>) {
		if interval.is_empty() {return}

		let tines = self.widen(Tine::from_interval(interval));

		match tines {
			OptionSplit::None		 => unreachable!(),
			OptionSplit::One(pt)	 => self.union_pt(pt),
			OptionSplit::Two(lb, ub) => self.union_normal(lb, ub),

		}
	}

	/// Returns whether the given point is in the map.
	fn contains_pt(&self, point: &T) -> bool {
		let lb = Tine {
			point: Some(point.clone()),
			lb: true,
			ub: false,
			incl: true,
		};
		if self.tine_map.contains(&lb) {return true}

		let ub = Tine {
			point: Some(point.clone()),
			lb: false,
			ub: true,
			incl: true,
		};
		if self.tine_map.contains(&ub) {return true}

		let pt = Tine {
			point: Some(point.clone()),
			lb: false,
			ub: false,
			incl: true,
		};
		self.tine_map.contains(&pt)
	}

	/// Widens the given `Tine` set to encompass neighboring points if their
	/// `Interval` normalization would include them.
	fn widen(&self, tines: OptionSplit<Tine<T>>) -> OptionSplit<Tine<T>> {
		use self::OptionSplit::*;
		match tines {
			None		=> None,
			One(pt)		=> {
				// Try to widen the point on both sides.
				let mut lb = pt.clone();
				lb.ub = false;
				lb.lb = true;

				let mut ub = pt.clone();
				ub.ub = true;
				ub.lb = false;
				
				let new_l = self.widen_l(lb);
				let new_r = self.widen_r(ub);

				if new_l.point == new_r.point {
					// Widening didn't do anthing.
					One(pt)
				} else {
					// Widening worked, use new tines.
					Two(new_l, new_r)
				}
			}
			Two(lb, ub) => Two(self.widen_l(lb), self.widen_r(ub)),
		}
	}

	/// Widens the given `Tine` to encompass leftward neigbors if its
	/// `Interval`'s lower bound normalization would include them.
	fn widen_l(&self, mut tine: Tine<T>) -> Tine<T> {
		while let Some(lb) = tine.point.clone().and_then(|pt| pt.next_lower()) {
			if self.contains_pt(&lb) {
				tine.point = Some(lb);
			} else {
				break
			}
		}
		tine
	}

	/// Widens the given `Tine` to encompass rightward neigbors if its
	/// `Interval`'s upper bound normalization would include them.
	fn widen_r(&self, mut tine: Tine<T>) -> Tine<T> {
		while let Some(ub) = tine.point.clone().and_then(|pt| pt.next_upper()) {
			if self.contains_pt(&ub) {
				tine.point = Some(ub);
			} else {
				break
			}
		}
		tine
	}

	/// Inserts a point `Interval` into the map.
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

	/// Inserts a non-point, non-empty `Interval` into the map.
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

		match (lb, ub) {
			(None, None) => {
				debug_assert!(before.expect("upper bound annhilation").lb);
				debug_assert!(after.expect("lower bound annhilation").ub);
			},
			(Some(lb), None) => {
				r_map.insert(lb);
				debug_assert!(after.expect("lower bound annhilation").ub);	
			},

			(None, Some(ub)) => {
				r_map.insert(ub);
				debug_assert!(before.expect("upper bound annhilation").lb);
			},

			(Some(lb), Some(ub)) => {
				if before.is_none() 
					|| before.as_ref().map(|b| b.ub) == Some(true)
					|| (before.map(|b| b.lb) == Some(true) && lb.ub)

				{
					r_map.insert(lb);
				}
				if after.is_none() 
					|| after.as_ref().map(|a| a.lb) == Some(true) 
					|| (after.map(|a| a.ub) == Some(true)  && ub.lb)
				{
					r_map.insert(ub);
				}

			}
		}
		
		self.tine_map.extend(r_map);
	}
}

impl<T> IntoIterator for DisjunctionMap<T> where T: PartialOrd + Ord + Clone {
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
/// An iterator over the intervals in the `Selection`.
pub struct SelectionIter<T> where T: PartialOrd + Ord + Clone {
	/// The `Tine` map yet to process.
	tine_iter: btree_set::IntoIter<Tine<T>>,
	
	/// A `Tine` from a previous interval that contains information for the next
	/// `Interval`.
	saved: Option<Tine<T>>,
}

impl<T> Iterator for SelectionIter<T> where T: PartialOrd + Ord + Clone {
	type Item = Interval<T>;

	fn next(&mut self) -> Option<Self::Item> {
		if let Some(saved) = self.saved.take() {
			debug_assert!(saved.lb);
			let second = self.tine_iter.next().expect("");
			debug_assert!(second.ub);
			if second.lb {self.saved = Some(second.clone());}

			return Some(Interval::new(
				Tine::into_bound_lower(saved),
				Tine::into_bound_upper(second)));

		}
		
		if let Some(first) = self.tine_iter.next() {
			debug_assert!(!first.ub);

			if first.is_point() {
				return Some(Interval::point(first.point.expect("point tine")));
			}
			let second = self.tine_iter.next().expect("upper bound");
			if second.lb {self.saved = Some(second.clone());}

			Some(Interval::new(
				Tine::into_bound_lower(first),
				Tine::into_bound_upper(second)))
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
	fn disjunction_map_insert_disjoint_normalized() {
		let mut dm: DisjunctionMap<i32> = DisjunctionMap::new();

		let a = Interval::open(0, 15);
		let b = Interval::open(20, 25);
		let c = Interval::open(30, 35);
		dm.union_insert(b.clone());
		dm.union_insert(a.clone());
		dm.union_insert(c.clone());

		let dm_res: Vec<Interval<i32>> = dm.into_iter().collect();
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
	fn disjunction_map_insert_overlap_normalized() {
		let mut dm: DisjunctionMap<i32> = DisjunctionMap::new();

		let a = Interval::open(0, 15);
		let b = Interval::open(20, 25);
		let c = Interval::open(14, 26);
		dm.union_insert(b.clone());
		dm.union_insert(a.clone());
		dm.union_insert(c.clone());

		let dm_res: Vec<Interval<i32>> = dm.into_iter().collect();
		assert_eq!(dm_res, vec![Interval::open(0, 26)]);
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
	fn disjunction_map_insert_disjoint_close_normalized() {
		let mut dm: DisjunctionMap<i32> = DisjunctionMap::new();

		let a = Interval::open(0, 15);
		let b = Interval::open(15, 25);
		let c = Interval::open(25, 30);
		dm.union_insert(c.clone());
		dm.union_insert(a.clone());
		dm.union_insert(b.clone());

		let dm_res: Vec<Interval<i32>> = dm.into_iter().collect();
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
	fn disjunction_map_insert_overlap_close_normalized() {
		let mut dm: DisjunctionMap<i32> = DisjunctionMap::new();

		let a = Interval::open(0, 15);
		let b = Interval::closed(15, 25);
		let c = Interval::open(25, 30);
		dm.union_insert(c.clone());
		dm.union_insert(a.clone());
		dm.union_insert(b.clone());

		let dm_res: Vec<Interval<i32>> = dm.into_iter().collect();
		assert_eq!(dm_res, vec![Interval::open(0, 30)]);
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
	fn disjunction_map_insert_disjoint_point_normalized() {
		let mut dm: DisjunctionMap<i32> = DisjunctionMap::new();

		let a = Interval::open(0, 15);
		let b = Interval::open(15, 25);
		let c = Interval::point(15);
		dm.union_insert(c.clone());
		dm.union_insert(a.clone());
		dm.union_insert(b.clone());

		let dm_res: Vec<Interval<i32>> = dm.into_iter().collect();
		assert_eq!(dm_res, vec![Interval::open(0, 25)]);
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
	fn disjunction_map_insert_overlap_exact_normalized() {
		let mut dm: DisjunctionMap<i32> = DisjunctionMap::new();

		let a = Interval::open(0, 15);
		let b = Interval::open(15, 25);
		let c = Interval::open(0, 25);
		dm.union_insert(a.clone());
		dm.union_insert(b.clone());
		dm.union_insert(c.clone());

		let dm_res: Vec<Interval<i32>> = dm.into_iter().collect();
		assert_eq!(dm_res, vec![Interval::open(0, 25)]);
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
	fn disjunction_map_insert_overlap_widen_normalized() {
		let mut dm: DisjunctionMap<i32> = DisjunctionMap::new();

		let a = Interval::open(0, 15);
		let b = Interval::open(15, 25);
		let c = Interval::closed(0, 25);
		dm.union_insert(a.clone());
		dm.union_insert(b.clone());
		dm.union_insert(c.clone());

		let dm_res: Vec<Interval<i32>> = dm.into_iter().collect();
		assert_eq!(dm_res, vec![Interval::closed(0, 25)]);
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
	fn disjunction_map_insert_overlap_point_normalized() {
		let mut dm: DisjunctionMap<i32> = DisjunctionMap::new();

		let a = Interval::open(0, 30);
		let b = Interval::point(15);
		dm.union_insert(a.clone());
		dm.union_insert(b.clone());

		let dm_res: Vec<Interval<i32>> = dm.into_iter().collect();
		assert_eq!(dm_res, vec![Interval::open(0, 30)]);
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

	#[test]
	fn disjunction_map_insert_overlap_unbounded_normalized() {
		let mut dm: DisjunctionMap<i32> = DisjunctionMap::new();

		let a = Interval::from(10);
		let b = Interval::open(15, 30);
		dm.union_insert(a.clone());
		dm.union_insert(b.clone());

		let dm_res: Vec<Interval<i32>> = dm.into_iter().collect();
		assert_eq!(dm_res, vec![Interval::from(10)]);
	}

	#[test]
	fn disjunction_map_insert_units() {
		let mut dm: DisjunctionMap<Opaque> = DisjunctionMap::new();

		let a = Interval::point(Opaque(10));
		let b = Interval::point(Opaque(11));
		let c = Interval::point(Opaque(12));
		let d = Interval::point(Opaque(13));
		let e = Interval::point(Opaque(14));
		dm.union_insert(a.clone());
		dm.union_insert(b.clone());
		dm.union_insert(c.clone());
		dm.union_insert(d.clone());
		dm.union_insert(e.clone());

		let dm_res: Vec<Interval<Opaque>> = dm.into_iter().collect();
		assert_eq!(dm_res, vec![
			Interval::point(Opaque(10)),
			Interval::point(Opaque(11)),
			Interval::point(Opaque(12)),
			Interval::point(Opaque(13)),
			Interval::point(Opaque(14)),
		]);
	}

	#[test]
	fn disjunction_map_insert_units_normalized() {
		let mut dm: DisjunctionMap<i32> = DisjunctionMap::new();

		let a = Interval::point(10);
		let b = Interval::point(11);
		let c = Interval::point(12);
		let d = Interval::point(13);
		let e = Interval::point(14);
		dm.union_insert(a.clone());
		dm.union_insert(b.clone());
		dm.union_insert(d.clone());
		dm.union_insert(e.clone());
		dm.union_insert(c.clone());

		let dm_res: Vec<Interval<i32>> = dm.into_iter().collect();
		assert_eq!(dm_res, vec![Interval::closed(10, 14)]);
	}
}