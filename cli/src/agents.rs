use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct AgentConfig {
    pub name: &'static str,
    pub display_name: &'static str,
    pub skills_dir: &'static str,
    pub global_skills_dir: PathBuf,
}

pub fn get_home_dir() -> PathBuf {
    dirs::home_dir().unwrap_or_else(|| PathBuf::from("~"))
}

pub fn get_all_agents() -> HashMap<&'static str, AgentConfig> {
    let home = get_home_dir();

    let mut agents = HashMap::new();

    agents.insert(
        "amp",
        AgentConfig {
            name: "amp",
            display_name: "Amp",
            skills_dir: ".agents/skills",
            global_skills_dir: home.join(".config/agents/skills"),
        },
    );

    agents.insert(
        "antigravity",
        AgentConfig {
            name: "antigravity",
            display_name: "Antigravity",
            skills_dir: ".agent/skills",
            global_skills_dir: home.join(".gemini/antigravity/skills"),
        },
    );

    agents.insert(
        "claude-code",
        AgentConfig {
            name: "claude-code",
            display_name: "Claude Code",
            skills_dir: ".claude/skills",
            global_skills_dir: home.join(".claude/skills"),
        },
    );

    agents.insert(
        "clawdbot",
        AgentConfig {
            name: "clawdbot",
            display_name: "Clawdbot",
            skills_dir: "skills",
            global_skills_dir: home.join(".clawdbot/skills"),
        },
    );

    agents.insert(
        "codex",
        AgentConfig {
            name: "codex",
            display_name: "Codex",
            skills_dir: ".codex/skills",
            global_skills_dir: home.join(".codex/skills"),
        },
    );

    agents.insert(
        "cursor",
        AgentConfig {
            name: "cursor",
            display_name: "Cursor",
            skills_dir: ".cursor/skills",
            global_skills_dir: home.join(".cursor/skills"),
        },
    );

    agents.insert(
        "droid",
        AgentConfig {
            name: "droid",
            display_name: "Droid",
            skills_dir: ".factory/skills",
            global_skills_dir: home.join(".factory/skills"),
        },
    );

    agents.insert(
        "gemini-cli",
        AgentConfig {
            name: "gemini-cli",
            display_name: "Gemini CLI",
            skills_dir: ".gemini/skills",
            global_skills_dir: home.join(".gemini/skills"),
        },
    );

    agents.insert(
        "github-copilot",
        AgentConfig {
            name: "github-copilot",
            display_name: "GitHub Copilot",
            skills_dir: ".github/skills",
            global_skills_dir: home.join(".copilot/skills"),
        },
    );

    agents.insert(
        "goose",
        AgentConfig {
            name: "goose",
            display_name: "Goose",
            skills_dir: ".goose/skills",
            global_skills_dir: home.join(".config/goose/skills"),
        },
    );

    agents.insert(
        "kilo",
        AgentConfig {
            name: "kilo",
            display_name: "Kilo Code",
            skills_dir: ".kilocode/skills",
            global_skills_dir: home.join(".kilocode/skills"),
        },
    );

    agents.insert(
        "kiro-cli",
        AgentConfig {
            name: "kiro-cli",
            display_name: "Kiro CLI",
            skills_dir: ".kiro/skills",
            global_skills_dir: home.join(".kiro/skills"),
        },
    );

    agents.insert(
        "opencode",
        AgentConfig {
            name: "opencode",
            display_name: "OpenCode",
            skills_dir: ".opencode/skills",
            global_skills_dir: home.join(".config/opencode/skills"),
        },
    );

    agents.insert(
        "pi",
        AgentConfig {
            name: "pi",
            display_name: "Pi",
            skills_dir: ".pi/skills",
            global_skills_dir: home.join(".pi/agent/skills"),
        },
    );

    agents.insert(
        "roo",
        AgentConfig {
            name: "roo",
            display_name: "Roo Code",
            skills_dir: ".roo/skills",
            global_skills_dir: home.join(".roo/skills"),
        },
    );

    agents.insert(
        "trae",
        AgentConfig {
            name: "trae",
            display_name: "Trae",
            skills_dir: ".trae/skills",
            global_skills_dir: home.join(".trae/skills"),
        },
    );

    agents.insert(
        "windsurf",
        AgentConfig {
            name: "windsurf",
            display_name: "Windsurf",
            skills_dir: ".windsurf/skills",
            global_skills_dir: home.join(".codeium/windsurf/skills"),
        },
    );

    agents
}

pub fn detect_installed_agents() -> Vec<&'static str> {
    let home = get_home_dir();
    let mut installed = Vec::new();

    let checks = [
        ("amp", home.join(".config/amp")),
        ("antigravity", home.join(".gemini/antigravity")),
        ("claude-code", home.join(".claude")),
        ("clawdbot", home.join(".clawdbot")),
        ("codex", home.join(".codex")),
        ("cursor", home.join(".cursor")),
        ("droid", home.join(".factory")),
        ("gemini-cli", home.join(".gemini")),
        ("github-copilot", home.join(".copilot")),
        ("goose", home.join(".config/goose")),
        ("kilo", home.join(".kilocode")),
        ("kiro-cli", home.join(".kiro")),
        ("opencode", home.join(".config/opencode")),
        ("pi", home.join(".pi/agent")),
        ("roo", home.join(".roo")),
        ("trae", home.join(".trae")),
        ("windsurf", home.join(".codeium/windsurf")),
    ];

    for (name, path) in checks {
        if path.exists() {
            installed.push(name);
        }
    }

    installed
}

pub fn get_agent_names() -> Vec<&'static str> {
    vec![
        "amp",
        "antigravity",
        "claude-code",
        "clawdbot",
        "codex",
        "cursor",
        "droid",
        "gemini-cli",
        "github-copilot",
        "goose",
        "kilo",
        "kiro-cli",
        "opencode",
        "pi",
        "roo",
        "trae",
        "windsurf",
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pi_installs_to_its_project_and_user_skill_dirs() {
        let agents = get_all_agents();
        let pi = agents.get("pi").expect("pi is a supported agent");

        assert_eq!(pi.display_name, "Pi");
        assert_eq!(pi.skills_dir, ".pi/skills");
        assert_eq!(
            pi.global_skills_dir,
            get_home_dir().join(".pi/agent/skills")
        );
    }

    #[test]
    fn every_accepted_agent_name_has_a_config_and_vice_versa() {
        let mut names = get_agent_names();
        names.sort_unstable();
        let mut configured: Vec<&str> = get_all_agents().into_keys().collect();
        configured.sort_unstable();

        assert_eq!(names, configured);
    }
}
