# aoc-auto

### *Simplifies and speeds up the process of starting a new advent of code challenge.*

This is a build script that generates files that automatically generates rust files that imports you solutions for each of the year/day/part according to a predetermined project structure.

## Usage

### Installation

Run the `cargo add aoc-auto --build` to add it to your project´s build dependencies.

Then add the following to your `build.rs` file:

```rust
// build.rs
use aoc_auto::aoc_auto;
fn main() {
   aoc_auto();
}
```

### Project Structure

The project structure you need to use folders named `y{year}` containing files named `d{day}.rs` for each day of that year´s AOC challenge.
An example of this is as follows:

```text
├── Cargo.toml
├── build.rs
└── src
    ├── y2023
    │   ├── d1.rs
    │   └── d2.rs
    ├── y2024
    │   ├── d1.rs
    │   └── d2.rs
    └── main.rs
```

This will turn into the following:

```text
├── Cargo.toml
├── build.rs
└── src
    ├── y2023
    │   ├── d1.rs
    │   ├── d2.rs
    │   └── mod.rs
    ├── y2024
    │   ├── d1.rs
    │   ├── d2.rs
    │   └── mod.rs
    ├── main.rs
    └── auto_import.rs
```

The `d1.rs` (and any other day) must contain two functions: `part1` and `part2` that take a `String` and return a `String`.

```rust
// src/y2023/d1.rs
pub fn part1(input: String) -> String {
    unimplemented!()
}

pub fn part2(input: String) -> String {
    unimplemented!()
}
```

### Running

In your `main.rs` file, you can now import the generated `auto_import` file and use the `select_function` function to select your chosen solution.

```rust
// src/main.rs
mod auto_import;

fn main() {
    // Some example input
    let input = "challenge input"
    // Select the function for year 2023, day 1, part 1
    let function = auto_import::select_function(2023, 1, 1).unwrap();
    // Call the function with the input and print the result
    println!("{}", function(input));
}
```
