# spotcheck

Simple program to check Amazon ASIN sales ranking.

**What is ASIN?**

https://www.amazon.com/gp/seller/asin-upc-isbn-info.html

## How to build and use

Requirements:

- Clone repository with `git`
- Install rust tool chain: https://rustup.rs/

```
# compile
cargo build

# run with a sample ASIN number B00C7C67O2
./target/debug/spotcheck B00C7C67O2
```

## Tests

```
cargo test
cargo test -- --nocapture
```
