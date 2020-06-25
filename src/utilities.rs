// Copyright 2018 Skylor R. Schermer.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
////////////////////////////////////////////////////////////////////////////////
//!
//! Miscellaneous tools
//!
////////////////////////////////////////////////////////////////////////////////

// Standard library imports.
use std::mem;
use std::ptr;


////////////////////////////////////////////////////////////////////////////////
// Split
////////////////////////////////////////////////////////////////////////////////
/// A type which may contain zero, one, or two of a value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) enum Split<T> {
	/// No value present.
	Zero,
	/// One value present.
	One(T),
	/// Two values present.
	Two(T, T),
}

impl<T> Default for Split<T> {
	fn default() -> Self {
		Split::Zero
	}
}

impl<T> Iterator for Split<T> {
	type Item = T;

	fn next(&mut self) -> Option<T> {
		let mut res = None;
		replace_with(self, |curr|
			match curr {
				Split::Zero      => {res = None;    Split::Zero}
				Split::One(v)    => {res = Some(v); Split::Zero},
				Split::Two(a, b) => {res = Some(a); Split::One(b)},
			}
		);
		res
	}
}

impl<T> From<T> for Split<T> {
	fn from(value: T) -> Self {
		Split::One(value)
	}
}

impl<T> From<(T, T)> for Split<T> {
	fn from(value: (T, T)) -> Self {
		Split::Two(value.0, value.1)
	}
}



////////////////////////////////////////////////////////////////////////////////
// replace_with
////////////////////////////////////////////////////////////////////////////////
// TODO: Replace this with std::mem::replace_with if it ever becomes 
// available.
/// Temporarily takes ownership of a value at a mutable location, and replace 
/// it with a new value based on the old one.
///
/// We move out of reference temporarily, to apply a closure, returning a new
/// value, which is then placed at the original value's location.
///
/// # An important note
///
/// The behavior on panic (or to be more precise, unwinding) is specified to
/// match the behavior of panicking inside a destructor, which itself is
/// simply specified to not unwind.
#[inline]
fn replace_with<T, F>(val: &mut T, replace: F)
    where F: FnOnce(T) -> T {
    // Guard against unwinding. Note that this is critical to safety, to avoid
    // the value behind the reference `val` is not dropped twice during
    // unwinding.
    let guard = ExitGuard;

    unsafe {
        // Take out the value behind the pointer.
        let old = ptr::read(val);
        // Run the closure.
        let new = replace(old);
        // Put the result back.
        ptr::write(val, new);
    }

    // Forget the guard, to avoid panicking.
    mem::forget(guard);
}

/// A guarding type which will abort upon drop.
///
/// This is used for catching unwinding and transforming it into abort.
///
/// The destructor should never be called naturally (use `mem::forget()`), and
/// only when unwinding.
struct ExitGuard;

impl Drop for ExitGuard {
    fn drop(&mut self) {
        // To avoid unwinding, we abort (we panic, which is equivalent to abort
        // inside an unwinding destructor) the program, which ensures that the
        // destructor of the invalidated value isn't runned, since this
        // destructor ought to be called only if unwinding happens.
        panic!("`replace_with` closure unwind");
    }
}
