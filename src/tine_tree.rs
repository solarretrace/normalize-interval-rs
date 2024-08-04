// Copyright 2018 Skylor R. Schermer.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
////////////////////////////////////////////////////////////////////////////////
//! Interval `TineTree` implementation.
////////////////////////////////////////////////////////////////////////////////
// NOTE: Unused results are permitted here because the `TineTree` calls
// `BTreeSet::insert` frequently without concern for its return value.
#![allow(unused_results)]

// Internal library imports.
use crate::bound::Bound;
use crate::raw_interval::RawInterval;
use crate::tine::Tine;

// External library imports.
use few::Few;

// Standard library imports.
use std::collections::BTreeSet;
use std::collections::btree_set;
use std::iter::FromIterator;


////////////////////////////////////////////////////////////////////////////////
// TineTree
////////////////////////////////////////////////////////////////////////////////
/// A possibly noncontiguous collection of `RawInterval`s of the type `T`.
/// Implemented as an ordered list of `Tine`s. Used to implement the internal
/// state of `Selection`.
///
/// Informally, a `TineTree` acts like a number line with markers (`Tine`s) on
/// it for each `Interval` bound in a possibly disjoint union of `Interval`s.
/// 
/// [`RawInterval`]: raw_interval/struct.RawInterval.html
/// [`Selection`]: selection/struct.Selection.html
/// [`Tine`]: tine_tree/struct.Tine.html
/// [`Interval`]: interval/struct.Interval.html
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TineTree<T>(BTreeSet<Tine<T>>);

impl<T> TineTree<T> where T: Ord + Clone {
    ////////////////////////////////////////////////////////////////////////////
    // Constructors
    ////////////////////////////////////////////////////////////////////////////

    /// Constructs an empty `TineTree`.
    #[must_use]
    pub fn new() -> Self {
        Self(BTreeSet::new())
    }

    /// Constructs a `TineTree` from a `RawInterval`.
    #[must_use]
    pub fn from_raw_interval(interval: RawInterval<T>) -> Self {
        Self(Tine::from_raw_interval(interval).collect())
    }

    ////////////////////////////////////////////////////////////////////////////
    // Bound accessors
    ////////////////////////////////////////////////////////////////////////////

    /// Returns the lower [`Bound`] of the `TineTree`, or `None` if the 
    /// `TineTree` is empty.
    #[inline]
    pub fn lower_bound(&self) -> Option<Bound<T>> {
        self.0.iter().next().cloned().map(Tine::into_inner)
    }

    /// Returns the upper [`Bound`] of the `TineTree`, or `None` if the 
    /// `TineTree` is empty.
    #[inline]
    pub fn upper_bound(&self) -> Option<Bound<T>> {
        self.0.iter().next_back().cloned().map(Tine::into_inner)
    }


    ////////////////////////////////////////////////////////////////////////////
    // Query operations
    ////////////////////////////////////////////////////////////////////////////
    
    /// Returns `true` if the `TineTree` is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns `true` if the `TineTree` is full.
    #[must_use]
    pub fn is_full(&self) -> bool {
        self.0.iter().collect::<Vec<_>>() == [
            &Tine::Lower(Bound::Infinite),
            &Tine::Upper(Bound::Infinite)]
    }

    /// Returns `true` if the `TineTree` contains the given point.
    #[must_use]
    pub fn contains(&self, point: &T) -> bool {
        // TODO: Could be optimized by splitting the tree and looking around.
        for interval in self.interval_iter() {
            if interval.contains(point) {return true;}
        }
        false
    }

    ////////////////////////////////////////////////////////////////////////////
    // Set Operations
    ////////////////////////////////////////////////////////////////////////////

