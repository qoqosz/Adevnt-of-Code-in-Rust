# Adevnt of Code in Rust 🎄

## Purpose

This repository contains solutions to the [Advent of Code](https://adventofcode.com/) programming puzzles, implemented in Rust :crab:. Advent of Code is a yearly set of coding challenges released daily throughout December, focusing on algorithmic and problem-solving skills.

The goals of this project are to:
- Practice and explore Rust programming techniques.
- Solve algorithmic challenges.
- Build an archive of solutions for reference and continuous learning.

## Directory Structure

```
.
├── aoc_derive/   # Crate with procedural macros used in the project
├── aoc_core/     # Extracted common utilities and helpers shared across solutions
├── aoc/          # Advent of Code solutions, organized by year and day
├── Cargo.toml    # Workspace manifest
└── README.md     # Project documentation
```

- `aoc_derive/`: Contains custom Rust procedural macros for use in solutions.
- `aoc_core/`: Provides shared types, functions, and utilities for parsing, input handling, etc.
- `aoc/`: Main crate with solutions to Advent of Code problems, organized by year and day.

## Requirements

- [Rust toolchain](https://www.rust-lang.org/tools/install) (recommended: latest stable version)
- An active Advent of Code account (to access puzzle inputs via session cookie)

## How to Run

1. **Store your Advent of Code session cookie in an environment variable:**

   ```bash
   export AOC_SESSION=531ab...    # Replace with your actual session token value
   ```

   This allows the project to fetch inputs automatically from the Advent of Code website.

2. **Run a solution for a given YEAR and DAY:**

   ```bash
   cargo run -- -y YEAR -d DAY
   ```

   - Replace `YEAR` with the relevant year (e.g., `2022`).
   - Replace `DAY` with the day number (e.g., `1`). Passing the `-d DAY` argument is optional. If omitted, all solutions for the specified year will be run.

   **Examples:**
   - Run the solution for Day 1 of 2022:
     ```bash
     cargo run -- -y 2022 -d 1
     ```
   - Run all solutions for the year 2022:
     ```bash
     cargo run -- -y 2022
     ```

3. **Test all solutions:**

   ```bash
   cargo test
   ```

## Progress

| Year      | Stars     |
|-----------|-----------|
| **2015**  | **50**⭐  |
| 2016      | 14⭐      |
| 2017      | 10⭐      |
| 2018      | 12⭐      |
| 2019      | 12⭐      |
| 2020      | 32⭐      |
| 2021      | 30⭐      |
| **2022**  | **50**⭐  |
| **2023**  | **50**⭐  |
| **2024**  | **50**⭐  |
| 2025      | 4⭐       |
| **Total** | **314**⭐ |

---

For details on individual solutions, see comments in the source files or puzzle documentation.
