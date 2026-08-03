# Guide: Adding a Dependency

Adding dependencies in Beru is designed to be as effortless as possible. You declare them in your `Beru.toml`, and Beru will fetch, resolve, and link them automatically.

Beru supports three types of dependencies: **Registry Dependencies**, **Git Dependencies**, and **Path Dependencies**.

---

## 1. Registry Dependencies (The Default)

If a library is available in the official Beru package index, you only need to specify its name and version requirement.

```toml
[dependencies]
fmt = "10.2.1"       # Exactly version 10.2.1
nlohmann_json = "3"  # Any 3.x.x version
```

When you run `beru build`, Beru queries the local index (located at `~/.beru/index`), resolves the version constraints using PubGrub, downloads the tarball defined in the library's recipe, and compiles it.

## 2. Git Dependencies

If a library is not in the official registry, or if you want to link directly to a specific branch or commit of an open-source project, you can use a Git dependency.

```toml
[dependencies]
my_custom_lib = { git = "https://github.com/myorg/my_custom_lib.git", tag = "v1.2.0" }
bleeding_edge = { git = "https://github.com/org/bleeding_edge.git", branch = "main" }
specific_fix  = { git = "https://github.com/org/specific_fix.git", rev = "a1b2c3d" }
```

**Note:** For a Git dependency to work, the target repository *must* contain a valid `Beru.toml` file at its root. Beru will clone the repository, parse its manifest, and build it from source.

## 3. Path Dependencies

When developing a library and an application simultaneously, or when working in a monorepo, you can link directly to a local folder on your filesystem.

```toml
[dependencies]
my_local_lib = { path = "../my_local_lib" }
```

Like Git dependencies, the target folder must contain a valid `Beru.toml`. Path dependencies are never cached globally; they are rebuilt if their source files change, making them perfect for active development.

