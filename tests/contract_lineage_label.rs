//! A parent edge is a drawing label only (architecture · decision/board-read-model-shape; the
//! lineage registry): no wheel, link, send or answer path reads it. A census over those paths'
//! product code — comments masked, `#[cfg(test)]` items cut, `test_`-stemmed files left out — reads
//! red the moment one of them names the lineage or a `.parent` field. Its planted known positive
//! shows it bites.

use std::fs;
use std::path::{Path, PathBuf};

use source_scan::{Mask, masked};

#[allow(dead_code)]
#[path = "support/source_scan.rs"]
mod source_scan;

/// The wheel, link, send and answer paths, and the channel that carries their frames.
const PATHS: [&str; 7] = [
    "src/run/wheel.rs",
    "src/run/links.rs",
    "src/run/send.rs",
    "src/run/send",
    "src/run/dialog.rs",
    "src/run/gate.rs",
    "crates/viola-channel/src",
];

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// Every product `.rs` file under `rel`: the file itself, or a directory walked whole, a
/// `test_`-stemmed file left out.
fn product_files(rel: &str) -> Vec<PathBuf> {
    let path = root().join(rel);
    if path.is_file() {
        return vec![path];
    }
    let mut files = Vec::new();
    let mut dirs = vec![path];
    while let Some(dir) = dirs.pop() {
        for entry in fs::read_dir(&dir).expect("a census dir") {
            let path = entry.expect("an entry").path();
            let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
            if path.is_dir() {
                dirs.push(path);
            } else if path.extension().is_some_and(|e| e == "rs") && !stem.starts_with("test_") {
                files.push(path);
            }
        }
    }
    files.sort();
    files
}

/// Where the item after each `#[cfg(test)]` attribute ends: at its `;` or its matching `}`,
/// counted over `structure` (comments and literals masked).
fn test_items(structure: &[u8]) -> Vec<(usize, usize)> {
    const ATTR: &[u8] = b"#[cfg(test)]";
    let mut cut = Vec::new();
    let mut from = 0;
    while let Some(at) = structure[from..]
        .windows(ATTR.len())
        .position(|w| w == ATTR)
        .map(|n| from + n)
    {
        let mut depth = 0usize;
        let mut end = structure.len();
        for (i, &b) in structure.iter().enumerate().skip(at + ATTR.len()) {
            match b {
                b'{' => depth += 1,
                b'}' => {
                    depth = depth.saturating_sub(1);
                    if depth == 0 {
                        end = i + 1;
                        break;
                    }
                }
                b';' if depth == 0 => {
                    end = i + 1;
                    break;
                }
                _ => {}
            }
        }
        cut.push((at, end));
        from = end;
    }
    cut
}

/// The lines of `text`'s product code that name the lineage (`lineage`, `Lineage`) or read a
/// `.parent` field (a `.parent(` call, `Path::parent`, is no field), as `line: text`.
fn hits(text: &str) -> Vec<String> {
    let structure = masked(text, Mask::CommentsAndLiterals);
    let mut code = masked(text, Mask::Comments);
    for (from, to) in test_items(&structure) {
        for b in &mut code[from..to] {
            if *b != b'\n' {
                *b = b' ';
            }
        }
    }
    let code = String::from_utf8_lossy(&code).into_owned();
    code.lines()
        .enumerate()
        .filter(|(_, line)| {
            let field = line.match_indices(".parent").any(|(at, _)| {
                let next = line[at + ".parent".len()..].chars().next();
                !next.is_some_and(|c| c == '(' || c == '_' || c.is_ascii_alphanumeric())
            });
            line.to_ascii_lowercase().contains("lineage") || field
        })
        .map(|(n, line)| format!("{}: {}", n + 1, line.trim()))
        .collect()
}

fn census() -> Vec<String> {
    let mut found = Vec::new();
    for rel in PATHS {
        for path in product_files(rel) {
            let text = fs::read_to_string(&path).expect("a source file");
            let shown = path
                .strip_prefix(root())
                .map_or_else(|_| path.clone(), Path::to_path_buf);
            found.extend(
                hits(&text)
                    .into_iter()
                    .map(|hit| format!("{}:{hit}", shown.display())),
            );
        }
    }
    found
}

#[test]
fn lineage_is_read_by_no_wheel_link_send_or_answer_path() {
    let files: usize = PATHS.iter().map(|rel| product_files(rel).len()).sum();
    assert!(files >= PATHS.len(), "the census read {files} files");
    let found = census();
    assert!(
        found.is_empty(),
        "a lineage read in a wheel, link, send or answer path: {found:#?}"
    );
}

/// The census's known positive: a planted reader of the field reads one hit per line, by name,
/// while a comment, a `Path::parent` call and a `#[cfg(test)]` item naming it read none.
#[test]
fn lineage_census_reads_a_planted_hit() {
    let planted = concat!(
        "fn wheel(snapshot: &Snapshot) -> bool {\n",
        "    snapshot.lineage.is_some()\n",
        "}\n",
        "fn link(item: &Item) -> Option<&str> {\n",
        "    item.parent.as_deref()\n",
        "}\n",
        "fn send(line: &Value) -> bool {\n",
        "    line[\"kind\"] == \"lineage\"\n",
        "}\n",
    );
    assert_eq!(
        hits(planted),
        [
            "2: snapshot.lineage.is_some()",
            "5: item.parent.as_deref()",
            "8: line[\"kind\"] == \"lineage\"",
        ]
    );
    let quiet = concat!(
        "// the lineage is a label only\n",
        "fn dir(path: &Path) -> Option<&Path> {\n",
        "    path.parent()\n",
        "}\n",
        "#[cfg(test)]\n",
        "mod tests {\n",
        "    fn first() -> Snapshot { Snapshot { lineage: None } }\n",
        "}\n",
        "#[cfg(test)]\n",
        "use crate::Lineage;\n",
        "fn after() -> bool { true }\n",
    );
    assert_eq!(hits(quiet), Vec::<String>::new());
    let after_a_test_item = concat!(
        "#[cfg(test)]\n",
        "use crate::Lineage;\n",
        "fn wheel(s: &Snapshot) -> bool { s.lineage.is_none() }\n",
    );
    assert_eq!(
        hits(after_a_test_item),
        ["3: fn wheel(s: &Snapshot) -> bool { s.lineage.is_none() }"]
    );
}
