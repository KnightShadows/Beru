# Guide: Writing a Recipe

A **recipe** tells Beru how to fetch, build, and link a third-party C++ library that doesn't natively use Beru. Recipes are written in TOML (`recipe.toml`) and are stored in the Beru Index.

Let's walk through packaging a popular library: `fmt`.

## 1. The Package Block

Every recipe starts with a `[package]` block defining the name and version.

```toml
[package]
name = "fmt"
version = "10.2.1"
```

## 2. The Source Block

Next, tell Beru where to download the source code. Most recipes use a tarball release from GitHub.

```toml
[source]
url = "https://github.com/fmtlib/fmt/archive/refs/tags/10.2.1.tar.gz"
sha256 = "312151a2d12c8336f5fc2e6cf8e2b4ceada06e599d4ee1537f732a5dfb0e2719"
```

> **Tip:** Always provide a `sha256` checksum for tarballs. Beru verifies this checksum to protect against supply-chain attacks and corrupted downloads.

Alternatively, you can fetch from a Git repository directly:
```toml
[source]
git = "https://github.com/fmtlib/fmt.git"
tag = "10.2.1"
```

## 3. The Build Block

How should Beru build this library? The vast majority of C++ libraries use CMake.

```toml
[build]
system = "cmake"
```

If the library is **Header-Only** (e.g., `nlohmann_json`), you can skip compilation entirely:
```toml
[build]
system = "header_only"
```

## 4. The Export Block (Crucial for CMake)

When Beru builds a CMake library, it installs it into a private cache directory. To link it to your project, Beru needs to know the exact CMake targets the library exports, and where its include headers are located relative to the installation root.

```toml
[export]
include_dirs = ["include"]
cmake_targets = ["fmt::fmt"]
```

## The Complete Recipe

Here is the complete `recipe.toml` for `fmt` v10.2.1:

```toml
[package]
name = "fmt"
version = "10.2.1"

[source]
url = "https://github.com/fmtlib/fmt/archive/refs/tags/10.2.1.tar.gz"
sha256 = "312151a2d12c8336f5fc2e6cf8e2b4ceada06e599d4ee1537f732a5dfb0e2719"

[build]
system = "cmake"

[export]
include_dirs = ["include"]
cmake_targets = ["fmt::fmt"]
```

Once this file is placed in the Beru Index under `fmt/10.2.1/recipe.toml`, anyone can use `fmt = "10.2.1"` in their `Beru.toml`!

