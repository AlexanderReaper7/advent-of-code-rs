// Module for testing visualization
#[path = "y2024/d1_viz.rs"]
mod d1_viz;

use d1_viz::part1_visualized;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visualization_generation() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3".to_string();
        
        let result = part1_visualized(input);
        assert_eq!(result, "11");
        
        // Check that the trace file was created
        let trace_file = std::path::Path::new("traces/part1_visualized.json");
        assert!(trace_file.exists(), "Visualization trace file should be created");
        
        // Read and verify the trace contains expected data
        let trace_content = std::fs::read_to_string(trace_file).unwrap();
        assert!(trace_content.contains("Starting part1 execution"));
        assert!(trace_content.contains("Parsing input"));
        assert!(trace_content.contains("Sorting"));
        
        println!("Visualization trace generated successfully!");
        println!("View the trace at: traces/part1_visualized.json");
    }
}
