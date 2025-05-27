// Enhanced algorithm examples with detailed visualization

use crate::VizContext;

pub fn enhanced_bubble_sort(mut arr: Vec<i32>) -> Vec<i32> {
    let mut ctx = VizContext::new("enhanced_bubble_sort");
    
    ctx.track_array("arr", &arr);
    ctx.track_var_creation("n", &arr.len().to_string());
    
    let n = arr.len();
    
    for i in 0..n {
        ctx.track_var_update("i", &i.to_string());
        ctx.add_step(&format!("Starting pass {} of {}", i + 1, n));
        
        let mut swapped = false;
        ctx.track_var_creation("swapped", "false");
        
        for j in 0..n - i - 1 {
            ctx.track_var_update("j", &j.to_string());
            
            // Compare elements
            ctx.compare_array_indices("arr", j, j + 1);
            
            if arr[j] > arr[j + 1] {
                ctx.add_step(&format!("arr[{}] ({}) > arr[{}] ({}), need to swap", 
                    j, arr[j], j + 1, arr[j + 1]));
                
                // Perform swap with visualization
                ctx.swap_array_elements("arr", &mut arr, j, j + 1);
                
                swapped = true;
                ctx.track_var_update("swapped", "true");
            } else {
                ctx.add_step(&format!("arr[{}] ({}) <= arr[{}] ({}), no swap needed", 
                    j, arr[j], j + 1, arr[j + 1]));
            }
        }
        
        if !swapped {
            ctx.add_step("No swaps in this pass - array is sorted!");
            break;
        }
    }
    
    ctx.add_step("Bubble sort completed");
    ctx.finalize();
    arr
}

pub fn enhanced_selection_sort(mut arr: Vec<i32>) -> Vec<i32> {
    let mut ctx = VizContext::new("enhanced_selection_sort");
    
    ctx.track_array("arr", &arr);
    ctx.track_var_creation("n", &arr.len().to_string());
    
    let n = arr.len();
    
    for i in 0..n {
        ctx.track_var_update("i", &i.to_string());
        ctx.add_step(&format!("Finding minimum element for position {}", i));
        
        let mut min_idx = i;
        ctx.track_var_creation("min_idx", &min_idx.to_string());
        ctx.highlight_array_indices("arr", vec![i]);
        
        for j in (i + 1)..n {
            ctx.track_var_update("j", &j.to_string());
            ctx.compare_array_indices("arr", min_idx, j);
            
            if arr[j] < arr[min_idx] {
                ctx.add_step(&format!("Found new minimum: arr[{}] ({}) < arr[{}] ({})", 
                    j, arr[j], min_idx, arr[min_idx]));
                min_idx = j;
                ctx.track_var_update("min_idx", &min_idx.to_string());
            } else {
                ctx.add_step(&format!("arr[{}] ({}) >= arr[{}] ({}), keeping current minimum", 
                    j, arr[j], min_idx, arr[min_idx]));
            }
        }
        
        if min_idx != i {
            ctx.add_step(&format!("Swapping minimum element at position {} with position {}", min_idx, i));
            ctx.swap_array_elements("arr", &mut arr, i, min_idx);
        } else {
            ctx.add_step(&format!("Element at position {} is already the minimum", i));
        }
    }
    
    ctx.add_step("Selection sort completed");
    ctx.finalize();
    arr
}

pub fn binary_search_visualization(arr: &[i32], target: i32) -> Option<usize> {
    let mut ctx = VizContext::new("binary_search");
    
    ctx.track_array("arr", arr);
    ctx.track_var_creation("target", &target.to_string());
    ctx.track_var_creation("left", "0");
    ctx.track_var_creation("right", &(arr.len() - 1).to_string());
    
    let mut left = 0;
    let mut right = arr.len() - 1;
    
    while left <= right {
        let mid = left + (right - left) / 2;
        ctx.track_var_update("mid", &mid.to_string());
        ctx.highlight_array_indices("arr", vec![left, mid, right]);
        
        ctx.add_step(&format!("Searching in range [{}..{}], checking middle element at index {}", left, right, mid));
        
        if arr[mid] == target {
            ctx.add_step(&format!("Found target {} at index {}", target, mid));
            ctx.finalize();
            return Some(mid);
        } else if arr[mid] < target {
            ctx.add_step(&format!("arr[{}] ({}) < target ({}), search right half", mid, arr[mid], target));
            left = mid + 1;
            ctx.track_var_update("left", &left.to_string());
        } else {
            ctx.add_step(&format!("arr[{}] ({}) > target ({}), search left half", mid, arr[mid], target));
            right = mid - 1;
            ctx.track_var_update("right", &right.to_string());
        }
    }
    
    ctx.add_step(&format!("Target {} not found in array", target));
    ctx.finalize();
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_enhanced_bubble_sort() {
        let arr = vec![64, 34, 25, 12, 22, 11, 90];
        let sorted = enhanced_bubble_sort(arr);
        assert_eq!(sorted, vec![11, 12, 22, 25, 34, 64, 90]);
    }
    
    #[test]
    fn test_enhanced_selection_sort() {
        let arr = vec![64, 25, 12, 22, 11];
        let sorted = enhanced_selection_sort(arr);
        assert_eq!(sorted, vec![11, 12, 22, 25, 64]);
    }
    
    #[test]
    fn test_binary_search() {
        let arr = vec![2, 3, 4, 10, 40];
        let result = binary_search_visualization(&arr, 10);
        assert_eq!(result, Some(3));
        
        let result2 = binary_search_visualization(&arr, 5);
        assert_eq!(result2, None);
    }
}