    /// Returns a `TineTree` containing all points not in present in the 
    /// `TineTree`.
    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn complement(&self) -> Self {
        use Bound::*;
        use Tine::*;

        // Early exit if we're complementing an empty interval.
        if self.0.is_empty() {
            return RawInterval::Full.into();
        }

        let mut complement = Self::new();
        let mut tine_iter = self.0.iter();
        
        // Early exit if we're complementing a point interval.
        if self.0.len() == 1 {
            let tine = tine_iter
                .next()
                .expect("nonempty TineTree")
                .clone()
                .invert();
            debug_assert!(tine.is_point_exclude());

            complement.0.insert(Lower(Infinite));
            complement.0.insert(tine);
            complement.0.insert(Upper(Infinite));
            return complement;
        }        

        // Get first and last to handle infinite bounds.
        match tine_iter.next() {
            Some(&Lower(Infinite)) => {/* Do Nothing. */},
            Some(tine)             => {
                complement.0.insert(Lower(Infinite));
                complement.0.insert(tine.clone().invert());
            },
            _ => unreachable!("TineTree len > 1"),
        }
        match tine_iter.next_back() {
            Some(&Upper(Infinite)) => {/* Do Nothing. */},
            Some(tine)             => {
                complement.0.insert(Upper(Infinite));
                complement.0.insert(tine.clone().invert());
            },
            _ => unreachable!("TineTree len > 0"),
        }

        // Invert all remaining tines.
        for tine in tine_iter {
            complement.0.insert(tine.clone().invert());
        }

        complement
    }

