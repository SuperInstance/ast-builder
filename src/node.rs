//! AST node types.

/// Unique identifier for a node in the AST.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub struct NodeId(u32);

impl NodeId {
    pub(crate) fn new(id: u32) -> Self {
        Self(id)
    }

    /// Returns the raw id.
    pub fn as_u32(self) -> u32 {
        self.0
    }
}

/// The kind of an AST node.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NodeKind {
    /// Root/module node.
    Module,
    /// Function definition.
    Function { name: String },
    /// Block of statements.
    Block,
    /// Variable declaration.
    VarDecl { name: String },
    /// Binary expression.
    BinaryOp { op: String },
    /// Unary expression.
    UnaryOp { op: String },
    /// Literal value.
    Literal { value: String },
    /// Identifier reference.
    Ident { name: String },
    /// Return statement.
    Return,
    /// If expression/statement.
    If,
    /// While loop.
    While,
}

/// An AST node with children.
#[derive(Clone, Debug)]
pub struct AstNode {
    pub id: NodeId,
    pub kind: NodeKind,
    pub children: Vec<NodeId>,
    pub parent: Option<NodeId>,
}

impl AstNode {
    /// Create a new AST node.
    pub fn new(id: NodeId, kind: NodeKind) -> Self {
        Self {
            id,
            kind,
            children: Vec::new(),
            parent: None,
        }
    }

    /// Add a child to this node.
    pub fn add_child(&mut self, child: NodeId) {
        self.children.push(child);
    }

    /// Number of children.
    pub fn child_count(&self) -> usize {
        self.children.len()
    }

    /// Returns true if this node has no children.
    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    /// Check if this node matches a specific kind pattern.
    pub fn is_function(&self) -> bool {
        matches!(self.kind, NodeKind::Function { .. })
    }

    /// Check if this is a literal node.
    pub fn is_literal(&self) -> bool {
        matches!(self.kind, NodeKind::Literal { .. })
    }

    /// Get the node name if it has one.
    pub fn name(&self) -> Option<&str> {
        match &self.kind {
            NodeKind::Function { name } | NodeKind::VarDecl { name } | NodeKind::Ident { name } => Some(name),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_creation() {
        let id = NodeId::new(0);
        let node = AstNode::new(id, NodeKind::Module);
        assert!(node.is_leaf());
        assert_eq!(node.child_count(), 0);
    }

    #[test]
    fn test_add_children() {
        let id = NodeId::new(0);
        let mut node = AstNode::new(id, NodeKind::Block);
        node.add_child(NodeId::new(1));
        node.add_child(NodeId::new(2));
        assert_eq!(node.child_count(), 2);
        assert!(!node.is_leaf());
    }

    #[test]
    fn test_function_node() {
        let node = AstNode::new(NodeId::new(0), NodeKind::Function { name: "main".into() });
        assert!(node.is_function());
        assert_eq!(node.name(), Some("main"));
    }

    #[test]
    fn test_literal_node() {
        let node = AstNode::new(NodeId::new(0), NodeKind::Literal { value: "42".into() });
        assert!(node.is_literal());
        assert!(node.name().is_none());
    }

    #[test]
    fn test_node_id_ordering() {
        let a = NodeId::new(0);
        let b = NodeId::new(1);
        assert!(a < b);
    }
}
