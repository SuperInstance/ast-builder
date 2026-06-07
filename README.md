# ast-builder

Typed AST construction with visitor pattern and tree traversal for compiler infrastructure.

## Features

- **Arena-based allocation** — Nodes stored in a flat vector with ID-based references
- **Typed node kinds** — Modules, functions, blocks, expressions, literals, etc.
- **Visitor pattern** — Pre/post-order traversal with skip/stop control
- **Tree traversal** — Pre-order, post-order, and breadth-first
- **Transformations** — Modify or clone subtrees
- **Zero dependencies** — Pure `std` implementation

## License

MIT
