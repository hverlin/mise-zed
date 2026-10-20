//! mise for Zed:
//! - `mise lsp` for the `mise` language, and for TOML when a user opts in. The server answers
//!   only for the files mise loads as configs (`mise.toml`, `mise/config.toml`, `conf.d/*.toml`,
//!   ...) and leaves the other TOML files alone.
//! - the `mise mcp` and `pitchfork mcp` context servers, in the project directory.
//!
//! A setting that gives a command (`lsp.mise.binary.path`, `context_servers.<id>.command`)
//! replaces the command entirely: Zed runs it and does not call the extension.

use std::collections::HashSet;

use zed_extension_api::{self as zed, ContextServerId, LanguageServerId, Os, Project, Result};

/// `mise mcp` no longer needs `experimental = true` since 2026.6.6.
const MISE_MCP_SINCE: Version = [2026, 6, 6];
/// The oldest pitchfork mise works with.
const PITCHFORK_SINCE: Version = [2, 25, 0];
/// `pitchfork mcp --local` keeps the server to the project since 2.32.0.
const PITCHFORK_LOCAL_MCP_SINCE: Version = [2, 32, 0];

type Version = [u64; 3];

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

/// The absolute path and the version of a binary on Zed's PATH. Context servers have no
/// worktree to search, and Zed resolves a relative command from the extension's own directory.
fn find(binary: &str, since: Version) -> Result<(String, Version)> {
    let configure = || format!("set `context_servers.{binary}.command` to its path");
    if zed::current_platform().0 == Os::Windows {
        return Err(format!("{binary} is looked up with `sh`; {}", configure()));
    }
    let output = zed::process::Command::new("sh")
        .args(["-c", &format!("command -v {binary} && {binary} --version")])
        .output()
        .ok()
        .filter(|output| output.status == Some(0))
        .ok_or_else(|| format!("{binary} is not on the PATH; {}", configure()))?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut lines = stdout.lines();
    let path = lines
        .next()
        .map(str::trim)
        .filter(|path| path.starts_with('/'))
        .ok_or_else(|| format!("{binary} is not on the PATH; {}", configure()))?
        .to_string();
    let version = lines
        .flat_map(|line| line.split(|c: char| !(c.is_ascii_digit() || c == '.')))
        .find_map(parse_version)
        .ok_or_else(|| format!("could not read the version of {path}"))?;
    if version < since {
        return Err(format!(
            "{binary} {} or later is needed, {path} is {}",
            show(since),
            show(version)
        ));
    }
    Ok((path, version))
}

fn parse_version(word: &str) -> Option<Version> {
    let mut parts = word.split('.').map(str::parse::<u64>);
    match (parts.next(), parts.next(), parts.next()) {
        (Some(Ok(major)), Some(Ok(minor)), Some(Ok(patch))) => Some([major, minor, patch]),
        _ => None,
    }
}

fn show(version: Version) -> String {
    version.map(|part| part.to_string()).join(".")
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
                "{path} has no `mise lsp`, which comes with jdx/mise#14744"
            ));
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
        _project: &Project,
    ) -> Result<zed::Command> {
        let (command, args) = match context_server_id.as_ref() {
            "mise" => {
                let (path, _) = find("mise", MISE_MCP_SINCE)?;
                (path, vec!["mcp".to_string()])
            }
            "pitchfork" => {
                let (path, version) = find("pitchfork", PITCHFORK_SINCE)?;
                let mut args = vec!["mcp".to_string()];
                if version >= PITCHFORK_LOCAL_MCP_SINCE {
                    args.push("--local".to_string());
                }
                (path, args)
            }
            id => return Err(format!("unknown context server {id}")),
        };
        Ok(zed::Command {
            command,
            args,
            env: Vec::new(),
        })
    }
}

zed::register_extension!(MiseExtension);
