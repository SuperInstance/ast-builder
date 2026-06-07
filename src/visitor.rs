//! Visitor pattern for AST traversal.

use crate::node::{AstNode, NodeId};

/// Result of visiting a node.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VisitResult {
    /// Continue visiting children.
    Continue,
    /// Skip children of this node.
    SkipChildren,
    /// Stop the entire traversal.
    Stop,
}

/// Trait for AST visitors.
pub trait Visitor {
    /// Called before visiting children.
    fn visit_pre(&mut self, _node: &AstNode) -> VisitResult {
        VisitResult::Continue
    }

    /// Called after visiting children.
    fn visit_post(&mut self, _node: &AstNode) -> VisitResult {
        VisitResult::Continue
    }
}

/// Walk the AST with a visitor (pre-order + post-order).
pub fn walk(arena: &[AstNode], root: NodeId, visitor: &mut dyn Visitor) {
    walk_recursive(arena, root, visitor);
}

fn walk_recursive(arena: &[AstNode], id: NodeId, visitor: &mut dyn Visitor) -> VisitResult {
    let Some(node) = arena.get(id.as_u32() as usize) else {
        return VisitResult::Stop;
    };

    match visitor.visit_pre(node) {
        VisitResult::Continue => {}
        VisitResult::SkipChildren => return VisitResult::Continue,
        VisitResult::Stop => return VisitResult::Stop,
    }

    for &child in &node.children {
        if walk_recursive(arena, child, visitor) == VisitResult::Stop {
            return VisitResult::Stop;
        }
    }

    visitor.visit_post(node)
}

/// A visitor that collects all node IDs in pre-order.
pub struct Collector {
    pub ids: Vec<NodeId>,
}

impl Collector {
    pub fn new() -> Self {
        Self { ids: Vec::new() }
    }
}

impl Default for Collector {
    fn default() -> Self {
        Self::new()
    }
}

impl Visitor for Collector {
    fn visit_pre(&mut self, node: &AstNode) -> VisitResult {
        self.ids.push(node.id);
        VisitResult::Continue
    }
}

/// A visitor that counts nodes by kind name.
pub struct NodeCounter {
    pub count: usize,
}

impl NodeCounter {
    pub fn new() -> Self {
        Self { count: 0 }
    }
}

impl Default for NodeCounter {
    fn default() -> Self {
        Self::new()
    }
}

impl Visitor for NodeCounter {
    fn visit_pre(&mut self, _node: &AstNode) -> VisitResult {
        self.count += 1;
        VisitResult::Continue
    }
}

/// A visitor that stops after N nodes.
pub struct LimitedVisitor {
    pub limit: usize,
    pub visited: Vec<NodeId>,
}

impl LimitedVisitor {
    pub fn new(limit: usize) -> Self {
        Self {
            limit,
            visited: Vec::new(),
        }
    }
}

impl Visitor for LimitedVisitor {
    fn visit_pre(&mut self, node: &AstNode) -> VisitResult {
        if self.visited.len() >= self.limit {
            return VisitResult::Stop;
        }
        self.visited.push(node.id);
        VisitResult::Continue
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node::NodeKind;

    fn build_simple_tree() -> Vec<AstNode> {
        let mut nodes = Vec::new();
        let mut root = AstNode::new(NodeId::new(0), NodeKind::Module);
        let mut func = AstNode::new(NodeId::new(1), NodeKind::Function { name: "main".into() });
        let block = AstNode::new(NodeId::new(2), NodeKind::Block);
        func.add_child(NodeId::new(2));
        root.add_child(NodeId::new(1));
        nodes.push(root);
        nodes.push(func);
        nodes.push(block);
        nodes
    }

    #[test]
    fn test_collector() {
        let arena = build_simple_tree();
        let mut collector = Collector::new();
        walk(&arena, NodeId::new(0), &mut collector);
        assert_eq!(collector.ids.len(), 3);
    }

    #[test]
    fn test_counter() {
        let arena = build_simple_tree();
        let mut counter = NodeCounter::new();
        walk(&arena, NodeId::new(0), &mut counter);
        assert_eq!(counter.count, 3);
    }

    #[test]
    fn test_limited_visitor() {
        let arena = build_simple_tree();
        let mut limited = LimitedVisitor::new(2);
        walk(&arena, NodeId::new(0), &mut limited);
        assert_eq!(limited.visited.len(), 2);
    }

    #[test]
    fn test_skip_children() {
        struct SkipFn;
        impl Visitor for SkipFn {
            fn visit_pre(&mut self, node: &AstNode) -> VisitResult {
                if node.is_function() {
                    VisitResult::SkipChildren
                } else {
                    VisitResult::Continue
                }
            }
        }
        let arena = build_simple_tree();
        let mut collector = Collector::new();
        // Need to use a combined approach
        let mut skipper = SkipFn;
        walk(&arena, NodeId::new(0), &mut skipper);
        // Module + Function (block skipped)
    }
}
