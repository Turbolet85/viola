//! Which missed mutants this host never compiled (test-plan §10 Mutation gate): the `#[cfg]`
//! attributes around a mutant's span, read from the checked-out source and judged for the host the
//! harness was built for. A mutant leaves the count only when a predicate that covers its whole
//! span is proven false here; a path, file, node, attribute or key this module cannot read keeps
//! it counted, so uncertainty never excuses a survivor.

use std::fs;
use std::path::{Component, Path, PathBuf};

use proc_macro2::{LineColumn, Span};
use serde::Serialize;
use serde_json::Value;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::visit::{self, Visit};
use syn::{Attribute, Expr, ExprLit, Lit, Meta, Token};

/// What a `cfg` predicate is judged against.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Host {
    pub(super) family: &'static str,
    pub(super) os: &'static str,
    pub(super) arch: &'static str,
}

/// The host the harness was compiled for, which is the one cargo-mutants builds every mutant for.
/// A const, never a child process or an environment read (the `HOST_SCRATCH` shape).
pub(super) const HOST: Host = Host {
    family: std::env::consts::FAMILY,
    os: std::env::consts::OS,
    arch: std::env::consts::ARCH,
};

/// One missed mutant left out of the count: cargo-mutants' own name for it, and the predicate, as
/// written in source, that is false on this host.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(super) struct Excluded {
    pub(super) name: String,
    pub(super) cfg: String,
}

/// A line and a 0-based column in characters, as proc-macro2 counts them.
type Position = (usize, usize);
type Range = (Position, Position);

/// The `MissedMutant` records of a run's `outcomes.json` whose whole span `host` never compiled.
/// A record of any other summary, or one whose file or span cannot be read, is never left out.
pub(super) fn excluded_missed(root: &Path, outcomes: &Value, host: Host) -> Vec<Excluded> {
    outcomes["outcomes"]
        .as_array()
        .into_iter()
        .flatten()
        .filter(|record| record["summary"] == "MissedMutant")
        .filter_map(|record| {
            let mutant = &record["scenario"]["Mutant"];
            let span = (
                position(&mutant["span"]["start"])?,
                position(&mutant["span"]["end"])?,
            );
            let cfg = false_cfg(root, mutant["file"].as_str()?, span, host)?;
            Some(Excluded {
                name: mutant["name"].as_str()?.to_owned(),
                cfg,
            })
        })
        .collect()
}

/// cargo-mutants writes a 1-based line and a 1-based column.
fn position(at: &Value) -> Option<Position> {
    let line = usize::try_from(at["line"].as_u64()?).ok()?;
    let column = usize::try_from(at["column"].as_u64()?).ok()?;
    Some((line, column.checked_sub(1)?))
}

/// The predicate that proves `host` never compiled `span` of the repo-relative `file`: one on the
/// `mod` declaration that brings the file in, else one on a node that holds the whole span.
fn false_cfg(root: &Path, file: &str, span: Range, host: Host) -> Option<String> {
    let file = Path::new(file);
    if !file.components().all(|c| matches!(c, Component::Normal(_))) {
        return None;
    }
    let parsed = read(&root.join(file))?;
    declared_false(root, file, host).or_else(|| false_cover(&parsed, span, host))
}

fn read(path: &Path) -> Option<syn::File> {
    syn::parse_file(&fs::read_to_string(path).ok()?).ok()
}

