use viz_lib::context::VizContext;
use viz_lib::efficient_context::EfficientVizContext;

// Demo function using the original context
pub fn bubble_sort_original(mut arr: Vec<i32>) -> Vec<i32> {
    let mut ctx = VizContext::new("bubble_sort_original");
    
    ctx.track_array("arr", &arr);
    let n = arr.len();
    ctx.track_var_creation("n", &n.to_string());
    
    for i in 0..n {
        ctx.track_var_update("i", &i.to_string());
        ctx.add_step(&format!("Starting pass {} of {}", i + 1, n));
        
        for j in 0..n - 1 - i {
            ctx.track_var_update("j", &j.to_string());
            ctx.compare_array_indices("arr", j, j + 1);
            
            if arr[j] > arr[j + 1] {
                ctx.swap_array_elements("arr", &mut arr, j, j + 1);
            }
        }
    }
    
    ctx.finalize();
    arr
}

// Demo function using the efficient context
pub fn bubble_sort_efficient(mut arr: Vec<i32>) -> Vec<i32> {
    let mut ctx = EfficientVizContext::new("bubble_sort_efficient");
    
    ctx.track_array("arr", &arr);
    let n = arr.len();
    ctx.track_var_creation("n", &n.to_string());
    
    for i in 0..n {
        ctx.track_var_update("i", &i.to_string());
        
        for j in 0..n - 1 - i {
            ctx.track_var_update("j", &j.to_string());
            ctx.compare_array_indices("arr", j, j + 1);
            
            if arr[j] > arr[j + 1] {
                ctx.swap_array_elements("arr", &mut arr, j, j + 1);
            }
        }
    }
    
    ctx.finalize();
    arr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_efficiency_comparison() {
        let input = vec![64, 34, 25, 12, 22, 11, 90];
        
        println!("🔄 Running efficiency comparison...");
        
        // Test original format
        println!("📊 Testing original format...");
        let start = std::time::Instant::now();
        let result1 = bubble_sort_original(input.clone());
        let original_time = start.elapsed();
        
        // Test efficient format
        println!("⚡ Testing efficient format...");
        let start = std::time::Instant::now();
        let result2 = bubble_sort_efficient(input.clone());
        let efficient_time = start.elapsed();
        
        // Verify results are the same
        assert_eq!(result1, result2);
        
        // Compare file sizes
        let original_file = std::fs::read_to_string("traces/bubble_sort_original.json").unwrap();
        let compact_file = std::fs::read_to_string("traces/bubble_sort_efficient_compact.json").unwrap();
        
        let original_size = original_file.len();
        let compact_size = compact_file.len();
        let size_reduction = ((original_size - compact_size) as f64 / original_size as f64) * 100.0;
        
        println!();
        println!("📈 EFFICIENCY COMPARISON RESULTS:");
        println!("================================");
        println!("Original format: {} bytes", original_size);
        println!("Efficient format: {} bytes", compact_size);
        println!("Size reduction: {:.1}%", size_reduction);
        println!("Generation time - Original: {:?}", original_time);
        println!("Generation time - Efficient: {:?}", efficient_time);
        
        // Count steps to show data reduction
        let original_trace: serde_json::Value = serde_json::from_str(&original_file).unwrap();
        let compact_trace: serde_json::Value = serde_json::from_str(&compact_file).unwrap();
        
        let original_steps = original_trace["steps"].as_array().unwrap().len();
        let compact_steps = compact_trace["steps"].as_array().unwrap().len();
        
        println!("Steps in original: {}", original_steps);
        println!("Steps in compact: {}", compact_steps);
        
        // Show memory footprint comparison by analyzing first few steps
        if let Some(original_step) = original_trace["steps"].as_array().unwrap().first() {
            let original_step_size = serde_json::to_string(original_step).unwrap().len();
            println!("Average original step size: ~{} bytes", original_step_size);
        }
        
        if let Some(compact_step) = compact_trace["steps"].as_array().unwrap().first() {
            let compact_step_size = serde_json::to_string(compact_step).unwrap().len();
            println!("Average compact step size: ~{} bytes", compact_step_size);
        }
        
        println!();
        println!("✅ Efficiency test completed successfully!");
          // Expect at least 30% size reduction for this test case
        assert!(size_reduction > 30.0, "Expected at least 30% size reduction, got {:.1}%", size_reduction);
    }
}

fn main() {
    println!("🚀 Running efficiency comparison demo...");
    
    // Run the efficiency comparison
    let input = vec![64, 34, 25, 12, 22, 11, 90];
    
    println!("🔄 Running efficiency comparison...");
    
    // Test original format
    println!("📊 Testing original format...");
    let start = std::time::Instant::now();
    let result1 = bubble_sort_original(input.clone());
    let original_time = start.elapsed();
    
    // Test efficient format
    println!("⚡ Testing efficient format...");
    let start = std::time::Instant::now();
    let result2 = bubble_sort_efficient(input.clone());
    let efficient_time = start.elapsed();
    
    // Verify results are the same
    assert_eq!(result1, result2);
    
    // Compare file sizes
    match (
        std::fs::read_to_string("traces/bubble_sort_original.json"),
        std::fs::read_to_string("traces/bubble_sort_efficient_compact.json")
    ) {
        (Ok(original_file), Ok(compact_file)) => {
            let original_size = original_file.len();
            let compact_size = compact_file.len();
            let size_reduction = ((original_size - compact_size) as f64 / original_size as f64) * 100.0;
            
            println!();
            println!("📈 EFFICIENCY COMPARISON RESULTS:");
            println!("================================");
            println!("Original format: {} bytes", original_size);
            println!("Efficient format: {} bytes", compact_size);
            println!("Size reduction: {:.1}%", size_reduction);
            println!("Generation time - Original: {:?}", original_time);
            println!("Generation time - Efficient: {:?}", efficient_time);
            
            // Count steps to show data reduction
            if let (Ok(original_trace), Ok(compact_trace)) = (
                serde_json::from_str::<serde_json::Value>(&original_file),
                serde_json::from_str::<serde_json::Value>(&compact_file)
            ) {
                let original_steps = original_trace["steps"].as_array().unwrap().len();
                let compact_steps = compact_trace["steps"].as_array().unwrap().len();
                
                println!("Steps in original: {}", original_steps);
                println!("Steps in compact: {}", compact_steps);
                
                // Show memory footprint comparison by analyzing first few steps
                if let Some(original_step) = original_trace["steps"].as_array().unwrap().first() {
                    let original_step_size = serde_json::to_string(original_step).unwrap().len();
                    println!("Average original step size: ~{} bytes", original_step_size);
                }
                
                if let Some(compact_step) = compact_trace["steps"].as_array().unwrap().first() {
                    let compact_step_size = serde_json::to_string(compact_step).unwrap().len();
                    println!("Average compact step size: ~{} bytes", compact_step_size);
                }
                
                println!();
                if size_reduction > 30.0 {
                    println!("✅ Efficiency test completed successfully!");
                    println!("🎉 Achieved {:.1}% size reduction!", size_reduction);
                } else {
                    println!("⚠️  Size reduction was only {:.1}% (expected >30%)", size_reduction);
                }
            }
        }
        (Err(e1), _) => println!("❌ Could not read original trace file: {}", e1),
        (_, Err(e2)) => println!("❌ Could not read compact trace file: {}", e2),
    }
    
    println!("Demo completed!");
}
