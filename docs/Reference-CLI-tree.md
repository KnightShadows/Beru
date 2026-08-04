# Command Reference: `beru tree`

The `beru tree` command visualizes your project's resolved dependency graph as a clean, hierarchical tree in the terminal.

---

## 1. Usage Synopsis

```bash
beru tree
```

## 2. Detailed Description

When working on large C++ projects with deep dependency chains, it is often difficult to determine exactly *why* a particular library is being pulled in, or what version of a transitive dependency was ultimately chosen by the resolver.

The `beru tree` command parses the exact, deterministic output of the `Beru.lock` file and traverses it using an optimal $O(V+E)$ Depth-First Search algorithm. It prints the resulting DAG (Directed Acyclic Graph) to your terminal using elegant box-drawing characters.

### 2.1. Cycle Detection
C++ dependency graphs can sometimes feature circular dependencies or redundant subtrees (where multiple libraries depend on the exact same version of another core library). To prevent infinite loops and keep the output concise, `beru tree` automatically detects previously visited nodes.

When it encounters a node that has already been printed higher up in the current branch, it prints `(*)` next to the package name and stops traversing that specific subtree.

---

## 3. Options and Flags

*This command currently accepts no options.*

---

## 4. Examples

**Visualizing a deeply nested dependency graph:**
```bash
$ beru tree
my-app v0.1.0
├── fmt v11.0.2
└── spdlog v1.14.1
    └── fmt v11.0.2 (*)
```
*Notice how `fmt` is listed as a direct dependency of `my-app`, and also as a transitive dependency of `spdlog`. The `(*)` indicates that the `fmt` subtree was truncated because it was already resolved earlier.*