/// Walks up from `file` through the files that declare it: the first level at which every `mod`
/// declaration of the module is under a predicate false on `host` gives that predicate. A level
/// with one declaration that compiles is passed, since the module above it may still be left out.
fn declared_false(root: &Path, file: &Path, host: Host) -> Option<String> {
    let mut file = file.to_path_buf();
    loop {
        let (name, dir) = module_of(&file)?;
        let (declarer, parsed, declared) = [
            dir.join("lib.rs"),
            dir.join("main.rs"),
            dir.join("mod.rs"),
            dir.with_extension("rs"),
        ]
        .into_iter()
        .find_map(|candidate| {
            let parsed = read(&root.join(&candidate))?;
            let declared = declarations(&parsed, &name);
            (!declared.is_empty()).then_some((candidate, parsed, declared))
        })?;
        let cfgs: Option<Vec<String>> = declared
            .into_iter()
            .map(|at| false_cover(&parsed, at, host))
            .collect();
        if let Some(cfg) = cfgs.and_then(|cfgs| cfgs.into_iter().next()) {
            return Some(cfg);
        }
        file = declarer;
    }
}

/// The module a file is, and the directory its declaring file sits in or is named after: `d/x.rs`
/// and `d/x/mod.rs` are both `x` under `d`. A crate root is declared by no `mod`.
fn module_of(file: &Path) -> Option<(String, PathBuf)> {
    let stem = file.file_stem()?.to_str()?;
    let dir = file.parent()?;
    match stem {
        "lib" | "main" => None,
        "mod" => Some((
            dir.file_name()?.to_str()?.to_owned(),
            dir.parent()?.to_path_buf(),
        )),
        _ => Some((stem.to_owned(), dir.to_path_buf())),
    }
}

/// Where `parsed` declares the out-of-line module `name` (`mod name;`), at any depth.
fn declarations(parsed: &syn::File, name: &str) -> Vec<Range> {
    let mut found = Declarations {
        name,
        at: Vec::new(),
    };
    found.visit_file(parsed);
    found.at
}

struct Declarations<'a> {
    name: &'a str,
    at: Vec<Range>,
}

impl<'ast> Visit<'ast> for Declarations<'_> {
    fn visit_item_mod(&mut self, node: &'ast syn::ItemMod) {
        if node.content.is_none() && node.ident == self.name {
            self.at.push(range(node.span()));
        }
        visit::visit_item_mod(self, node);
    }
}

fn range(span: Span) -> Range {
    let at = |p: LineColumn| (p.line, p.column);
    (at(span.start()), at(span.end()))
}

/// The outermost predicate false on `host` among the nodes of `parsed` that hold all of `span`.
fn false_cover(parsed: &syn::File, span: Range, host: Host) -> Option<String> {
    let mut cover = Cover {
        span,
        host,
        found: None,
    };
    cover.visit_file(parsed);
    cover.found
}

struct Cover {
    span: Range,
    host: Host,
    found: Option<String>,
}

impl Cover {
    fn note(&mut self, attrs: &[Attribute], node: &impl Spanned) {
        if self.found.is_none() && holds(range(node.span()), self.span) {
            self.found = attrs.iter().find_map(|a| false_predicate(a, self.host));
        }
    }
}

/// Whether `node` holds the whole of `span`: a span that starts before the node or ends after it
/// is compiled in part elsewhere, and is not this node's to leave out.
fn holds(node: Range, span: Range) -> bool {
    node.0 <= span.0 && span.1 <= node.1
}

/// The nodes a `#[cfg]` sits on in this workspace's sources: items that hold a body or an
/// initializer, a `let`, and the statement forms below. A `cfg` on any other node is not read,
/// which keeps the mutant counted.
impl<'ast> Visit<'ast> for Cover {
    fn visit_item_fn(&mut self, node: &'ast syn::ItemFn) {
        self.note(&node.attrs, node);
        visit::visit_item_fn(self, node);
    }

    fn visit_item_impl(&mut self, node: &'ast syn::ItemImpl) {
        self.note(&node.attrs, node);
        visit::visit_item_impl(self, node);
    }

    fn visit_item_mod(&mut self, node: &'ast syn::ItemMod) {
        self.note(&node.attrs, node);
        visit::visit_item_mod(self, node);
    }

    fn visit_item_const(&mut self, node: &'ast syn::ItemConst) {
        self.note(&node.attrs, node);
        visit::visit_item_const(self, node);
    }

