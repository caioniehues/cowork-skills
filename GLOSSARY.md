# CoWork Skills

A CLI (`cowork` / `co`) that fetches skills from GitHub repositories or local directories and installs them into the coding agents on a machine.

## Where things go

**Agent**:
A coding tool that cowork installs skills into, such as Claude Code, Cursor or Pi, named by its agent id (`claude-code`, `cursor`, `pi`).
_Avoid_: tool, client, IDE, target

**Scope**:
How far an install reaches: **Global** (every project for this user) or **Project** (one project directory).
_Avoid_: local (Claude Code uses "local" for a personal, gitignored project setting), user

**Skills directory**:
The directory an Agent loads Skills from in a given Scope, such as `~/.cursor/skills` for Global or `<project>/.cursor/skills` for Project.
_Avoid_: skills dir, install path, target dir

## What gets installed

**Skill**:
A directory containing a `SKILL.md` that an Agent loads as instructions; its name is the directory name.
_Avoid_: prompt, command, rule

**Trigger**:
A phrase listed in a Skill's description that should make an Agent load that Skill.
_Avoid_: keyword, activation phrase

**Router skill**:
A Skill that cowork generates to point an Agent at the right one of the other installed Skills for a request.
_Avoid_: index skill, dispatcher

## Claude Code plugins

**Plugin**:
A directory with a `.claude-plugin/plugin.json` manifest that Claude Code loads as a bundle of Skills, commands, hooks and servers.
_Avoid_: extension, package

**Marketplace**:
A catalogue of Plugins that Claude Code knows by name, and from which it installs them.
_Avoid_: registry, store, source

**Plugin id**:
The `plugin@marketplace` name by which Claude Code identifies a Plugin; `plugin` is the manifest name, and a Plugin loaded from a Skills directory has the marketplace `skills-dir`.
_Avoid_: plugin key, full name
