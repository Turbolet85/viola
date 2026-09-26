//! Which CI mutation legs compile a mutant's line (test-plan §3 `gate`): the `#[cfg]` attributes
//! around the line, read from the checked-out source and judged per leg. A leg is dropped only when a
//! covering cfg is proven false for its target; a name, file, node or predicate this module cannot
//! read keeps every leg, so uncertainty never exempts a mutant from the union.

use std::fs;
use std::path::{Component, Path};

use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::visit::{self, Visit};
use syn::{Attribute, Expr, ExprLit, Lit, Meta, Token};

/// A leg's compilation target, from its runner name.
#[derive(Debug, Clone, Copy)]
struct Target {
    family: &'static str,
    os: &'static str,
}

fn target(leg: &str) -> Option<Target> {
    [
        ("windows-", "windows", "windows"),
        ("ubuntu-", "unix", "linux"),
        ("macos-", "unix", "macos"),
    ]
    .into_iter()
    .find(|(prefix, _, _)| leg.starts_with(prefix))
    .map(|(_, family, os)| Target { family, os })
}

/// The named legs whose target compiles `mutant`'s line, in their given order.
pub fn compiled_legs(root: &Path, mutant: &str, legs: &[String]) -> Vec<String> {
    let cfgs = location(mutant)
        .and_then(|(path, line)| covering_cfgs(&root.join(path), line))
        .unwrap_or_default();
    legs.iter()
        .filter(|leg| {
            target(leg).is_none_or(|t| cfgs.iter().all(|cfg| eval(cfg, t) != Some(false)))
        })
        .cloned()
        .collect()
}

/// `{path}:{line}:{col}: {description}` → the repo-relative path and its 1-based line.
fn location(mutant: &str) -> Option<(&str, usize)> {
    let (at, _) = mutant.split_once(": ")?;
    let mut parts = at.rsplitn(3, ':');
    parts.next()?.parse::<usize>().ok()?;
    let line = parts.next()?.parse().ok()?;
    let path = parts.next()?;
    Path::new(path)
        .components()
        .all(|c| matches!(c, Component::Normal(_)))
        .then_some((path, line))
}

fn covering_cfgs(file: &Path, line: usize) -> Option<Vec<Meta>> {
    let source = fs::read_to_string(file).ok()?;
    let parsed = syn::parse_file(&source).ok()?;
    let mut cover = Cover {
        line,
        cfgs: Vec::new(),
    };
    cover.visit_file(&parsed);
    Some(cover.cfgs)
}

/// Collects the `cfg` predicates of every node whose span covers `line`.
struct Cover {
    line: usize,
    cfgs: Vec<Meta>,
}

impl Cover {
    fn note(&mut self, attrs: &[Attribute], node: &impl Spanned) {
        let span = node.span();
        if (span.start().line..=span.end().line).contains(&self.line) {
            self.cfgs.extend(
                attrs
                    .iter()
                    .filter(|a| a.path().is_ident("cfg"))
                    .filter_map(|a| a.parse_args::<Meta>().ok()),
            );
        }
    }
}

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

    fn visit_impl_item_fn(&mut self, node: &'ast syn::ImplItemFn) {
        self.note(&node.attrs, node);
        visit::visit_impl_item_fn(self, node);
    }

    fn visit_local(&mut self, node: &'ast syn::Local) {
        self.note(&node.attrs, node);
        visit::visit_local(self, node);
    }

    fn visit_expr(&mut self, node: &'ast Expr) {
        self.note(expr_attrs(node), node);
        visit::visit_expr(self, node);
    }
}

/// The expression forms a statement-level `#[cfg]` sits on in this workspace; any other form
/// reads as uncfg'd, which keeps every leg.
fn expr_attrs(expr: &Expr) -> &[Attribute] {
    match expr {
        Expr::Block(e) => &e.attrs,
        Expr::Call(e) => &e.attrs,
        Expr::ForLoop(e) => &e.attrs,
        Expr::If(e) => &e.attrs,
        Expr::MethodCall(e) => &e.attrs,
        Expr::Tuple(e) => &e.attrs,
        Expr::Unsafe(e) => &e.attrs,
        _ => &[],
    }
}

