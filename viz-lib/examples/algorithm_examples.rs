// filepath: c:\Projects\advent-of-code-rs\viz-lib\examples\algorithm_examples.rs
// Runner for algorithm examples demonstrating fibonacci and binary search

use viz_lib::examples::algorithms::{fibonacci_demo, binary_search_demo};

fn main() {
    println!("🎯 Algorithm Examples Demo");
    println!("==========================");
    
    // Run fibonacci demo
    fibonacci_demo();
    
    println!();
    
    // Run binary search demo
    binary_search_demo();
    
    println!();
    println!("🎉 All algorithm examples completed!");
    println!("📁 Check the traces/ directory for generated visualization files");
    println!("🌐 View them at: http://localhost:3031 (start server with: cargo run --bin viz-server --features server)");
}
