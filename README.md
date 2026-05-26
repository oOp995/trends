# trends

Simple trend detection for Rust.

Compare two ordered values and classify the result as:

- `Trend::Rising`
- `Trend::Falling`
- `Trend::Stable`

## Example

```rust
use trends::TrendExt;

let trend = 10.to_trend(&20);

assert!(trend.is_rising());
assert_eq!(trend.direction(), 1);
```

Works with any `T: Ord`.

# Explore
- tests  - > [tests](/tests/trend_test.rs)
- documentation -> [docs](https://docs.rs/trends/0.1.1/trends/)
- source -> [src](https://docs.rs/crate/trends/0.1.1/source/)