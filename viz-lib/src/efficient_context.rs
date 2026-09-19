use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::context::VizPerformanceMetrics;

// Compact data representation using deltas and compression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompactVizStep {
    pub step_id: usize,
    pub description: String,
    pub timestamp: u64,
    pub operation_type: String,
    
    // Only store changes, not full state
    pub variable_changes: Option<HashMap<String, String>>,
    pub array_changes: Option<HashMap<String, ArrayChange>>,
    pub tree_changes: Option<HashMap<String, TreeChange>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArrayChange {
    pub data_updates: Option<Vec<(usize, String)>>, // index, new_value pairs
    pub highlighted_indices: Option<Vec<usize>>,
    pub comparison_indices: Option<Vec<(usize, usize)>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreeChange {
    pub node_updates: Option<Vec<TreeNodeUpdate>>,
    pub highlighted_nodes: Option<Vec<String>>,
    pub comparison_nodes: Option<Vec<String>>,
    pub current_path: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreeNodeUpdate {
    pub node_id: String,
    pub value: Option<String>,
    pub position: Option<(usize, usize)>, // level, position
    pub highlighted: Option<bool>,
    pub comparing: Option<bool>,
}

// Compressed visualization trace using deltas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompactVizTrace {
    pub function_name: String,
    pub steps: Vec<CompactVizStep>,
    pub input: String,
    pub output: String,
    pub performance_metrics: VizPerformanceMetrics,
    
    // Initial state (stored once)
    pub initial_variables: HashMap<String, String>,
    pub initial_arrays: HashMap<String, CompactArrayState>,
    pub initial_trees: HashMap<String, CompactTreeState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompactArrayState {
    pub data: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompactTreeState {
    pub root: Option<CompactTreeNode>,
    pub tree_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompactTreeNode {
    pub id: String,
    pub value: String,
    pub left: Option<Box<CompactTreeNode>>,
    pub right: Option<Box<CompactTreeNode>>,
    pub level: usize,
    pub position: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EfficientVizPerformanceMetrics {
    pub total_comparisons: usize,
    pub total_swaps: usize,
    pub total_array_accesses: usize,
    pub execution_time_ms: u64,
}

// Binary format for even better compression
#[derive(Debug, Clone)]
pub struct BinaryVizTrace {
    pub header: TraceHeader,
    pub steps: Vec<BinaryStep>,
}

#[derive(Debug, Clone)]
pub struct TraceHeader {
    pub function_name: String,
    pub input: String,
    pub output: String,
    pub performance_metrics: VizPerformanceMetrics,
    pub variable_names: Vec<String>,  // Dictionary for variable names
    pub array_names: Vec<String>,     // Dictionary for array names
    pub tree_names: Vec<String>,      // Dictionary for tree names
}

#[derive(Debug, Clone)]
pub struct BinaryStep {
    pub step_id: u16,                // 2 bytes instead of 8
    pub description_id: u16,         // Index into string dictionary
    pub timestamp: u32,              // 4 bytes instead of 8
    pub operation_type: u8,          // Enum instead of string
    pub changes: Vec<u8>,            // Packed binary data
}

// Efficient context that builds compact traces
pub struct EfficientVizContext {
    function_name: String,
    steps: Vec<CompactVizStep>,
    
    // Current state tracking
    current_variables: HashMap<String, String>,
    current_arrays: HashMap<String, CompactArrayState>,
    current_trees: HashMap<String, CompactTreeState>,
    
    // Initial state (stored once)
    initial_variables: HashMap<String, String>,
    initial_arrays: HashMap<String, CompactArrayState>,
    initial_trees: HashMap<String, CompactTreeState>,
    
    step_counter: usize,
    start_time: std::time::Instant,
    performance_metrics: VizPerformanceMetrics,
    
    // Compression settings
    enable_compression: bool,
    min_steps_for_compression: usize,
}

impl EfficientVizContext {
    pub fn new(function_name: &str) -> Self {
        Self {
            function_name: function_name.to_string(),
            steps: Vec::new(),
            current_variables: HashMap::new(),
            current_arrays: HashMap::new(),
            current_trees: HashMap::new(),
            initial_variables: HashMap::new(),
            initial_arrays: HashMap::new(),
            initial_trees: HashMap::new(),
            step_counter: 0,
            start_time: std::time::Instant::now(),
            performance_metrics: VizPerformanceMetrics {
                total_comparisons: 0,
                total_swaps: 0,
                total_array_accesses: 0,
                execution_time_ms: 0,
            },
            enable_compression: true,
            min_steps_for_compression: 10,
        }
    }
    
    pub fn configure_compression(&mut self, enable: bool, min_steps: usize) {
        self.enable_compression = enable;
        self.min_steps_for_compression = min_steps;
    }
    
    pub fn track_var_creation(&mut self, name: &str, value: &str) {
        let old_value = self.current_variables.get(name).cloned();
        self.current_variables.insert(name.to_string(), value.to_string());
        
        // Store initial state if this is the first time we see this variable
        if old_value.is_none() {
            self.initial_variables.insert(name.to_string(), value.to_string());
        }
        
        let mut variable_changes = HashMap::new();
        variable_changes.insert(name.to_string(), value.to_string());
        
        self.add_efficient_step(
            &format!("Created variable {} = {}", name, value),
            "general",
            Some(variable_changes),
            None,
            None,
        );
    }
    
    pub fn track_var_update(&mut self, name: &str, value: &str) {
        let old_value = self.current_variables.get(name).cloned().unwrap_or_default();
        
        // Only record if value actually changed
        if old_value != value {
            self.current_variables.insert(name.to_string(), value.to_string());
            
            let mut variable_changes = HashMap::new();
            variable_changes.insert(name.to_string(), value.to_string());
            
            self.add_efficient_step(
                &format!("Updated {} from {} to {}", name, old_value, value),
                "general",
                Some(variable_changes),
                None,
                None,
            );
        }
    }
    
    pub fn track_array<T: std::fmt::Debug>(&mut self, name: &str, array: &[T]) {
        let array_data: Vec<String> = array.iter().map(|x| format!("{:?}", x)).collect();
        let array_state = CompactArrayState { data: array_data };
        
        // Store initial state if this is the first time we see this array
        if !self.current_arrays.contains_key(name) {
            self.initial_arrays.insert(name.to_string(), array_state.clone());
        }
        
        self.current_arrays.insert(name.to_string(), array_state);
        
        let mut array_changes = HashMap::new();
        // For initial tracking, we need to send all data
        array_changes.insert(name.to_string(), ArrayChange {
            data_updates: Some((0..array.len()).map(|i| (i, format!("{:?}", array[i]))).collect()),
            highlighted_indices: Some(Vec::new()),
            comparison_indices: Some(Vec::new()),
        });
        
        self.add_efficient_step(
            &format!("Tracking array: {}", name),
            "array_track",
            None,
            Some(array_changes),
            None,
        );
    }
    
    pub fn highlight_array_indices(&mut self, name: &str, indices: Vec<usize>) {
        let mut array_changes = HashMap::new();
        array_changes.insert(name.to_string(), ArrayChange {
            data_updates: None,
            highlighted_indices: Some(indices.clone()),
            comparison_indices: None,
        });
        
        self.add_efficient_step(
            &format!("Highlighting indices {:?} in {}", indices, name),
            "highlight",
            None,
            Some(array_changes),
            None,
        );
    }
    
    pub fn swap_array_elements<T: std::fmt::Debug>(&mut self, name: &str, array: &mut [T], idx1: usize, idx2: usize) {
        self.performance_metrics.total_swaps += 1;
        array.swap(idx1, idx2);
        
        // Only send the two changed elements
        let mut array_changes = HashMap::new();
        array_changes.insert(name.to_string(), ArrayChange {
            data_updates: Some(vec![
                (idx1, format!("{:?}", array[idx1])),
                (idx2, format!("{:?}", array[idx2])),
            ]),
            highlighted_indices: Some(vec![idx1, idx2]),
            comparison_indices: Some(Vec::new()),
        });
        
        // Update our local state
        if let Some(array_state) = self.current_arrays.get_mut(name) {
            array_state.data[idx1] = format!("{:?}", array[idx1]);
            array_state.data[idx2] = format!("{:?}", array[idx2]);
        }
        
        self.add_efficient_step(
            &format!("Swapped {}[{}] with {}[{}]", name, idx1, name, idx2),
            "swap",
            None,
            Some(array_changes),
            None,
        );
    }
    
    pub fn compare_array_indices(&mut self, name: &str, idx1: usize, idx2: usize) {
        self.performance_metrics.total_comparisons += 1;
        
        let mut array_changes = HashMap::new();
        array_changes.insert(name.to_string(), ArrayChange {
            data_updates: None,
            highlighted_indices: None,
            comparison_indices: Some(vec![(idx1, idx2)]),
        });
        
        self.add_efficient_step(
            &format!("Comparing {}[{}] with {}[{}]", name, idx1, name, idx2),
            "comparison",
            None,
            Some(array_changes),
            None,
        );
    }
    
    fn add_efficient_step(
        &mut self,
        description: &str,
        operation_type: &str,
        variable_changes: Option<HashMap<String, String>>,
        array_changes: Option<HashMap<String, ArrayChange>>,
        tree_changes: Option<HashMap<String, TreeChange>>,
    ) {
        let step = CompactVizStep {
            step_id: self.step_counter,
            description: description.to_string(),
            timestamp: self.start_time.elapsed().as_millis() as u64,
            operation_type: operation_type.to_string(),
            variable_changes,
            array_changes,
            tree_changes,
        };
        
        self.steps.push(step);
        self.step_counter += 1;
    }
    
    pub fn finalize(&self) {
        let mut final_metrics = self.performance_metrics.clone();
        final_metrics.execution_time_ms = self.start_time.elapsed().as_millis() as u64;
        
        let compact_trace = CompactVizTrace {
            function_name: self.function_name.clone(),
            steps: self.steps.clone(),
            input: "".to_string(),
            output: "".to_string(),
            performance_metrics: final_metrics,
            initial_variables: self.initial_variables.clone(),
            initial_arrays: self.initial_arrays.clone(),
            initial_trees: self.initial_trees.clone(),
        };
        
        self.save_compact_trace(&compact_trace);
        
        // Also save in binary format if steps are numerous enough
        if self.enable_compression && self.steps.len() >= self.min_steps_for_compression {
            self.save_binary_trace(&compact_trace);
        }
    }
    
    fn save_compact_trace(&self, trace: &CompactVizTrace) {
        std::fs::create_dir_all("traces").unwrap();
        
        let filename = format!("traces/{}_compact.json", self.function_name);
        let json = serde_json::to_string_pretty(trace).unwrap();
        std::fs::write(&filename, json).unwrap();
        
        // Calculate compression ratio vs original format
        if let Ok(original_file) = std::fs::read_to_string(format!("traces/{}.json", self.function_name)) {
            let original_size = original_file.len();
            let compact_size = std::fs::metadata(&filename).unwrap().len() as usize;
            let ratio = (100.0 * compact_size as f64 / original_size as f64) as u32;
            
            println!("📊 Compact trace saved: {} ({}% of original size)", filename, ratio);
        } else {
            println!("📊 Compact trace saved: {}", filename);
        }
    }
    
    fn save_binary_trace(&self, trace: &CompactVizTrace) {
        let filename = format!("traces/{}.binviz", self.function_name);
        
        // This would implement a custom binary format
        // For now, just save compressed JSON
        let json = serde_json::to_string(trace).unwrap();
        let compressed = self.compress_data(&json);
        std::fs::write(&filename, compressed).unwrap();
        
        let original_size = json.len();
        let compressed_size = std::fs::metadata(&filename).unwrap().len() as usize;
        let ratio = (100.0 * compressed_size as f64 / original_size as f64) as u32;
        
        println!("🗜️  Binary trace saved: {} ({}% of JSON size)", filename, ratio);
    }
    
    fn compress_data(&self, data: &str) -> Vec<u8> {
        // Simple compression using flate2
        use std::io::Write;
        let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::best());
        encoder.write_all(data.as_bytes()).unwrap();
        encoder.finish().unwrap()
    }
}

// Helper function to convert compact trace to original format for compatibility
pub fn expand_compact_trace(compact: &CompactVizTrace) -> crate::context::VizTrace {
    use crate::context::{VizTrace, VizStep, VizArrayState, VizTreeState};
    
    let mut expanded_steps = Vec::new();
    let mut current_variables = compact.initial_variables.clone();
    let mut current_arrays: HashMap<String, VizArrayState> = HashMap::new();
    let mut current_trees: HashMap<String, VizTreeState> = HashMap::new();
    
    // Initialize arrays from compact format
    for (name, compact_array) in &compact.initial_arrays {
        current_arrays.insert(name.clone(), VizArrayState {
            data: compact_array.data.clone(),
            highlighted_indices: Vec::new(),
            comparison_indices: Vec::new(),
        });
    }
    
    // Process each step, applying deltas
    for compact_step in &compact.steps {
        // Apply variable changes
        if let Some(var_changes) = &compact_step.variable_changes {
            for (name, value) in var_changes {
                current_variables.insert(name.clone(), value.clone());
            }
        }
        
        // Apply array changes
        if let Some(array_changes) = &compact_step.array_changes {
            for (name, change) in array_changes {
                if let Some(array_state) = current_arrays.get_mut(name) {
                    if let Some(data_updates) = &change.data_updates {
                        for (index, value) in data_updates {
                            if *index < array_state.data.len() {
                                array_state.data[*index] = value.clone();
                            }
                        }
                    }
                    if let Some(highlighted) = &change.highlighted_indices {
                        array_state.highlighted_indices = highlighted.clone();
                    }
                    if let Some(comparisons) = &change.comparison_indices {
                        array_state.comparison_indices = comparisons.clone();
                    }
                }
            }
        }
        
        // Create expanded step
        let expanded_step = VizStep {
            step_id: compact_step.step_id,
            description: compact_step.description.clone(),
            variables: current_variables.clone(),
            arrays: current_arrays.clone(),
            trees: current_trees.clone(),
            timestamp: compact_step.timestamp,
            operation_type: compact_step.operation_type.clone(),
        };
        
        expanded_steps.push(expanded_step);
    }
    
    VizTrace {
        function_name: compact.function_name.clone(),
        steps: expanded_steps,
        input: compact.input.clone(),
        output: compact.output.clone(),
        performance_metrics: compact.performance_metrics.clone(),
    }
}
