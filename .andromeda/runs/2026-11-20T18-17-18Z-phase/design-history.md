@history_item n=1 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — founded `viola verify`'s step counter (one static stdout line per ledger row, `[NN/MM] <id> <words>  pass`, then `stamped <ver>  <n> pass  <m> fail`) and its exit-1 refusals; a failing row prints no stderr word. The two new rows land as two more step lines in this exact pattern.

@history_item n=2 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — counter to ten rows; replaced the fixed payload refusal with the named `unable: a recorded fixture is not clean: <file> <code>` (closed codes `home-path` · `absolute-path` · `username` · `email`, never the content). The identity-floor probe's captured env names pass through this `--record` scrub; a name carrying private text must meet this closed-code refusal, never print its content.

@history_item n=3 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** — counter to fourteen rows, each new row named with its words; precedent that each landed ledger row amends the counter/summary example and the pattern itself stays unchanged.

@history_item n=4 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — counter to seventeen, three rows named with their words; the summary example follows the one stamped fixture set's version. Precedent for naming the harness-prefix and identity-floor rows' words and for which version the example names.

@history_item n=5 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** — counter to eighteen; the count derives from `LedgerRow::ALL.len()`, the count carrying its rule. The new rows grow `MM` through the same derivation, not a literal.

@history_item n=6 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — one step line per RELIED ledger row, `MM` = `LedgerRow::relied()`'s count (the basis of `contract/viola-verify-output/step-line` and `summary`); the chunk must decide the two rows are relied so they count in `MM` and the stamp's relied-id set.

@history_item n=7 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — the 22nd row appended at the end of the ledger order, relied on every OS, counter/summary updated; precedent for appending new rows last.

@history_item n=8 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — (the `verify` bullet) counter to 23 with `agents-join` last in the member list; same ledger-growth precedent.

@history_item n=9 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — the 24th row and `[NN/24]`; on an unverified CLI the readback stays `unconfirmable`. Shows a relied row gating behaviour by the verified state, as the harness-prefix and identity-floor rows would.

@history_item n=10 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (verify's relied count on every build) `relied()` reads the same on every build, no OS qualifier; the `shim-resolution` row passes when `claude --version` answers; also retired verify's `.cmd/.bat` refusal. The new rows carry no OS cut.

@history_item n=11 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — a verify stopped by Ctrl-C or TERM before its rows are checked prints no step line or summary, only the fixed `unable:` / `hint:` pair, exit 1 (row `contract/viola-verify-refusals/interrupted`); a longer probe set (second session, env capture) must keep this.

@history_item n=12 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — verify's interrupted pair wording ratified as the founder's; it stands as worded and is not to be reworded by new probes.

@history_item n=13 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (verify's home-tree and stamps refusals; a home under a regular file) verify checks the home, `ledger/`, `bin/`, `plugin/` before its log and prints only the home-state pair on refusal; a refused `stamps.json` update prints the same pair. Rows `contract/viola-verify-refusals/home-tree-check`, `stamps-update`, `contract/viola-verify-output/stamps-refused` stand over any new child or capture file written in the rig.

@history_item n=14 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — the stderr rule: the fixed `unable: <text>` form covers the exit-1 refusals of `run`, `revive` and `verify` (origin of `contract/cli-stderr/fixed-message-refusal`, `contract/cli-refusal-line/fixed-unable-form`); any new verify refusal (e.g. a captured value) takes this form with its hint.

@history_item n=15 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — `mcp` made a hidden verb, run by the plugin, in no `--help` row (`decision/mcp-hidden-verb`, which also gives `hook` no human surface); the hidden `hook --capture` arm and any new capture id stay off every human surface.

@history_item n=16 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (the catch site's `internal-error` document) `run`, `revive`'s start, `mcp` and `hook` stay silent at the catch site, under `--json` too (`contract/cli-catch-site/silent-verbs`, `decision/silent-catch-sites`); a capture arm inside `hook` writing env names must keep `hook` silent on stderr and exit 0.

@history_item n=17 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — (brand, surface and bans scoped to the CLI) the spinner / progress-bar / live-redraw ban now reads «in a verb's output» (`decision/no-spinners-in-verb-output`); a second-session probe that adds time must still show progress only as verify's static step lines.

@history_item n=18 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — (Brand Identity) `~ decision/fixed-phraseology-vocabulary`: `budget-paused` dropped from the fixed words; the vocabulary any new step-line words or refusal must draw from, with no filler.

@history_item n=19 marker=2026-11-16-send-confirmation
**2026-11-16-send-confirmation** — `~ contract/send-readback/prompt`: the matching `prompt-submitted` may equal the sent text with its trailing ASCII whitespace run removed; a prompt trimmed at the start does not match. The harness-prefix probe reads a prompt's raw start, the same raw-start reading this row rests on.

@history_item n=20 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** — `~ contract/send-readback/prompt` provisional mark removed: the trailing-only tolerance ratified by the founder; a leading trim waits for a live reading. The harness-prefix row must not loosen the prompt's start match.

@history_item n=21 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** — `+ contract/viola-verify-output/trust-record-names-nothing`: a verify row touching private state prints its step line and no output names the config file, key or value. The direct precedent for the identity-floor row's «env NAMES, never values» and for the harness probe's footprint in the user's Claude config.

@history_item n=22 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — `~ contract/viola-verify-output/step-line`, `~ summary`, `~ trust-record-names-nothing`, `~ summary-line`: the verdict word `absent` on the `browser-tools-prompt` row only, counting as a pass; `n` counts `pass` plus that `absent`. Any new verdict word for the new rows would be a further amendment; the closed set is `pass`, `absent`, `fail`.

@history_item n=23 marker=2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin
**2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin** — `+ contract/viola-verify-output/browser-tools-line`: a fixed stdout line before the summary when a run met the browser-tools prompt; a second-session or child-env probe that meets that prompt prints it, and the live home's re-stamp stays the re-pin step's.

@history_item n=24 marker=2026-11-19-context-as-a-local-command
**2026-11-19-context-as-a-local-command** — `~ contract/cli-verification/verified`, `~ contract/cli-verification/unverified-cli`: a stamp lacking a relied row reads unverified until re-stamped. Adding two relied rows makes every standing 27-row stamp (the rig's 2.1.328, the live home's 24-row) unverified — the reason «stamped live» and the re-pin step are needed.

@history_item n=25 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** — `+ contract/auto-verify-line/verifying`, `did-not-verify`, `earlier-red`, `verifying-elsewhere`, `no-stamp`, `line-form` (PROVISIONAL, delegate's confirm): a start's automatic verify runs the relied rows, so the new probes' cost and a stamp lacking them (`earlier-red` reading «holding every relied row») ride into `run`/`revive` start lines.
