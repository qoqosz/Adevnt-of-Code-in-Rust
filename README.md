# Adevnt of Code in Rust 🎄
Solutions to Advent of Code puzzles in Rust :crab:.

## Usage

Store the value of the AoC session cookie in the `AOC_SESSION` env var:

```
export AOC_SESSION=531ab...
```

Run a solution for a given `YEAR` and `DAY`:

```
cargo run --bin aoc -y YEAR -d DAY
```

## Structure
- aoc_derive - proc macro
- aoc - solutions and helper utilities
