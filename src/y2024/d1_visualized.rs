use viz_lib::{VizContext, viz_track, viz_step, viz_operation};

/// Manually instrumented version of part1 for visualization
pub fn part1_visualized(input: String) -> String {
    let mut viz_ctx = VizContext::new("part1_visualized");
    
    viz_step!(viz_ctx, "Starting part1 execution");
    viz_track!(viz_ctx, input, "input");
    
    viz_step!(viz_ctx, "Parsing input into two columns");
    let (mut left_column, mut right_column) = parse_input_visualized(&input, &mut viz_ctx);
    
    viz_track!(viz_ctx, left_column, "left_column (unsorted)");
    viz_track!(viz_ctx, right_column, "right_column (unsorted)");
    
    viz_step!(viz_ctx, "Sorting left column");
    let left_before = left_column.clone();
    left_column.sort();
    viz_operation!(viz_ctx, "sort left_column", left_before, left_column);
    
    viz_step!(viz_ctx, "Sorting right column");
    let right_before = right_column.clone();
    right_column.sort();
    viz_operation!(viz_ctx, "sort right_column", right_before, right_column);
    
    viz_step!(viz_ctx, "Calculating total distance");
    let mut total_distance = 0;
    viz_track!(viz_ctx, total_distance, "total_distance (initial)");
    
    for (i, (right, left)) in right_column.iter().zip(left_column.iter()).enumerate() {
        let distance = (left - right).abs();
        total_distance += distance;
        
        if i < 5 { // Only track first few iterations to avoid spam
            viz_step!(viz_ctx, &format!("Iteration {}: left={}, right={}, distance={}, total={}", 
                i + 1, left, right, distance, total_distance));
        }
    }
    
    viz_track!(viz_ctx, total_distance, "total_distance (final)");
    viz_step!(viz_ctx, "Finished calculation");
    
    let result = total_distance.to_string();
    viz_track!(viz_ctx, result, "result");
    
    viz_ctx.finalize();
    result
}

fn parse_input_visualized(input: &str, viz_ctx: &mut VizContext) -> (Vec<i32>, Vec<i32>) {
    let mut left_column: Vec<i32> = Vec::new();
    let mut right_column: Vec<i32> = Vec::new();
    
    viz_step!(viz_ctx, "Creating empty vectors for columns");
    viz_track!(viz_ctx, left_column, "left_column (empty)");
    viz_track!(viz_ctx, right_column, "right_column (empty)");
    
    for (line_num, line) in input.lines().enumerate() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let left_value = parts[0].parse::<i32>().unwrap();
        let right_value = parts[1].parse::<i32>().unwrap();
        
        left_column.push(left_value);
        right_column.push(right_value);
        
        if line_num < 3 { // Only track first few lines
            viz_step!(viz_ctx, &format!("Parsed line {}: {} -> left={}, right={}", 
                line_num + 1, line.trim(), left_value, right_value));
        }
    }
    
    viz_track!(viz_ctx, left_column, "left_column (populated)");
    viz_track!(viz_ctx, right_column, "right_column (populated)");
    
    (left_column, right_column)
}
