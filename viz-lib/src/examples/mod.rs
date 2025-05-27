// Examples module for the visualization library
// This module contains various algorithm examples that demonstrate
// how to use the visualization library for educational purposes

pub mod basic;
pub mod enhanced;
pub mod trees;
pub mod pathfinding;
pub mod algorithms;
pub mod proc_macro;

// Re-export demo functions for convenience
pub use trees::{bst_demo, manual_tree_demo, heap_demo};
pub use algorithms::{fibonacci_demo, binary_search_demo};
pub use proc_macro::{proc_macro_demo};