    /// Returns a `TineTree` containing all points in present in both of the 
    /// `TineTree`s.
    #[must_use]
    pub fn intersect(&self, other: &Self) -> Self {
        let mut intersection = Self::new();
        let self_intervals = self.interval_iter();
        let mut other_intervals = other.interval_iter();

        for self_interval in self_intervals {
            'segment: loop {
                if let Some(other_interval) = other_intervals.next() {
                    let i = self_interval.intersect(&other_interval);
                    if i.is_empty() {
                        // Nothing else overlaps in this segment.
                        break 'segment;
                    }

                    intersection.union_in_place(&i);
                } else {
                    // Nothing else overlaps anywhere.
                    return intersection;
                }
            }
        }
        intersection
    }

    /// Returns a `TineTree` containing all points present in either of the 
    /// `TineTree`s.
    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        let mut union = self.clone();
        for interval in other.interval_iter() {
            union.union_in_place(&interval);
        }
        union
    }

    /// Returns a `TineTree` containing the intersection of the given 
    /// `TineTree`'s intervals.    
    #[must_use]
    pub fn minus(&self, other: &Self) -> Self {
        let mut minus = self.clone();
        for interval in other.interval_iter() {
            minus.minus_in_place(&interval);
        }
        minus
    }

    /// Returns the smallest `RawInterval` containing all of the points in the 
    /// `TineTree`.
    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn enclose(&self) -> RawInterval<T> {
        // Early exit if we're enclosing an empty interval.
        if self.0.is_empty() {
            return RawInterval::Empty;
        } 

        let mut tine_iter = self.0.iter();

        // Early exit if we're enclosing a point interval.
        if self.0.len() == 1 {
            let tine = tine_iter
                .next()
                .expect("nonempty TineTree");
            debug_assert!(tine.is_point_include());
            let pt = tine
                .as_ref()
                .expect("point Tine value")
                .clone();
            return RawInterval::Point(pt);
        } 

        // Get first and last tines.
        let lb = tine_iter
            .next()
            .expect("first tine with len > 1")
            .clone()
            .into_inner();
        let ub = tine_iter
            .next_back()
            .expect("last tine with len > 1")
            .clone()
            .into_inner();

        RawInterval::new(lb, ub)
    }

    /// Returns the smallest closed `RawInterval` containing all of the points
    /// in the `TineTree`.
    #[must_use]
    pub fn closure(&self) -> RawInterval<T> {
        self.enclose().closure()
    }

    ////////////////////////////////////////////////////////////////////////////
    // In-place operations
    ////////////////////////////////////////////////////////////////////////////

    /// Intersects the given interval with the contents of the tree.
    pub fn intersect_in_place(&mut self, interval: &RawInterval<T>) {
        use Bound::*;
        use Tine::*;

        // Early exit if we're intersecting a full interval or are empty.
        if self.0.is_empty() || interval.is_full() {return};

        // Early exit if we're intersection an empty interval.
        if interval.is_empty() {
            *self = Self::new();
            return;
        }

        // Early exit if we're intersection a point interval.
        if let RawInterval::Point(pt) = interval {
            if self.contains(pt) {
                *self = Self::from_raw_interval(interval.clone());
            } else {
                *self = Self::new();
            }
            return;
        }

        match Tine::from_raw_interval(interval.clone()) {
            Few::Zero                   => {
                *self = Self::new();
            },
            Few::One(Point(Include(p))) => {
                if self.contains(&p) {
                    *self = Self::from_raw_interval(RawInterval::Point(p));
                } else {
                    *self = Self::new();
                }
            },
            Few::Two(l, u)              => {
                self.intersect_proper_interval(l, u);
            },
            Few::One(_) => unreachable!("invalid Tine from interval"),
        }
    }

    /// Internal implementation of `intersect_in_place`, handling the proper
    /// interval case.
    fn intersect_proper_interval(&mut self, l: Tine<T>, u: Tine<T>) {
        let mut ts = self.interior_split_for_proper_interval(&l, &u);

        // Merge tines if overlap or use given ones. We should only have `None`
        // in the case of a intersection annhiliation.
        let merged_l = if ts[2].is_some() {
            ts[2].take().and_then(|lower| lower.intersect(&l))
        } else {
            Some(l)
        };

        let merged_u = if ts[3].is_some() {
            ts[3].take().and_then(|upper| upper.intersect(&u))
        } else {
            Some(u)
        };
        
        // Ensure inner tines have the correct bounds.
        debug_assert!(merged_l
            .as_ref()
            .map_or(true, Tine::is_lower_bound));
        debug_assert!(merged_u
            .as_ref()
            .map_or(true, Tine::is_upper_bound));

        
        // We need to detect whether the point is inside or outside an interval.
        // To do this, we look at the tines inside and outside the interval.
        let open_before = ts[0]
            .as_ref()
            .is_some_and(Tine::is_lower_bound);
        let closed_after = ts[5]
            .as_ref()
            .is_some_and(Tine::is_upper_bound);

        let in_l = ts[1]
            .as_ref()
            .is_some_and(Tine::is_upper_bound);
        let in_r = ts[4]
            .as_ref()
            .is_some_and(Tine::is_lower_bound);


        // Insert tines into the tree, ignoring them if the are not wrapped by a
        // surrounding interval, or not wrapping a surrounding interval.
        match (open_before, merged_l, in_l, in_r, merged_u, closed_after) {
            (_,     Some(l), true,  true,  Some(u), _   )  |
            (_,     Some(l), false, false, Some(u), _   )  => {
                // (   ) (   )
                //   (     )
                //     O R
                // (     )
                //   ( )
                //     O R
                // (     )
                // (  )
                //     O R
                // (     )
                //    (  )
                self.0.insert(l);
                self.0.insert(u);
            },
            (true, Some(l),  true,  false, _,       false) => {
                // (   )
                //   (   )
                //     O R
                // (   ) ( )
                //   (   )
                self.0.insert(l);
            },
            (false, _,       false, true,  Some(u), true)  => {
                //   (   )
                // (   )
                //     O R
                // ( ) (   )
                //   (   )
                self.0.insert(u);
            },
            (false, _,       false, false, _,       false) => {
                // )     (
                // (     )
                //     O R
                //   ( )
                // (     )
                /* Do nothing. */
            },
            _ => unreachable!("invalid bounds for intersection interval"),
        }
    }

    /// Unions the given interval with the contents of the tree.
    pub fn union_in_place(&mut self, interval: &RawInterval<T>) {
        // Early exit if we're unioning a full interval.
        if interval.is_full() {
            *self = Self::from_raw_interval(RawInterval::Full);
            return;
        }

        match Tine::from_raw_interval(interval.clone()) {
            Few::Zero      => (),
            Few::One(p)    => self.union_point_interval(p),
            Few::Two(l, u) => self.union_proper_interval(l, u),
        }
    }

    /// Internal implementation of `union_in_place`, handling the point interval
    /// case.
    #[allow(clippy::cognitive_complexity)]
    fn union_point_interval(&mut self, p: Tine<T>) {
        let mut ts = self.exterior_split_for_point_interval(&p);

        let p = if ts[1].is_some() {
            if let Some(merged) = ts[1]
                .take()
                .and_then(|pt| pt.union(&p)) 
            {
                merged
            } else {
                // If the point annhilates, then we've already joined two
                // intervals by removing the Point(Exclude(_)) from the tree in
                // exterior_split_for_point_interval. So nothing else needs to happen.
                return;
            }
        } else {
            p
        };
        
        // We need to detect whether the point is inside or outside an interval.
        // To do this, we look at the tines before and after the interval.
        let open_before = ts[0]
            .as_ref()
            .is_some_and(Tine::is_lower_bound);
        let closed_after = ts[2]
            .as_ref()
            .is_some_and(Tine::is_upper_bound);

        // Insert tine into the tree, ignoring it if it is wrapped by a
        // surrounding interval.
        match (open_before, closed_after) {
            (true,  true)  => {
                // (   )
                //   |
                // Do nothing.
            },
            (true,  false) => {
                // ( )   ( )
                //   |
                debug_assert!(!p.is_lower_bound());
                self.0.insert(p);
            },
            (false, true)  => {
                // ( )   ( )
                //       |
                debug_assert!(!p.is_upper_bound());
                self.0.insert(p);
            },
            (false, false) => {
                // ( )   ( )
                //     |
                self.0.insert(p);
            },
        }
    }

    /// Internal implementation of `union_in_place`, handling the proper
    /// interval case.
    fn union_proper_interval(&mut self, l: Tine<T>, u: Tine<T>) {
        let mut ts = self.exterior_split_for_proper_interval(&l, &u);

        // Merge tines if overlap or use given one. We should only have `None`
        // in the case of a union annhiliation.
        let merged_l = if ts[1].is_some() {
            ts[1].take().and_then(|lower| lower.union(&l))
        } else {
            Some(l)
        };

        let merged_u = if ts[2].is_some() {
            ts[2].take().and_then(|upper| upper.union(&u))
        } else {
            Some(u)
        };

        // Ensure inner tines have the correct bounds.
        debug_assert!(merged_l
            .as_ref()
            .map_or(true, Tine::is_lower_bound));
        debug_assert!(merged_u
            .as_ref()
            .map_or(true, Tine::is_upper_bound));

        // We need to detect whether the interval is inside or outside an 
        // existing interval. To do this, we look at the tines before and after
        // the interval.
        let open_before = ts[0]
            .as_ref()
            .is_some_and(Tine::is_lower_bound);
        let closed_after = ts[3]
            .as_ref()
            .is_some_and(Tine::is_upper_bound);
        
        // Insert tines into the tree, ignoring them if the are wrapped by a
        // surrounding interval.
        match (open_before, merged_l, merged_u, closed_after) {
            (true,  Some(l), Some(u), true)  => {
                // ( ) ( )
                //   ( )
                if l.is_upper_bound() {self.0.insert(l);}
                if u.is_lower_bound() {self.0.insert(u);}
            },
            (true,  Some(l), Some(u), false) => {
                // ( ) ( ) ( )
                //   (   )
                //     O R
                // ( ) ( )
                //   (   )
                if l.is_upper_bound() {self.0.insert(l);}
                debug_assert!(!u.is_lower_bound());
                self.0.insert(u);
            },
            (false, Some(l), Some(u), true)  => {
                // ( ) ( ) ( )
                //     (   )
                //     O R
                // ( ) ( ) ( )
                // [   )
                debug_assert!(!l.is_upper_bound());
                self.0.insert(l);
                if u.is_lower_bound() {self.0.insert(u);}

            },
            (false, Some(l), Some(u), false) => {
                // ( ) ( ) ( )
                //     [ ]
                //     O R
                // ( ) ( )
                //     [ ]
                //     O R
                // ( ) ( ) ( )
                // [     )
                //     O R
                // ( ) ( )
                // [     ]
                debug_assert!(!l.is_upper_bound());
                self.0.insert(l);
                debug_assert!(!u.is_lower_bound());
                self.0.insert(u);
            },

            (true,  Some(l), None,    true)  => {
                // ( ) ( ) ( )
                //   (     ]
                if l.is_point_exclude() {self.0.insert(l);}
            },
            (false, Some(l), None,    true)  => {
                // ( ) ( ) ( )
                //     [   ]
                //     O R
                // ( ) ( ) ( )
                // [       ]
                debug_assert!(!l.is_upper_bound());
                self.0.insert(l);
            },

            (true,  None,    Some(u), true)  => {
                // ( ) ( ) ( )
                //   [     )
                if u.is_point_exclude() {self.0.insert(u);}
            },
            (true,  None,    Some(u), false)  => {
                // ( ) ( ) ( )
                //   [   ]
                //     O R
                // ( ) ( ) ( )
                //   [       ]
                debug_assert!(!u.is_lower_bound());
                self.0.insert(u);
            },

            (true,  None,    None,    true) => {
                // ( ) ( ) ( )
                //   [     ] 
                // Do nothing.
            },
            _ => unreachable!("invalid bounds for union interval"),
        }
    }

    /// Minuses the given interval from the contents of the tree.
    pub fn minus_in_place(&mut self, interval: &RawInterval<T>) {
        // Early exit if we're minusing an empty interval or are empty.
        if self.0.is_empty() || interval.is_empty() {return};

        // Early exit if we're minusing a full interval.
        if interval.is_full() {
            *self = Self::new();
            return;
        }

        match Tine::from_raw_interval(interval.clone()) {
            Few::Zero      => (),
            Few::One(p)    => self.minus_point_interval(p),
            Few::Two(l, u) => self.minus_proper_interval(l, u),
        }
    }

    /// Internal implementation of `minus_in_place`, handling the point interval
    /// case.
    fn minus_point_interval(&mut self, p: Tine<T>) {
        let mut ts = self.exterior_split_for_point_interval(&p);

        let p = if ts[1].is_some() {
            if let Some(merged) = ts[1]
                .take()
                .and_then(|pt| pt.minus(&p)) 
            {
                merged
            } else {
                // If the point annhilates, then we've already joined two
                // intervals by removing the Point(Exclude(_)) from the tree in
                // minus_split_tree_point. So nothing else needs to happen.
                return;
            }
        } else {
            p
        };
        
        // We need to detect whether the point is inside or outside an interval.
        // To do this, we look at the tines before and after the interval.
        let open_before = ts[0]
            .as_ref()
            .is_some_and(Tine::is_lower_bound);
        let closed_after = ts[2]
            .as_ref()
            .is_some_and(Tine::is_upper_bound);

        // Insert tine into the tree, ignoring it if it is wrapped by a
        // surrounding interval.
        // NOTE: We cannot have a Point(Exclude) here, because those will never
        // result from an interval-tine conversion.
        match (open_before, closed_after) {
            (true,  true)  => {
                // (   )
                //   |
                self.0.insert(p.invert());
            },
            (true,  false) => {
                // ( )   ( )
                //   |
                debug_assert!(p.is_upper_bound());
                self.0.insert(p);
            },
            (false, true)  => {
                // ( )   ( )
                //       |
                debug_assert!(p.is_lower_bound());
                self.0.insert(p);
            },
            (false, false) => {
                // ( )   ( )
                //     |
                // Do nothing.
            },
        }
    }


    /// Internal implementation of `minus_in_place`, handling the proper
    /// interval case.
    #[allow(clippy::items_after_statements)]
    fn minus_proper_interval(&mut self, l: Tine<T>, u: Tine<T>) {
        let mut ts = self.exterior_split_for_proper_interval(&l, &u);

        // Merge tines if overlap
        let merged_l = if ts[1].is_some() {
            ts[1].take().and_then(|lower| lower.minus(&l))
        } else {
            Some(l)
        };

        let merged_u = if ts[2].is_some() {
            ts[2].take().and_then(|upper| upper.minus(&u))
        } else {
            Some(u)
        };

        // We need to detect whether the interval is inside or outside an 
        // existing interval. To do this, we look at the tines before and after
        // the interval.
        let open_before = ts[0]
            .as_ref()
            .is_some_and(Tine::is_lower_bound);
        let closed_after = ts[3]
            .as_ref()
            .is_some_and(Tine::is_upper_bound);
        
        // Insert tines into the tree, ignoring them if the are not wrapped by a
        // surounding interval.
        use Bound::*;
        use Tine::*;
        match (open_before, merged_l, merged_u, closed_after) {
            (true,  Some(l), Some(u), true)  => {
                // ( ) ( )
                //  (   )
                //     O R
                // ( ) ( )
                //   ( )
                self.0.insert(if l.is_upper_bound() {l} else {l.invert()});
                self.0.insert(if u.is_lower_bound() {u} else {u.invert()});
            },
            (true,  Some(l), upper,   false)  => {
                // ( )
                //  ( )
                //     O R
                // ( )
                //   ( )
                //     O R
                // (   )
                //   ( )
                //     O R
                // (   ]
                //   ( )
                //     O R
                self.0.insert(if l.is_upper_bound() {l} else {l.invert()});
                if let Some(Point(Include(p))) = upper {
                    self.0.insert(Point(Include(p)));
                }
            },
            (false, lower,   Some(u), true)   => {
                //  ( )
                // ( )
                //     O R
                //   ( )
                // ( )
                //     O R
                // (   )
                // ( )
                //     O R
                // [   )
                // ( )
                self.0.insert(if u.is_lower_bound() {u} else {u.invert()});
                if let Some(Point(Include(p))) = lower {
                    self.0.insert(Point(Include(p)));
                }
            },
            (false, Some(l), Some(u), false)  => {
                //  ( )
                // (   )
                //     O R
                // [ ]
                // ( )
                if l.is_point_include() { self.0.insert(l); }
                if u.is_point_include() { self.0.insert(u); }
            },

            (false, Some(l), None,    false)  => {
                // [ )
                // ( )
                //     O R
                //   |
                // ( ]
                if l.is_point_include() { self.0.insert(l); }
            },

            (false, None,    Some(u), false)  => {
                // ( ]
                // ( )
                //     O R
                // |
                // [ )
                if u.is_point_include() { self.0.insert(u); }
            },

            (false, None,    None,    false)  => {
                // ( )
                // ( )
                // Do nothing.
            },
            _ => unreachable!("invalid bounds for minus interval"),
        }
    }

    /// Splits the tine tree into three sections for an interval-like Tine to
    /// prepare for an intersect operation.
    ///
    /// The resulting array contains the following values:
    /// ```rust,ignore
    /// [
    ///     0 => Copy of the first tine less than the lower tine.
    ///     1 => Copy of the first tine greater than the lower tine.
    ///     2 => The tine equal to the lower tine.
    ///     3 => The tine equal to the upper tine.
    ///     4 => Copy of the first tine less than the upper tine.
    ///     5 => Copy of the first tine greater than the upper tine.
    /// ]
    /// ```
    ///
    /// Any tines not between lower and upper are dropped.
    fn interior_split_for_proper_interval(
        &mut self,
        lower: &Tine<T>,
        upper: &Tine<T>) 
        -> [Option<Tine<T>>; 6]
    {
        debug_assert!(lower < upper);
        let mut res = [None, None, None, None, None, None];

        // Get lower and upper if they are in the tree.
        res[2] = self.0.take(lower);
        res[3] = self.0.take(upper);
        
        // Get before and after points and drop anything not in the center.
        let mut center = self.0.split_off(lower);
        let right_side = center.split_off(upper);

        {
            let mut backward = self.0.iter();
            res[0] = backward.next_back().cloned();

            let mut forward = center.iter();
            res[1] = forward.next().cloned();
        }

        {
            let mut backward = center.iter().rev();
            res[4] = backward.next().cloned();

            let mut forward = right_side.iter();
            res[5] = forward.next().cloned();
        }

        debug_assert_eq!(res[1].is_some(), res[4].is_some());
        
        self.0 = center;
        res
    }

    /// Splits the tine tree into three sections for a point-like Tine to
    /// prepare for a union operation.
    ///
    /// The resulting array contains the following values:
    /// ```rust,ignore
    /// [
    ///     0 => Copy of the first tine less than the given tine.
    ///     1 => The tine equal to the given tine.
    ///     2 => Copy of the first tine greater than the given tine.
    /// ]
    /// ```
    fn exterior_split_for_point_interval(&mut self, tine: &Tine<T>)
        -> [Option<Tine<T>>; 3]
    {
        let mut res = [None, None, None];

        // Get pt if it is in the tree.
        res[1] = self.0.take(tine);

        // Get before and after points.
        let mut right_side = self.0.split_off(tine);
        res[0] = self.0.iter().next_back().cloned();
        res[2] = right_side.iter().next().cloned();

        self.0.append(&mut right_side);
        res
    }

    /// Splits the tine tree into three sections for an interval-like Tine in
    /// preperation for a union or minus operation.
    ///
    /// The resulting array contains the following values:
    /// ```rust,ignore
    /// [
    ///     0 => Copy of the first tine less than the lower tine.
    ///     1 => The tine equal to the lower tine.
    ///     2 => The tine equal to the upper tine.
    ///     3 => Copy of the first tine greater than the upper tine.
    /// ]
    /// ```
    ///
    /// Any tines between lower and upper are dropped.
    fn exterior_split_for_proper_interval(
        &mut self,
        lower: &Tine<T>,
        upper: &Tine<T>)
        -> [Option<Tine<T>>; 4]
    {
        let mut res = [None, None, None, None];

        // Get lower and upper if they are in the tree.
        res[1] = self.0.take(lower);
        res[2] = self.0.take(upper);

        // Get before and after points and drop anything in the center.
        let mut center = self.0.split_off(lower);
        {
            let mut backward = self.0.iter();
            res[0] = backward.next_back().cloned();
        }

        let mut right_side = center.split_off(upper);
        {
            let mut forward = right_side.iter();
            res[3] = forward.next().cloned();
        }
        
        self.0.append(&mut right_side);
        res
    }

    ////////////////////////////////////////////////////////////////////////////
    // Iterator conversions
    ////////////////////////////////////////////////////////////////////////////

    /// Returns an iterator over each of the `RawInterval`s in the tree.
    #[must_use]
    pub fn interval_iter(&self) -> Iter<'_, T> {
        Iter {
            tine_iter: self.0.iter(),
            saved_lower: None,
            saved_upper: None,
        }
    }
}

