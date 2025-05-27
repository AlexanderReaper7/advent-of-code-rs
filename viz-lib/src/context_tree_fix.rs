use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Generic tree node structure for visualization
#[derive(Debug, Clone)]
pub struct TreeNode<T> {
    pub value: T,
    pub left: Option<Box<TreeNode<T>>>,
    pub right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VizArrayState {
    pub data: Vec<String>,
    pub highlighted_indices: Vec<usize>,
    pub comparison_indices: Vec<(usize, usize)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VizTreeNode {
    pub id: String,
    pub value: String,
    pub left: Option<Box<VizTreeNode>>,
    pub right: Option<Box<VizTreeNode>>,
    pub is_highlighted: bool,
    pub is_comparing: bool,
    pub level: usize,
    pub position: usize, // Position within the level
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VizTreeState {
    pub root: Option<VizTreeNode>,
    pub tree_type: String, // "binary", "bst", "avl", "heap", etc.
    pub highlighted_nodes: Vec<String>,
    pub comparison_nodes: Vec<String>,
    pub current_path: Vec<String>, // For traversal visualization
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VizStep {
    pub step_id: usize,
    pub description: String,
    pub variables: HashMap<String, String>,
    pub arrays: HashMap<String, VizArrayState>,
    pub trees: HashMap<String, VizTreeState>,
    pub timestamp: u64,
    pub operation_type: String, // "comparison", "swap", "access", "update", "insert", "delete", etc.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VizPerformanceMetrics {
    pub total_comparisons: usize,
    pub total_swaps: usize,
    pub total_array_accesses: usize,
    pub execution_time_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VizTrace {
    pub function_name: String,
    pub steps: Vec<VizStep>,
    pub input: String,
    pub output: String,
    pub performance_metrics: VizPerformanceMetrics,
}

pub struct VizContext {
    function_name: String,
    steps: Vec<VizStep>,
    current_variables: HashMap<String, String>,
    current_arrays: HashMap<String, VizArrayState>,
    current_trees: HashMap<String, VizTreeState>,
    step_counter: usize,
    start_time: std::time::Instant,
    performance_metrics: VizPerformanceMetrics,
}

impl VizContext {
    pub fn new(function_name: &str) -> Self {
        Self {
            function_name: function_name.to_string(),
            steps: Vec::new(),
            current_variables: HashMap::new(),
            current_arrays: HashMap::new(),
            current_trees: HashMap::new(),
            step_counter: 0,
            start_time: std::time::Instant::now(),
            performance_metrics: VizPerformanceMetrics {
                total_comparisons: 0,
                total_swaps: 0,
                total_array_accesses: 0,
                execution_time_ms: 0,
            },
        }
    }
    
    pub fn track_var_creation(&mut self, name: &str, value: &str) {
        self.current_variables.insert(name.to_string(), value.to_string());
        self.add_step(&format!("Created variable {} = {}", name, value));
    }
    
    pub fn track_var_update(&mut self, name: &str, value: &str) {
        let old_value = self.current_variables.get(name).cloned().unwrap_or_default();
        self.current_variables.insert(name.to_string(), value.to_string());
        self.add_step(&format!("Updated {} from {} to {}", name, old_value, value));
    }
    
    pub fn track_value(&mut self, expr: &str, value: &str) {
        self.add_step(&format!("Evaluated {} = {}", expr, value));
    }
    
    pub fn track_operation(&mut self, operation: &str, before: &str, after: &str) {
        self.add_step(&format!("Operation: {} | Before: {} | After: {}", operation, before, after));
    }
    
    pub fn add_step(&mut self, description: &str) {
        self.add_step_with_type(description, "general");
    }
    
    pub fn add_step_with_type(&mut self, description: &str, operation_type: &str) {
        let step = VizStep {
            step_id: self.step_counter,
            description: description.to_string(),
            variables: self.current_variables.clone(),
            arrays: self.current_arrays.clone(),
            trees: self.current_trees.clone(),
            timestamp: self.start_time.elapsed().as_millis() as u64,
            operation_type: operation_type.to_string(),
        };
        self.steps.push(step);
        self.step_counter += 1;
    }
    
    // Array visualization methods
    pub fn track_array<T: std::fmt::Debug>(&mut self, name: &str, array: &[T]) {
        let array_state = VizArrayState {
            data: array.iter().map(|x| format!("{:?}", x)).collect(),
            highlighted_indices: Vec::new(),
            comparison_indices: Vec::new(),
        };
        self.current_arrays.insert(name.to_string(), array_state);
        self.add_step_with_type(&format!("Tracking array: {}", name), "array_track");
    }
    
    pub fn highlight_array_indices(&mut self, name: &str, indices: Vec<usize>) {
        if let Some(array_state) = self.current_arrays.get_mut(name) {
            array_state.highlighted_indices = indices.clone();
        }
        self.add_step_with_type(&format!("Highlighting indices {:?} in {}", indices, name), "highlight");
    }
    
    pub fn compare_array_indices(&mut self, name: &str, idx1: usize, idx2: usize) {
        self.performance_metrics.total_comparisons += 1;
        if let Some(array_state) = self.current_arrays.get_mut(name) {
            array_state.comparison_indices = vec![(idx1, idx2)];
        }
        self.add_step_with_type(&format!("Comparing {}[{}] with {}[{}]", name, idx1, name, idx2), "comparison");
    }
    
    pub fn swap_array_elements<T: std::fmt::Debug>(&mut self, name: &str, array: &mut [T], idx1: usize, idx2: usize) {
        self.performance_metrics.total_swaps += 1;
        array.swap(idx1, idx2);
        if let Some(array_state) = self.current_arrays.get_mut(name) {
            array_state.data = array.iter().map(|x| format!("{:?}", x)).collect();
            array_state.highlighted_indices = vec![idx1, idx2];
            array_state.comparison_indices.clear();
        }
        self.add_step_with_type(&format!("Swapped {}[{}] with {}[{}]", name, idx1, name, idx2), "swap");
    }
    
    pub fn update_array_element<T: std::fmt::Debug>(&mut self, name: &str, array: &[T], idx: usize) {
        self.performance_metrics.total_array_accesses += 1;
        if let Some(array_state) = self.current_arrays.get_mut(name) {
            array_state.data = array.iter().map(|x| format!("{:?}", x)).collect();
            array_state.highlighted_indices = vec![idx];
        }
        self.add_step_with_type(&format!("Updated {}[{}] = {:?}", name, idx, array[idx]), "update");
    }
    
    // Tree visualization methods
    pub fn track_tree<T: std::fmt::Debug + Clone>(&mut self, name: &str, root: Option<&TreeNode<T>>, tree_type: &str) {
        let tree_state = VizTreeState {
            root: root.map(|node| Self::convert_tree_node(node, 0, 0)),
            tree_type: tree_type.to_string(),
            highlighted_nodes: Vec::new(),
            comparison_nodes: Vec::new(),
            current_path: Vec::new(),
        };
        self.current_trees.insert(name.to_string(), tree_state);
        self.add_step_with_type(&format!("Tracking {} tree: {}", tree_type, name), "tree_track");
    }
    
    fn convert_tree_node<T: std::fmt::Debug + Clone>(node: &TreeNode<T>, level: usize, position: usize) -> VizTreeNode {
        let node_id = format!("{}_{}", level, position);
        VizTreeNode {
            id: node_id,
            value: format!("{:?}", node.value),
            left: node.left.as_ref().map(|left| Box::new(Self::convert_tree_node(left, level + 1, position * 2))),
            right: node.right.as_ref().map(|right| Box::new(Self::convert_tree_node(right, level + 1, position * 2 + 1))),
            is_highlighted: false,
            is_comparing: false,
            level,
            position,
        }
    }
    
    pub fn highlight_tree_node(&mut self, tree_name: &str, node_id: &str) {
        if let Some(tree_state) = self.current_trees.get_mut(tree_name) {
            tree_state.highlighted_nodes = vec![node_id.to_string()];
        }
        self.add_step_with_type(&format!("Highlighting node {} in tree {}", node_id, tree_name), "tree_highlight");
    }
    
    pub fn compare_tree_nodes(&mut self, tree_name: &str, node1_id: &str, node2_id: &str) {
        self.performance_metrics.total_comparisons += 1;
        if let Some(tree_state) = self.current_trees.get_mut(tree_name) {
            tree_state.comparison_nodes = vec![node1_id.to_string(), node2_id.to_string()];
        }
        self.add_step_with_type(&format!("Comparing nodes {} and {} in tree {}", node1_id, node2_id, tree_name), "tree_comparison");
    }
    
    pub fn track_tree_traversal(&mut self, tree_name: &str, path: Vec<String>) {
        if let Some(tree_state) = self.current_trees.get_mut(tree_name) {
            tree_state.current_path = path.clone();
        }
        self.add_step_with_type(&format!("Tree traversal path: {:?}", path), "tree_traversal");
    }
    
    pub fn update_tree<T: std::fmt::Debug + Clone>(&mut self, tree_name: &str, updated_root: Option<&TreeNode<T>>) {
        let tree_state = self.current_trees.get_mut(tree_name);
        if let Some(state) = tree_state {
            state.root = updated_root.map(|node| Self::convert_tree_node(node, 0, 0));
        }
        self.add_step_with_type(&format!("Updated tree {}", tree_name), "tree_update");
    }
    
    // Performance tracking methods
    pub fn track_comparison(&mut self) {
        self.performance_metrics.total_comparisons += 1;
    }
    
    pub fn track_swap(&mut self) {
        self.performance_metrics.total_swaps += 1;
    }
    
    pub fn track_array_access(&mut self) {
        self.performance_metrics.total_array_accesses += 1;
    }
    
    pub fn get_performance_metrics(&self) -> &VizPerformanceMetrics {
        &self.performance_metrics
    }
    
    pub fn finalize(&self) {
        // Calculate final performance metrics
        let mut final_metrics = self.performance_metrics.clone();
        final_metrics.execution_time_ms = self.start_time.elapsed().as_millis() as u64;
        
        // Save the trace for visualization
        let trace = VizTrace {
            function_name: self.function_name.clone(),
            steps: self.steps.clone(),
            input: "".to_string(), // Would be filled by the macro
            output: "".to_string(), // Would be filled by the macro
            performance_metrics: final_metrics,
        };
        
        // Save to a file or send to visualization server
        self.save_trace(&trace);
    }
    
    fn save_trace(&self, trace: &VizTrace) {
        // Create a traces directory if it doesn't exist
        std::fs::create_dir_all("traces").unwrap();
        
        // Save as JSON
        let filename = format!("traces/{}.json", self.function_name);
        let json = serde_json::to_string_pretty(trace).unwrap();
        std::fs::write(filename, json).unwrap();
        
        println!("Visualization trace saved for function: {}", self.function_name);
    }
}

// Helper macros for manual instrumentation
#[macro_export]
macro_rules! viz_track {
    ($ctx:expr, $var:ident) => {
        $ctx.track_value(stringify!($var), &format!("{:?}", $var));
    };
    ($ctx:expr, $expr:expr, $name:expr) => {
        $ctx.track_value($name, &format!("{:?}", $expr));
    };
}

#[macro_export]
macro_rules! viz_step {
    ($ctx:expr, $msg:expr) => {
        $ctx.add_step($msg);
    };
}

#[macro_export]
macro_rules! viz_operation {
    ($ctx:expr, $op:expr, $before:expr, $after:expr) => {
        $ctx.track_operation($op, &format!("{:?}", $before), &format!("{:?}", $after));
    };
}
