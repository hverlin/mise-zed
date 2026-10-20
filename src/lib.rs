//! mise for Zed:
//! - `mise lsp` for the TOML files of a project. The server answers only for the files mise
//!   loads as configs (`mise.toml`, `mise/config.toml`, `conf.d/*.toml`, ...) and leaves the other
//!   TOML files alone, so it can be attached to the whole language.
//! - the `mise mcp` and `pitchfork mcp` context servers, in the project directory.
//!
//! A `lsp.mise.binary.path` setting replaces the language server command entirely: Zed runs that
//! path with `lsp.mise.binary.arguments` (none by default) and does not call the extension. A
//! `context_servers.<id>.command.path` setting replaces the context server command the same way.

use std::collections::HashMap;

use zed_extension_api::{
    self as zed, ContextServerId, LanguageServerId, Project, Result,
    settings::ContextServerSettings,
};

/// `mise mcp` no longer needs `experimental = true` since 2026.6.6.
const MISE_MCP_SINCE: Version = [2026, 6, 6];
/// The version mise works with.
const PITCHFORK_SINCE: Version = [2, 25, 0];
/// `pitchfork mcp --local` keeps the server to the project since 2.32.0.
const PITCHFORK_LOCAL_MCP_SINCE: Version = [2, 32, 0];

type Version = [u64; 3];

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

/// The absolute path of a binary on Zed's PATH. Context servers have no worktree to search, and
/// Zed resolves a relative command from the extension's own directory.
fn find(binary: &str, lookup: &str) -> Result<String> {
    zed::process::Command::new("sh")
        .args(["-c", lookup])
        .output()
        .ok()
        .filter(|output| output.status == Some(0))
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .filter(|path| path.starts_with('/'))
        .ok_or_else(|| {
            format!("{binary} is not on the PATH; set `context_servers.{binary}.command.path`")
        })
}

/// The first `x.y.z` of `<binary> --version`.
fn version(path: &str) -> Option<Version> {
    let output = zed::process::Command::new(path)
        .arg("--version")
        .output()
        .ok()?;
    let text = String::from_utf8_lossy(&output.stdout);
    text.split(|c: char| !(c.is_ascii_digit() || c == '.'))
        .find_map(|word| {
            let mut parts = word.split('.').map(str::parse::<u64>);
            match (parts.next(), parts.next(), parts.next()) {
                (Some(Ok(major)), Some(Ok(minor)), Some(Ok(patch))) => Some([major, minor, patch]),
                _ => None,
            }
        })
}

fn require(binary: &str, path: &str, since: Version) -> Result<Version> {
    let found = version(path).ok_or_else(|| format!("could not read the version of {path}"))?;
    if found < since {
        let [major, minor, patch] = since;
        return Err(format!(
            "{binary} {major}.{minor}.{patch} or later is needed, {path} is {}",
            found.map(|part| part.to_string()).join(".")
        ));
    }
    Ok(found)
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

    /// Zed starts them in the active project directory.
    fn context_server_command(
        &mut self,
        context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<zed::Command> {
        let id = context_server_id.as_ref();
        let configured = ContextServerSettings::for_project(id, project)
            .ok()
            .and_then(|settings| settings.command)
            .filter(|command| command.path.is_some());
        if let Some(command) = configured {
            return Ok(zed::Command {
                command: command.path.unwrap_or_default(),
                args: command.arguments.unwrap_or_else(|| vec!["mcp".to_string()]),
                env: command.env.unwrap_or_default().into_iter().collect(),
            });
        }
        let (path, args) = match id {
            "mise" => {
                let path = find("mise", "command -v mise")?;
                require("mise", &path, MISE_MCP_SINCE)?;
                (path, vec!["mcp".to_string()])
            }
            "pitchfork" => {
                let path = find("pitchfork", "command -v pitchfork")?;
                let version = require("pitchfork", &path, PITCHFORK_SINCE)?;
                let mut args = vec!["mcp".to_string()];
                if version >= PITCHFORK_LOCAL_MCP_SINCE {
                    args.push("--local".to_string());
                }
                (path, args)
            }
            _ => return Err(format!("unknown context server {id}")),
        };
        Ok(zed::Command {
            command: path,
            args,
            env: Vec::new(),
        })
    }
}

zed::register_extension!(MiseExtension);