impl<T> Default for TineTree<T> where T: Ord + Clone {
    fn default() -> Self {
        Self::new()
    }
}

////////////////////////////////////////////////////////////////////////////////
// Conversion traits
////////////////////////////////////////////////////////////////////////////////
impl<T> From<RawInterval<T>> for TineTree<T> where T: Ord + Clone {
    fn from(interval: RawInterval<T>) -> Self {
        Self::from_raw_interval(interval)
    }
}

impl<T, I> From<I> for TineTree<T>
    where
        T: Ord + Clone,
        I: Iterator<Item=RawInterval<T>>
{
    fn from(iter: I) -> Self {
        let mut tine_tree = Self::new();
        for interval in iter {
            tine_tree.union_in_place(&interval);
        }
        tine_tree
    }
}

impl<T> FromIterator<RawInterval<T>> for TineTree<T>
    where T: Ord + Clone
{
    fn from_iter<I>(iter: I) -> Self
        where I: IntoIterator<Item=RawInterval<T>>
    {
        let mut tine_tree = Self::new();
        for interval in iter {
            tine_tree.union_in_place(&interval);
        }
        tine_tree
    }
}

impl<T> IntoIterator for TineTree<T>
    where T: Ord + Clone 
{
    type Item = RawInterval<T>;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            inner: self.0.into_iter(),
            saved_lower: None,
            saved_upper: None,
        }
    }
}


