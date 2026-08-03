# Concepts: Registry and Index

Unlike npm or PyPI, which require massive centralized databases to track packages, Beru uses a **decentralized Git index**.

## The Central Index

The official Beru package index is simply a Git repository hosted on GitHub at `KnightShadows/beru_index`. 

When you first run Beru, it automatically clones this repository into `~/.beru/index/`. This clone contains thousands of tiny `recipe.toml` files organized in folders.

## Offline and Instant

Because the entire index is cloned to your local machine, `beru resolve` operates entirely offline and virtually instantly. Beru doesn't need to make HTTP requests to a server to ask "what versions of `fmt` exist?"—it just looks at the folders in `~/.beru/index/fmt/`.

To get new packages that were published recently, you simply run `beru index update`, which performs a fast `git pull` on the cache.

