"""Copies the snippets of mise-vscode (src/snippets) into this extension.

Usage: python3 scripts/sync-snippets.py <mise-vscode checkout>

Zed reads the VS Code snippet format, but its parser takes every `$` for the start of a tab
stop: a literal `$`, as in bun's `await $` template tag, is escaped with a backslash, which VS
Code reads the same way.
"""

import json
import re
import sys
from pathlib import Path

source = Path(sys.argv[1]) / "src" / "snippets"
target = Path(__file__).resolve().parent.parent / "snippets"
LITERAL_DOLLAR = re.compile(r"(?<!\\)\$(?![0-9{]|[A-Z_]+\b)")


def escape(body):
    if isinstance(body, list):
        return [LITERAL_DOLLAR.sub(r"\\$", line) for line in body]
    return LITERAL_DOLLAR.sub(r"\\$", body)


def load(name):
    snippets = json.loads((source / name).read_text())
    return {key: {**value, "body": escape(value["body"])} for key, value in snippets.items()}


files = {
    "mise.json": ["toml-tasks-snippets.json", "toml-daemons-snippets.json"],
    "shell script.json": ["file-tasks-snippets.json"],
}
for output, inputs in files.items():
    merged = {}
    for name in inputs:
        merged.update(load(name))
    (target / output).write_text(json.dumps(merged, indent=2) + "\n")
    print(f"{output}: {len(merged)} snippets")