    fn visit_item_static(&mut self, node: &'ast syn::ItemStatic) {
        self.note(&node.attrs, node);
        visit::visit_item_static(self, node);
    }

    fn visit_impl_item_fn(&mut self, node: &'ast syn::ImplItemFn) {
        self.note(&node.attrs, node);
        visit::visit_impl_item_fn(self, node);
    }

    fn visit_local(&mut self, node: &'ast syn::Local) {
        self.note(&node.attrs, node);
        visit::visit_local(self, node);
    }

    fn visit_expr_block(&mut self, node: &'ast syn::ExprBlock) {
        self.note(&node.attrs, node);
        visit::visit_expr_block(self, node);
    }

    fn visit_expr_call(&mut self, node: &'ast syn::ExprCall) {
        self.note(&node.attrs, node);
        visit::visit_expr_call(self, node);
    }

    fn visit_expr_for_loop(&mut self, node: &'ast syn::ExprForLoop) {
        self.note(&node.attrs, node);
        visit::visit_expr_for_loop(self, node);
    }

    fn visit_expr_if(&mut self, node: &'ast syn::ExprIf) {
        self.note(&node.attrs, node);
        visit::visit_expr_if(self, node);
    }

    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
        self.note(&node.attrs, node);
        visit::visit_expr_method_call(self, node);
    }

    fn visit_expr_tuple(&mut self, node: &'ast syn::ExprTuple) {
        self.note(&node.attrs, node);
        visit::visit_expr_tuple(self, node);
    }

    fn visit_expr_unsafe(&mut self, node: &'ast syn::ExprUnsafe) {
        self.note(&node.attrs, node);
        visit::visit_expr_unsafe(self, node);
    }
}

/// The predicate of a `cfg` attribute, as written, when it is false on `host`. A `cfg_attr`, and
/// any other attribute, is not a `cfg`.
fn false_predicate(attr: &Attribute, host: Host) -> Option<String> {
    if !attr.path().is_ident("cfg") {
        return None;
    }
    let meta = attr.parse_args::<Meta>().ok()?;
    if eval(&meta, host) != Some(false) {
        return None;
    }
    meta.span().source_text()
}

/// One predicate on one host: `Some` when decided, `None` when unknown. `unix`, `windows`,
/// `target_family`, `target_os` and `target_arch` are decided; every other key is unknown.
fn eval(cfg: &Meta, host: Host) -> Option<bool> {
    match cfg {
        Meta::Path(p) if p.is_ident("unix") => Some(host.family == "unix"),
        Meta::Path(p) if p.is_ident("windows") => Some(host.family == "windows"),
        Meta::Path(_) => None,
        Meta::NameValue(nv) => {
            let Expr::Lit(ExprLit {
                lit: Lit::Str(value),
                ..
            }) = &nv.value
            else {
                return None;
            };
            let fact = if nv.path.is_ident("target_family") {
                host.family
            } else if nv.path.is_ident("target_os") {
                host.os
            } else if nv.path.is_ident("target_arch") {
                host.arch
            } else {
                return None;
            };
            Some(fact == value.value())
        }
        Meta::List(list) => {
            let args = list
                .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
                .ok()?;
            let values: Vec<Option<bool>> = args.iter().map(|m| eval(m, host)).collect();
            if list.path.is_ident("not") {
                match values.as_slice() {
                    [v] => v.map(|b| !b),
                    _ => None,
                }
            } else if list.path.is_ident("all") {
                all(&values)
            } else if list.path.is_ident("any") {
                any(&values)
            } else {
                None
            }
        }
    }
}

fn all(values: &[Option<bool>]) -> Option<bool> {
    if values.contains(&Some(false)) {
        Some(false)
    } else if values.iter().all(|v| *v == Some(true)) {
        Some(true)
    } else {
        None
    }
}

