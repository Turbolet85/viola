//! The MCP-only plugin `viola plugin install` writes for a driver viola does not wrap: a local
//! marketplace holding one plugin, whose `.mcp.json` registers the server `viola` by the absolute
//! path of the exe that ran the install, on the home it was made for. It carries no hooks, so a
//! wrapped session that also has it installed never runs two hook processes for one event
//! (architecture §Established Decisions [Plugin Scope]). Each document is built as JSON, never by
//! a string template: an exe or home path may hold any character JSON must escape.

use serde_json::{Value, json};

/// The marketplace's name and its one plugin's: `claude plugin install viola@viola`.
pub const NAME: &str = "viola";

/// The plugin as the CLI names it, `<plugin>@<marketplace>`.
pub const PLUGIN_REF: &str = "viola@viola";

const VERSION: &str = env!("CARGO_PKG_VERSION");
const MARKETPLACE_DESCRIPTION: &str =
    "viola's MCP server for a Claude Code session that viola does not wrap";
const PLUGIN_DESCRIPTION: &str =
    "MCP tools send, wait, last, answer and list for driving sessions viola wraps";

/// The marketplace tree's three files, each `(path relative to the marketplace dir, content)`:
/// `exe_fwd` and `home_fwd` are absolute, in forward slashes.
pub fn marketplace_files(exe_fwd: &str, home_fwd: &str) -> [(&'static str, String); 3] {
    let author = json!({"name": NAME});
    let marketplace = json!({
        "name": NAME,
        "owner": {"name": NAME},
        "description": MARKETPLACE_DESCRIPTION,
        "plugins": [{
            "name": NAME,
            "source": format!("./{NAME}"),
            "description": PLUGIN_DESCRIPTION,
            "author": author,
        }],
    });
    let plugin = json!({
        "name": NAME,
        "version": VERSION,
        "description": PLUGIN_DESCRIPTION,
        "author": author,
    });
    let mcp = json!({
        "mcpServers": {
            NAME: {"command": exe_fwd, "args": ["--home", home_fwd, "mcp"]},
        },
    });
    [
        (".claude-plugin/marketplace.json", document(&marketplace)),
        ("viola/.claude-plugin/plugin.json", document(&plugin)),
        ("viola/.mcp.json", document(&mcp)),
    ]
}

fn document(value: &Value) -> String {
    format!("{value:#}\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXE: &str = "/opt/viola/bin/viola";
    const HOME: &str = "/home/u/.viola";

    fn parsed(files: &[(&'static str, String); 3]) -> Vec<(&'static str, Value)> {
        files
            .iter()
            .map(|(path, content)| (*path, serde_json::from_str(content).expect("json")))
            .collect()
    }

    /// The shape claude 2.1.287's `claude plugin validate --strict` passed: the marketplace
    /// manifest, the plugin manifest and the server registration.
    #[test]
    fn marketplace_files_are_the_validated_shape() {
        let files = marketplace_files(EXE, HOME);
        assert!(files.iter().all(|(_, content)| content.ends_with("}\n")));
        let docs = parsed(&files);
        let author = json!({"name": "viola"});
        assert_eq!(
            docs,
            [
                (
                    ".claude-plugin/marketplace.json",
                    json!({
                        "name": "viola",
                        "owner": {"name": "viola"},
                        "description": MARKETPLACE_DESCRIPTION,
                        "plugins": [{
                            "name": "viola",
                            "source": "./viola",
                            "description": PLUGIN_DESCRIPTION,
                            "author": author,
                        }],
                    })
                ),
                (
                    "viola/.claude-plugin/plugin.json",
                    json!({
                        "name": "viola",
                        "version": env!("CARGO_PKG_VERSION"),
                        "description": PLUGIN_DESCRIPTION,
                        "author": author,
                    })
                ),
                (
                    "viola/.mcp.json",
                    json!({"mcpServers": {"viola": {
                        "command": EXE,
                        "args": ["--home", HOME, "mcp"],
                    }}})
                ),
            ]
        );
        assert_eq!(format!("{NAME}@{NAME}"), PLUGIN_REF);
        for text in [MARKETPLACE_DESCRIPTION, PLUGIN_DESCRIPTION] {
            assert!(text.is_ascii() && !text.is_empty(), "{text}");
        }
    }

    #[test]
    fn marketplace_mcp_json_escapes_the_paths() {
        let exe = r#"C:/Program Files/a"b\c/viola.exe"#;
        let home = r#"C:/Users/a b/"q"\.viola"#;
        let files = marketplace_files(exe, home);
        let mcp: Value = serde_json::from_str(&files[2].1).expect("json");
        let server = &mcp["mcpServers"]["viola"];
        assert_eq!(server["command"], exe);
        assert_eq!(server["args"], json!(["--home", home, "mcp"]));
    }

    fn holds_a_hooks_key(value: &Value) -> bool {
        match value {
            Value::Object(map) => map
                .iter()
                .any(|(key, value)| key == "hooks" || holds_a_hooks_key(value)),
            Value::Array(items) => items.iter().any(holds_a_hooks_key),
            _ => false,
        }
    }

    #[test]
    fn marketplace_files_hold_no_hooks() {
        assert!(holds_a_hooks_key(&json!({"a": [{"hooks": {}}]})));
        let files = marketplace_files(EXE, HOME);
        for (path, doc) in parsed(&files) {
            assert!(!holds_a_hooks_key(&doc), "{path}");
        }
        for (path, content) in &files {
            assert!(!content.contains("hooks"), "{path}");
            assert!(!path.contains("hooks"), "{path}");
        }
    }
}
