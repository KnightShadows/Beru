# Reference: recipe.toml

The `recipe.toml` file instructs Beru on how to package and build a third-party C++ library. These files are stored centrally in the Beru Index.

## `[package]` Section

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | String | Yes | - | The name of the package. |
| `version` | String | Yes | - | The exact semantic version of this recipe (e.g., `"10.2.1"`). |

## `[source]` Section

Defines where to download the library's source code.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `url` | String | No* | - | URL to a tarball (e.g., `.tar.gz`) containing the source. |
| `sha256` | String | No | - | Checksum for the tarball. Highly recommended. |
| `git` | String | No* | - | URL to a Git repository to clone. |
| `tag` | String | No | - | Git tag to checkout (used with `git`). |

*\* Note: You must provide either `url` or `git`.*

## `[build]` Section

Defines how the library is compiled.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `system` | String | Yes | - | The build system. Valid options: `"cmake"`, `"header_only"`. |

## `[export]` Section

Defines the outputs of the library so Beru knows how to link them to downstream projects.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `include_dirs` | Array of Strings | No | `["include"]` | Directories relative to the install root containing public headers. |
| `cmake_targets` | Array of Strings | No | `[]` | The names of the CMake targets this library exports (e.g., `["fmt::fmt"]`). |