////////////////////////////////////////////////////////////////////////////////
// IntoIter
////////////////////////////////////////////////////////////////////////////////
/// An owning `Iterator` over the `TineTree`s `RawInterval`s.
#[derive(Debug)]
pub struct IntoIter<T> {
    /// The tree's `Tine`s in order.
    inner: btree_set::IntoIter<Tine<T>>,
    /// A saved lower-bound tine.
    saved_lower: Option<Tine<T>>,
    /// A saved upper-bound tine.
    saved_upper: Option<Tine<T>>,
}

impl<T> Iterator for IntoIter<T> where T: Ord + Clone {
    type Item = RawInterval<T>;

    fn next(&mut self) -> Option<Self::Item> {
        use Bound::*;
        use Tine::*;
        self.saved_lower
            .take()
            .or_else(|| self.inner.next())
            .map(|lower| {
                if let Point(Include(p)) = lower {
                    // Next tine is a single point.
                    RawInterval::Point(p)
                } else {
                    // Next tine must be a lower bound of an interval.
                    debug_assert!(lower.is_lower_bound());

                    let upper = self.inner.next()
                        .or_else(|| self.saved_upper.take())
                        .expect("interval is not partial");

                    if upper.is_point_exclude() {
                        self.saved_lower = Some(upper.clone());
                    }

                    // ... and the next tine after must be an upper bound.
                    debug_assert!(upper.is_upper_bound());

                    let lower = lower.into_inner();
                    let upper = upper.into_inner();
                    RawInterval::new(lower, upper)
                }
            })
    }
}

