// Sorting algorithm examples using the visualization library
// Run with: cargo run --example sorting_algorithms

fn main() {
    println!("🔄 Running Sorting Algorithm Visualizations");
    println!("===========================================");
    
    // Test basic functionality
    println!("\n1. Enhanced Bubble Sort");
    println!("   Input: [64, 34, 25, 12, 22, 11, 90]");
    let arr = vec![64, 34, 25, 12, 22, 11, 90];
    let sorted = viz_lib::examples::enhanced::enhanced_bubble_sort(arr);
    println!("   Output: {:?}", sorted);
    println!("   ✅ Trace saved to: traces/enhanced_bubble_sort.json");
    
    println!("\n🎯 Basic example completed!");
    println!("📊 Start the visualization server to view the traces:");
    println!("   cargo run --bin viz-server --features server");
    println!("   Then open: http://localhost:3030");
}
