# AOC-LIB

aoc-lib is a simplified Rust library for interacting with [Advent of Code](https://adventofcode.com/) based on aoc-lib by panicbit.

It provides methods for getting your puzzle input and submitting solutions.

## Example

```rust
use aoc_lib::Client;

fn main() {
  let session_token = std::fs::read_to_string(".session").unwrap();
  let client = Client::new(session_token).unwrap();
  
  let input = client.get_input(2019, 1).unwrap();
  let result = client.submit_solution(2019, 1, 1, "solution").unwrap();
  println!("{}", result);
}
```´
