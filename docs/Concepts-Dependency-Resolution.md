# Concepts: Dependency Resolution

When you run `beru build`, the first thing Beru does is figure out exactly which versions of every dependency to download. This process is called **Resolution**.

Beru uses the **PubGrub algorithm**, the same state-of-the-art version solver powering Cargo (Rust) and Dart.

## How it works

1. Beru reads your `Beru.toml` to find your direct dependencies (e.g., `fmt = "10"`).
2. It queries the local Index to find all published versions of `fmt` matching `10.x.x`.
3. It downloads the `recipe.toml` for the highest matching version of `fmt` to check if `fmt` has its own dependencies.
4. If it does, PubGrub begins building a dependency graph, ensuring that no two packages demand mutually exclusive versions of a shared transitive dependency.
5. Once a valid graph is found, it is written to the `Beru.lock` file.

## Resolving Conflicts

If Package A requires `spdlog = "1.10.0"` and Package B requires `spdlog = "1.14.0"`, PubGrub will try to find a version of `spdlog` that satisfies both. If that is impossible, PubGrub provides highly detailed, human-readable error messages explaining exactly why the resolution failed and which packages are causing the conflict.

