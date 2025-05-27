# Viz-Lib: Code Execution Visualization Library

A comprehensive Rust library for creating step-by-step visualizations of algorithm execution. Perfect for educational purposes, debugging, and understanding complex algorithms.

## Features

- 🎯 **Manual Instrumentation**: Full control over what gets visualized
- 🤖 **Procedural Macro Support**: Automatic instrumentation (with viz-proc-macro)
- 🌐 **Web Interface**: Built-in web server for viewing visualizations
- 📊 **Rich Algorithm Examples**: Sorting, trees, pathfinding, and more
- 🎨 **Multiple Visualization Types**: Arrays, trees, graphs, and custom data structures

## Quick Start

### 1. Add to your Cargo.toml

```toml
[dependencies]
viz-lib = { path = "../viz-lib" }

# Optional: for web server functionality
[features]
server = ["viz-lib/server"]
```

### 2. Basic Usage

```rust
use viz_lib::VizContext;

fn bubble_sort_visualized(mut arr: Vec<i32>) -> Vec<i32> {
    let mut ctx = VizContext::new("bubble_sort");
    
    ctx.add_step("Starting bubble sort");
    ctx.track_value("initial_array", &format!("{:?}", arr));
    
    let n = arr.len();
    for i in 0..n {
        ctx.add_step(&format!("Pass {} of bubble sort", i + 1));
        
        for j in 0..n - 1 - i {
            if arr[j] > arr[j + 1] {
                ctx.add_step(&format!("Swapping {} and {}", arr[j], arr[j + 1]));
                arr.swap(j, j + 1);
                ctx.track_value("array", &format!("{:?}", arr));
            }
        }
    }
    
    ctx.add_step("Sort completed");
    ctx.finalize();
    arr
}
```

### 3. Run Examples

```bash
# Run sorting algorithms examples
cargo run --example sorting_algorithms

# Run tree algorithms examples  
cargo run --example tree_algorithms

# Run pathfinding algorithms examples
cargo run --example pathfinding_algorithms

# Run basic algorithm examples (fibonacci, binary search)
cargo run --example algorithm_examples

# Run procedural macro examples
cargo run --example proc_macro_examples
```

### 4. Start Web Server

```bash
# Start the visualization server
cargo run --bin viz-server --features server

# View visualizations at: http://localhost:3031
```

## Available Examples

### Sorting Algorithms
- **Enhanced Bubble Sort**: Step-by-step sorting with comparisons
- **Enhanced Selection Sort**: Minimum element selection visualization
- **Binary Search**: Search visualization with left/right boundaries

### Tree Algorithms  
- **Binary Search Tree**: Insert, search, and traversal operations
- **Manual Tree Construction**: Building trees with visualization
- **Max Heap**: Heap operations and structure visualization

### Pathfinding Algorithms
- **BFS (Breadth-First Search)**: Maze pathfinding with queue visualization
- **Dijkstra's Algorithm**: Shortest path with distance tracking
- **A* Algorithm**: Heuristic-based pathfinding with cost visualization

### Basic Algorithms
- **Fibonacci**: Iterative computation with step tracking
- **Binary Search**: Array search with bounds visualization

### Procedural Macro Examples
- **Automatic Instrumentation**: Using `#[visualize]` macro
- **Manual vs Automatic**: Comparison of instrumentation approaches

## Library Structure

```
viz-lib/
├── src/
│   ├── lib.rs              # Main library exports
│   ├── context.rs          # VizContext and core types
│   ├── server.rs           # Web server implementation
│   ├── bin/
│   │   └── viz-server.rs   # Standalone server binary
│   └── examples/           # Algorithm implementations
│       ├── basic.rs        # Basic algorithm examples
│       ├── enhanced.rs     # Enhanced sorting algorithms
│       ├── trees.rs        # Tree data structure algorithms
│       ├── pathfinding.rs  # Pathfinding algorithms
│       ├── algorithms.rs   # Additional algorithm examples
│       └── proc_macro.rs   # Procedural macro demonstrations
├── examples/               # Runnable example files
├── web/                    # Web interface source files
├── viz-web/               # Generated web interface
└── traces/                # Generated visualization traces
```

## Usage Patterns

### Manual Instrumentation
```rust
use viz_lib::VizContext;

fn my_algorithm() {
    let mut ctx = VizContext::new("my_algorithm");
    ctx.add_step("Step description");
    ctx.track_value("variable_name", "value");
    ctx.finalize();
}
```

### Tree Visualization
```rust
use viz_lib::{VizContext, TreeNode};

let mut ctx = VizContext::new("tree_demo");
ctx.visualize_tree(&tree_root, "Binary Search Tree");
```

### Array Visualization
```rust
let mut ctx = VizContext::new("sorting");
ctx.visualize_array(&array, &[0, 1], &[(2, 3)], "Comparing elements");
```

## Development

### Running Tests
```bash
cargo test
```

### Building Documentation
```bash
cargo doc --open
```

### Checking Code Quality
```bash
cargo clippy
cargo fmt
```

## Integration with Main Projects

This library can be easily integrated into existing Rust projects:

1. Add viz-lib as a dependency
2. Import `VizContext` where needed  
3. Add visualization calls to your algorithms
4. Run with the web server to view results

## Performance Notes

- Visualization adds overhead - use conditionally in production
- Trace files can become large for complex algorithms
- Consider using feature flags to disable visualization in release builds

## License

This project is part of the advent-of-code-rs workspace and follows the same licensing terms.