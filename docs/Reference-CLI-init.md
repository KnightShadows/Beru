# `beru init`

Initializes a Beru manifest in the current directory.

## Usage
```bash
beru init
```

## Description
Unlike `beru new`, which creates a new directory and scaffolds boilerplate C++ files, `beru init` simply drops a `Beru.toml` into the *current* directory. 

This is extremely useful when you have an existing C++ project (e.g., one using raw CMake) and you want to start managing its dependencies with Beru. Beru will safely ignore any existing `CMakeLists.txt` files and focus solely on fetching and configuring dependencies.

## Options
*None.*

