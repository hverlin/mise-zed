//! Runs `mise lsp` for the TOML files of a Zed project. The server answers only for the files
//! mise loads as configs (`mise.toml`, `mise/config.toml`, `conf.d/*.toml`, ...) and leaves the
//! other TOML files alone, so it can be attached to the whole language.
//!
//! A `lsp.mise.binary.path` setting replaces this command entirely: Zed runs that path with
//! `lsp.mise.binary.arguments` (none by default) and does not call the extension.

use std::collections::HashMap;

use zed_extension_api::{self as zed, LanguageServerId, Result};

struct MiseExtension {
    /// Whether each mise binary has `mise lsp`.
    has_lsp: HashMap<String, bool>,
}

impl MiseExtension {
    /// A mise without `mise lsp` takes `lsp` for a task name and runs the project's `lsp` task, so
    /// the command is checked first. `--help` only shows that task's help.
    fn has_lsp(&mut self, path: &str) -> bool {
        *self.has_lsp.entry(path.to_string()).or_insert_with(|| {
            zed::process::Command::new(path)
                .args(["lsp", "--help"])
                .output()
                .is_ok_and(|output| {
                    output.status == Some(0)
                        && String::from_utf8_lossy(&output.stdout)
                            .lines()
                            .any(|line| line.starts_with("Usage: mise lsp"))
                })
        })
    }
}

impl zed::Extension for MiseExtension {
    fn new() -> Self {
        Self {
            has_lsp: HashMap::new(),
        }
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let path = worktree
            .which("mise")
            .ok_or("mise is not on the PATH; set `lsp.mise.binary.path`")?;
        if !self.has_lsp(&path) {
            return Err(format!("{path} has no `mise lsp`; upgrade mise"));
        }
        Ok(zed::Command {
            command: path,
            args: vec!["lsp".to_string()],
            env: worktree.shell_env(),
        })
    }
}

zed::register_extension!(MiseExtension);
