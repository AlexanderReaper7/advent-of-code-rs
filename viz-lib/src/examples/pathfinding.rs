// Pathfinding algorithm examples for visualization
// This module contains various pathfinding algorithms that demonstrate
// how to visualize graph traversal and shortest path algorithms

use crate::VizContext;
use std::collections::VecDeque;

#[derive(Clone, Copy, PartialEq)]
pub enum Cell {
    Empty,
    Wall,
    Start,
    End,
    Visited,
    Path,
}

impl std::fmt::Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = match self {
            Cell::Empty => "·",
            Cell::Wall => "█",
            Cell::Start => "S",
            Cell::End => "E",
            Cell::Visited => "○",
            Cell::Path => "*",
        };
        write!(f, "{}", symbol)
    }
}

/// BFS pathfinding with visualization
pub fn visualized_bfs_pathfinder(maze: &mut Vec<Vec<Cell>>, start: (usize, usize), end: (usize, usize)) -> bool {
    let mut ctx = VizContext::new("bfs_pathfinder");
    
    ctx.add_step("Starting BFS pathfinding algorithm");
    ctx.track_var_creation("start", &format!("{:?}", start));
    ctx.track_var_creation("end", &format!("{:?}", end));
    
    // Create a flattened representation for visualization
    let mut flat_maze: Vec<String> = maze.iter()
        .map(|row| row.iter().map(|cell| format!("{:?}", cell)).collect::<Vec<_>>().join(""))
        .collect();
    
    ctx.track_array("maze", &flat_maze);
    
    let mut queue = VecDeque::new();
    let mut visited = vec![vec![false; maze[0].len()]; maze.len()];
    let mut parent = vec![vec![None; maze[0].len()]; maze.len()];
    
    queue.push_back(start);
    visited[start.0][start.1] = true;
    ctx.track_var_creation("queue_size", &queue.len().to_string());
    ctx.add_step(&format!("Added start position {:?} to queue", start));
    
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)]; // right, down, left, up
    
    while !queue.is_empty() {
        ctx.track_var_update("queue_size", &queue.len().to_string());
        let current = queue.pop_front().unwrap();
        ctx.add_step(&format!("Processing position {:?}", current));
        
        if current == end {
            ctx.add_step("Found the end position! Reconstructing path...");
            
            // Reconstruct path
            let mut path_pos = Some(end);
            while let Some(pos) = path_pos {
                if pos != start && pos != end {
                    maze[pos.0][pos.1] = Cell::Path;
                }
                path_pos = parent[pos.0][pos.1];
            }
            
            // Update visualization with final path
            flat_maze = maze.iter()
                .map(|row| row.iter().map(|cell| format!("{:?}", cell)).collect::<Vec<_>>().join(""))
                .collect();
            ctx.track_array("maze", &flat_maze);
            ctx.add_step("Path reconstruction complete!");
            ctx.finalize();
            return true;
        }
        
        // Mark current position as visited (unless it's start or end)
        if current != start && current != end {
            maze[current.0][current.1] = Cell::Visited;
        }
        
        // Update visualization
        flat_maze = maze.iter()
            .map(|row| row.iter().map(|cell| format!("{:?}", cell)).collect::<Vec<_>>().join(""))
            .collect();
        ctx.track_array("maze", &flat_maze);
        
        // Explore neighbors
        for (dx, dy) in directions.iter() {
            let new_row = current.0 as i32 + dx;
            let new_col = current.1 as i32 + dy;
            
            if new_row >= 0 && new_row < maze.len() as i32 
                && new_col >= 0 && new_col < maze[0].len() as i32 {
                let new_pos = (new_row as usize, new_col as usize);
                
                if !visited[new_pos.0][new_pos.1] && 
                   (maze[new_pos.0][new_pos.1] == Cell::Empty || 
                    maze[new_pos.0][new_pos.1] == Cell::End) {
                    
                    visited[new_pos.0][new_pos.1] = true;
                    parent[new_pos.0][new_pos.1] = Some(current);
                    queue.push_back(new_pos);
                    
                    ctx.add_step(&format!("Added neighbor {:?} to queue", new_pos));
                }
            }
        }
    }
    
    ctx.add_step("No path found to the end position");
    ctx.finalize();
    false
}

