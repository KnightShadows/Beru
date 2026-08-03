# `beru resolve`

Resolves the dependency graph and updates `Beru.lock`.

## Usage
```bash
beru resolve
```

## Description
This command parses your `Beru.toml` and uses the PubGrub algorithm to find a conflict-free set of versions for all direct and transitive dependencies. The resulting tree is saved to `Beru.lock`.

If a valid `Beru.lock` already exists and your `Beru.toml` hasn't changed, this command exits immediately. It does **not** download sources or compile any code.

## Options
*None.*

