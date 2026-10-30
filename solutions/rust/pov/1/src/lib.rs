use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tree<T: Debug + Ord> {
    value: T,
    child: Vec<Tree<T>>,
}

impl<T: Debug + Ord> Tree<T> {
    pub fn new(label: T) -> Self {
        Self {
            value: label,
            child: Vec::new(),
        }
    }

    /// Builder-method for constructing a tree with children
    pub fn with_child(mut self, child: Self) -> Self {
        self.child.push(child);
        self.child.sort();
        self
    }

    // Helper to find a path of references from root to a target value
    fn get_path_to<'a>(&'a self, target: &T, path: &mut Vec<&'a T>) -> bool {
        path.push(&self.value);

        if self.value == *target {
            return true;
        }

        for child in &self.child {
            if child.get_path_to(target, path) {
                return true;
            }
        }

        path.pop();
        false
    }

    pub fn path_between<'a>(&'a mut self, from: &T, to: &T) -> Option<Vec<&'a T>> {
        let mut path_from = Vec::new();
        let mut path_to = Vec::new();

        if !self.get_path_to(from, &mut path_from) || !self.get_path_to(to, &mut path_to) {
            return None;
        }

        // Find Least Common Ancestor (LCA)
        let mut common_len = 0;
        for (a, b) in path_from.iter().zip(path_to.iter()) {
            if a == b {
                common_len += 1;
            } else {
                break;
            }
        }

        let mut result = Vec::new();
        // Go up from 'from' to LCA (in reverse)
        for node in path_from[common_len - 1..].iter().rev() {
            result.push(*node);
        }
        // Go down from LCA to 'to'
        for node in &path_to[common_len..] {
            result.push(*node);
        }

        Some(result)
    }

    pub fn pov_from(&mut self, from: &T) -> bool {
        // 1. Verify that 'from' actually exists in the tree
        let mut path = Vec::new();
        let exists = self.get_path_to(from, &mut path);
        
        // 'path' goes out of scope right here, completely ending the immutable borrow of self!
        if !exists {
            return false;
        }

        // 2. If it's already the root, nothing needs to change
        if self.value == *from {
            return true;
        }

        // 3. Extract and re-root using the external 'from' reference
        if let Some(extracted) = self.extract_subtree(from) {
            *self = extracted;
            true
        } else {
            false
        }
    }

    // Helper to extract and re-root
    fn extract_subtree(&mut self, target: &T) -> Option<Self> {
        if self.value == *target {
            return Some(Self {
                value: unsafe { std::ptr::read(&self.value) },
                child: unsafe { std::ptr::read(&self.child) },
            });
        }

        for (i, child) in self.child.iter_mut().enumerate() {
            if let Some(mut found) = child.extract_subtree(target) {
                // Remove child from current parent
                self.child.remove(i);
                
                // Push the current tree (minus the extracted child) into the found subtree as a child, sorted
                let mut parent_copy = Self {
                    value: unsafe { std::ptr::read(&self.value) },
                    child: unsafe { std::ptr::read(&self.child) },
                };
                
                // Merge remaining children upward
                parent_copy.child.append(&mut found.child);
                parent_copy.child.sort();
                
                found.child.push(parent_copy);
                found.child.sort();
                return Some(found);
            }
        }
        None
    }
}