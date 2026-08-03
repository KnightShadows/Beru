# `beru run`

Builds the project and immediately executes the resulting binary.

## Usage
```bash
beru run
```

## Description
This command is a convenient wrapper around `beru build`. After successfully compiling your project, it locates the resulting executable in the build output directory and spawns it. 

If your `Beru.toml` is configured as `type = "library"`, this command will fail, as there is no executable to run.

## Options
*None.*

