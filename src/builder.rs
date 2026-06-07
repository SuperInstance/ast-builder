//! AST builder for constructing trees.

use crate::node::{AstNode, NodeId, NodeKind};

/// An arena-based AST builder.
pub struct AstBuilder {
    nodes: Vec<AstNode>,
}

impl AstBuilder {
    /// Create a new empty AST builder.
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    /// Allocate a new node and return its ID.
    pub fn alloc(&mut self, kind: NodeKind) -> NodeId {
        let id = NodeId::new(self.nodes.len() as u32);
        self.nodes.push(AstNode::new(id, kind));
        id
    }

    /// Create a module node.
    pub fn module(&mut self) -> NodeId {
        self.alloc(NodeKind::Module)
    }

    /// Create a function node.
    pub fn function(&mut self, name: &str) -> NodeId {
        self.alloc(NodeKind::Function { name: name.to_string() })
    }

    /// Create a block node.
    pub fn block(&mut self) -> NodeId {
        self.alloc(NodeKind::Block)
    }

    /// Create a variable declaration.
    pub fn var_decl(&mut self, name: &str) -> NodeId {
        self.alloc(NodeKind::VarDecl { name: name.to_string() })
    }

    /// Create a binary operation.
    pub fn binary_op(&mut self, op: &str) -> NodeId {
        self.alloc(NodeKind::BinaryOp { op: op.to_string() })
    }

    /// Create a unary operation.
    pub fn unary_op(&mut self, op: &str) -> NodeId {
        self.alloc(NodeKind::UnaryOp { op: op.to_string() })
    }

    /// Create a literal.
    pub fn literal(&mut self, value: &str) -> NodeId {
        self.alloc(NodeKind::Literal { value: value.to_string() })
    }

    /// Create an identifier.
    pub fn ident(&mut self, name: &str) -> NodeId {
        self.alloc(NodeKind::Ident { name: name.to_string() })
    }

    /// Create a return node.
    pub fn return_stmt(&mut self) -> NodeId {
        self.alloc(NodeKind::Return)
    }

    /// Create an if node.
    pub fn if_stmt(&mut self) -> NodeId {
        self.alloc(NodeKind::If)
    }

    /// Create a while node.
    pub fn while_loop(&mut self) -> NodeId {
        self.alloc(NodeKind::While)
    }

    /// Add a child to a parent node.
    pub fn add_child(&mut self, parent: NodeId, child: NodeId) {
        if let Some(p) = self.nodes.get_mut(parent.as_u32() as usize) {
            p.add_child(child);
        }
        if let Some(c) = self.nodes.get_mut(child.as_u32() as usize) {
            c.parent = Some(parent);
        }
    }

    /// Get a reference to a node.
    pub fn get(&self, id: NodeId) -> Option<&AstNode> {
        self.nodes.get(id.as_u32() as usize)
    }

    /// Get a mutable reference to a node.
    pub fn get_mut(&mut self, id: NodeId) -> Option<&mut AstNode> {
        self.nodes.get_mut(id.as_u32() as usize)
    }

    /// Total number of nodes allocated.
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Returns true if no nodes allocated.
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Returns the arena for traversal.
    pub fn arena(&self) -> &[AstNode] {
        &self.nodes
    }
}

impl Default for AstBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_basic() {
        let mut b = AstBuilder::new();
        let id = b.literal("42");
        let node = b.get(id).unwrap();
        assert!(node.is_literal());
    }

    #[test]
    fn test_builder_tree() {
        let mut b = AstBuilder::new();
        let module = b.module();
        let func = b.function("main");
        let block = b.block();
        b.add_child(module, func);
        b.add_child(func, block);
        assert_eq!(b.get(module).unwrap().child_count(), 1);
        assert_eq!(b.get(func).unwrap().child_count(), 1);
    }

    #[test]
    fn test_builder_parent_tracking() {
        let mut b = AstBuilder::new();
        let parent = b.module();
        let child = b.literal("1");
        b.add_child(parent, child);
        assert_eq!(b.get(child).unwrap().parent, Some(parent));
    }

    #[test]
    fn test_builder_node_types() {
        let mut b = AstBuilder::new();
        let v = b.var_decl("x");
        let binop = b.binary_op("+");
        let unop = b.unary_op("-");
        let ident = b.ident("y");
        let ret = b.return_stmt();
        let iff = b.if_stmt();
        let wh = b.while_loop();
        assert!(matches!(b.get(v).unwrap().kind, NodeKind::VarDecl { .. }));
        assert!(matches!(b.get(binop).unwrap().kind, NodeKind::BinaryOp { .. }));
        assert!(matches!(b.get(unop).unwrap().kind, NodeKind::UnaryOp { .. }));
        assert!(matches!(b.get(ident).unwrap().kind, NodeKind::Ident { .. }));
        assert!(matches!(b.get(ret).unwrap().kind, NodeKind::Return));
        assert!(matches!(b.get(iff).unwrap().kind, NodeKind::If));
        assert!(matches!(b.get(wh).unwrap().kind, NodeKind::While));
    }

    #[test]
    fn test_builder_arena() {
        let mut b = AstBuilder::new();
        b.module();
        b.function("f");
        assert_eq!(b.arena().len(), 2);
    }

    #[test]
    fn test_complex_expression() {
        let mut b = AstBuilder::new();
        // Build: x + (y * 2)
        let plus = b.binary_op("+");
        let x = b.ident("x");
        let mul = b.binary_op("*");
        let y = b.ident("y");
        let two = b.literal("2");
        b.add_child(plus, x);
        b.add_child(plus, mul);
        b.add_child(mul, y);
        b.add_child(mul, two);
        let node = b.get(plus).unwrap();
        assert_eq!(node.child_count(), 2);
    }
}
