# design-system · the rows that bind 2026-11-17-supply-chain-green-and-booted-tests-end — picked 13 of 315 (6 by every draw) · rule any · pin 2304

## brand-identity · ## Brand Identity
contract/cli-verification/unverified-cli · CLI verification state · unverified-cli — An unstamped CLI build runs transport only, shown as `unverified-cli`.

## surface-cli · ## Surface: cli · twin surface
decision/linux-terminal-platform · Linux terminals as the CLI platform — The cli targets Linux terminals: the core on any current distribution, CI on `ubuntu-latest`.
contract/viola-verify-output/stopped-or-refused · viola verify output · stopped or refused run — A run stopped by Ctrl-C or TERM before its rows are checked, or refused by its home tree's strict-modes check before its log, prints no step line and no summary, only its fixed `unable:` / `hint:` pair, and exits 1.
contract/exit-21-cause/socket-dir-refused · exit 21 cause · socket-dir-refused — One of the closed set of causes named in exit 21's `detail`.
contract/viola-verify-output/stamps-refused · viola verify output · stamps.json update refused — A `stamps.json` update refused by its strict-modes check prints the home-state `unable:` / `hint:` pair.

## surface · ## Surface: cli · twin surface-cli
contract/refusal-hints/socket-dir · refusal hints · socket-dir — The socket-directory hint is `run`'s: `viola's socket directory must be a directory you own, readable only by you; viola does not change its permissions`.
contract/viola-run-start-refusals/socket-dir · viola run start refusals · socket directory — Before any snapshot, child or bind: `unable: the socket directory for <name> cannot be trusted` then `hint: viola's socket directory must be a directory you own, readable only by you; viola does not change its permissions`.
contract/viola-verify-refusals/interrupted · viola verify refusals · interrupted run — A run stopped by Ctrl-C or TERM before its rows are checked prints no step line and no summary; stderr carries exactly `unable: verify was stopped before it finished` then `hint: run viola verify again to stamp this CLI`; exit 1.
contract/wrapper-socket/refused-peer · wrapper socket · refused peer — A peer the wrapper refuses is dropped before any frame: no line written, nothing on the terminal the child holds, no wheel moved; the client reads end-of-file.
contract/log-detail-codes/socket-dir-refused · log detail codes · socket-dir-refused — Written to the log by `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` at their exit-21 socket-directory refusal, and at `run`'s and `revive`'s start.
contract/viola-run-start-refusals/already-live · viola run start refusals · already-live — `unable: <name> is already live` then `hint: viola list`; a live holder and a fresh replayed beat log `already-live`.
contract/viola-run-start-refusals/squatted-name · viola run start refusals · squatted-name — `unable: the endpoint for <name> is held by another process` then `hint: another process holds this name's endpoint; stop it or pick another name`.
contract/viola-verify-refusals/stamps-update · viola verify refusals · stamps.json update — A `stamps.json` update refused by its strict-modes check prints the home-state pair and exits 1.

# design-system · its absences, names and overviews, whole

## brand-identity · ## Brand Identity
N/A — viola has no web or graphical frontend, so no frontend expression level.
overview/brand-identity — The direction the CLI carries, and the terminal fleet console inherits, is a control-tower strip bay after dark: buff paper strips in anthracite holder boots on ruled aluminium racks. Every session holds the same slot for its whole life, and the only warm light is the one arrival holder cocked out of line when a session needs the human.

## color-palette · ## Color Palette
Surface Scale (elevation hierarchy) › N/A — viola renders no page, so no surface scale. ¶ Text Hierarchy › N/A — viola renders no page, so no text hierarchy; the cli's ink is the terminal's (Surface: cli). ¶ Semantic Colors › N/A — viola renders no page, so no colour states; the cli's status guards below stand.

## typography · ## Typography
N/A — viola renders no page; the cli's type is the terminal's (below).

## spacing · ## Spacing
N/A — viola renders no page; the cli's spacing is the terminal grid (below).

## depth-strategy · ## Depth Strategy
N/A — viola renders no page, so no depth tokens; the cli draws no depth.

## border-radius · ## Border Radius
N/A — viola renders no page, so no radius tokens.
absence/rendered-page — A rendered page: viola renders no page, so no radius, motion or spacing tokens; the cli's type is the terminal's

## surface-web-spa · ## Surface: web-spa
absence/web-page-surface — A web page or web surface of any kind

## surface-cli · ## Surface: cli · twin surface
absence/cli-tape — A tape in the CLI
name/command/answer — answer
name/command/hook — hook
name/command/last — last
name/command/link — link
name/command/list — list
name/command/mcp — mcp
name/command/pause — pause
name/command/plugin-install — plugin install
name/command/release — release
name/command/revive — revive
name/command/run — run
name/command/send — send
name/command/unlink — unlink
name/command/verify — verify
name/command/wait — wait

## self-validation-protocol · ## Self-Validation Protocol
4. Token Test › N/A — viola renders no page, so no stylesheet tokens to trace. ¶ 4. Token Test › N/A — viola renders no page, so no stylesheet token to trace.
