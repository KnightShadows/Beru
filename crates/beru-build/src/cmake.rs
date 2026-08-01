use anyhow::{Context, Result, bail};
use std::path::Path;
use std::process::Command;
use tracing::{debug, info};

/// Invoke CMake to configure a project.
///
/// Runs: `cmake -S <source_dir> -B <build_dir> [extra_args...]`
pub fn cmake_configure(
    source_dir: &Path,
    build_dir: &Path,
    install_prefix: &Path,
    extra_args: &[String],
    toolchain_file: Option<&Path>,
) -> Result<()> {
    let cmake = which::which("cmake")
        .context("cmake not found on PATH. Install CMake (https://cmake.org) to use Beru.")?;

    let mut cmd = Command::new(&cmake);
    cmd.arg("-S").arg(source_dir);
    cmd.arg("-B").arg(build_dir);
    cmd.arg(format!(
        "-DCMAKE_INSTALL_PREFIX={}",
        install_prefix.display()
    ));

    if let Some(tc) = toolchain_file {
        cmd.arg(format!("-DCMAKE_TOOLCHAIN_FILE={}", tc.display()));
    }

    for arg in extra_args {
        cmd.arg(arg);
    }

    info!(
        "configuring: cmake -S {} -B {}",
        source_dir.display(),
        build_dir.display()
    );
    debug!("full command: {:?}", cmd);

    let output = cmd
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .output()
        .with_context(|| "failed to run cmake configure")?;

    if !output.status.success() {
        bail!(
            "cmake configure failed (exit code: {:?})",
            output.status.code()
        );
    }

    Ok(())
}

/// Invoke CMake to build a configured project.
///
/// Runs: `cmake --build <build_dir> --parallel`
pub fn cmake_build(build_dir: &Path) -> Result<()> {
    let cmake = which::which("cmake").context("cmake not found on PATH")?;

    info!("building: cmake --build {}", build_dir.display());

    let output = Command::new(&cmake)
        .arg("--build")
        .arg(build_dir)
        .arg("--parallel")
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .output()
        .context("failed to run cmake build")?;

    if !output.status.success() {
        bail!("cmake build failed (exit code: {:?})", output.status.code());
    }

    Ok(())
}

/// Invoke CMake to install built artifacts.
///
/// Runs: `cmake --install <build_dir>`
pub fn cmake_install(build_dir: &Path) -> Result<()> {
    let cmake = which::which("cmake").context("cmake not found on PATH")?;

    info!("installing: cmake --install {}", build_dir.display());

    let output = Command::new(&cmake)
        .arg("--install")
        .arg(build_dir)
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .output()
        .context("failed to run cmake install")?;

    if !output.status.success() {
        bail!(
            "cmake install failed (exit code: {:?})",
            output.status.code()
        );
    }

    Ok(())
}

/// Full build pipeline for a dependency using CMake:
/// configure → build → install into the cache.
pub fn build_dependency_cmake(
    source_dir: &Path,
    install_prefix: &Path,
    cmake_args: &[String],
    toolchain_file: Option<&Path>,
) -> Result<()> {
    let build_dir = source_dir.join("_beru_build");

    if build_dir.exists() {
        std::fs::remove_dir_all(&build_dir)
            .with_context(|| format!("failed to clean {}", build_dir.display()))?;
    }

    cmake_configure(
        source_dir,
        &build_dir,
        install_prefix,
        cmake_args,
        toolchain_file,
    )?;
    cmake_build(&build_dir)?;
    cmake_install(&build_dir)?;

    Ok(())
}

/// Build the user's project using CMake.
///
/// This generates a toolchain file in the project's build directory,
/// then runs configure + build.
pub fn build_project(project_dir: &Path, build_dir: &Path, toolchain_file: &Path) -> Result<()> {
    cmake_configure(project_dir, build_dir, build_dir, &[], Some(toolchain_file))?;
    cmake_build(build_dir)?;
    Ok(())
}
