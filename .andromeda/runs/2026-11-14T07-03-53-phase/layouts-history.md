@history_item n=1 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — turned the layout master into rows (226 rows: decision 51, contract 173, absence 2), each one checked against the code. Every picked row here comes from this migration, so the rows are current truth and the code holds the rest. When the seam work runs into a row, read the code first, not older prose.

@history_item n=2 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the socket-directory refusal of the `run` and `revive` starts) — added the exit-1 pair `unable: the socket directory for <name> cannot be trusted` / `hint: viola's socket directory must be a directory you own, readable only by you; viola does not change its permissions`. It fires before any snapshot, child or bind, and the directory is left as found. `refuse_socket_dir` never changes a mode, and it added the `socket-dir-refused` code to revive's `--json`. This is the source of `contract/run-revive-start/socket-dir-refused`. It is also the precedent the scope names: a Hyprland request socket under `$XDG_RUNTIME_DIR/hypr/<instance>/` would need the same 0700/owner check, never a mode change, or a recorded reason it doesn't.

@history_item n=3 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (exit 21's sixth cause, `socket-dir-refused`, at the frame verbs) — `check_endpoint_dir` in the CLI's `connect` refuses with no frame written, exit 21, the socket-dir hint, and `detail":"socket-dir-refused"` under `--json`. This is the source of `contract/frame-verb-hints/socket-dir-refused`. Its Kept says a refused socket directory is not the strict-modes refusal. The seam's socket check, if it shares this checker, must not change these refusals or how they are told apart.

@history_item n=4 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the six frame verbs' exit-21 instance pair) — `client::refuse_untrusted` runs the strict-modes check first in every frame verb, before any role file or log opens. A refusal writes no log line and leaves the mode as found. This is the origin of `contract/frame-verbs/strict-modes-refused`. Its rule that a refused check writes no line (the founder's answer to Q2) carries over to the seam's codes-only logging. Nothing the seam adds may open a log before this check.

@history_item n=5 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the home-tree refusals of verify and the run and revive starts) — `check_home_tree`/`TreeRefused` refuses a widened home tree before the wrapper log opens, exit 1, no line, no child. This is the order the `run`/`revive` start refusals rest on. Any new seam read wired into a start must sit after these checks and must not reorder them.

@history_item n=6 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (the strict-modes refusal of the driven instance) — first form of the strict-modes refusal: exit 21 before any log, mode left as found. The trap it records: opening a role file narrows an existing home to 0700, so a check placed after `client::start` passes a 0770 home. `link_verbs_refuse_a_widened_instance_before_any_log` guards the order by reading the mode after the refusal. The same ordering trap applies to any new owner/mode check the seam's socket needs.

@history_item n=7 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (exit 21's document carries its cause in `detail`) — gave every exit-21 cause a closed `detail` code word (`strict-modes-failed`, …, later `socket-dir-refused`) in place of null, with `error`/`v` fixed. This is the closed-code pattern the seam's outcome codes (ok, unsupported, unreachable, malformed) should follow. It is also the `detail` form both picked frame-verb rows quote.

@history_item n=8 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — `reply_of`/`of_reply` screen a peer's reply: a refusal that is no code word reads `unknown`, and a string detail that is no code word reads as none, so no opaque upstream text reaches a refusal. This is the precedent for the seam treating Hyprland reply text (titles, classes) as untrusted content that never reaches a log, error or refusal.

@history_item n=9 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — turned the dirty-recording refusal into `unable: a recorded fixture is not clean: <file> <code>` (screen form `<file> row <n>[ seam] <code>`; codes `home-path`, `absolute-path`, `username`, `email`; never the content). This is the source of `contract/viola-verify-refusals/fixture-not-clean`. The Hyprland replies recorded live at L1/L2 for the fake compositor must be scrubbed to this same standard. Whether the clean-fixture scan covers a new Hyprland fixture directory touches this refusal's text and codes.

@history_item n=10 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — first added `viola verify`'s refusal pairs, including the dirty-recording pair that later became `fixture-not-clean`. It also set the shape of the Claude capability ledger rows. The scope keeps that ledger Claude-only and puts a parallel Hyprland measured-live record beside it, so the verify wireframe and its relied count must not grow from this chunk.

@history_item n=11 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the .cmd/.bat refusal leaves revive and verify; the server-check hint without its OS scope) — from the founder's Linux-only rulings, removed per-OS arms and closed words no build can emit. Kept: the socket-directory refusals' «on Unix» qualifier. This is the Linux-only boundary the seam inherits (`decision/linux-only`): it adds no per-OS arm, and closed words with no emitter leave.

@history_item n=12 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** (version census 0.2.0) — replaced the web stub with no web surface (`absence/web-surface`). It moved the keyboard-driven terminal fleet console tiled on Hyprland (matrix#v2-14) to its own surface at «Console scaffold», and kept the CLI ban on interactive verbs, scoped to verbs. It dropped the «on Unix» qualifiers on both socket-directory refusals (rows 1 and 2 picked here). The seam serves the console and window verbs and adds no surface. The console's surface belongs to «Console scaffold», not to this chunk.

@history_item n=13 marker=2026-10-16T14-58-10-wrap
**2026-10-16T14-58-10-wrap** — cut Surface: web-spa to a stub, leaving the cli surface to stand alone. This is the history behind `absence/web-surface` and `decision/cli-holds-every-action`: every action lives in the CLI, so window features reach the user only through verbs the later entries build on this seam.
