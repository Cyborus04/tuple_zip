# `tuple_zip`

A crate for converting a tuple of iterators into an iterator of tuples

```rust
use tuple_zip::TupleZip;

let a = [1, 2, 3, 4];
let b = [5, 6, 7];
let c = ["x", "y", "z"];

let zipped = (a, b, c).tuple_zip();

let expected = [(1, 5, "x"), (2, 6, "y"), (3, 7, "z")];
assert!(zipped.eq(expected));
```

Licensed under [Apache-2.0](http://www.apache.org/licenses/LICENSE-2.0) or
[MIT](http://opensource.org/licenses/MIT), at your choice