impl<T> DoubleEndedIterator for IntoIter<T>
    where T: Ord + Clone 
{
    fn next_back(&mut self) -> Option<Self::Item> {
        use Bound::*;
        use Tine::*;
        self.saved_upper
            .take()
            .or_else(|| self.inner.next_back())
            .map(|upper| {
                if let Point(Include(p)) = upper {
                    // Next tine is a single point.
                    RawInterval::Point(p)
                } else {
                    // Next tine must be an upper bound of an interval.
                    debug_assert!(upper.is_upper_bound());

                    let lower = self.inner.next_back()
                        .or_else(|| self.saved_lower.take())
                        .expect("interval is not partial");

                    if lower.is_point_exclude() {
                        self.saved_lower = Some(lower.clone());
                    }

                    // ... and the next tine after must be a lower bound.
                    debug_assert!(lower.is_lower_bound());

                    let upper = upper.into_inner();
                    let lower = lower.into_inner();
                    RawInterval::new(lower, upper)
                }
            })
    }
}

////////////////////////////////////////////////////////////////////////////////
// Iter
////////////////////////////////////////////////////////////////////////////////
/// An `Iterator` that constructs `RawInterval`s from a sequence of `Tine`s.
#[derive(Debug)]
pub struct Iter<'t, T> {
    /// The tree's `Tine`s in order.
    #[allow(clippy::struct_field_names)]
    tine_iter: btree_set::Iter<'t, Tine<T>>,
    /// A saved lower-bound tine.
    saved_lower: Option<Tine<T>>,
    /// A saved upper-bound tine.
    saved_upper: Option<Tine<T>>,
}