/// Dijkstra's algorithm with visualization
pub fn dijkstra_visualization(graph: &[Vec<(usize, i32)>], start: usize, end: usize) -> Option<(Vec<usize>, i32)> {
    let mut ctx = VizContext::new("dijkstra_algorithm");
    
    ctx.add_step("Starting Dijkstra's shortest path algorithm");
    ctx.track_var_creation("start", &start.to_string());
    ctx.track_var_creation("end", &end.to_string());
    ctx.track_var_creation("nodes", &graph.len().to_string());
    
    let n = graph.len();
    let mut distances = vec![i32::MAX; n];
    let mut visited = vec![false; n];
    let mut previous = vec![None; n];
    
    distances[start] = 0;
    ctx.track_array("distances", &distances.iter().map(|d| if *d == i32::MAX { "∞".to_string() } else { d.to_string() }).collect::<Vec<_>>());
    ctx.track_array("visited", &visited);
    
    for _ in 0..n {
        // Find unvisited node with minimum distance
        let mut current_node = None;
        let mut min_distance = i32::MAX;
        
        for node in 0..n {
            if !visited[node] && distances[node] < min_distance {
                min_distance = distances[node];
                current_node = Some(node);
            }
        }
        
        if let Some(current) = current_node {
            if current == end {
                ctx.add_step(&format!("Reached destination node {} with distance {}", end, distances[end]));
                break;
            }
            
            visited[current] = true;
            ctx.highlight_array_indices("visited", vec![current]);
            ctx.add_step(&format!("Visiting node {} with distance {}", current, distances[current]));
            
            // Update distances to neighbors
            for &(neighbor, weight) in &graph[current] {
                if !visited[neighbor] {
                    let new_distance = distances[current].saturating_add(weight);
                    if new_distance < distances[neighbor] {
                        distances[neighbor] = new_distance;
                        previous[neighbor] = Some(current);
                        
                        ctx.compare_array_indices("distances", current, neighbor);
                        ctx.add_step(&format!("Updated distance to node {}: {} -> {}", neighbor, 
                            if distances[neighbor] == i32::MAX { "∞".to_string() } else { distances[neighbor].to_string() },
                            new_distance));
                        
                        ctx.track_array("distances", &distances.iter().map(|d| if *d == i32::MAX { "∞".to_string() } else { d.to_string() }).collect::<Vec<_>>());
                    }
                }
            }
            
            ctx.track_array("visited", &visited);
        } else {
            ctx.add_step("No more reachable unvisited nodes");
            break;
        }
    }
    
    if distances[end] == i32::MAX {
        ctx.add_step("No path found to destination");
        ctx.finalize();
        return None;
    }
    
    // Reconstruct path
    let mut path = Vec::new();
    let mut current = Some(end);
    
    while let Some(node) = current {
        path.push(node);
        current = previous[node];
    }
    path.reverse();
    
    ctx.track_array("final_path", &path);
    ctx.add_step(&format!("Found shortest path: {:?} with total distance: {}", path, distances[end]));
    ctx.finalize();
    
    Some((path, distances[end]))
}

