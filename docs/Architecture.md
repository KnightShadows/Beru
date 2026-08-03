# Architecture Deep Dive

Beru is built in Rust using a multi-crate workspace. The architecture is split across four distinct crates that mirror the four stages of the build pipeline.

## The 4-Stage Pipeline

1. **Resolution (`beru-manifest`, `beru-resolve`)**: 
   Parses `Beru.toml` into a typed manifest. Hands the constraints to `beru-resolve`, which invokes the PubGrub solver against the local Git index to compute a deterministic dependency graph.
2. **Fetch & Build (`beru-recipe`)**: 
   Iterates through the locked dependency graph. For each library, it downloads the source tarball from its `url`, verifies the `sha256`, and compiles it natively using CMake in the global `~/.beru/cache/`.
3. **Orchestration (`beru-build`)**: 
   Collects all the exported `include_dirs` and `cmake_targets` from the compiled dependencies. It dynamically generates a toolchain `CMakeLists.txt` file inside the local project's `.beru/` folder, injecting these targets via `CMAKE_PREFIX_PATH`.
4. **Compilation (`beru-cli`)**: 
   Invokes the system `cmake` and `cmake --build` commands to compile the local project's `src/` directory against the orchestrated toolchain.

## Diagram

*(A visual diagram of the pipeline would show the user's `Beru.toml` flowing into the PubGrub Resolver, combining with the `~/.beru/index/` git clone. The resolved tree flows into the Fetcher, which downloads sources into `~/.beru/cache/`. The Builder compiles them into static/shared objects. The Orchestrator combines these objects with the user's `src/` files into a final CMake invocation, outputting the final binary.)*

## Cache Invalidation

Beru caches compiled dependencies heavily. A cached dependency is invalidated only if:
- The package version changes in `Beru.lock`.
- The user's system compiler or architecture changes (determined by a hash of the compiler version).
- The C++ standard requested by the root project changes (e.g., switching from C++17 to C++20 invalidates all cached C++17 libraries to prevent ABI mismatch).

