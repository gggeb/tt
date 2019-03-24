use serde::{Serialize, Deserialize};

use super::tree::Tree;

/// A movement within a tree.
/// * `Jump::Up` - To the parent of focused subtree.
/// * `Jump::Down` - To the first child of focused subtree.
/// * `Jump::Lateral(n)` - To the sibling subtree of distance `n`.
pub enum Jump {
    Up,
    Down,
    Lateral(i32)
}

/// A path to a point within a tree.
pub type Path = Vec<usize>;

/// A tool for building trees.
/// # Example
/// ```
/// use tt::focus::{Jump, Focus};
///
/// let mut focus = Focus::new(0);
/// assert_eq!(focus.focused().label(), &0);
///
/// focus.create_subtree(1);
/// assert_eq!(focus.focused().label(), &1);
/// assert_eq!(focus.path().len(), 1);
///
/// focus.jump(Jump::Up);
/// assert_eq!(focus.focused().label(), &0);
///
/// focus.create_subtree(2);
/// assert_eq!(focus.focused().label(), &2);
///
/// focus.jump(Jump::Lateral(-1));
/// assert_eq!(focus.focused().label(), &1);
///
/// focus.create_subtree(3);
/// assert_eq!(focus.labels(), vec![&0, &1, &3]);
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct Focus<T> {
    tree: Tree<T>,
    path: Path
}

impl<T> Focus<T> {
    /// Constructs and returns a new `Focus` from provided `Tree`.
    /// * If `None` is provided as `path` - the focus will have an empty path.
    /// * If `Some(path)` is provided as `path` - the focus path will be `path`.
    /// Returns none if the provided path doesn't point to an existing point on the tree.
    /// # Example
    /// ```
    /// use tt::tree::Tree;
    /// use tt::focus::Focus;
    ///
    /// let mut tree = Tree::new(0);
    /// tree.create_subtree(1);
    /// 
    /// // Incorrect path.
    /// assert!(Focus::from(tree, Some(vec![0, 1])).is_none());
    /// ```
    /// ```
    /// use tt::tree::Tree;
    /// use tt::focus::Focus;
    ///
    /// let mut tree = Tree::new(0);
    /// tree.create_subtree(1);
    ///
    /// // Correct path to the label '1'.
    /// let focus = Focus::from(tree, Some(vec![0]));
    /// assert!(focus.is_some());
    /// assert_eq!(focus.unwrap().focused().label(), &1);
    /// ```
    pub fn from(tree: Tree<T>, path: Option<Path>) -> Option<Self> {
        let focus = Self {
            tree,
            path: match path {
                Some(path) => path,
                None       => Path::new()
            }
        };

        match focus.at_path(&focus.path) {
            Some(_) => Some(focus),
            None    => None
        }
    }

    /// Constructs and returns a new `Focus` from provided `label`.
    pub fn new(label: T) -> Self {
        Self::from(Tree::new(label), None).unwrap()
    }

    /// Returns the path to the focused tree/subtree.
    pub fn path(&self) -> &Path { &self.path }

    /// Returns the tree/subtree reached by `path`.
    fn at_path(&self, path: &Path) -> Option<&Tree<T>> {
        let mut cur = &self.tree;
        for i in path.iter() {
            match cur.child_at(*i) {
                Some(child) => cur = child,
                None => return None
            };
        }

        Some(cur)
    }

    /// Returns the currently focused tree/subtree.
    pub fn focused(&self) -> &Tree<T> {
        self.at_path(&self.path).unwrap()
    }

    /// Returns a mutable reference to the focused tree/subtree.
    fn focused_mut(&mut self) -> &mut Tree<T> {
        let mut cur = &mut self.tree;
        for i in self.path.iter() {
            cur = cur.child_at_mut(*i).unwrap();
        }

        cur
    }

    /// Changes focus according to the provided `jump`.
    pub fn jump(&mut self, jump: Jump) {
        match jump {
            Jump::Up => { self.path.pop(); }
            Jump::Down if self.focused().children() > 0 => { self.path.push(0) }
            Jump::Lateral(x) if self.path.len() > 0 => {
                let o = self.path.pop().unwrap() as i32;
                let ub = self.focused().children() as i32;

                let n = if o + x < 0 { 0 } 
                        else if o + x >= ub { (ub - 1) }
                        else { o + x };

                self.path.push(n as usize);
            }
            _ => {}
        }
    }

    /// Creates new child subtree of focused tree/subtree.
    pub fn create_subtree(&mut self, label: T) {
        self.focused_mut().create_subtree(label);
        self.path.push(self.focused().children() - 1);
    }

    /// Get all labels along current path.
    pub fn labels(&self) -> Vec<&T> {
        let mut labels = Vec::new();
        self.path.iter().fold(Vec::new(), |mut acc, x| {
            labels.push(self.at_path(&acc).unwrap().label());
            acc.push(*x);
            acc
        });
        labels.push(self.focused().label());
        labels
    }
}