/// A* pathfinding with visualization
pub fn a_star_visualization(
    maze: &mut Vec<Vec<Cell>>, 
    start: (usize, usize), 
    end: (usize, usize)
) -> Option<Vec<(usize, usize)>> {
    let mut ctx = VizContext::new("a_star_pathfinder");
    
    ctx.add_step("Starting A* pathfinding algorithm");
    ctx.track_var_creation("start", &format!("{:?}", start));
    ctx.track_var_creation("end", &format!("{:?}", end));
    
    use std::collections::BinaryHeap;
    use std::cmp::Ordering;
    
    #[derive(Copy, Clone, Eq, PartialEq)]
    struct State {
        cost: i32,
        position: (usize, usize),
    }
    
    impl Ord for State {
        fn cmp(&self, other: &Self) -> Ordering {
            other.cost.cmp(&self.cost)
        }
    }
    
    impl PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
      fn heuristic(a: (usize, usize), b: (usize, usize)) -> i32 {
        (a.0 as i32 - b.0 as i32).abs() + (a.1 as i32 - b.1 as i32).abs()
    }
    
    let mut heap = BinaryHeap::new();
    let mut distances = vec![vec![i32::MAX; maze[0].len()]; maze.len()];
    let mut parent = vec![vec![None; maze[0].len()]; maze.len()];
    
    distances[start.0][start.1] = 0;
    heap.push(State { cost: heuristic(start, end), position: start });
    
    ctx.track_var_creation("open_set_size", &heap.len().to_string());
    ctx.add_step("Initialized A* with start position");
    
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    
    while let Some(State { cost: _, position: current }) = heap.pop() {
        ctx.track_var_update("open_set_size", &heap.len().to_string());
        ctx.add_step(&format!("Processing position {:?}", current));
        
        if current == end {
            ctx.add_step("Found the end position! Reconstructing path...");
            
            // Reconstruct path
            let mut path = Vec::new();
            let mut pos = Some(current);
            
            while let Some(p) = pos {
                path.push(p);
                if p != start && p != end {
                    maze[p.0][p.1] = Cell::Path;
                }
                pos = parent[p.0][p.1];
            }
            path.reverse();
            
            // Update visualization with final path
            let flat_maze: Vec<String> = maze.iter()
                .map(|row| row.iter().map(|cell| format!("{:?}", cell)).collect::<Vec<_>>().join(""))
                .collect();
            ctx.track_array("maze", &flat_maze);
            ctx.track_array("final_path", &path.iter().map(|p| format!("{:?}", p)).collect::<Vec<_>>());
            ctx.add_step("A* pathfinding complete!");
            ctx.finalize();
            return Some(path);
        }
        
        // Mark as visited
        if current != start && current != end {
            maze[current.0][current.1] = Cell::Visited;
        }
        
        for (dx, dy) in directions.iter() {
            let new_row = current.0 as i32 + dx;
            let new_col = current.1 as i32 + dy;
            
            if new_row >= 0 && new_row < maze.len() as i32 
                && new_col >= 0 && new_col < maze[0].len() as i32 {
                let next = (new_row as usize, new_col as usize);
                
                if maze[next.0][next.1] != Cell::Wall {
                    let new_cost = distances[current.0][current.1] + 1;
                    
                    if new_cost < distances[next.0][next.1] {
                        distances[next.0][next.1] = new_cost;
                        parent[next.0][next.1] = Some(current);
                        let priority = new_cost + heuristic(next, end);
                        heap.push(State { cost: priority, position: next });
                        
                        ctx.add_step(&format!("Added neighbor {:?} with cost {} + {} = {}", 
                            next, new_cost, heuristic(next, end), priority));
                    }
                }
            }
        }
        
        // Update visualization
        let flat_maze: Vec<String> = maze.iter()
            .map(|row| row.iter().map(|cell| format!("{:?}", cell)).collect::<Vec<_>>().join(""))
            .collect();
        ctx.track_array("maze", &flat_maze);
    }
    
    ctx.add_step("No path found to the end position");
    ctx.finalize();
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_bfs_pathfinder() {
        let mut maze = vec![
            vec![Cell::Start, Cell::Empty, Cell::Wall, Cell::Empty],
            vec![Cell::Empty, Cell::Empty, Cell::Wall, Cell::Empty],
            vec![Cell::Wall, Cell::Empty, Cell::Empty, Cell::Empty],
            vec![Cell::Empty, Cell::Empty, Cell::Wall, Cell::End],
        ];
        
        let result = visualized_bfs_pathfinder(&mut maze, (0, 0), (3, 3));
        assert!(result);
    }
    
    #[test]
    fn test_dijkstra_algorithm() {
        // Create a simple graph: 0 -> 1 (4), 0 -> 2 (2), 1 -> 3 (3), 2 -> 3 (1), 2 -> 1 (1)
        let graph = vec![
            vec![(1, 4), (2, 2)],  // Node 0 connections
            vec![(3, 3)],          // Node 1 connections  
            vec![(1, 1), (3, 1)],  // Node 2 connections
            vec![],                // Node 3 connections (destination)
        ];
        
        let result = dijkstra_visualization(&graph, 0, 3);
        assert!(result.is_some());
        let (path, distance) = result.unwrap();
        assert_eq!(distance, 3); // 0 -> 2 (2) -> 3 (1) = 3
        assert_eq!(path, vec![0, 2, 3]);
    }

    #[test]
    fn test_a_star_pathfinder() {
        let mut maze = vec![
            vec![Cell::Start, Cell::Empty, Cell::Wall, Cell::Empty],
            vec![Cell::Empty, Cell::Empty, Cell::Wall, Cell::Empty],
            vec![Cell::Wall, Cell::Empty, Cell::Empty, Cell::Empty],
            vec![Cell::Empty, Cell::Empty, Cell::Wall, Cell::End],
        ];
        
        let result = a_star_visualization(&mut maze, (0, 0), (3, 3));
        assert!(result.is_some());
        let path = result.unwrap();
        assert!(path.len() > 0);
        assert_eq!(path[0], (0, 0)); // starts at start
        assert_eq!(path[path.len() - 1], (3, 3)); // ends at end
    }
}
