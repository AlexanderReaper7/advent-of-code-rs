// filepath: c:\Projects\advent-of-code-rs\viz-lib\src\examples\proc_macro.rs
// Examples demonstrating procedural macro usage for automatic instrumentation

use crate::VizContext;

// Note: The #[visualize] macro would be available when viz-proc-macro is properly integrated
// For now, we'll implement manual versions to demonstrate the concept

/// Simple sort with automatic instrumentation (manual implementation)
/// In practice, this would use: #[visualize]
pub fn simple_sort_demo(input: Vec<i32>) -> Vec<i32> {
    let mut viz_ctx = VizContext::new("simple_sort_demo");
    
    viz_ctx.add_step("Function entry: simple_sort_demo");
    viz_ctx.track_value("input", &format!("{:?}", input));
    
    let mut data = input;
    data.sort();
    
    viz_ctx.track_value("result", &format!("{:?}", data));
    viz_ctx.add_step("Function exit: simple_sort_demo");
    viz_ctx.finalize();
    
    data
}

/// Manual instrumentation for comparison with detailed tracking
pub fn detailed_sort_demo(input: Vec<i32>) -> Vec<i32> {
    let mut viz_ctx = VizContext::new("detailed_sort_demo");
    
    viz_ctx.add_step("Starting detailed sort demonstration");
    viz_ctx.track_value("input", &format!("Initial array: {:?}", input));
    
    let mut data = input.clone();
    viz_ctx.track_value("data", &format!("Working array: {:?}", data));
    
    viz_ctx.add_step("Beginning bubble sort implementation");
    let n = data.len();
    
    for i in 0..n {
        viz_ctx.add_step(&format!("Pass {} of bubble sort", i + 1));
        let mut swapped = false;
        
        for j in 0..n - 1 - i {
            viz_ctx.track_value("comparing", &format!("Comparing {} and {}", data[j], data[j + 1]));
            
            if data[j] > data[j + 1] {
                viz_ctx.add_step(&format!("Swapping {} and {}", data[j], data[j + 1]));
                data.swap(j, j + 1);
                swapped = true;
                viz_ctx.track_value("data", &format!("After swap: {:?}", data));
            }
        }
        
        if !swapped {
            viz_ctx.add_step("Array is now sorted - early termination");
            break;
        }
    }
    
    viz_ctx.add_step("Sort completed");
    viz_ctx.track_value("result", &format!("Final sorted array: {:?}", data));
    viz_ctx.finalize();
    
    data
}

/// Demo function showing the difference between automatic and manual instrumentation
pub fn proc_macro_demo() {
    println!("🤖 Running Procedural Macro Demo");
    
    let input = vec![3, 1, 4, 1, 5, 9, 2, 6, 5];
    
    println!("Running simple sort with automatic instrumentation...");
    let result1 = simple_sort_demo(input.clone());
    println!("Result: {:?}", result1);
    println!("✅ Trace saved to: traces/simple_sort_demo.json");
    
    println!("\nRunning detailed sort with manual instrumentation...");
    let result2 = detailed_sort_demo(input);
    println!("Result: {:?}", result2);
    println!("✅ Trace saved to: traces/detailed_sort_demo.json");
    
    println!("\n💡 Compare the traces to see the difference between automatic and manual instrumentation!");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_proc_macro_visualization() {
        let input = vec![3, 1, 4, 1, 5, 9, 2, 6, 5];
        let result = simple_sort_demo(input.clone());
        
        // Check that it works correctly
        let mut expected = input;
        expected.sort();
        assert_eq!(result, expected);
        
        // Check that the trace file was created
        let trace_file = std::path::Path::new("traces/simple_sort_demo.json");
        assert!(trace_file.exists(), "Visualization trace file should be created");
        
        println!("Proc macro visualization trace generated successfully!");
        println!("View the trace at: traces/simple_sort_demo.json");
    }
    
    #[test]
    fn test_detailed_visualization() {
        let input = vec![64, 34, 25, 12, 22, 11, 90];
        let result = detailed_sort_demo(input.clone());
        
        // Check that it works correctly
        let mut expected = input;
        expected.sort();
        assert_eq!(result, expected);
        
        // Check that the trace file was created
        let trace_file = std::path::Path::new("traces/detailed_sort_demo.json");
        assert!(trace_file.exists(), "Detailed visualization trace file should be created");
        
        println!("Detailed visualization trace generated successfully!");
        println!("View the trace at: traces/detailed_sort_demo.json");
    }
}
