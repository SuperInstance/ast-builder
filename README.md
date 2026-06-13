# AST Builder

**A Rust library for programmatically constructing Abstract Syntax Trees (ASTs)** for compilers, linters, transpilers, and code generation tools. Instead of parsing source text, you build tree nodes directly in code.

## Why It Matters

Most compiler tools focus on *parsing* text into an AST. But many use cases need the reverse: building an AST from scratch. Code generators (like protocol buffer compilers), AST-to-AST transpilers, macro systems, and refactoring tools all need to construct syntax trees programmatically. This crate provides a fluent, type-safe API for assembling ASTs without dealing with string concatenation or token streams.

An AST (Abstract Syntax Tree) is a tree representation of source code where each node represents a language construct — expressions, statements, declarations. Unlike a parse tree, an AST omits syntactic details like parentheses and whitespace, keeping only semantically meaningful structure.

## How It Works

The crate models a generic imperative language with three core type hierarchies:

1. **`Expr`** — Expressions that produce values: literals (`42`, `"hello"`), variables, binary ops (`a + b`), unary ops (`!x`), function calls, member access, array indexing, lambda abstractions, conditionals (`if-else` as expressions), and `match` arms with destructuring patterns.

2. **`Stmt`** — Statements that produce effects: `let` bindings, expression statements, `return`, blocks, `if`/`while`/`for` control flow, function definitions, struct/enum declarations, and `impl` blocks.

3. **`Pattern`** — Match patterns for destructuring: wildcards (`_`), identifiers, literals, nested destructuring, and OR-patterns.

`AstBuilder` accumulates statements and provides convenience constructors (`fn let_stmt()`, `fn func()`, `fn binary()`, `fn call()`) to reduce boilerplate when assembling nodes. All types derive `Debug`, `Clone`, and `PartialEq` for easy testing and transformation.

## Quick Start

```rust
use ast_builder::{AstBuilder, Expr, Stmt, BinOp, Literal};

let mut b = AstBuilder::new();

// Build: fn add(a, b) { a + b }
b.push(Stmt::Func {
    name: "add".into(),
    params: vec!["a".into(), "b".into()],
    ret: None,
    body: vec![
        Stmt::Return(Some(
            AstBuilder::binary(
                AstBuilder::var("a"),
                BinOp::Add,
                AstBuilder::var("b"),
            )
        ))
    ],
});

let ast = b.build();
```

## API

- **`AstBuilder`** — Accumulator with helper constructors and `build()` → `Vec<Stmt>`
- **`Expr`** enum — Literals, variables, binary/unary ops, calls, member access, indexing, lambdas, if-expressions, match-expressions
- **`Stmt`** enum — Let bindings, returns, blocks, control flow, function/struct/enum/impl declarations
- **`Type`** enum — `Int`, `Float`, `Bool`, `String`, `Void`, `Custom`, `Array`, `Optional`
- **`Pattern`** enum — Wildcard, Ident, Literal, Destructure, Or
- **`Literal`** / **`BinOp`** / **`UnOp`** — Supporting enums for values and operators

## Architecture Notes

This crate provides the construction layer for AST tooling in the SuperInstance compiler infrastructure. It pairs naturally with `ast-visitor` (for traversal) and `ast-diff` (for structural comparison). See the [architecture overview](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md).

## License

MIT
