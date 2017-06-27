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
//! Provides an interval type for doing complex set selections.
//!
////////////////////////////////////////////////////////////////////////////////

// Local imports.
use interval::{
	Interval,
	Normalize,
};
use selection::Selection;


////////////////////////////////////////////////////////////////////////////////
// SelectionElement
////////////////////////////////////////////////////////////////////////////////
/// Provides functions for parsing a type. Used by `Selection<T>` to parse the
/// intervals and elements of the selection.
pub trait SelectionElement where Self: Sized + PartialOrd + Ord + Clone {
	/// Parses the given string into a `Self`.
	///
	/// # Errors
	///
	/// Returns a `ParseError` if the string cannot be parsed.
	fn parse(text: &str) -> Result<Self, ParseError> {
		consume(Self::parse_element, &mut &*text)
	}

	/// Parses a prefix of the given string into a `Self`, shifting the input
	/// reference to the remainder of the unparsed portion of the string.
	///
	/// # Errors
	///
	/// Returns a `ParseError` if the string cannot be parsed.
	fn parse_element<'t>(text: &mut &'t str) -> Result<Self, ParseError<'t>>;
	
	/// Parses a prefix of the given string if it matches the interval range
	/// seperator for this `SelectionElement`. The input reference is shifted
	/// to the remainder of the unparsed portion of the string.
	///
	/// # Errors
	///
	/// Returns a `ParseError` if the string cannot be parsed.
	fn parse_interval_range_seperator<'t>(mut text: &mut &'t str)
		-> Result<(), ParseError<'t>> 
	{
		let mut chars = text.char_indices();
		match chars.next() {
			Some((p, c)) if c == '-' => {
				*text = &text[clamp(p+c.len_utf8(), 0, text.len())..];
				Ok(())
			}
			Some((p, _)) => Err(ParseError::UnexpectedSymbol {
				expected: "'-'",
				found: &text[p..],
			}),
			None => Err(ParseError::UnexpectedEndOfStream),
		}
	}

	/// Parses a prefix of the given string if it matches the interval seperator
	/// for this `SelectionElement`. The input reference is shifted  to the
	/// remainder of the unparsed portion of the string.
	///
	/// # Errors
	///
	/// Returns a `ParseError` if the string cannot be parsed.
	fn parse_interval_seperator<'t>(mut text: &mut &'t str)
		-> Result<(), ParseError<'t>> 
	{
		let mut chars = text.char_indices();
		match chars.next() {
			Some((p, c)) if c == ',' => {
				*text = &text[clamp(p+c.len_utf8(), 0, text.len())..];
				Ok(())
			}
			Some((p, _)) => Err(ParseError::UnexpectedSymbol {
				expected: "','",
				found: &text[p..]
			}),
			None => Err(ParseError::UnexpectedEndOfStream),
		}
	}
}



////////////////////////////////////////////////////////////////////////////////
// ParseError
////////////////////////////////////////////////////////////////////////////////
/// A representation of an error occurring during `Selection` and 
/// `SelectionElement` parsing.
#[derive(Debug, PartialEq, Eq)]
pub enum ParseError<'t> {
	/// The end of the input stream was reached before the parse was complete.
	UnexpectedEndOfStream,
	/// A symbol was encountered which could not be parsed.
	UnexpectedSymbol {
		/// The symbol or pattern that was expected.
		expected: &'static str,
		/// The text that failed to parse.
		found: &'t str,
	},
}

////////////////////////////////////////////////////////////////////////////////
// Selection parser
////////////////////////////////////////////////////////////////////////////////
// Adds parsing functions to `Selection`.
impl<T> Selection<T> where T: SelectionElement + Normalize {
	/// Parses the given string into a `Selection`.
	///
	/// # Errors
	///
	/// Returns a `ParseError` if the string cannot be parsed.
	pub fn parse(text: &str) -> Result<Self, ParseError> {
		consume(Self::parse_selection, &mut &*text)
	}

	/// Parses a prefix of the given string into a `Selection`, shifting the 
	/// input reference to the remainder of the unparsed portion of the string.
	///
	/// # Errors
	///
	/// Returns a `ParseError` if the string cannot be parsed.
	pub fn parse_selection<'t>(mut text: &mut &'t str)
		-> Result<Self, ParseError<'t>> 
	{
		let mut intervals = Vec::new();

		skip_all(parse_whitespace, text);
		intervals.push(Self::parse_interval(text)?);
		skip_all(parse_whitespace, text);

		while maybe(T::parse_interval_seperator, text).is_ok() {
			skip_all(parse_whitespace, text);
			intervals.push(Self::parse_interval(text)?);
			skip_all(parse_whitespace, text);
		}

		Ok(Selection::from_intervals(intervals))
	}

	/// Parses a prefix of the given string into an `Interval`, shifting the 
	/// input reference to the remainder of the unparsed portion of the string.
	///
	/// # Errors
	///
	/// Returns a `ParseError` if the string cannot be parsed.
	pub fn parse_interval<'t>(mut text: &mut &'t str)
		-> Result<Interval<T>, ParseError<'t>> 
	{
		let left = T::parse_element(text)?;

		skip_all(parse_whitespace, text);
		if maybe(T::parse_interval_range_seperator, text).is_ok() {
			skip_all(parse_whitespace, text);
			let right = T::parse_element(text)?;

			Ok(Interval::closed(left, right))
		} else {
			Ok(Interval::from(left))
		}
	}
}


