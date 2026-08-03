# Reference: Configuration & Environment

Beru is designed to be zero-configuration. It strictly relies on the `Beru.toml` manifest in the root of your project. 

There are currently **no global configuration files** and **no environment variables** required to run Beru. 

## Cache and Internal Directories

Beru creates and manages two directories automatically:

1. **The Global Cache (`~/.beru/`)**:
   - `~/.beru/index/`: A clone of the central Git registry containing all `recipe.toml` files.
   - `~/.beru/cache/`: The global binary cache where fetched sources and compiled `.a`/`.so` artifacts are stored.
   - Beru resolves paths relative to the user's home directory (e.g., `/home/username/.beru` on Linux, `C:\Users\Username\.beru` on Windows).

2. **The Local Build Directory (`.beru/`)**:
   - When you run `beru build`, Beru generates a hidden `.beru/` folder inside your project directory. 
   - This folder contains the generated `CMakeLists.txt` (if applicable) and all temporary build artifacts (object files, CMake caches).
   - This folder should be added to your `.gitignore`.

*Note: Future versions of Beru may introduce environment variables (e.g., `BERU_HOME`) to override the global cache location.*

