//! This library provides a `Range` trait to that unifies the ranges
//! `std::ops::Range`, `std::ops::RangeFrom`, `std::ops::RangeTo`
//! and `std::ops::RangeFull`. It offers a conversion function that
//! 
//! for converting artibrary kinds of
//! ranges to "concrete" ranges that have a number for start/end.

use std::ops;

pub trait AnyRange<Index>: Sized {

    /// returns `Sone(s)` if a start has been specified
    fn start(&self) -> Option<Index>;

    /// returns `Sone(s)` if an end has been specified
    fn end(&self) -> Option<Index>;

    /// converts this range into a concrete `std::ops::Range<usize>`
    /// by defaulting non-existant bounds to the given start/end values
    fn to_range(&self, start: Index, end: Index) -> ops::Range<Index> {
        ops::Range {
            start: self.start().unwrap_or(start),
            end: self.end().unwrap_or(end),
        }
    }
}

impl<I: Clone> AnyRange<I> for ops::Range<I> {

    fn start(&self) -> Option<I> { Some(self.start.clone()) }

    fn end(&self) -> Option<I> { Some(self.end.clone()) }

    fn to_range(&self, _: I, _: I) -> ops::Range<I> { self.clone() }
}

impl<I: Clone> AnyRange<I> for ops::RangeFrom<I> {

    fn start(&self) -> Option<I> { Some(self.start.clone()) }

    fn end(&self) -> Option<I> { None }
}

impl<I: Clone> AnyRange<I> for ops::RangeTo<I> {

    fn start(&self) -> Option<I> { None }

    fn end(&self) -> Option<I> { Some(self.end.clone()) }
}

impl<I> AnyRange<I> for ops::RangeFull {

    fn start(&self) -> Option<I> { None }

    fn end(&self) -> Option<I> { None }

    fn to_range(&self, start: I, end: I) -> ops::Range<I> {
        ops::Range { start: start, end: end }
    }
}

#[test]
fn it_works() {
    assert!( (10..20).to_range(0,100) == (10.. 20) );
    assert!( (10..  ).to_range(0,100) == (10..100) );
    assert!( (  ..20).to_range(0,100) == ( 0.. 20) );
    assert!( (  ..  ).to_range(0,100) == ( 0..100) );
}

