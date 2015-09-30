This library provides an `AnyRange` trait that unifies the ranges
`std::ops::Range`, `std::ops::RangeFrom`, `std::ops::RangeTo`
and `std::ops::RangeFull`. It offers a function for converting
artibrary kinds of ranges to "concrete" ranges that have a start
and an end.

# Examples

```rust
assert!( (10..20).to_range(0,100) == (10.. 20) );
assert!( (10..  ).to_range(0,100) == (10..100) );
assert!( (  ..20).to_range(0,100) == ( 0.. 20) );
assert!( (  ..  ).to_range(0,100) == ( 0..100) );
```