impl<'t, T> Iterator for Iter<'t, T>
    where T: Ord + Clone
{
    type Item = RawInterval<T>;

    fn next(&mut self) -> Option<Self::Item> {
        use Bound::*;
        use Tine::*;
        self.saved_lower
            .take()
            .or_else(|| self.tine_iter.next().cloned())
            .map(|lower| {
                if let Point(Include(p)) = lower {
                    // Next tine is a single point.
                    RawInterval::Point(p)
                } else {
                    // Next tine must be a lower bound of an interval.
                    debug_assert!(lower.is_lower_bound());

                    let upper = self.tine_iter.next().cloned()
                        .or_else(|| self.saved_upper.take())
                        .expect("interval is not partial");

                    if upper.is_point_exclude() {
                        self.saved_lower = Some(upper.clone());
                    }

                    // ... and the next tine after must be an upper bound.
                    debug_assert!(upper.is_upper_bound());

                    let lower = lower.into_inner();
                    let upper = upper.into_inner();
                    RawInterval::new(lower, upper)
                }
            })

    }
}

impl<'t, T> DoubleEndedIterator for Iter<'t, T>
    where T: Ord + Clone 
{
    fn next_back(&mut self) -> Option<Self::Item> {
        use Bound::*;
        use Tine::*;
        self.saved_upper
            .take()
            .or_else(|| self.tine_iter.next_back().cloned())
            .map(|upper| {
                if let Point(Include(p)) = upper {
                    // Next tine is a single point.
                    RawInterval::Point(p)
                } else {
                    // Next tine must be an upper bound of an interval.
                    debug_assert!(upper.is_upper_bound());

                    let lower = self.tine_iter.next_back().cloned()
                        .or_else(|| self.saved_lower.take())
                        .expect("interval is not partial");

                    if lower.is_point_exclude() {
                        self.saved_lower = Some(lower.clone());
                    }

                    // ... and the next tine after must be a lower bound.
                    debug_assert!(lower.is_lower_bound());

                    let upper = upper.into_inner();
                    let lower = lower.into_inner();
                    RawInterval::new(lower, upper)
                }
            })
    }
}
