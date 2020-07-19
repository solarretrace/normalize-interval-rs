Overview
========

This library provides an alternative to the std [`Range`](https://doc.rust-lang.org/stable/std/ops/struct.Range.html) types which supports finite interval normalization. When used with a bounded, finite data type, the interval will support set operations (unions, intersections, etc), and iteration over potentially disjoint sets of intervals can be represented efficiently as a tree of interval bounds.


Representations for infinite intervals
======================================

This library was previously designed to support infinite and finite data types, utilizing trait specialization to make normalization of infinite intervals a no-op. To allow building on stable, this feature has been disabled, so `Interval<T>` and `Selection<T>` are only usable if `T: Finite`. As such, there are many methods defined for selections and intervals which are essentially useless, as all intervals will be normalized into a closed representation after construction.


What is interval normalization?
===============================

Interval normalization ensures that equivalent intervals have the same representation. For instance, if we have an `Interval<i32>` covering (0, 15], the left bound is exclusive, and due to the finiteness of `i32`, the interval will be equivalent to [1, 15]. In this way, intervals over finite types can always be 'normalized' as closed finite intervals. Additionally, unions of nearby intervals may overlap if denormalized. [0, 4] union [5, 6] selects the same points as [0, 6], even though the intervals do not share bounds. Thus we also have to normalize intervals with respect to set operations.


How is interval normalization achieved?
======================================

`Interval<T>` is implemented as a normalizing wrapper around `RawInterval<T>`. Any type which implements `Normalize` will be automatically normalized after any operation performed on `Interval<T>`. Dynamic unions of intervals are implemented through `Selection<T>`, which is a normalizing wrapper around `TineTree<T>`, which ensures that interval operations are performed on the broadest 'denormalized' set of intervals possible before normalization occurs.
