# Reference: Beru.toml Manifest

The `Beru.toml` file is the manifest for your C++ project. It contains metadata about your package, specifies the C++ standard to compile against, and declares your dependencies.

## `[package]` Section

The `[package]` block defines the project itself.

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | String | Yes | - | The name of your project. This determines the name of the final executable or library. |
| `version` | String | Yes | - | A semantic version string (e.g., `0.1.0`). |
| `cxx_std` | String | No | `"17"` | The C++ standard to use. Valid options: `"11"`, `"14"`, `"17"`, `"20"`, `"23"`, `"26"`. |
| `type` | String | No | `"executable"` | The output type. Valid options: `"executable"`, `"library"`. |

**Example:**
```toml
[package]
name = "my_app"
version = "1.0.0"
cxx_std = "20"
type = "executable"
```

## `[dependencies]` Section

The `[dependencies]` block lists the libraries your project requires. Keys are the package names, and values define where to get them and what version to use.

### Registry Dependencies (String)
The simplest dependency is a string specifying a version constraint for a package in the Beru Index.

```toml
[dependencies]
fmt = "10.2.1"
```

### Git Dependencies (Table)
Fetch a dependency directly from a Git repository. The repository must contain a valid `Beru.toml`.

| Field | Type | Required | Description |
|---|---|---|---|
| `git` | String | Yes | The URL of the Git repository. |
| `tag` | String | No | The specific Git tag to checkout (e.g., `"v1.2.0"`). |
| `branch`| String | No | The branch to checkout (e.g., `"main"`). |
| `rev` | String | No | A specific commit SHA to checkout. |

*(Note: You can only specify one of `tag`, `branch`, or `rev`)*

**Example:**
```toml
[dependencies]
my_custom_lib = { git = "https://github.com/myorg/my_custom_lib.git", tag = "v1.2.0" }
```

### Path Dependencies (Table)
Link to a local project on your filesystem. The target directory must contain a valid `Beru.toml`.

| Field | Type | Required | Description |
|---|---|---|---|
| `path` | String | Yes | The relative or absolute path to the local dependency. |

**Example:**
```toml
[dependencies]
my_local_lib = { path = "../my_local_lib" }
```

