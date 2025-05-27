// Pathfinding algorithm examples using the visualization library
// Run with: cargo run --example pathfinding_algorithms

use viz_lib::examples::pathfinding::{
    Cell, visualized_bfs_pathfinder, dijkstra_visualization, a_star_visualization
};

fn main() {
    println!("🗺️  Running Pathfinding Algorithm Visualizations");
    println!("===============================================");
    
    // BFS Pathfinding
    println!("\n1. BFS Pathfinding in a Grid Maze");
    println!("   Maze: 4x4 grid with walls, finding path from start to end");
    let mut maze = vec![
        vec![Cell::Start, Cell::Empty, Cell::Wall, Cell::Empty],
        vec![Cell::Empty, Cell::Empty, Cell::Wall, Cell::Empty],
        vec![Cell::Wall, Cell::Empty, Cell::Empty, Cell::Empty],
        vec![Cell::Empty, Cell::Empty, Cell::Wall, Cell::End],
    ];
    
    print_maze(&maze, "Initial Maze");
    let found_path = visualized_bfs_pathfinder(&mut maze, (0, 0), (3, 3));
    println!("   Path found: {}", found_path);
    print_maze(&maze, "Final Maze with Path");
    println!("   ✅ Trace saved to: traces/bfs_pathfinder.json");
    
    // Dijkstra's Algorithm
    println!("\n2. Dijkstra's Shortest Path Algorithm");
    println!("   Graph: 4 nodes with weighted edges");
    println!("   Finding shortest path from node 0 to node 3");
    
    // Create a simple graph: 0 -> 1 (4), 0 -> 2 (2), 1 -> 3 (3), 2 -> 3 (1), 2 -> 1 (1)
    let graph = vec![
        vec![(1, 4), (2, 2)],  // Node 0 connections
        vec![(3, 3)],          // Node 1 connections  
        vec![(1, 1), (3, 1)],  // Node 2 connections
        vec![],                // Node 3 connections (destination)
    ];
    
    print_graph(&graph);
    let result = dijkstra_visualization(&graph, 0, 3);
    match result {
        Some((path, distance)) => {
            println!("   Shortest path: {:?}", path);
            println!("   Total distance: {}", distance);
        }
        None => println!("   No path found"),
    }
    println!("   ✅ Trace saved to: traces/dijkstra_algorithm.json");
    
    // A* Pathfinding
    println!("\n3. A* Pathfinding Algorithm");
    println!("   Same 4x4 maze using A* with Manhattan distance heuristic");
    let mut maze2 = vec![
        vec![Cell::Start, Cell::Empty, Cell::Wall, Cell::Empty],
        vec![Cell::Empty, Cell::Empty, Cell::Wall, Cell::Empty],
        vec![Cell::Wall, Cell::Empty, Cell::Empty, Cell::Empty],
        vec![Cell::Empty, Cell::Empty, Cell::Wall, Cell::End],
    ];
    
    print_maze(&maze2, "A* Initial Maze");
    let a_star_result = a_star_visualization(&mut maze2, (0, 0), (3, 3));
    match a_star_result {
        Some(path) => {
            println!("   A* path found: {:?}", path);
            println!("   Path length: {}", path.len());
        }
        None => println!("   No path found with A*"),
    }
    print_maze(&maze2, "A* Final Maze with Path");
    println!("   ✅ Trace saved to: traces/a_star_pathfinder.json");
    
    // Complex maze example
    println!("\n4. Complex Maze Example");
    println!("   Larger 6x6 maze with more obstacles");
    let mut complex_maze = vec![
        vec![Cell::Start, Cell::Empty, Cell::Wall, Cell::Empty, Cell::Empty, Cell::Wall],
        vec![Cell::Wall,  Cell::Empty, Cell::Wall, Cell::Empty, Cell::Wall, Cell::Empty],
        vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Wall, Cell::Empty],
        vec![Cell::Empty, Cell::Wall,  Cell::Wall, Cell::Empty, Cell::Empty, Cell::Empty],
        vec![Cell::Empty, Cell::Empty, Cell::Empty, Cell::Wall, Cell::Empty, Cell::Wall],
        vec![Cell::Wall,  Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::End],
    ];
    
    print_maze(&complex_maze, "Complex Maze");
    let complex_result = visualized_bfs_pathfinder(&mut complex_maze, (0, 0), (5, 5));
    println!("   Complex maze path found: {}", complex_result);
    print_maze(&complex_maze, "Complex Maze Solution");
    println!("   ✅ Trace saved to: traces/bfs_pathfinder.json (overwritten)");
    
    println!("\n🎯 All pathfinding algorithm examples completed!");
    println!("📊 Pathfinding visualizations showcase:");
    println!("   • BFS (Breadth-First Search) for unweighted shortest path");
    println!("   • Dijkstra's algorithm for weighted shortest path");
    println!("   • A* algorithm with heuristic optimization");
    println!("   • Grid-based maze navigation");
    println!("   • Graph-based shortest path finding");
    println!("\n📊 Start the visualization server to view the traces:");
    println!("   cargo run --bin viz-server --features server");
    println!("   Then open: http://localhost:3030");
}

fn print_maze(maze: &[Vec<Cell>], title: &str) {
    println!("   {}:", title);
    for row in maze {
        print!("     ");
        for cell in row {
            print!("{:?} ", cell);
        }
        println!();
    }
}

fn print_graph(graph: &[Vec<(usize, i32)>]) {
    println!("   Graph structure:");
    for (node, edges) in graph.iter().enumerate() {
        print!("     Node {}: ", node);
        for (neighbor, weight) in edges {
            print!("-> {} ({}), ", neighbor, weight);
        }
        println!();
    }
}
