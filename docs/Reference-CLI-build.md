# `beru build`

Orchestrates the entire build pipeline for your project.

## Usage
```bash
beru build
```

## Description
The `build` command is the core of Beru. When executed, it performs the following steps:
1. **Resolution**: Runs the equivalent of `beru resolve` to lock dependencies.
2. **Fetch**: Downloads missing tarballs or clones Git repositories into the global cache (`~/.beru/cache`).
3. **Dependency Compilation**: Compiles any un-cached dependencies.
4. **Orchestration**: Generates the local `CMakeLists.txt` toolchain in the hidden `.beru/` directory.
5. **Project Compilation**: Invokes CMake to build your local project source files.

## Options
*None.*

