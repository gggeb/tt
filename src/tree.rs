use serde::{Serialize, Deserialize};

/// An infinitely branching tree.
/// # Example
/// ```
/// use tt::tree::Tree;
///
/// let mut tree = Tree::new(0);
/// assert_eq!(tree.label(), &0);
/// assert_eq!(tree.children(), 0);
///
/// tree.create_subtree(1);
/// assert_eq!(tree.children(), 1);
/// 
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct Tree<T> {
    label: T,
    children: Vec<Tree<T>>
}

impl<T> Tree<T> {
    /// Constructs and returns a new `Tree`.
    pub fn new(label: T) -> Self {
        Self {
            label,
            children: Vec::new()
        }
    }

    pub fn label(&self) -> &T { &self.label }
    /// Returns how many children this tree contains.
    pub fn children(&self) -> usize { self.children.len() }

    pub fn child_at(&self, i: usize) -> Option<&Self> { self.children.get(i) }
    pub fn child_at_mut(&mut self, i: usize) -> Option<&mut Self> { self.children.get_mut(i) }

    pub fn create_subtree(&mut self, label: T) {
        self.children.push(Tree::new(label));
    }
}
