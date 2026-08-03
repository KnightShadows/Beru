# Getting Started

This guide will walk you through installing Beru, creating a new C++ project, and running it for the first time.

## 1. Installation

Beru provides automated installation scripts for all major platforms.

### Linux and macOS
Open your terminal and run the following command to download and execute the installation script:
```bash
curl -sSL https://raw.githubusercontent.com/KnightShadows/Beru/main/install.sh | bash
```

### Windows
Open PowerShell and run the following command:
```powershell
irm https://raw.githubusercontent.com/KnightShadows/Beru/main/install.ps1 | iex
```

### Building from Source (Cargo)
If you already have a Rust toolchain installed, you can install Beru natively from crates.io:
```bash
cargo install beru
```

*Note: You must have a working C++ compiler (GCC, Clang, or MSVC) and `cmake` installed on your system PATH for Beru to orchestrate builds.*

## 2. Creating a New Project

To scaffold a new C++ project, use the `new` command:

```bash
beru new hello_beru
cd hello_beru
```

Beru will generate a standardized project structure:
```text
hello_beru/
├── Beru.toml       # Your project manifest
├── src/
│   └── main.cpp    # The entry point
└── include/        # Your public headers
```

Take a look at the generated `Beru.toml`:
```toml
[package]
name = "hello_beru"
version = "0.1.0"
cxx_std = "17"
type = "executable"

[dependencies]
```

## 3. Initializing an Existing Project

If you already have a directory with source code, you can initialize Beru inside it without scaffolding new files:

```bash
cd my_existing_project
beru init
```
This simply drops a `Beru.toml` in the current directory. It will safely ignore any existing `CMakeLists.txt` files you might have.

## 4. Building and Running

Let's compile and execute the project.

```bash
beru run
```

Behind the scenes, Beru will:
1. Parse your `Beru.toml`.
2. Generate an internal `CMakeLists.txt` (stored in `.beru/`).
3. Compile the `src/main.cpp` file.
4. Execute the resulting binary.

**Expected Output:**
```text
  Configuring hello_beru v0.1.0
     Building hello_beru
      Running `build/hello_beru`
Hello, Beru!
```

That's it! You have successfully built and run a C++ project without touching CMake. Next, let's learn how to add dependencies.

