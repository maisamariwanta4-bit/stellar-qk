//! Project scaffolding — the `init` command.
//!
//! Two templates are available:
//!
//! | Template | What it creates |
//! |----------|-----------------|
//! | `basic`  | Minimal Rust crate that generates a keypair and prints it |
//! | `funded` | Extends `basic` with a Friendbot call on Testnet |
//!
//! Templates are embedded in the binary as string literals so the CLI has no
//! runtime file-system dependencies beyond writing the output project.

use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

// ---------------------------------------------------------------------------
// Template files (embedded)
// ---------------------------------------------------------------------------

mod templates {
    pub mod basic {
        pub const CARGO_TOML: &str = include_str!("../templates/basic/Cargo.toml");
        pub const MAIN_RS: &str    = include_str!("../templates/basic/src/main.rs");
        pub const README: &str     = include_str!("../templates/basic/README.md");
    }
    pub mod funded {
        pub const CARGO_TOML: &str = include_str!("../templates/funded/Cargo.toml");
        pub const MAIN_RS: &str    = include_str!("../templates/funded/src/main.rs");
        pub const README: &str     = include_str!("../templates/funded/README.md");
    }
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Scaffold a new Stellar Rust project in `./<name>/`.
///
/// `template` must be `"basic"` or `"funded"`.
pub fn init_project(name: &str, template: &str) -> Result<()> {
    validate_name(name)?;

    let root = Path::new(name);
    if root.exists() {
        anyhow::bail!(
            "directory `{}` already exists — choose a different name or remove it first.",
            name
        );
    }

    let src = root.join("src");
    fs::create_dir_all(&src)
        .with_context(|| format!("failed to create {}", src.display()))?;

    match template.to_lowercase().as_str() {
        "basic" => write_basic(root, name)?,
        "funded" => write_funded(root, name)?,
        other => anyhow::bail!(
            "unknown template `{other}`. Available templates: basic, funded"
        ),
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Template writers
// ---------------------------------------------------------------------------

fn write_basic(root: &Path, name: &str) -> Result<()> {
    let cargo = templates::basic::CARGO_TOML.replace("{{PROJECT_NAME}}", name);
    let main  = templates::basic::MAIN_RS;
    let readme = templates::basic::README.replace("{{PROJECT_NAME}}", name);

    write_file(root.join("Cargo.toml"), &cargo)?;
    write_file(root.join("src/main.rs"), main)?;
    write_file(root.join("README.md"), &readme)?;
    write_file(root.join(".gitignore"), "/target\n")?;
    Ok(())
}

fn write_funded(root: &Path, name: &str) -> Result<()> {
    let cargo  = templates::funded::CARGO_TOML.replace("{{PROJECT_NAME}}", name);
    let main   = templates::funded::MAIN_RS;
    let readme = templates::funded::README.replace("{{PROJECT_NAME}}", name);

    write_file(root.join("Cargo.toml"), &cargo)?;
    write_file(root.join("src/main.rs"), main)?;
    write_file(root.join("README.md"), &readme)?;
    write_file(root.join(".gitignore"), "/target\n")?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn write_file<P: AsRef<Path>>(path: P, content: &str) -> Result<()> {
    let path = path.as_ref();
    // Ensure parent directory exists
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create {}", parent.display()))?;
    }
    fs::write(path, content)
        .with_context(|| format!("failed to write {}", path.display()))
}

/// Validate that `name` is a reasonable Rust crate name.
fn validate_name(name: &str) -> Result<()> {
    if name.is_empty() {
        anyhow::bail!("project name cannot be empty");
    }
    if !name.chars().next().map(|c| c.is_ascii_alphabetic() || c == '_').unwrap_or(false) {
        anyhow::bail!("project name must start with a letter or underscore");
    }
    if !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-') {
        anyhow::bail!(
            "project name may only contain ASCII letters, digits, hyphens, and underscores"
        );
    }
    if name.len() > 64 {
        anyhow::bail!("project name is too long (max 64 characters)");
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_names_pass() {
        assert!(validate_name("my_project").is_ok());
        assert!(validate_name("stellar-app").is_ok());
        assert!(validate_name("_internal").is_ok());
        assert!(validate_name("a").is_ok());
    }

    #[test]
    fn invalid_names_fail() {
        assert!(validate_name("").is_err());
        assert!(validate_name("1starts-with-digit").is_err());
        assert!(validate_name("has spaces").is_err());
        assert!(validate_name("has.dot").is_err());
    }

    #[test]
    fn unknown_template_fails() {
        let result = init_project("_test_unknown_template_xyz", "nonexistent");
        assert!(result.is_err());
        // Clean up if it somehow created the directory
        let _ = std::fs::remove_dir_all("_test_unknown_template_xyz");
    }
}
