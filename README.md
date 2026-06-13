# ast-builder

**A Rust library for programmatically constructing Abstract Syntax Trees (ASTs) — type-safe compiler IR without parsing.**

An AST is a tree representation of source code where interior nodes represent language constructs (function definitions, loops, binary operations) and leaves represent atomic values (literals, identifiers). Unlike a parse tree, an AST omits syntactic punctuation (parentheses, semicolons) and retains only semantically meaningful structure. `ast-builder` provides a builder-pattern API for assembling these trees directly in Rust, bypassing the parsing stage entirely.

## Why It Matters

Compiler frontends are well-studied: lexer → parser → AST. But many tools need to construct ASTs *without* source text:

- **Code generators** — Protocol buffer compilers, ORM scaffolders, and IDL translators produce code by assembling AST nodes programmatically.
- **AST-to-AST transpilers** — Transform one language's AST into another's (e.g., TypeScript → Rust type mapping).
- **Macro systems** — Procedural macros build AST fragments that get spliced into the compilation stream.
- **Refactoring tools** — Modify existing ASTs by replacing subtrees.
- **Synthetic benchmark generation** — Create worst-case ASTs for testing compiler optimization passes.

This crate models a generic imperative/functional language with ML-style pattern matching, first-class functions (lambdas), algebraic data types (struct/enum), and trait-like impl blocks.

## How It Works

### Type Hierarchy

The AST has three primary node families:

**Expressions (`Expr`)** — Evaluate to a value:
- `Literal(Literal)` — Int, Float, Bool, Str, Null
- `Variable(String)` — Identifier reference
- `Binary { left, op, right }` — Left-associative binary op with `BinOp` ∈ {Add, Sub, Mul, Div, Mod, Eq, Ne, Lt, Gt, Le, Ge, And, Or}
- `Unary { op, operand }` — Prefix op with `UnOp` ∈ {Neg, Not}
- `Call { callee, args }` — Function application
- `Member { object, field }` — Field access
- `Index { object, index }` — Subscript
- `Lambda { params, body }` — Anonymous closure
- `If { cond, then, else_ }` — Ternary expression
- `Match { scrutinee, arms }` — Pattern match expression

**Statements (`Stmt`)** — Execute for side effects:
- `Let { pattern, ty, init }` — Binding with optional type annotation
- `Expr(Expr)` — Expression statement
- `Return(Option<Expr>)` — Early exit
- `Block(Vec<Stmt>)` — Scoped block
- `If { cond, then, else_ }` — Conditional branching
- `While { cond, body }` — Loop
- `For { pattern, iter, body }` — Iterator loop
- `Func { name, params, ret, body }` — Function definition
- `Struct { name, fields }` — Record type declaration
- `Enum { name, variants }` — Sum type declaration
- `Impl { target, methods }` — Method implementation block

**Patterns (`Pattern`)** — For destructuring in match arms and let bindings:
- `Wildcard` — `_`
- `Ident(String)` — Binding
- `Literal(Literal)` — Constant pattern
- `Destructure(Vec<(String, Pattern)>)` — Field-level nesting
- `Or(Vec<Pattern>)` — Alternative patterns

### Builder Pattern

`AstBuilder` accumulates statements via `push()` and provides convenience constructors:

```rust
// Convenience: AstBuilder::binary(left, BinOp::Add, right)
// is equivalent to:
Expr::Binary { left: Box::new(left), op: BinOp::Add, right: Box::new(right) }
```

The `build()` method consumes the builder and returns `Vec<Stmt>` — the root of the AST.

### Complexity

| Operation | Time |
|-----------|------|
| Node construction | O(1) |
| `push()` | O(1) amortized |
| `build()` | O(n) where n = stmts |
| Clone | O(n) deep copy |
| Equality | O(n) structural |

All types derive `Debug`, `Clone`, `PartialEq` — essential for AST transformation passes and snapshot testing.

## Quick Start

```rust
use ast_builder::{AstBuilder, Stmt, Expr, BinOp, Literal, Pattern};

let mut b = AstBuilder::new();

// Build: fn factorial(n) {
//   if n <= 1 { return 1 }
//   return n * factorial(n - 1)
// }
b.push(Stmt::Func {
    name: "factorial".into(),
    params: vec!["n".into()],
    ret: None,
    body: vec![
        Stmt::If {
            cond: AstBuilder::binary(
                AstBuilder::var("n"),
                BinOp::Le,
                AstBuilder::int_lit(1),
            ),
            then: vec![Stmt::Return(Some(AstBuilder::int_lit(1)))],
            else_: None,
        },
        Stmt::Return(Some(
            AstBuilder::binary(
                AstBuilder::var("n"),
                BinOp::Mul,
                AstBuilder::call(AstBuilder::var("factorial"),
                    vec![AstBuilder::binary(
                        AstBuilder::var("n"),
                        BinOp::Sub,
                        AstBuilder::int_lit(1),
                    )]),
            ),
        )),
    ],
});

let ast = b.build();
assert_eq!(ast.len(), 1);
```

## API

- **`AstBuilder`** — Accumulator: `new()`, `push(stmt)`, `build() → Vec<Stmt>`
- **Static constructors**: `let_stmt()`, `func()`, `binary()`, `call()`, `var()`, `int_lit()`, `str_lit()`
- **`Expr`** — 10 variants covering literals through match expressions
- **`Stmt`** — 11 variants covering declarations through control flow
- **`Type`** — Int, Float, Bool, String, Void, Custom, Array, Optional
- **`Pattern`** — Wildcard, Ident, Literal, Destructure, Or
- **`Param`** — { name, ty: Option<Type>, default: Option<Expr> }
- **`BinOp`** / **`UnOp`** / **`Literal`** — Operator and value enums

## Architecture Notes

This crate provides the **construction layer** (γ — generative capacity) for the SuperInstance compiler infrastructure. It pairs with `ast-visitor` (η — evaluative traversal) and `ast-diff` (structural comparison). The γ+η=C identity holds: construction defines what trees are *possible*, traversal defines what trees *mean*, and their composition C is the full compiler capability.

## References

1. Aho, Lam, Sethi, Ullman (2006). *Compilers: Principles, Techniques, and Tools* (2nd ed.). — The "Dragon Book"; standard reference for AST design.
2. Appel, A. (1998). *Modern Compiler Implementation in ML*. Cambridge. — ML-style pattern matching in ASTs.
3. Klabnik, S. & Nichols, C. (2019). *The Rust Programming Language*. — Rust enum and Box patterns for tree structures.
4. Nystrom, R. (2021). *Crafting Interpreters*. — Practical AST construction and tree-walking interpretation.

## License

MIT
