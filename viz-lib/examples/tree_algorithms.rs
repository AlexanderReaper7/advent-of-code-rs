// Tree algorithm examples using the visualization library
// Run with: cargo run --example tree_algorithms

use viz_lib::examples::trees::{bst_demo, manual_tree_demo, heap_demo};

fn main() {
    println!("🌳 Running Tree Algorithm Visualizations");
    println!("========================================");
    
    // Binary Search Tree Demo
    println!("\n1. Binary Search Tree Operations");
    println!("   Operations: Insert [50, 30, 70, 20, 40, 60, 80], Search 40, Search 65");
    bst_demo();
    println!("   ✅ Trace saved to: traces/bst_demo.json");
    
    // Manual Tree Construction
    println!("\n2. Manual Binary Tree Construction");
    println!("   Creating a binary tree with root=10, demonstrating traversal");
    manual_tree_demo();
    println!("   ✅ Trace saved to: traces/manual_tree_demo.json");
    
    // Max Heap Visualization
    println!("\n3. Max Heap Visualization");
    println!("   Heap values: [100, 85, 90, 70, 80, 75, 60]");
    heap_demo();
    println!("   ✅ Trace saved to: traces/heap_demo.json");
    
    // Run individual tests to show different tree operations
    println!("\n4. Running additional tree tests...");
    
    // Test BST operations in detail
    use viz_lib::examples::trees::*;
    let mut ctx = viz_lib::VizContext::new("detailed_bst_test");
    let mut bst = BST::<i32>::new();
    
    println!("   Building BST with detailed steps...");
    let values = vec![25, 15, 35, 10, 20, 30, 40];
    for value in values {
        bst.insert(value, &mut ctx);
    }
    
    // Search operations
    println!("   Searching for values in BST...");
    let found = bst.search(20, &mut ctx);
    println!("   Found 20: {}", found);
    
    let not_found = bst.search(99, &mut ctx);
    println!("   Found 99: {}", not_found);
    
    // Traversal
    println!("   Performing in-order traversal...");
    let traversal = bst.inorder_traversal(&mut ctx);
    println!("   In-order traversal: {:?}", traversal);
    
    ctx.finalize();
    println!("   ✅ Trace saved to: traces/detailed_bst_test.json");
    
    println!("\n🎯 All tree algorithm examples completed!");
    println!("📊 Tree visualizations showcase:");
    println!("   • Binary Search Tree insertion, search, and traversal");
    println!("   • Manual tree construction with node highlighting");
    println!("   • Max heap structure and parent-child comparisons");
    println!("   • Tree traversal path visualization");
    println!("\n📊 Start the visualization server to view the traces:");
    println!("   cargo run --bin viz-server --features server");
    println!("   Then open: http://localhost:3030");
}
