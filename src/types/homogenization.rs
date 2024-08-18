use super::array::{first_end_handle, Stack};
use anyhow::Result;
use indexmap::IndexMap;
use indextree::Arena;
use serde_json::Value;

pub struct HomoArr<'a> {
    arr: &'a [Value],
    len: usize,
}

impl<'a> HomoArr<'a> {
    pub fn new(arr: &'a [Value]) -> Self {
        Self { arr, len: 0 }
    }

    pub fn run(&self) -> Result<()> {
        let mut max = &self.arr[0];
        let mut len = self.arr.len();
        for per in self.arr {
            //let mut fetch = FetchLen::new(per);
            //let ret = fetch.len()?;
            //if ret > len {
            //    len = ret;
            //    max = per;
            //}
        }

        //println!("{}", max);

        Ok(())
    }

    fn homo(&self, template: &'a Value) {}
}

pub struct BuildTree {
    stack: Stack<TreeFork>,
}

pub fn build_tree<'a>(val: &'a Value) {
    if val.is_string() {
        let treenode = TreeNode::new(first_end_handle(val.to_string()));
    } else if val.is_array() {
    } else {
    }
}

#[derive(Debug, Clone)]
pub struct TreeNode<T> {
    pub node: T,
}

impl<T> TreeNode<T> {
    pub fn new(node: T) -> Self {
        Self { node }
    }
}

#[derive(Debug, Clone)]
pub struct TreeFork<T> {
    pub title: String,
    pub fork: Vec<T>,
}

impl<T> TreeFork<T> {
    pub fn new() -> Self {
        Self {
            title: String::new(),
            fork: Vec::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            title: String::new(),
            fork: Vec::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, node: T) {
        self.fork.push(node)
    }
}
