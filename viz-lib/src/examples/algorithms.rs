// filepath: c:\Projects\advent-of-code-rs\viz-lib\src\examples\algorithms.rs
// Example algorithms showing different visualization approaches

use crate::VizContext;

/// Example 1: Fibonacci with manual instrumentation
/// This allows you to track every step of the algorithm
pub fn fibonacci_manual(n: u32) -> u64 {
    let mut viz_ctx = VizContext::new("fibonacci_manual");
    
    viz_ctx.add_step(&format!("Computing fibonacci({}) manually", n));
    viz_ctx.track_value("n", &n.to_string());
    
    if n <= 1 {
        viz_ctx.add_step("Base case: n <= 1");
        viz_ctx.track_value("result", &(n as u64).to_string());
        viz_ctx.finalize();
        return n as u64;
    }
    
    viz_ctx.add_step("Recursive case: computing iteratively for visualization");
    
    let mut a = 0u64;
    let mut b = 1u64;
    viz_ctx.track_value("fib(0)", &a.to_string());
    viz_ctx.track_value("fib(1)", &b.to_string());
    
    for i in 2..=n {
        let next = a + b;
        viz_ctx.add_step(&format!("Computing fib({}) = fib({}) + fib({})", i, i-2, i-1));
        viz_ctx.track_value(&format!("fib({})", i), &next.to_string());
        
        a = b;
        b = next;
    }
    
    viz_ctx.add_step("Fibonacci computation complete");
    viz_ctx.track_value("final_result", &b.to_string());
    viz_ctx.finalize();
    
    b
}

/// Example 2: Algorithm visualization - Binary Search
pub fn binary_search_visualized(arr: &[i32], target: i32) -> Option<usize> {
    let mut viz_ctx = VizContext::new("binary_search_visualized");
    
    viz_ctx.add_step("Starting binary search");
    viz_ctx.track_value("array", &format!("{:?}", arr));
    viz_ctx.track_value("target", &target.to_string());
    
    let mut left = 0;
    let mut right = arr.len();
    
    while left < right {
        let mid = left + (right - left) / 2;
        viz_ctx.add_step(&format!("Checking middle element at index {}", mid));
        viz_ctx.track_value("left", &left.to_string());
        viz_ctx.track_value("right", &right.to_string());
        viz_ctx.track_value("mid", &mid.to_string());
        viz_ctx.track_value("mid_value", &arr[mid].to_string());
        
        if arr[mid] == target {
            viz_ctx.add_step(&format!("Found target {} at index {}", target, mid));
            viz_ctx.track_value("result", &format!("Some({})", mid));
            viz_ctx.finalize();
            return Some(mid);
        } else if arr[mid] < target {
            viz_ctx.add_step(&format!("{} < {}, searching right half", arr[mid], target));
            left = mid + 1;
        } else {
            viz_ctx.add_step(&format!("{} > {}, searching left half", arr[mid], target));
            right = mid;
        }
    }
    
    viz_ctx.add_step("Target not found");
    viz_ctx.track_value("result", "None");
    viz_ctx.finalize();
    
    None
}

/// Demo function for fibonacci algorithms
pub fn fibonacci_demo() {
    println!("🔢 Running Fibonacci Algorithm Demo");
    
    let n = 10;
    println!("Computing fibonacci({}) with manual instrumentation", n);
    let result = fibonacci_manual(n);
    println!("Result: {}", result);
    println!("✅ Trace saved to: traces/fibonacci_manual.json");
}

/// Demo function for binary search
pub fn binary_search_demo() {
    println!("🔍 Running Binary Search Demo");
    
    let arr = vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
    let target = 7;
    
    println!("Searching for {} in {:?}", target, arr);
    let result = binary_search_visualized(&arr, target);
    println!("Result: {:?}", result);
    println!("✅ Trace saved to: traces/binary_search_visualized.json");
    
    // Search for a value that doesn't exist
    let target2 = 8;
    println!("\nSearching for {} in {:?}", target2, arr);
    let result2 = binary_search_visualized(&arr, target2);
    println!("Result: {:?}", result2);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_fibonacci_manual() {
        let result = fibonacci_manual(7);
        assert_eq!(result, 13);
        
        println!("Fibonacci manual test completed!");
        println!("Check trace: traces/fibonacci_manual.json");
    }
    
    #[test]
    fn test_binary_search_example() {
        let arr = vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
        let result = binary_search_visualized(&arr, 7);
        assert_eq!(result, Some(3));
        
        let result2 = binary_search_visualized(&arr, 8);
        assert_eq!(result2, None);
        
        println!("Binary search example completed!");
        println!("Check trace: traces/binary_search_visualized.json");
    }
}
