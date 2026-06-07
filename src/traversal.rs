//! Tree traversal utilities.

use crate::node::{AstNode, NodeId};

/// Traversal order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TraversalOrder {
    PreOrder,
    PostOrder,
    BreadthFirst,
}

/// Tree traversal over an arena of nodes.
pub struct Traversal<'a> {
    arena: &'a [AstNode],
    root: NodeId,
    order: TraversalOrder,
}

impl<'a> Traversal<'a> {
    /// Create a new traversal.
    pub fn new(arena: &'a [AstNode], root: NodeId, order: TraversalOrder) -> Self {
        Self { arena, root, order }
    }

    /// Collect all node IDs in the specified order.
    pub fn collect(&self) -> Vec<NodeId> {
        match self.order {
            TraversalOrder::PreOrder => self.collect_preorder(),
            TraversalOrder::PostOrder => self.collect_postorder(),
            TraversalOrder::BreadthFirst => self.collect_bfs(),
        }
    }

    fn collect_preorder(&self) -> Vec<NodeId> {
        let mut result = Vec::new();
        self.preorder_recursive(self.root, &mut result);
        result
    }

    fn preorder_recursive(&self, id: NodeId, result: &mut Vec<NodeId>) {
        let Some(node) = self.arena.get(id.as_u32() as usize) else { return };
        result.push(id);
        for &child in &node.children {
            self.preorder_recursive(child, result);
        }
    }

    fn collect_postorder(&self) -> Vec<NodeId> {
        let mut result = Vec::new();
        self.postorder_recursive(self.root, &mut result);
        result
    }

    fn postorder_recursive(&self, id: NodeId, result: &mut Vec<NodeId>) {
        let Some(node) = self.arena.get(id.as_u32() as usize) else { return };
        for &child in &node.children {
            self.postorder_recursive(child, result);
        }
        result.push(id);
    }

    fn collect_bfs(&self) -> Vec<NodeId> {
        let mut result = Vec::new();
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(self.root);
        while let Some(id) = queue.pop_front() {
            let Some(node) = self.arena.get(id.as_u32() as usize) else { continue };
            result.push(id);
            for &child in &node.children {
                queue.push_back(child);
            }
        }
        result
    }

    /// Returns the depth of the tree.
    pub fn depth(&self) -> usize {
        self.depth_recursive(self.root)
    }

    fn depth_recursive(&self, id: NodeId) -> usize {
        let Some(node) = self.arena.get(id.as_u32() as usize) else { return 0 };
        if node.children.is_empty() {
            return 1;
        }
        node.children
            .iter()
            .map(|&c| self.depth_recursive(c))
            .max()
            .unwrap_or(0)
            + 1
    }

    /// Count total nodes.
    pub fn node_count(&self) -> usize {
        self.collect_preorder().len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builder::AstBuilder;
    use crate::node::NodeKind;

    fn build_tree() -> (AstBuilder, NodeId) {
        let mut b = AstBuilder::new();
        let root = b.module();
        let func = b.function("f");
        let block = b.block();
        let stmt = b.var_decl("x");
        b.add_child(root, func);
        b.add_child(func, block);
        b.add_child(block, stmt);
        (b, root)
    }

    #[test]
    fn test_preorder() {
        let (b, root) = build_tree();
        let t = Traversal::new(b.arena(), root, TraversalOrder::PreOrder);
        let ids = t.collect();
        assert_eq!(ids.len(), 4);
        assert_eq!(ids[0], root);
    }

    #[test]
    fn test_postorder() {
        let (b, root) = build_tree();
        let t = Traversal::new(b.arena(), root, TraversalOrder::PostOrder);
        let ids = t.collect();
        assert_eq!(ids.len(), 4);
        // Leaf should come first in post-order
        assert_eq!(ids[0], b.arena().iter().find(|n| n.is_leaf()).unwrap().id);
    }

    #[test]
    fn test_bfs() {
        let (b, root) = build_tree();
        let t = Traversal::new(b.arena(), root, TraversalOrder::BreadthFirst);
        let ids = t.collect();
        assert_eq!(ids.len(), 4);
        assert_eq!(ids[0], root);
    }

    #[test]
    fn test_depth() {
        let (b, root) = build_tree();
        let t = Traversal::new(b.arena(), root, TraversalOrder::PreOrder);
        assert_eq!(t.depth(), 4);
    }

    #[test]
    fn test_node_count() {
        let (b, root) = build_tree();
        let t = Traversal::new(b.arena(), root, TraversalOrder::PreOrder);
        assert_eq!(t.node_count(), 4);
    }

    #[test]
    fn test_single_node() {
        let mut b = AstBuilder::new();
        let root = b.literal("42");
        let t = Traversal::new(b.arena(), root, TraversalOrder::PreOrder);
        assert_eq!(t.node_count(), 1);
        assert_eq!(t.depth(), 1);
    }
}
