Overview
========

This library (`Interval`) provides an implementation of range and interval types as an alternative to the std library's `Range` and related types. The reasoning for this alternative is that `Interval` provides machinery for the normalization of (finite) intervals. This reduces ambiguity and enhances analysis of finite types.

Usage
=====

Add this line to your crate's Cargo.toml file:

interval = "0.12.3"


What is interval normalization?
===============================

Interval normalization ensures that equivalent intervals have the same representation. For instance, if we have an `Interval<i32>` covering (0, 15], the left bound is exclusive, and due to the finiteness of `i32`, the interval will be equivalent to [1, 15]. In this way, intervals over finite types can always be 'normalized' as closed finite intervals. Additionally, unions of nearby intervals my overlap if denormalized. [0, 4] union [5, 6] selects the same points as [0, 6], even though the intervals do not share bounds. Thus we also have to normalize intervals with respect to set operations.


How is interval normalization achieved?
======================================

`Interval<T>` is implemented as a normalizing wrapper around `RawInterval<T>`. Any type which implements `Normalize` will be automatically normalized after any operation performed on `Interval<T>`. Dynamic unions of intervals are implemented through `Selection<T>`, which is a normalizing wrapper around `TineTree<T>`, which ensures that interval operations are performed on the broadest 'denormalized' set of intervals possible before normalization occurs.