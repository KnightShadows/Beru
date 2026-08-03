# Guide: Publishing to the Registry

Beru's package registry (the "Index") is entirely decentralized. It is simply a Git repository hosted on GitHub at [KnightShadows/beru_index](https://github.com/KnightShadows/beru_index).

To publish a new library—or a new version of an existing library—you simply submit a Pull Request to the index repository.

## Step 1: Fork and Clone the Index

```bash
git clone https://github.com/YOUR_USERNAME/beru_index.git
cd beru_index
```

## Step 2: Create the Recipe Directory

The index is organized by package name and version: `<package_name>/<version>/recipe.toml`.

For example, to add `spdlog` version `1.14.1`:
```bash
mkdir -p spdlog/1.14.1
touch spdlog/1.14.1/recipe.toml
```

## Step 3: Write the Recipe

Open the `recipe.toml` you just created and write the recipe instructions. (See [Writing a Recipe](Guide-Writing-A-Recipe.md) for a detailed tutorial).

```toml
[package]
name = "spdlog"
version = "1.14.1"

[source]
url = "https://github.com/gabime/spdlog/archive/refs/tags/v1.14.1.tar.gz"
sha256 = "1586508029a7d0670dfcb2d97575dcdc242d3868a259742b69f100801ab4e16b"

[build]
system = "cmake"

[export]
include_dirs = ["include"]
cmake_targets = ["spdlog::spdlog"]
```

## Step 4: Test Your Recipe Locally

Before submitting a PR, you should test that your recipe actually builds. You can temporarily point Beru to your local fork of the index by changing your project's index configuration (or just copy the recipe into your local `~/.beru/index/` directory for a quick test).

1. Copy your recipe to `~/.beru/index/spdlog/1.14.1/recipe.toml`.
2. In a test project, add `spdlog = "1.14.1"` to `Beru.toml`.
3. Run `beru build`. If it compiles and links successfully, your recipe works!

## Step 5: Submit a Pull Request

Commit your changes and push them to your fork:
```bash
git add spdlog/1.14.1/recipe.toml
git commit -m "feat: add recipe for spdlog 1.14.1"
git push origin main
```

Finally, go to [KnightShadows/beru_index](https://github.com/KnightShadows/beru_index) and open a Pull Request. Once merged, the new package will be available to all Beru users the next time they run `beru index update`!

