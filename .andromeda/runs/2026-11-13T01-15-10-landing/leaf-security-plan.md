@leaf_group group=security-plan
@leaf_read leaf=.claude/docs/security-summary.md lines=77
last: **Full plan:** `.andromeda/security-plan.md`.
@leaf_read leaf=.claude/rules/security.md lines=44
last: _This section is owned by `/wrap-session`. setup-project pres
@leaf_edit n=1 leaf=.claude/rules/security.md line=37 class=stale
leaf-text: The only env vars outside `VIOLA_*` any `viola` build reads are the terminal colour facts
source: dec-nussn6nn «except: the test-only seams compiled out of release builds, the terminal colour facts read by viola list's human mode, and XDG_RUNTIME_DIR read by the channel endpoint to place the socket directory»
replace-with: The only env vars outside `VIOLA_*` any `viola` build reads are `XDG_RUNTIME_DIR` (read by the channel endpoint only to place the socket directory), the terminal colour facts
@leaf_edit n=2 leaf=.claude/docs/security-summary.md line=38 class=omission
leaf-text: the terminal colour facts `NO_COLOR`, `TERM` and `COLORTERM` are read in every build
source: dec-nussn6nn «except: the test-only seams compiled out of release builds, the terminal colour facts read by viola list's human mode, and XDG_RUNTIME_DIR read by the channel endpoint to place the socket directory»
replace-with: `XDG_RUNTIME_DIR` is read by the channel endpoint only to place the socket directory; the terminal colour facts `NO_COLOR`, `TERM` and `COLORTERM` are read in every build
@leaf_edit n=3 leaf=.claude/rules/security.md line=31 class=omission
leaf-text: Never widen an ignore, add a `skip`/`allow` or silence zizmor to reach green.
source: dec-runl2edi «Within 7 days of an advisory confined to a disabled feature, a cargo-deny ignore entry is added with a reason naming the disabled feature.»
replace-with: Never widen an ignore, add a `skip`/`allow` or silence zizmor to reach green; the one admitted ignore is the Critical CVE response: within 7 days of an advisory confined to a disabled feature, an ignore entry whose reason names that feature (an advisory against an enabled feature: bump the crate within 7 days).
@leaf_edit n=4 leaf=.claude/rules/security.md line=35 class=omission
leaf-text: Actions pinned by full commit SHA; workflow `permissions: {}`
source: dec-nzthkcjz «Every GitHub Action is referenced by a full commit SHA with a version comment, never by @stable, @v2 or any mutable ref, and actions/checkout runs with persist-credentials: false.»
replace-with: Actions pinned by full commit SHA with a version comment, never a mutable ref, `actions/checkout` with `persist-credentials: false`; workflow `permissions: {}`
