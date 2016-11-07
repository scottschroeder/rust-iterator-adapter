# rust-iterator-adapter

Toy example of creating an iterator adapter.
This is a pattern I like, but it involves quite a few complex signatures.


The example will take a stream of objects, and output a single `Point` for every
two inputs.

```rust
let numbers: Vec<f64> = vec![1f64, 2f64, 3f64, 4f64, 5f64, 6f64, 7f64];
let points: Vec<Point<f64>> = numbers.into_iter().build_points().collect();
println!("{:?}", points);
```
Prints out
```
[Point { x: 1, y: 2 }, Point { x: 3, y: 4 }, Point { x: 5, y: 6 }]
```

_Note that the `7` did not get paired up and got dropped._

[View Source](src/main.rs)
