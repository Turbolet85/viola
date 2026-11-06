@leaf_group group=architecture+security-plan
@leaf_read leaf=.claude/rules/api.md lines=41
last: _This section is owned by `/wrap-session`. setup-project pre
@leaf_edit n=1 leaf=.claude/rules/api.md line=30 class=retired
leaf-text: `canonical_home` then canonicalises a home that exists (on Windows the plain drive form) and keeps
source: fct-k6htnrfj «`canonical_home` canonicalises a home that exists (`std::fs::canonicalize`) and keeps `std::path::absolute`'s spelling for one that does not, creating nothing»
replace-with: `canonical_home` then canonicalises a home that exists (`std::fs::canonicalize`) and keeps
@leaf_edit n=2 leaf=.claude/rules/api.md line=30 class=retired
leaf-text: Prompt text only from stdin or `--file`; warn on a Git Bash rewritten-path argument; `ViolaName` arguments
source: fct-hss5qy6y «No Windows or macOS code, test, fixture or CI job stands in the tree; that code is recoverable at the tag `multi-os-final` (`ecbe7ab3`).»
replace-with: Prompt text only from stdin or `--file`; `ViolaName` arguments
@leaf_edit n=3 leaf=.claude/rules/api.md line=30 class=retired
leaf-text: `check_path` on `instances/`, on every OS, and `check_instance` per instance
source: fct-l7hy7pzk «`viola list`'s `board::scan` then runs `check_read` on the home and `check_path` on `instances/` (each through `symlink_metadata`, a link never followed) and `check_instance` per instance»
replace-with: `check_path` on `instances/` (each through `symlink_metadata`, a link never followed), and `check_instance` per instance