/// One predicate for one target: `Some` when decided, `None` when unknown.
fn eval(cfg: &Meta, t: Target) -> Option<bool> {
    match cfg {
        Meta::Path(p) if p.is_ident("unix") => Some(t.family == "unix"),
        Meta::Path(p) if p.is_ident("windows") => Some(t.family == "windows"),
        Meta::Path(_) => None,
        Meta::NameValue(nv) => {
            let Expr::Lit(ExprLit {
                lit: Lit::Str(value),
                ..
            }) = &nv.value
            else {
                return None;
            };
            if nv.path.is_ident("target_os") {
                Some(t.os == value.value())
            } else if nv.path.is_ident("target_family") {
                Some(t.family == value.value())
            } else {
                None
            }
        }
        Meta::List(list) => {
            let args = list
                .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
                .ok()?;
            let values: Vec<Option<bool>> = args.iter().map(|m| eval(m, t)).collect();
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
    use super::*;

    fn legs(names: &[&str]) -> Vec<String> {
        names.iter().map(|s| (*s).to_owned()).collect()
    }

    const BOTH: [&str; 2] = ["ubuntu-latest", "windows-2025"];

    fn ev(cfg: &str, leg: &str) -> Option<bool> {
        let meta: Meta = syn::parse_str(cfg).expect("cfg");
        eval(&meta, target(leg).expect("target"))
    }

    #[test]
    fn compiled_legs_evaluates_each_predicate_per_target() {
        let cases: [(&str, [Option<bool>; 3]); 9] = [
            ("unix", [Some(true), Some(false), Some(true)]),
            ("windows", [Some(false), Some(true), Some(false)]),
            ("not(unix)", [Some(false), Some(true), Some(false)]),
            ("not(windows)", [Some(true), Some(false), Some(true)]),
            (
                "target_os = \"linux\"",
                [Some(true), Some(false), Some(false)],
            ),
            (
                "target_os = \"macos\"",
                [Some(false), Some(false), Some(true)],
            ),
            (
                "target_family = \"windows\"",
                [Some(false), Some(true), Some(false)],
            ),
            ("test", [None, None, None]),
            ("feature = \"x\"", [None, None, None]),
        ];
        for (cfg, want) in cases {
            let got = ["ubuntu-latest", "windows-2025", "macos-latest"].map(|leg| ev(cfg, leg));
            assert_eq!(got, want, "{cfg}");
        }
        assert_eq!(ev("not(unix, windows)", "ubuntu-latest"), None);
        assert_eq!(ev("not(test)", "ubuntu-latest"), None);
        assert_eq!(ev("target_os = 1", "ubuntu-latest"), None);
        assert_eq!(ev("cfg_x(unix)", "ubuntu-latest"), None);
    }

    #[test]
    fn compiled_legs_all_and_any_are_three_valued() {
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
            assert_eq!(ev(&cfg, "ubuntu-latest"), want, "{cfg}");
        }
    }

    const SOURCE: &str = "#[cfg(windows)]
fn win() {
    let _ = 1;
}
#[cfg(unix)]
impl S {
    fn a() {
        let _ = 2;
    }
}
#[cfg(not(unix))]
mod m {
    fn b() {
        let _ = 3;
    }
}
impl S {
    #[cfg(unix)]
    fn c() {
        let _ = 4;
    }
}
fn d() {
    #[cfg(windows)]
    let _ = 5;
    #[cfg(unix)]
    for _ in 0..1 {}
    #[cfg(windows)]
    unsafe {}
    #[cfg(unix)]
    {}
    #[cfg(windows)]
    if true {}
    #[cfg(unix)]
    f();
    #[cfg(windows)]
    x.m();
    #[cfg(unix)]
    (1, 2);
    let _ = 6;
}
#[cfg(target_os = \"macos\")]
fn mac() {}
#[cfg(unix)]
impl T {
    #[cfg(target_os = \"linux\")]
    fn e() {
        let _ = 7;
    }
}
#[cfg(feature = \"x\")]
fn feat() {
    let _ = 8;
}
";

    fn tree() -> tempfile::TempDir {
        let tmp = tempfile::tempdir().expect("tempdir");
        fs::create_dir_all(tmp.path().join("src")).expect("mkdir");
        fs::write(tmp.path().join("src").join("lib.rs"), SOURCE).expect("source");
        tmp
    }

    fn at(root: &Path, line: usize, legs: &[String]) -> Vec<String> {
        compiled_legs(
            root,
            &format!("src/lib.rs:{line}:5: replace x with y"),
            legs,
        )
    }

    #[test]
    fn compiled_legs_follow_item_and_statement_cfgs() {
        let t = tree();
        let both = legs(&BOTH);
        let (unix, win) = (legs(&["ubuntu-latest"]), legs(&["windows-2025"]));
        for (line, want) in [
            (3, &win),
            (8, &unix),
            (14, &win),
            (20, &unix),
            (25, &win),
            (27, &unix),
            (29, &win),
            (31, &unix),
            (33, &win),
            (35, &unix),
            (37, &win),
            (39, &unix),
            (40, &both),
            (48, &unix),
        ] {
            assert_eq!(&at(t.path(), line, &both), want, "line {line}");
        }
        assert!(at(t.path(), 43, &both).is_empty(), "a macOS-only body");
        let three = legs(&["ubuntu-latest", "windows-2025", "macos-latest"]);
        assert_eq!(
            at(t.path(), 48, &three),
            unix,
            "every covering cfg must hold"
        );
        assert_eq!(at(t.path(), 43, &three), legs(&["macos-latest"]));
    }

    #[test]
    fn compiled_legs_keep_every_leg_when_unsure() {
        let t = tree();
        let both = legs(&BOTH);
        assert_eq!(at(t.path(), 52, &both), both, "an unknown predicate");
        let odd = legs(&["freebsd-14", "windows-2025", "ubuntu-latest"]);
        assert_eq!(
            at(t.path(), 3, &odd),
            legs(&["freebsd-14", "windows-2025"]),
            "an unknown leg family"
        );
        for name in [
            "not a mutant name",
            "src/lib.rs:x:5: replace x with y",
            "src/lib.rs:3:y: replace x with y",
            "src/lib.rs: replace x with y",
            "src/missing.rs:3:5: replace x with y",
            "../src/lib.rs:3:5: replace x with y",
        ] {
            assert_eq!(compiled_legs(t.path(), name, &both), both, "{name}");
        }
        fs::write(t.path().join("src").join("bad.rs"), "#[cfg(unix)] fn (").expect("bad");
        assert_eq!(
            compiled_legs(t.path(), "src/bad.rs:1:5: replace x with y", &both),
            both
        );
    }

    #[test]
    fn compiled_legs_location_reads_path_and_line() {
        assert_eq!(
            location(
                "crates/viola-pty/src/lib.rs:395:9: replace HostTerminal::enter -> Option<Self> with None"
            ),
            Some(("crates/viola-pty/src/lib.rs", 395))
        );
        assert_eq!(location("/abs/lib.rs:1:1: d"), None);
        assert_eq!(location("lib.rs:1:1:d"), None);
    }
}
