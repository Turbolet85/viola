//! Fixture hygiene (test-plan §7, security-plan §Data Protection "Repository fixtures"): a
//! committed fixture holds no absolute path, no real username, no email-shaped token and passes
//! its schema. A violation reports its class only, never the offending content.

use std::path::Path;

use serde_json::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Violation {
    AbsolutePath,
    Username,
    Email,
    Schema,
}

impl Violation {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AbsolutePath => "absolute-path",
            Self::Username => "username",
            Self::Email => "email",
            Self::Schema => "schema",
        }
    }
}

/// The only username a fixture may carry.
pub const PLACEHOLDER_USER: &str = "<user>";

/// `^[A-Za-z]:[\\/]`, `/home/`, `/Users/`, `\Users\` over a decoded string.
pub fn has_absolute_path(s: &str) -> bool {
    let b = s.as_bytes();
    let drive =
        b.len() >= 3 && b[0].is_ascii_alphabetic() && b[1] == b':' && matches!(b[2], b'\\' | b'/');
    drive || s.contains("/home/") || s.contains("/Users/") || s.contains("\\Users\\")
}

/// `user` as a whole word (case-insensitive) anywhere in `s`.
pub fn has_username(s: &str, user: &str) -> bool {
    if user.is_empty() || user == PLACEHOLDER_USER {
        return false;
    }
    let hay = s.to_lowercase();
    let needle = user.to_lowercase();
    let is_word = |c: char| c.is_alphanumeric() || c == '_';
    hay.match_indices(&needle).any(|(at, m)| {
        let before = hay[..at].chars().next_back().is_none_or(|c| !is_word(c));
        let after = hay[at + m.len()..]
            .chars()
            .next()
            .is_none_or(|c| !is_word(c));
        before && after
    })
}

/// `local@domain.tld`: a local part, then a domain holding a dot followed by two or more letters.
pub fn has_email(text: &str) -> bool {
    text.match_indices('@').any(|(at, _)| {
        let local = text[..at]
            .chars()
            .next_back()
            .is_some_and(|c| c.is_ascii_alphanumeric() || "._%+-".contains(c));
        let domain: String = text[at + 1..]
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric() || *c == '.' || *c == '-')
            .collect();
        let tld = domain.rsplit_once('.').is_some_and(|(host, tld)| {
            !host.is_empty() && tld.len() >= 2 && tld.chars().all(|c| c.is_ascii_alphabetic())
        });
        local && tld
    })
}

fn strings<'a>(v: &'a Value, out: &mut Vec<&'a str>) {
    match v {
        Value::String(s) => out.push(s),
        Value::Array(a) => a.iter().for_each(|x| strings(x, out)),
        Value::Object(o) => o.iter().for_each(|(k, x)| {
            out.push(k);
            strings(x, out);
        }),
        _ => {}
    }
}

/// The host's own username, the one PII value a recorded fixture could leak.
pub fn host_user() -> Option<String> {
    ["USER", "USERNAME"]
        .iter()
        .find_map(|k| std::env::var(k).ok())
        .filter(|u| !u.is_empty())
}

pub fn check(bytes: &[u8], schema: &Value, user: Option<&str>) -> Result<(), Violation> {
    let doc: Value = serde_json::from_slice(bytes).map_err(|_| Violation::Schema)?;
    let mut all = Vec::new();
    strings(&doc, &mut all);
    if all.iter().any(|s| has_absolute_path(s)) {
        return Err(Violation::AbsolutePath);
    }
    if let Some(user) = user
        && all.iter().any(|s| has_username(s, user))
    {
        return Err(Violation::Username);
    }
    if all.iter().any(|s| has_email(s)) {
        return Err(Violation::Email);
    }
    let validator = jsonschema::validator_for(schema).map_err(|_| Violation::Schema)?;
    if validator.is_valid(&doc) {
        Ok(())
    } else {
        Err(Violation::Schema)
    }
}

pub fn load_schema(path: &Path) -> Value {
    serde_json::from_str(&std::fs::read_to_string(path).expect("schema")).expect("schema JSON")
}
