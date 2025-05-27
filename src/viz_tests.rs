use crate::y2024::d1_viz::part1_visualized;

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
    assert!(std::path::Path::new("traces/part1_visualized.json").exists());
}
