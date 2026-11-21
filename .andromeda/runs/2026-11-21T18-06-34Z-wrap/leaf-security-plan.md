@leaf_group group=security-plan
@leaf_read leaf=.claude/docs/security-summary.md lines=77
last: **Full plan:** `.andromeda/security-plan.md`.
@leaf_read leaf=.claude/rules/security.md lines=45
last: - 2026-11-16: Write a bounded read as `take(reader, MAX_FRAM
@leaf_edit n=1 leaf=.claude/docs/security-summary.md line=38 class=stale
leaf-text: `XDG_RUNTIME_DIR` is read by the channel endpoint only to place the socket directory;
source: dec-r5tqqhgm «viola verify reads XDG_RUNTIME_DIR once, holds it in memory and uses it only to compose the claude CLI's inbox socket address for the sender's prompt; it enters no log line, stamp, error or step line»
replace-with: `XDG_RUNTIME_DIR` is read by the channel endpoint only to place the socket directory, and by `viola verify` once, held in memory and used only to compose the claude CLI's inbox socket address for the sender's prompt (it enters no log line, stamp, error or step line, nor any fixture file verify writes);
@leaf_edit n=2 leaf=.claude/docs/security-summary.md line=72 class=omission
leaf-text: Run C (`-dialogs/`
source: dec-nwuzavzk «viola verify's interactive claude children are each a direct spawn, never through a shell, under the R8 strip at a fixed 80×24, and paste only compiled literals; Run B's settings take a compiled accept literal»
replace-with: Run B's settings take a compiled accept literal, and the cross-session sender's prompt holds compiled literals and the composed uds address alone; Run C (`-dialogs/`
@leaf_edit n=3 leaf=.claude/docs/security-summary.md line=72 class=omission
leaf-text: Run E last, trusted in
source: dec-7cnv2zy2 «The uds address verify composes is built only from an absolute runtime dir of bytes A-Z a-z 0-9 / . _ - with no . or .. component, its path at most 103 bytes, else no sender is spawned and the row reads fail»
replace-with: the uds address verify composes is built only from an absolute runtime dir of bytes A-Z a-z 0-9 / . _ - with no . or .. component, its path at most 103 bytes, else no sender is spawned and the row reads fail, the sender's prompt re-checking the shape and no directory read to compose it; Run E last, trusted in
@leaf_edit n=4 leaf=.claude/docs/security-summary.md line=72 class=omission
leaf-text: Dev-host prerequisite:
source: dec-yzzrfohc «The cross-session probe leaves: misaddressing closed by address save the pid-reuse window; settings the accept literal cannot override, each a red; the sender's unmeasured leavings and the closed-word files»
replace-with: Accepted residuals of the cross-session probe: misaddressing closed by address save the pid-reuse window; settings the accept literal cannot override, each a red; the sender's unmeasured leavings and the closed-word files a SIGKILL leaves; the address sent to the model provider; an unvouched peer frame unmeasured. Dev-host prerequisite:
@leaf_edit n=5 leaf=.claude/rules/security.md line=13 class=omission
leaf-text: No other caller may register it.
source: dec-pbo7nma3 «Under --names-only the arm files only closed words, a tool class and a send result read from the success boolean alone, never a payload byte; an unknown shape reads unread.»
replace-with: Under `--names-only` the arm files only closed words, a tool class and a send result read from the success boolean alone, never a payload byte; an unknown shape reads unread. No other caller may register it.
@leaf_edit n=6 leaf=.claude/rules/security.md line=17 class=omission
leaf-text: Run B's pastes are compiled literals (the probe prompt
source: dec-nwuzavzk «viola verify's interactive claude children are each a direct spawn, never through a shell, under the R8 strip at a fixed 80×24, and paste only compiled literals; Run B's settings take a compiled accept literal»
replace-with: Run B's settings take a compiled accept literal, and the cross-session sender's prompt holds compiled literals and the composed uds address alone; Run B's pastes are compiled literals (the probe prompt
@leaf_edit n=7 leaf=.claude/rules/security.md line=17 class=omission
leaf-text: Run C's one `allow` runs `touch viola-probe-permission` inside its own dir;
source: dec-7cnv2zy2 «The uds address verify composes is built only from an absolute runtime dir of bytes A-Z a-z 0-9 / . _ - with no . or .. component, its path at most 103 bytes, else no sender is spawned and the row reads fail»
replace-with: the uds address verify composes is built only from an absolute runtime dir of bytes A-Z a-z 0-9 / . _ - with no . or .. component, its path at most 103 bytes, else no sender is spawned and the row reads fail; the sender's prompt re-checks the shape, and no directory is read to compose it; Run C's one `allow` runs `touch viola-probe-permission` inside its own dir;
@leaf_edit n=8 leaf=.claude/rules/security.md line=17 class=omission
leaf-text: Run E adds no transcript file).
source: dec-yzzrfohc «The cross-session probe leaves: misaddressing closed by address save the pid-reuse window; settings the accept literal cannot override, each a red; the sender's unmeasured leavings and the closed-word files»
replace-with: Run E adds no transcript file; the cross-session probe's residuals are accepted too: misaddressing closed by address save the pid-reuse window, settings the accept literal cannot override (each a red), the sender's unmeasured leavings and the closed-word files a SIGKILL leaves, the address sent to the model provider, and an unvouched peer frame unmeasured).
@leaf_edit n=9 leaf=.claude/rules/security.md line=37 class=stale
leaf-text: `XDG_RUNTIME_DIR` (read by the channel endpoint only to place the socket directory)
source: dec-r5tqqhgm «viola verify reads XDG_RUNTIME_DIR once, holds it in memory and uses it only to compose the claude CLI's inbox socket address for the sender's prompt; it enters no log line, stamp, error or step line»
replace-with: `XDG_RUNTIME_DIR` (read by the channel endpoint only to place the socket directory, and by `viola verify` once, held in memory and used only to compose the claude CLI's inbox socket address for the sender's prompt; it enters no log line, stamp, error or step line, nor any fixture file verify writes)
