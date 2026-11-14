@leaf_group group=architecture+security-plan+test-plan+obs-plan
@leaf_read leaf=CLAUDE.md lines=133
last: <!-- USER:session-learnings end -->
@leaf_edit n=1 leaf=CLAUDE.md line=13 class=omission
leaf-text: the workspace library crates `viola-core`, `-pty`, `-channel`, `-state`, `-agent-claude`, `-mcp` and the test-only `viola-e2e`
source: nam-kmycjonm «name/crate/viola-hyprland — viola-hyprland»
replace-with: the workspace library crates `viola-core`, `-pty`, `-channel`, `-state`, `-agent-claude`, `-mcp`, `-hyprland` and the test-only `viola-e2e`
@leaf_edit n=2 leaf=CLAUDE.md line=15 class=omission
leaf-text: replayed by the fake agent in CI; `fake-scripts/` the committed fake-agent turn scripts
source: dec-d5bfbqzd «decision/fixtures-recorded-by-live-verify · Fixtures refreshed by live verify — The recorded agent hook, dialog, framing, status line, resume and screen fixtures are refreshed only by a live viola verify run»
replace-with: replayed by the fake agent in CI; `hyprland/<version>/` the Hyprland replies recorded by the test-only `viola-hypr-tap`; `fake-scripts/` the committed fake-agent turn scripts
@leaf_edit n=3 leaf=CLAUDE.md line=22 class=omission
leaf-text: depends on no viola crate, knows no agent, takes no thiserror
source: dec-vepzu6fa «decision/hyprland-seam-crate · viola-hyprland is the one compositor seam — Compositor queries and dispatches go only through viola-hyprland's closed request set; the crate is sync and tokio-free and depends on vi»
replace-with: depends on no viola crate, knows no agent, takes no thiserror; **`viola-hyprland`** — the one compositor seam: compositor queries and dispatches go only through its closed request set; sync and tokio-free, it depends on `viola-core` alone among viola crates
@leaf_edit n=4 leaf=CLAUDE.md line=27 class=omission
leaf-text: `viola-harness` behind `scripts/agent-run.sh`, `viola-mcp-tap` and the Tokio-based E2E clients
source: nam-iuka5gbj «name/binary/viola-hypr-tap — viola-hypr-tap»
replace-with: `viola-harness` behind `scripts/agent-run.sh`, `viola-mcp-tap`, `viola-hypr-tap` and the Tokio-based E2E clients
@leaf_edit n=5 leaf=CLAUDE.md line=28 class=retired
leaf-text: depends on every member: clap subcommand dispatch
source: dec-utofgtfh «decision/root-bin-consumes-normalised-events · Root bin consumes only normalised events — The root bin holds the subcommand dispatch, the run pump, the wheel and the budget governor, all of which consume only normalised events.»
replace-with: holds the clap subcommand dispatch
@leaf_edit n=6 leaf=CLAUDE.md line=36 class=omission
leaf-text: content-bearing detail goes only to the instance's `detail-*.ndjson`.
source: dec-fv2lrqu3 «decision/hyprland-reply-text-dropped · Hyprland reply text dropped at the parse — Hyprland reply types hold no text field: window titles, classes and every other reply string are dropped at the parse and reach no log line and no error.»
replace-with: content-bearing detail goes only to the instance's `detail-*.ndjson`; Hyprland reply strings (window titles, classes, every other) are dropped at the parse and reach no log line and no error.
@leaf_edit n=7 leaf=CLAUDE.md line=87 class=omission
leaf-text: (`pty · channel · state · agent-claude · mcp` around `core`)
source: dec-vepzu6fa «decision/hyprland-seam-crate · viola-hyprland is the one compositor seam — Compositor queries and dispatches go only through viola-hyprland's closed request set; the crate is sync and tokio-free and depends on vi»
replace-with: (`pty · channel · state · agent-claude · mcp · hyprland` around `core`)
