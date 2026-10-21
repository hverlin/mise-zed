//! mise for Zed:
//! - `mise lsp` for the `mise` language, and for TOML when a user opts in. The server answers
//!   only for the files mise loads as configs (`mise.toml`, `mise/config.toml`, `conf.d/*.toml`,
//!   ...) and leaves the other TOML files alone.
//!
//! A `lsp.mise.binary.path` setting replaces the command entirely: Zed runs it and does not
//! call the extension.

use std::collections::HashSet;

use zed_extension_api::{self as zed, LanguageServerId, Result};

struct MiseExtension {
    /// The mise binaries found to have `mise lsp`. A binary without it is asked again on the
    /// next start, as mise upgrades itself in place.
    has_lsp: HashSet<String>,
}

impl MiseExtension {
    /// A mise without `mise lsp` takes `lsp` for a task name: it would run the project's `lsp`
    /// task, and `mise lsp --help` shows that task's help, which the task's description can make
    /// look like the command's. `mise help lsp` shows mise's own help instead.
    fn has_lsp(&mut self, path: &str) -> bool {
        if self.has_lsp.contains(path) {
            return true;
        }
        let found = zed::process::Command::new(path)
            .args(["help", "lsp"])
            .output()
            .is_ok_and(|output| {
                output.status == Some(0)
                    && String::from_utf8_lossy(&output.stdout)
                        .lines()
                        .any(|line| line.starts_with("Usage: mise lsp"))
            });
        if found {
            self.has_lsp.insert(path.to_string());
        }
        found
    }
}

impl zed::Extension for MiseExtension {
    fn new() -> Self {
        Self {
            has_lsp: HashSet::new(),
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
            return Err(format!(
                "{path} has no `mise lsp`, which needs mise 2026.10.18 or later"
            ));
        }
        Ok(zed::Command {
            command: path,
            args: vec!["lsp".to_string()],
            env: worktree.shell_env(),
        })
    }
}

zed::register_extension!(MiseExtension);
