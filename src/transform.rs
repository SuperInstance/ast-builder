//! AST transformation utilities.

use crate::builder::AstBuilder;
use crate::node::{NodeId, NodeKind};

/// A transformation that can modify an AST.
pub trait Transform {
    /// Transform a node. Returns the (possibly new) node ID.
    fn transform(&self, builder: &mut AstBuilder, id: NodeId) -> NodeId;
}

/// A transformation that replaces all literals with a new value.
pub struct ReplaceLiterals {
    pub new_value: String,
}

impl Transform for ReplaceLiterals {
    fn transform(&self, builder: &mut AstBuilder, id: NodeId) -> NodeId {
        let kind = builder.get(id).map(|n| n.kind.clone());
        let Some(kind) = kind else { return id };

        if let NodeKind::Literal { .. } = &kind {
            let children = builder.get(id).map(|n| n.children.clone()).unwrap_or_default();
            let new_id = builder.literal(&self.new_value);
            for child in children {
                builder.add_child(new_id, child);
            }
            new_id
        } else {
            id
        }
    }
}

/// A transformation that wraps each function's non-block children in a block.
pub struct EnsureBlock;

impl Transform for EnsureBlock {
    fn transform(&self, builder: &mut AstBuilder, id: NodeId) -> NodeId {
        let info = builder.get(id).map(|n| (n.kind.clone(), n.children.clone()));
        let Some((kind, children)) = info else { return id };

        if let NodeKind::Function { name } = &kind {
            let has_block = children.iter().any(|&c| {
                matches!(
                    builder.get(c).map(|n| &n.kind),
                    Some(NodeKind::Block)
                )
            });
            if !has_block {
                let block = builder.block();
                let func = builder.function(name);
                builder.add_child(func, block);
                for child in children {
                    builder.add_child(block, child);
                }
                return func;
            }
        }
        id
    }
}

/// Apply a transform to a node.
pub fn apply_transform(
    builder: &mut AstBuilder,
    root: NodeId,
    transform: &dyn Transform,
) -> NodeId {
    transform.transform(builder, root)
}

/// Deep-clone a subtree from one builder into another, returning the new root ID.
pub fn deep_clone(target: &mut AstBuilder, source: &AstBuilder, id: NodeId) -> NodeId {
    let node = source.get(id).unwrap();
    let kind = node.kind.clone();
    let children = node.children.clone();
    let new_id = target.alloc(kind);
    for child in children {
        let new_child = deep_clone(target, source, child);
        target.add_child(new_id, new_child);
    }
    new_id
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_literals() {
        let mut b = AstBuilder::new();
        let lit = b.literal("0");
        let id_node = b.ident("x");
        let binop = b.binary_op("+");
        b.add_child(binop, lit);
        b.add_child(binop, id_node);

        let transform = ReplaceLiterals { new_value: "1".into() };
        let new_lit = transform.transform(&mut b, lit);
        let node = b.get(new_lit).unwrap();
        assert!(matches!(&node.kind, NodeKind::Literal { value } if value == "1"));
    }

    #[test]
    fn test_replace_preserves_non_literal() {
        let mut b = AstBuilder::new();
        let id_node = b.ident("x");
        let transform = ReplaceLiterals { new_value: "1".into() };
        let result = transform.transform(&mut b, id_node);
        assert_eq!(result, id_node);
    }

    #[test]
    fn test_deep_clone() {
        let mut b = AstBuilder::new();
        let root = b.module();
        let func = b.function("f");
        b.add_child(root, func);

        let mut b2 = AstBuilder::new();
        let cloned = deep_clone(&mut b2, &b, root);
        assert_eq!(b2.get(cloned).unwrap().child_count(), 1);
    }

    #[test]
    fn test_apply_transform() {
        let mut b = AstBuilder::new();
        let lit = b.literal("old");
        let transform = ReplaceLiterals { new_value: "new".into() };
        let result = apply_transform(&mut b, lit, &transform);
        let node = b.get(result).unwrap();
        if let NodeKind::Literal { value } = &node.kind {
            assert_eq!(value, "new");
        }
    }

    #[test]
    fn test_ensure_block() {
        let mut b = AstBuilder::new();
        let func = b.function("f");
        let stmt = b.var_decl("x");
        b.add_child(func, stmt);

        let transform = EnsureBlock;
        let new_func = transform.transform(&mut b, func);
        let node = b.get(new_func).unwrap();
        // Should have wrapped in a block
        assert_eq!(node.child_count(), 1);
    }
}
