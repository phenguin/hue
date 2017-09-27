use errors::*;
use std::mem;
use std::convert::{From, TryFrom};

#[derive(Debug, Clone)]
struct Forest<T> {
    trees: Vec<Tree<T>>,
}

#[derive(Debug, Clone)]
struct Tree<T> {
    root: T,
    children: Forest<T>,
}

impl<T> Tree<T> {
    fn new(x: T) -> Self {
        Tree {
            root: x,
            children: Forest { trees: Vec::new() },
        }
    }
}

impl<T> TryFrom<Forest<T>> for Tree<T> {
    type Error = Error;
    fn try_from(forest: Forest<T>) -> Res<Self> {
        let mut trees = forest.trees;
        if trees.len() == 1 {
            return Ok(trees.pop().unwrap());
        }
        Err("Cant convert non-singular forest into tree".into())
    }
}

impl<T> From<Tree<T>> for Forest<T> {
    fn from(tree: Tree<T>) -> Self {
        Forest { trees: vec![tree] }
    }
}

/// //////////////////////////
/// Opeations on trees/forests
/// //////////////////////////
#[derive(Debug, Clone)]
enum NodeOp<T> {
    Delete(usize),
    Change(T),
    Insert(usize, T),
}
use self::NodeOp::*;

impl<T> NodeOp<T> {
    fn is_insert(&self) -> bool {
        match self {
            &Insert(..) => true,
            _ => false,
        }
    }

    fn change<'a>(node: &'a mut T, new: T) -> T {
        mem::replace(node, new)
    }

    fn insert<'a>(forest: &'a mut Forest<T>, i: usize, new: T) {
        forest.trees.insert(i, Tree::new(new))
    }

    fn delete<'a>(forest: &'a mut Forest<T>, i: usize) {
        let mut rest = forest.trees.split_off(i);
        let mut deleted = rest.remove(0);
        forest.trees.append(&mut deleted.children.trees);
        forest.trees.append(&mut rest);
    }
}

type Path = Vec<usize>;

#[derive(Debug, Clone)]
struct TreeOp<T> {
    path: Path,
    op: NodeOp<T>,
}

impl<T> Tree<T> {
    fn path_to_mut(&mut self, mut path: Path) -> &mut Self {
        if path.is_empty() {
            return self;
        }

        let i = path.remove(0);
        let next = &mut self.children.trees[i];
        next.path_to_mut(path)
    }

    fn apply_mut(&mut self, TreeOp { path, op }: TreeOp<T>) {
        let part = self.path_to_mut(path);
        match op {
            Change(new) => {
                NodeOp::change(&mut part.root, new);
            }
            Delete(i) => NodeOp::delete(&mut part.children, i),
            Insert(i, new) => NodeOp::insert(&mut part.children, i, new),
        }
    }
}