////////////////////////////////////////////////////////////////////////////////
// Parser support functions.
////////////////////////////////////////////////////////////////////////////////

/// Parses a prefix of the given string if it is whitespace, shifting the 
/// input reference to the remainder of the unparsed portion of the string.
///
/// # Errors
///
/// Returns a `ParseError` if the string cannot be parsed.
pub fn parse_whitespace<'t>(mut text: &mut &'t str)
	-> Result<(), ParseError<'t>> 
{
	let mut chars = text.char_indices();
	match chars.next() {
		Some((p, c)) if c.is_whitespace() => {
			*text = &text[clamp(p+c.len_utf8(), 0, text.len())..];
			Ok(())
		}
		Some((p, _)) => Err(ParseError::UnexpectedSymbol {
			expected: "whitespace",
			found: &text[p..],
		}),
		None => Err(ParseError::UnexpectedEndOfStream),
	}
}

/// Clamps the given `val` between `low` and `high`.
fn clamp(val: usize, low: usize, high: usize) -> usize {
	if val < low { low } else if val > high { high } else { val }
}

/// A parser modifier which executes the given `parser` function on the given 
/// `text`, ensuring that current the parse position is maintained in case of a
/// failure.
///
/// # Errors
///
/// Returns a parse error if the given parse fails.
pub fn maybe<'t, T>(
	parser: fn(&mut &'t str) -> Result<T, ParseError<'t>>,
	mut text: &mut &'t str)
	-> Result<T, ParseError<'t>>
{
	let save = *text;
	(parser)(text).map_err(|e| { *text = &save; e })
}

/// A parser modifier which executes the given `parser` function on the given 
/// `text`, skipping past any number of successful parses and shifting the input
/// reference to the remainder of the unparsed portion of the string. Returns
/// the number of successful parses skipped.
pub fn skip_all<'t, T>(
	parser: fn(&mut &'t str) -> Result<T, ParseError<'t>>,
	mut text: &mut &'t str)
	-> usize
{
	let mut skips = 0;
	while let Ok(_) = maybe(parser, text) {
		skips += 1;
	}
	skips
}

/// A parser modifier which executes the given `parser` function on the given 
/// `text`, skipping past at most the given number of successful parses and 
/// shifting the input reference to the remainder of the unparsed portion of the
/// string. Returns the number of successful parses skipped.
pub fn skip_n<'t, T>(
	parser: fn(&mut &'t str) -> Result<T, ParseError<'t>>,
	mut text: &mut &'t str,
	skip_count: usize)
	-> usize
{
	let mut skips = 0;
	while let Ok(_) = maybe(parser, text) {
		skips += 1;
		if skips >= skip_count { break; }
	}
	skips
}

/// A parser modifier which executes the given `parser` function on the given 
/// `text`, ensuring that the entire string in consumed by the parse.
///
/// # Errors
///
/// Returns a parse error if the given parse fails, or if the entire input
/// string is not consumed.
pub fn consume<'t, T>(
	parser: fn(&mut &'t str) -> Result<T, ParseError<'t>>,
	mut text: &mut &'t str)
	-> Result<T, ParseError<'t>>
{
	let res = maybe(parser, text)?;
	if text.len() == 0 {
		Ok(res)
	} else {
		Err(ParseError::UnexpectedSymbol {
			expected: "end of stream",
			found: *text,
		})
	}
}



impl SelectionElement for usize {
	fn parse_element<'t>(mut text: &mut &'t str)
		-> Result<Self, ParseError<'t>> 
	{
		if text.len() == 0 { return Err(ParseError::UnexpectedEndOfStream); }

		let digits = text.as_bytes();
		let mut idx = 0;
		
		let mut res = match (digits[0] as char).to_digit(10) {
			Some(x) => x as usize,
			None => return Err(ParseError::UnexpectedSymbol {
				expected: "digit",
				found: &text[idx..],
			}),
		};
		idx += 1;

		for &c in &digits[1..] {
			let x = match (c as char).to_digit(10) {
				Some(x) => x as usize,
				None => break,
			};
			let mut res_new = match res.checked_mul(10) {
				Some(n) => n,
				None => break,
			};
			res_new = match res_new.checked_add(x) {
				Some(n) => n,
				None => break,
			};
			res = res_new;
			idx += 1;
		}

		*text = &text[clamp(idx, 0, text.len())..];
		Ok(res)
	}
}


