# Plugins live in Claude Code's Skills directory; cowork never writes Claude Code's plugin files

cowork used to register Plugins by hand-writing Claude Code's internal, versioned state (`installed_plugins.json` version 2, `known_marketplaces.json`, `enabledPlugins`), and copied each plugin repo to `~/.claude/<repo-name>`. Those writes drifted from what Claude Code itself writes (a fake `source: github` for local repos, dropped keys, non-atomic writes), and the copy path could `remove_dir_all` Claude Code's own directories. We decided that cowork installs a Plugin for Claude Code by placing it, symlinked by default, in Claude Code's Skills directory for the Scope, where Claude Code loads it as `<manifest name>@skills-dir`. cowork reads and toggles Plugins only through the `claude plugin` CLI (`list --json`, `enable`, `disable`, and `uninstall`/`marketplace remove` to clean up old-style installs), and has no fallback that edits Claude Code's files.

Verified on Claude Code 2.1.331 (2026-11-24): a symlinked `~/.claude/skills/<any-dir>` with `.claude-plugin/plugin.json` loads as `<manifest name>@skills-dir`, enabled by default with no `enabledPlugins` entry; `<project>/.claude/skills/<dir>` is detected as project scope but loads only once the workspace is trusted.

## Considered Options

- **Keep hand-writing the files.** Rejected: the formats are Claude Code's internal state, already versioned once, and cowork's copies of them had diverged.
- **The marketplace route** (`claude plugin marketplace add` + `claude plugin install`). Supported and documented, but it needs a Scope mapping onto Claude Code's user/project/local, a second update mechanism beside `git pull`, and a synthesized marketplace for repos that ship only `plugin.json`. Still the route for Plugins a marketplace sources from another repo: cowork prints the command instead of installing them.

## Consequences

- Plugin installs depend on Claude Code's skills-dir loading, which secondary sources date to about v2.1.157 and which was undocumented until mid-2026. If Claude Code drops it, the marketplace route above is the fallback design.
- Project-scope Plugins don't load until the user trusts the workspace in Claude Code.
- Features that list Plugin Skills (trigger tests, the Router skill) need `claude` on PATH, and fall back to plain Skills without it.
