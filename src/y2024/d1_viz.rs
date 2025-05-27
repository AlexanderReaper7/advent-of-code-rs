//! --- Day 1: Historian Hysteria --- (Visualization Demo)
use viz_lib::VizContext;

pub fn part1_visualized(input: String) -> String {
    let mut viz_ctx = VizContext::new("part1_visualized");
    
    viz_ctx.add_step("Starting part1 execution");
    viz_ctx.track_value("input", &format!("String with {} characters", input.len()));
    
    viz_ctx.add_step("Parsing input into two columns");
    let (mut left_column, mut right_column) = parse_input_visualized(&input, &mut viz_ctx);
    
    viz_ctx.track_value("left_column", &format!("{:?}", left_column));
    viz_ctx.track_value("right_column", &format!("{:?}", right_column));
    viz_ctx.add_step(&format!("Parsed {} pairs of numbers", left_column.len()));
    
    viz_ctx.add_step("Sorting left column");
    left_column.sort();
    viz_ctx.track_value("left_column (sorted)", &format!("{:?}", left_column));
    
    viz_ctx.add_step("Sorting right column");
    right_column.sort();
    viz_ctx.track_value("right_column (sorted)", &format!("{:?}", right_column));
    
    viz_ctx.add_step("Calculating distances between paired elements");
    let mut total_distance = 0;
    for (i, (right, left)) in right_column.iter().zip(left_column.iter()).enumerate() {
        let distance = (left - right).abs();
        total_distance += distance;
        viz_ctx.add_step(&format!("Pair {}: |{} - {}| = {}, running total = {}", i + 1, left, right, distance, total_distance));
        viz_ctx.track_value("total_distance", &total_distance.to_string());
    }
    
    viz_ctx.add_step(&format!("Final result: {}", total_distance));
    viz_ctx.finalize();
    
    total_distance.to_string()
}

fn parse_input_visualized(input: &str, viz_ctx: &mut VizContext) -> (Vec<i32>, Vec<i32>) {
    let mut left_column: Vec<i32> = Vec::new();
    let mut right_column: Vec<i32> = Vec::new();
    
    viz_ctx.add_step("Processing each line of input");
    
    for (line_num, line) in input.lines().enumerate() {
        viz_ctx.add_step(&format!("Processing line {}: '{}'", line_num + 1, line));
        
        let parts: Vec<&str> = line.split_whitespace().collect();
        let left_value = parts[0].parse::<i32>().unwrap();
        let right_value = parts[1].parse::<i32>().unwrap();
        
        left_column.push(left_value);
        right_column.push(right_value);
        
        viz_ctx.add_step(&format!("Added {} to left column, {} to right column", left_value, right_value));
        viz_ctx.track_value("left_column", &format!("{:?}", left_column));
        viz_ctx.track_value("right_column", &format!("{:?}", right_column));
    }
    
    (left_column, right_column)
}