fn any(values: &[Option<bool>]) -> Option<bool> {
    if values.contains(&Some(true)) {
        Some(true)
    } else if values.iter().all(|v| *v == Some(false)) {
        Some(false)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    const LINUX: Host = Host {
        family: "unix",
        os: "linux",
        arch: "x86_64",
    };
    const MACOS: Host = Host {
        family: "unix",
        os: "macos",
        arch: "aarch64",
    };
    const WINDOWS: Host = Host {
        family: "windows",
        os: "windows",
        arch: "x86_64",
    };
    const WINDOWS_ARM: Host = Host {
        arch: "aarch64",
        ..WINDOWS
    };

    #[test]
    fn host_is_the_one_the_harness_was_built_for() {
        let family = if cfg!(windows) { "windows" } else { "unix" };
        let os = if cfg!(target_os = "linux") {
            "linux"
        } else if cfg!(target_os = "macos") {
            "macos"
        } else {
            "windows"
        };
        let arch = if cfg!(target_arch = "x86_64") {
            "x86_64"
        } else {
            "aarch64"
        };
        assert_eq!(HOST, Host { family, os, arch });
    }

    fn ev(cfg: &str, host: Host) -> Option<bool> {
        let meta: Meta = syn::parse_str(cfg).expect("cfg");
        eval(&meta, host)
    }

    /// Each predicate on Linux, on Windows x86_64, on macOS and on Windows aarch64.
    #[test]
    fn eval_decides_family_os_and_arch_and_nothing_else() {
        let (t, f) = (Some(true), Some(false));
        let cases: [(&str, [Option<bool>; 4]); 20] = [
            ("unix", [t, f, t, f]),
            ("windows", [f, t, f, t]),
            ("not(unix)", [f, t, f, t]),
            ("not(windows)", [t, f, t, f]),
            ("target_os = \"linux\"", [t, f, f, f]),
            ("target_os = \"macos\"", [f, f, t, f]),
            ("target_os = \"windows\"", [f, t, f, t]),
            ("target_family = \"windows\"", [f, t, f, t]),
            ("target_family = \"unix\"", [t, f, t, f]),
            ("target_arch = \"x86_64\"", [t, t, f, f]),
            ("target_arch = \"aarch64\"", [f, f, t, t]),
            ("all(windows, not(target_arch = \"x86_64\"))", [f, f, f, t]),
            ("test", [None; 4]),
            ("feature = \"x\"", [None; 4]),
            ("target_env = \"msvc\"", [None; 4]),
            ("target_pointer_width = \"64\"", [None; 4]),
            ("not(unix, windows)", [None; 4]),
            ("not(test)", [None; 4]),
            ("target_os = 1", [None; 4]),
            ("cfg_x(unix)", [None; 4]),
        ];
        for (cfg, want) in cases {
            let got = [LINUX, WINDOWS, MACOS, WINDOWS_ARM].map(|host| ev(cfg, host));
            assert_eq!(got, want, "{cfg}");
        }
    }

    /// `unix` is true, `windows` false and `test` unknown on the Linux host.
    #[test]
    fn eval_all_and_any_are_three_valued() {
        let (t, f, u) = ("unix", "windows", "test");
        for (cfg, want) in [
            ("all()".to_owned(), Some(true)),
            (format!("all({t}, {t})"), Some(true)),
            (format!("all({t}, {u})"), None),
            (format!("all({u}, {t})"), None),
            (format!("all({t}, {f})"), Some(false)),
            (format!("all({u}, {f})"), Some(false)),
            ("any()".to_owned(), Some(false)),
            (format!("any({f}, {t})"), Some(true)),
            (format!("any({u}, {t})"), Some(true)),
            (format!("any({f}, {u})"), None),
            (format!("any({u}, {f})"), None),
            (format!("any({f}, {f})"), Some(false)),
        ] {
            assert_eq!(ev(&cfg, LINUX), want, "{cfg}");
        }
    }

    /// One `mark(n)` per node kind the reader takes a `cfg` from, and per way it must not.
    const LIB: &str = r#"#[cfg(windows)]
fn item_fn() {
    mark(1);
}
#[cfg(unix)]
impl S {
    fn in_impl() {
        mark(2);
    }
}
#[cfg(not(unix))]
mod inline {
    fn in_mod() {
        mark(3);
    }
}
impl S {
    #[cfg(unix)]
    fn impl_fn() {
        mark(4);
    }
}
#[cfg(windows)]
const FLAGS: u32 = mark(5);
#[cfg(unix)]
static MODE: u32 = mark(6);
fn statements() {
    #[cfg(windows)]
    let _ = mark(7);
    #[cfg(unix)]
    for _ in 0..1 {
        mark(8);
    }
    #[cfg(windows)]
    unsafe {
        mark(9);
    }
    #[cfg(unix)]
    {
        mark(10);
    }
    #[cfg(windows)]
    if mark(11) {}
    #[cfg(unix)]
    mark(12);
    #[cfg(windows)]
    x.mark(13);
    #[cfg(unix)]
    (mark(14), 2);
    mark(15);
}
#[cfg(target_os = "macos")]
fn mac() {
    mark(16);
}
#[cfg(unix)]
impl T {
    #[cfg(target_os = "linux")]
    fn nested() {
        mark(17);
    }
}
#[cfg(feature = "x")]
fn unknown_key() {
    mark(18);
}
#[cfg(all(windows, not(target_arch = "x86_64")))]
fn other_arch() {
    mark(19);
}
#[cfg_attr(windows, inline)]
fn cfg_attr() {
    mark(20);
}
#[deny(windows)]
fn another_attribute() {
    mark(21);
}
#[cfg(windows)]
fn before() {
    mark(22);
}
fn after() {
    mark(23);
}
#[cfg(windows)]
mod gated;
mod plain;
#[cfg(unix)]
mod deep;
#[cfg(target_os = "linux")]
mod twice;
#[cfg(windows)]
mod twice;
"#;

    /// A throwaway tree: `LIB` as its crate root, the modules it declares, a `main.rs` that
    /// declares the crate root as a module of its own, and two files no `mod` brings in.
    fn tree() -> tempfile::TempDir {
        let tmp = tempfile::tempdir().expect("tempdir");
        let src = tmp.path().join("src");
        fs::create_dir_all(src.join("deep")).expect("mkdir");
        for (file, text) in [
            ("lib.rs", LIB),
            ("main.rs", "#[cfg(windows)]\nmod lib;\n"),
            ("gated.rs", "fn g() {\n    mark(30);\n}\n"),
            ("plain.rs", "fn p() {\n    mark(31);\n}\n"),
            ("deep/mod.rs", "mod inner;\nfn q() {\n    mark(32);\n}\n"),
            ("deep/inner.rs", "fn r() {\n    mark(33);\n}\n"),
            ("twice.rs", "fn t() {\n    mark(34);\n}\n"),
            ("orphan.rs", "#[cfg(windows)]\nfn o() {\n    mark(40);\n}\n"),
            ("bad.rs", "#[cfg(windows)]\nfn (\n    mark(41);\n"),
        ] {
            fs::write(src.join(file), text).expect("source");
        }
        tmp
    }

    /// Where `needle` first lies in `text`: its first and one-past-last character, each as
    /// cargo-mutants writes a position (1-based line, 1-based column).
    fn found(text: &str, needle: &str) -> (Position, Position) {
        let at = text
            .find(needle)
            .unwrap_or_else(|| panic!("{needle} in the source"));
        let position = |offset: usize| {
            let before = &text[..offset];
            let line = before.matches('\n').count() + 1;
            let column = before.rsplit('\n').next().unwrap_or("").chars().count() + 1;
            (line, column)
        };
        (position(at), position(at + needle.len()))
    }

    /// The outcome record of a mutant of `file` that spans `from` through `to`.
    fn record(root: &Path, file: &str, from: &str, to: &str, summary: &str) -> Value {
        let text = fs::read_to_string(root.join(file)).unwrap_or_default();
        let (start, _) = found(&text, from);
        let (_, end) = found(&text, to);
        json!({
            "scenario": {"Mutant": {
                "name": format!("{file}:{}:{}: replace {from}", start.0, start.1),
                "file": file,
                "span": {
                    "start": {"line": start.0, "column": start.1},
                    "end": {"line": end.0, "column": end.1},
                },
            }},
            "summary": summary,
        })
    }

    /// The predicate that leaves out a missed mutant of `file` spanning `from` through `to`.
    fn left_out(root: &Path, file: &str, from: &str, to: &str, host: Host) -> Option<String> {
        let outcomes = json!({"outcomes": [record(root, file, from, to, "MissedMutant")]});
        let mut excluded = excluded_missed(root, &outcomes, host);
        assert!(excluded.len() <= 1);
        excluded.pop().map(|e| e.cfg)
    }

    #[test]
    fn excluded_missed_reads_the_cfg_on_each_node_kind() {
        let t = tree();
        let (unix, windows) = (Some("unix"), Some("windows"));
        let cases: [(&str, &str, Option<&str>, Option<&str>); 21] = [
            ("an item fn", "mark(1)", windows, None),
            ("an impl", "mark(2)", None, unix),
            ("an inline module", "mark(3)", Some("not(unix)"), None),
            ("a fn in an impl", "mark(4)", None, unix),
            ("a const", "mark(5)", windows, None),
            ("a static", "mark(6)", None, unix),
            ("a let", "mark(7)", windows, None),
            ("a for loop", "mark(8)", None, unix),
            ("an unsafe block", "mark(9)", windows, None),
            ("a block", "mark(10)", None, unix),
            ("an if", "mark(11)", windows, None),
            ("a call", "mark(12)", None, unix),
            ("a method call", "mark(13)", windows, None),
            ("a tuple", "mark(14)", None, unix),
            ("no cfg", "mark(15)", None, None),
            (
                "another OS",
                "mark(16)",
                Some("target_os = \"macos\""),
                Some("target_os = \"macos\""),
            ),
            ("the outer of two", "mark(17)", None, unix),
            ("an unknown key", "mark(18)", None, None),
            (
                "an architecture",
                "mark(19)",
                Some("all(windows, not(target_arch = \"x86_64\"))"),
                Some("all(windows, not(target_arch = \"x86_64\"))"),
            ),
            ("a cfg_attr", "mark(20)", None, None),
            ("an attribute that is no cfg", "mark(21)", None, None),
        ];
        for (case, mark, on_linux, on_windows) in cases {
            let at = |host| left_out(t.path(), "src/lib.rs", mark, mark, host);
            assert_eq!(at(LINUX).as_deref(), on_linux, "{case}, on Linux");
            assert_eq!(at(WINDOWS).as_deref(), on_windows, "{case}, on Windows");
        }
        let at = |mark, host| left_out(t.path(), "src/lib.rs", mark, mark, host);
        assert_eq!(
            at("mark(17)", MACOS).as_deref(),
            Some("target_os = \"linux\""),
            "the inner of two, when the outer holds"
        );
        assert_eq!(at("mark(16)", MACOS), None, "its own OS");
        assert_eq!(at("mark(19)", WINDOWS_ARM), None, "its own architecture");
    }

    /// A span is left out only by a node that holds all of it.
    #[test]
    fn excluded_missed_keeps_a_span_that_leaves_its_node() {
        let t = tree();
        let lib = |from, to| left_out(t.path(), "src/lib.rs", from, to, LINUX);
        assert_eq!(lib("mark(22)", "mark(22)").as_deref(), Some("windows"));
        assert_eq!(lib("mark(22)", "mark(23)"), None, "it ends after the node");
        assert_eq!(
            lib("mark(21)", "mark(22)"),
            None,
            "it starts before the node"
        );
        assert_eq!(
            lib("#[cfg(windows)]\nfn before", "mark(22);\n}").as_deref(),
            Some("windows"),
            "the node's own extent"
        );
        assert_eq!(
            lib("#[cfg(windows)]\nfn before", "mark(22);\n}\n"),
            None,
            "one character past the node"
        );
        assert_eq!(
            lib("\n#[cfg(windows)]\nfn before", "mark(22)"),
            None,
            "one character ahead of the node"
        );
    }

    #[test]
    fn excluded_missed_follows_the_mod_declarations_above_a_file() {
        let t = tree();
        let cases: [(&str, &str, &str, [Option<&str>; 3]); 7] = [
            (
                "a file under a false declaration",
                "src/gated.rs",
                "mark(30)",
                [Some("windows"), None, Some("windows")],
            ),
            ("a file under no cfg", "src/plain.rs", "mark(31)", [None; 3]),
            (
                "a mod.rs",
                "src/deep/mod.rs",
                "mark(32)",
                [None, Some("unix"), None],
            ),
            (
                "a file under a module that is left out",
                "src/deep/inner.rs",
                "mark(33)",
                [None, Some("unix"), None],
            ),
            (
                "a file declared twice",
                "src/twice.rs",
                "mark(34)",
                [None, None, Some("target_os = \"linux\"")],
            ),
            (
                "a file no mod declares",
                "src/orphan.rs",
                "mark(40)",
                [Some("windows"), None, Some("windows")],
            ),
            (
                "a crate root another root declares",
                "src/lib.rs",
                "mark(15)",
                [None; 3],
            ),
        ];
        for (case, file, mark, want) in cases {
            let got =
                [LINUX, WINDOWS, MACOS].map(|host| left_out(t.path(), file, mark, mark, host));
            assert_eq!(got.each_ref().map(|g| g.as_deref()), want, "{case}");
        }
    }

    /// Only a missed record is left out, and only when its file and span can be read.
    #[test]
    fn excluded_missed_leaves_out_only_missed_records_it_can_read() {
        let t = tree();
        let root = t.path();
        let lib = |mark, summary| record(root, "src/lib.rs", mark, mark, summary);
        let missed = lib("mark(1)", "MissedMutant");
        let name = missed["scenario"]["Mutant"]["name"].clone();
        let reshaped = |change: &dyn Fn(&mut Value)| {
            let mut record = missed.clone();
            change(&mut record["scenario"]["Mutant"]);
            record
        };
        let outcomes = json!({"outcomes": [
            {"scenario": "Baseline", "summary": "Success"},
            lib("mark(5)", "CaughtMutant"),
            lib("mark(7)", "Timeout"),
            lib("mark(9)", "Unviable"),
            lib("mark(11)", "Failure"),
            lib("mark(15)", "MissedMutant"),
            missed.clone(),
            reshaped(&|m| m["file"] = json!("src/missing.rs")),
            reshaped(&|m| m["file"] = json!("src/bad.rs")),
            reshaped(&|m| m["file"] = json!("../src/lib.rs")),
            reshaped(&|m| m["file"] = json!(root.join("src/lib.rs"))),
            reshaped(&|m| m["file"] = json!(7)),
            reshaped(&|m| m["span"]["start"]["column"] = json!(0)),
            reshaped(&|m| m["span"]["end"] = json!(null)),
            reshaped(&|m| m["span"]["start"]["line"] = json!("3")),
            reshaped(&|m| m["name"] = json!(null)),
        ]});
        assert_eq!(
            serde_json::to_value(excluded_missed(root, &outcomes, LINUX)).expect("json"),
            json!([{"name": name, "cfg": "windows"}])
        );
        assert!(excluded_missed(root, &json!({}), LINUX).is_empty());
        assert!(excluded_missed(root, &json!({"outcomes": 3}), LINUX).is_empty());
    }
}
