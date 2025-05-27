// Tree algorithm examples for visualization
// This module contains various tree data structure examples that demonstrate
// how to visualize tree operations including BST, heap, and general trees

use crate::{TreeNode, VizContext};

// Binary Search Tree implementation with visualization
#[derive(Debug, Clone)]
pub struct BST<T> {
    root: Option<Box<TreeNode<T>>>,
}

impl<T: Ord + std::fmt::Debug + Clone> BST<T> {
    pub fn new() -> Self {
        BST { root: None }
    }

    pub fn insert(&mut self, value: T, ctx: &mut VizContext) {
        ctx.track_value("operation", &format!("BST insert: {:?}", value));
        self.root = Self::insert_node(self.root.take(), value, ctx);
        ctx.track_tree("bst", self.root.as_deref(), "Binary Search Tree");
    }

    fn insert_node(node: Option<Box<TreeNode<T>>>, value: T, ctx: &mut VizContext) -> Option<Box<TreeNode<T>>> {
        match node {
            None => {
                ctx.add_step_with_type(&format!("Creating new node with value {:?}", value), "tree_insert");
                Some(Box::new(TreeNode {
                    value,
                    left: None,
                    right: None,
                }))
            }
            Some(mut node) => {
                ctx.add_step_with_type(&format!("Comparing {:?} with current node {:?}", value, node.value), "tree_comparison");
                
                if value < node.value {
                    ctx.add_step(&format!("{:?} < {:?}, going left", value, node.value));
                    node.left = Self::insert_node(node.left.take(), value, ctx);
                } else if value > node.value {
                    ctx.add_step(&format!("{:?} > {:?}, going right", value, node.value));
                    node.right = Self::insert_node(node.right.take(), value, ctx);
                } else {
                    ctx.add_step(&format!("{:?} already exists in tree", value));
                }
                Some(node)
            }
        }
    }

    pub fn search(&self, value: T, ctx: &mut VizContext) -> bool {
        ctx.track_value("operation", &format!("BST search: {:?}", value));
        ctx.track_tree("bst", self.root.as_deref(), "Binary Search Tree");
        Self::search_node(&self.root, &value, ctx, vec![])
    }

    fn search_node(node: &Option<Box<TreeNode<T>>>, value: &T, ctx: &mut VizContext, mut path: Vec<String>) -> bool {
        match node {
            None => {
                ctx.add_step_with_type(&format!("Reached null node, {:?} not found", value), "tree_search");
                ctx.track_tree_traversal("bst", path);
                false
            }
            Some(node) => {
                let node_id = format!("{}_{}", path.len(), path.len()); // Simple ID generation
                path.push(node_id.clone());
                ctx.track_tree_traversal("bst", path.clone());
                ctx.highlight_tree_node("bst", &node_id);
                
                ctx.add_step_with_type(&format!("Visiting node {:?}, comparing with {:?}", node.value, value), "tree_comparison");
                
                if value == &node.value {
                    ctx.add_step_with_type(&format!("Found {:?}!", value), "tree_found");
                    true
                } else if value < &node.value {
                    ctx.add_step(&format!("{:?} < {:?}, searching left subtree", value, node.value));
                    Self::search_node(&node.left, value, ctx, path)
                } else {
                    ctx.add_step(&format!("{:?} > {:?}, searching right subtree", value, node.value));
                    Self::search_node(&node.right, value, ctx, path)
                }
            }
        }
    }

    pub fn inorder_traversal(&self, ctx: &mut VizContext) -> Vec<T> {
        ctx.track_value("operation", "In-order traversal");
        ctx.track_tree("bst", self.root.as_deref(), "Binary Search Tree");
        let mut result = Vec::new();
        Self::inorder_node(&self.root, &mut result, ctx, vec![]);
        result
    }

