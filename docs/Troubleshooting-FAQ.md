# Troubleshooting & FAQ

## "Failed to resolve dependency"
**Symptoms:** PubGrub panics or prints an error about incompatible versions.
**Fix:** You have requested versions of libraries that demand conflicting transitive dependencies. Read the output from PubGrub carefully—it will print the exact conflict chain. You must either update or downgrade one of your direct dependencies in `Beru.toml` to a compatible version.

## "CMake not found"
**Symptoms:** `beru build` fails immediately with an IO error or `command not found: cmake`.
**Fix:** Beru relies on your system's CMake installation to perform the actual compilation. Ensure CMake is installed and available on your system `PATH`.
- Linux: `sudo apt install cmake`
- macOS: `brew install cmake`
- Windows: Download from cmake.org or use `winget install cmake`

## "Unknown package in index"
**Symptoms:** Beru says the package doesn't exist, but you know it does.
**Fix:** Your local index is likely out of date. Run `beru index update` to fetch the latest recipes.

## "Cache corruption / Weird compile errors"
**Symptoms:** A library that used to compile suddenly fails, or linking fails with undefined symbols.
**Fix:** You can forcefully wipe the global Beru cache. Delete the `~/.beru/cache/` directory. Beru will automatically re-download and re-compile dependencies from scratch on the next build.

