# Migrating to Beru

If you're coming from a traditional C++ workflow using raw CMake, vcpkg, or Conan, Beru requires a slight shift in mindset. Instead of manually bridging your package manager to your build system, **Beru is the bridge.**

Here is a side-by-side comparison of common workflows to help you transition.

## Adding a Dependency

### The vcpkg / CMake Way
1. Run `vcpkg install fmt`.
2. Open your `CMakeLists.txt`.
3. Add `find_package(fmt CONFIG REQUIRED)`.
4. Add `target_link_libraries(my_app PRIVATE fmt::fmt)`.
5. When configuring, pass `-DCMAKE_TOOLCHAIN_FILE=[path to vcpkg.cmake]`.

### The Conan / CMake Way
1. Add `fmt/10.0.0` to your `conanfile.txt`.
2. Run `conan install . --build=missing`.
3. Open `CMakeLists.txt` and add `find_package` and `target_link_libraries`.
4. Run `cmake` with the generated Conan toolchain.

### The Beru Way
1. Add `fmt` to your `Beru.toml`:
```toml
[dependencies]
fmt = "10.0.0"
```
2. Run `beru run`.

**There is no Step 3.** Beru automatically fetches `fmt`, compiles it (if it isn't cached), generates the CMake boilerplate, and links it to your target.

## Setting the C++ Standard

### Raw CMake
```cmake
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)
```

### Beru
```toml
[package]
cxx_std = "17"
```

## Adding Multiple Source Files

### Raw CMake
Whenever you add a new `.cpp` file, you must manually append it to your `add_executable()` or `add_library()` call, or use a glob (which CMake explicitly advises against).

### Beru
Beru automatically discovers and compiles all `.cpp` files located in the `src/` directory. You never need to manually list source files.

## What if I *need* custom CMake?

Beru is designed to eliminate CMake for 95% of standard projects. However, if you are migrating a legacy project or need advanced CMake features (e.g., custom code generation, Qt MOC, specific compiler flags), Beru stays out of your way.

If Beru detects a `CMakeLists.txt` in the root of your project, it will **not** generate its own. Instead, it will resolve your dependencies, download them, and invoke *your* `CMakeLists.txt`, seamlessly injecting the dependency paths via `CMAKE_PREFIX_PATH`. 

You still get the benefits of `Beru.toml` for dependency management while retaining full control over your build!