    fn inorder_node(node: &Option<Box<TreeNode<T>>>, result: &mut Vec<T>, ctx: &mut VizContext, mut path: Vec<String>) {
        if let Some(node) = node {
            let node_id = format!("{}_{}", path.len(), path.len());
            path.push(node_id.clone());
            ctx.highlight_tree_node("bst", &node_id);
            ctx.add_step_with_type(&format!("Visiting node {:?}", node.value), "tree_traversal");
            
            // Left subtree
            ctx.add_step("Traversing left subtree");
            Self::inorder_node(&node.left, result, ctx, path.clone());
            
            // Current node
            ctx.add_step(&format!("Processing current node: {:?}", node.value));
            result.push(node.value.clone());
            
            // Right subtree
            ctx.add_step("Traversing right subtree");
            Self::inorder_node(&node.right, result, ctx, path);
        }
    }
}

/// Example: Manual binary tree construction and visualization
pub fn manual_tree_demo() {
    let mut ctx = VizContext::new("manual_tree_demo");
    
    // Create a simple binary tree manually
    let root = TreeNode {
        value: 10,
        left: Some(Box::new(TreeNode {
            value: 5,
            left: Some(Box::new(TreeNode {
                value: 3,
                left: None,
                right: None,
            })),
            right: Some(Box::new(TreeNode {
                value: 7,
                left: None,
                right: None,
            })),
        })),
        right: Some(Box::new(TreeNode {
            value: 15,
            left: Some(Box::new(TreeNode {
                value: 12,
                left: None,
                right: None,
            })),
            right: Some(Box::new(TreeNode {
                value: 18,
                left: None,
                right: None,
            })),
        })),
    };
    
    ctx.track_tree("binary_tree", Some(&root), "Binary Tree");
    ctx.add_step("Created binary tree with root 10");
    
    // Highlight some nodes to show traversal
    ctx.highlight_tree_node("binary_tree", "0_0");
    ctx.add_step("Starting from root");
    
    ctx.highlight_tree_node("binary_tree", "1_0");
    ctx.add_step("Moving to left child (5)");
    
    ctx.highlight_tree_node("binary_tree", "2_0");
    ctx.add_step("Moving to left grandchild (3)");
    
    ctx.highlight_tree_node("binary_tree", "2_1");
    ctx.add_step("Moving to right grandchild (7)");
    
    ctx.highlight_tree_node("binary_tree", "1_1");
    ctx.add_step("Moving to right child (15)");
    
    ctx.compare_tree_nodes("binary_tree", "0_0", "1_1");
    ctx.add_step("Comparing root with right child");
    
    ctx.finalize();
}

/// Example: Max heap visualization
pub fn heap_demo() {
    let mut ctx = VizContext::new("heap_demo");
    
    ctx.add_step("Creating max heap visualization");
    ctx.track_value("operation", "Max Heap Demo");
    
    // Create heap array representation first
    let heap_values = vec![100, 85, 90, 70, 80, 75, 60];
    for (i, &value) in heap_values.iter().enumerate() {
        ctx.track_value(&format!("heap[{}]", i), &value.to_string());
        ctx.add_step(&format!("Adding element {} to heap at index {}", value, i));
    }
    
    // Build the tree structure manually for heap
    let root = TreeNode {
        value: 100,
        left: Some(Box::new(TreeNode {
            value: 85,
            left: Some(Box::new(TreeNode {
                value: 70,
                left: None,
                right: None,
            })),
            right: Some(Box::new(TreeNode {
                value: 80,
                left: None,
                right: None,
            })),
        })),
        right: Some(Box::new(TreeNode {
            value: 90,
            left: Some(Box::new(TreeNode {
                value: 75,
                left: None,
                right: None,
            })),
            right: Some(Box::new(TreeNode {
                value: 60,
                left: None,
                right: None,
            })),
        })),
    };
    
    ctx.track_tree("max_heap", Some(&root), "Max Heap");
    ctx.add_step("Created max heap structure");
    
    // Simulate heap operations
    ctx.highlight_tree_node("max_heap", "0_0");
    ctx.add_step("Max element is at root");
    
    ctx.compare_tree_nodes("max_heap", "0_0", "1_0");
    ctx.add_step("Comparing parent with left child");
    
    ctx.compare_tree_nodes("max_heap", "0_0", "1_1");
    ctx.add_step("Comparing parent with right child");
    
    ctx.finalize();
}

