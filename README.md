# `tuple_zip`

A crate for converting a tuple of iterators into an iterator of tuples

```rust
use tuple_zip::TupleZip;

let a = [1, 2, 3, 4];
let b = [5, 6, 7];

let zipped = (a, b).tuple_zip();

let expected = [(1, 5), (2, 6), (3, 7)];
assert!(zipped.eq(expected));
```

Licensed under [Apache-2.0](http://www.apache.org/licenses/LICENSE-2.0) or
[MIT](http://opensource.org/licenses/MIT), at your choice
