# Concepts: The PubGrub Dependency Resolver

To master a package manager, you must understand how it selects versions. The logic that takes a massive, conflicting web of user requests and distills it into a single, concrete `Beru.lock` file is the most complex component of the entire Beru architecture.

Beru delegates this monumental task to the **PubGrub algorithm**.

---

## 1. The Core Problem: The One Definition Rule

In dynamically interpreted languages (like Node.js/JavaScript), dependency resolution is relatively forgiving. If Package A needs `LibraryX v1` and Package B needs `LibraryX v2`, npm can simply install both versions in separate nested `node_modules` folders, and the runtime will load both simultaneously.

C++ does not afford this luxury. C++ binaries are statically or dynamically linked into a single memory space. The **One Definition Rule (ODR)** strictly prohibits linking two different versions of the same library into a single executable. If you attempt this, the linker will either throw a fatal "multiple definition" error, or worse, silently produce a binary that corrupts memory at runtime due to mismatched struct sizes.

Therefore, the dependency graph in C++ must be mathematically flattened. A package manager must find exactly *one* version of every library that satisfies the constraints of every package that depends on it.

## 2. Enter PubGrub

For decades, package managers (like apt, or early versions of bundler) relied on brute-force backtracking or generic SAT (Boolean Satisfiability) solvers. These approaches were slow and produced notoriously incomprehensible error messages when a conflict occurred ("Version conflict: try changing something").

**PubGrub** (originally invented by Natalie Weizenbaum for the Dart language's `pub` package manager) revolutionizes this process. It is a highly optimized version solver based on the principles of CDCL (Conflict-Driven Clause Learning).

### 2.1. How the Algorithm Operates

When you run `beru build`, PubGrub initiates a conversation with the Beru Git index.

1.  **The Root Incompatibilities:** It starts with the constraints in your root `Beru.toml` (e.g., `fmt = "10.0.0"`). It creates a logical statement (an *Incompatibility*): "It is impossible to build this project unless we pick a version of `fmt` matching `^10.0.0`."
2.  **Exploration and Decision:** It queries the index, finds the highest published version of `fmt` (say, `10.2.1`), and *decides* to use it.
3.  **Constraint Gathering:** It reads the `recipe.toml` for `fmt 10.2.1` to find its dependencies. It adds those new constraints to its logical database.
4.  **Conflict Detection:** It continues exploring the graph. Eventually, it may discover a contradiction. Perhaps a transitively required library demands `fmt = 9.0.0`.
5.  **Clause Learning (The Genius of PubGrub):** When a conflict occurs, PubGrub does not just blindly backtrack. It analyzes the exact sequence of decisions that led to the conflict and mathematically derives a *new* rule. It learns: "Because Package A requires C, and Package B requires D, and C and D are incompatible, Package A and Package B can never coexist in this exact configuration."

It adds this new learned rule to its database and restarts the decision process. This clause learning makes the algorithm exponentially faster than brute-force solvers on large graphs.

## 3. Human-Readable Error Traces

The greatest benefit of PubGrub is its failure mode. If it is mathematically impossible to resolve your dependencies, PubGrub uses the rules it learned during the conflict-driven analysis to print a step-by-step logical proof of *why* the build failed.

Instead of a cryptic linker error, Beru provides an explicit trace:

```text
Error: Failed to resolve dependencies.

Because 'my_app' depends on 'graphics_engine 2.0.0', which depends on 'math_lib 1.5.0'.
And 'math_lib 1.5.0' depends on 'fmt 9.0.0'.
But 'my_app' directly depends on 'fmt 11.0.0',
Therefore, no combination of versions can satisfy the constraints.
```

Armed with this proof, the developer knows exactly what to do: either upgrade `graphics_engine` to a version that supports a newer `fmt`, or downgrade the direct `fmt` dependency in the root `Beru.toml`.