/// Example: Binary Search Tree operations demo
pub fn bst_demo() {
    let mut ctx = VizContext::new("bst_demo");
    let mut bst = BST::<i32>::new();
    
    // Insert some values into the BST
    bst.insert(50, &mut ctx);
    bst.insert(30, &mut ctx);
    bst.insert(70, &mut ctx);
    bst.insert(20, &mut ctx);
    bst.insert(40, &mut ctx);
    bst.insert(60, &mut ctx);
    bst.insert(80, &mut ctx);
    
    // Search for values
    bst.search(40, &mut ctx);
    bst.search(65, &mut ctx); // Not found
    
    // Perform traversal
    let traversal = bst.inorder_traversal(&mut ctx);
    ctx.track_value("inorder_result", &format!("{:?}", traversal));
    
    ctx.finalize();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bst_operations() {
        let mut ctx = VizContext::new("bst_operations");
        let mut bst = BST::new();
        
        // Insert values
        let values = vec![5, 3, 7, 1, 4, 6, 8];
        for value in values {
            bst.insert(value, &mut ctx);
        }
        
        // Search for values
        assert!(bst.search(4, &mut ctx));
        assert!(!bst.search(9, &mut ctx));
        
        // Perform traversal
        let traversal = bst.inorder_traversal(&mut ctx);
        ctx.track_value("inorder_result", &format!("{:?}", traversal));
        
        ctx.finalize();
    }

    #[test] 
    fn test_binary_tree_build() {
        let mut ctx = VizContext::new("binary_tree_build");
        
        // Manually create a binary tree
        let root = TreeNode {
            value: 10,
            left: Some(Box::new(TreeNode {
                value: 5,
                left: Some(Box::new(TreeNode {
                    value: 2,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(TreeNode {
                    value: 7,
                    left: None,
                    right: None,
                })),
            })),
            right: Some(Box::new(TreeNode {
                value: 15,
                left: Some(Box::new(TreeNode {
                    value: 12,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(TreeNode {
                    value: 18,
                    left: None,
                    right: None,
                })),
            })),
        };
        
        ctx.track_tree("manual_tree", Some(&root), "Manual Binary Tree");
        ctx.add_step("Created complete binary tree");
        
        // Simulate operations on the tree
        ctx.highlight_tree_node("manual_tree", "0_0"); // Root
        ctx.add_step("Highlighting root node");
        
        ctx.compare_tree_nodes("manual_tree", "1_0", "1_1"); // Compare left and right children
        ctx.add_step("Comparing left and right subtrees");
        
        ctx.track_tree_traversal("manual_tree", vec!["0_0".to_string(), "1_0".to_string(), "2_0".to_string()]);
        ctx.add_step("Traversing to leftmost node");
        
        ctx.finalize();
    }

    #[test]
    fn test_heap_operations() {
        let mut ctx = VizContext::new("heap_operations");
        
        // Create a max heap structure
        let heap_values = vec![100, 85, 90, 70, 80, 75, 60];
        let mut heap_nodes = Vec::new();
        
        // Build heap level by level for visualization
        for (i, &value) in heap_values.iter().enumerate() {
            heap_nodes.push(TreeNode {
                value,
                left: None,
                right: None,
            });
            ctx.track_value("heap_size", &format!("{}", i + 1));
        }
        
        // Build the tree structure manually for heap
        let root = TreeNode {
            value: 100,
            left: Some(Box::new(TreeNode {
                value: 85,
                left: Some(Box::new(TreeNode {
                    value: 70,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(TreeNode {
                    value: 80,
                    left: None,
                    right: None,
                })),
            })),
            right: Some(Box::new(TreeNode {
                value: 90,
                left: Some(Box::new(TreeNode {
                    value: 75,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(TreeNode {
                    value: 60,
                    left: None,
                    right: None,
                })),
            })),
        };
        
        ctx.track_tree("max_heap", Some(&root), "Max Heap");
        ctx.add_step("Created max heap structure");
        
        // Simulate heap operations
        ctx.highlight_tree_node("max_heap", "0_0");
        ctx.add_step("Max element is at root");
        
        ctx.compare_tree_nodes("max_heap", "0_0", "1_0");
        ctx.add_step("Comparing parent with left child");
        
        ctx.compare_tree_nodes("max_heap", "0_0", "1_1");
        ctx.add_step("Comparing parent with right child");
        
        ctx.finalize();
    }
}
