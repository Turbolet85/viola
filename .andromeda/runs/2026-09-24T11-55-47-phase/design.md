# design extract

## Relevance
Partial. The chunk renders nothing and adds no tokens, colour, type or motion. Design applies only through the CLI stream and print contract that the new print bans and `#[allow]` sites must respect, and through the panic path (no stack traces reach the terminal).

## Constraints
- design-system §Surface: cli → Streams sets the stream split. stdout carries results and data. stderr carries `waiting:` context lines, refusals with their `hint:` lines, `error: …` lines and the `viola ui` launch line. Under `--json` the whole result is one JSON document on stdout. The output modules that get a local `#[allow(clippy::print_stdout/print_stderr)]` are the owners of exactly these streams. Research has to answer which of them exist as separate modules at HEAD.
- design-system §Surface: cli → Colour decision order (items 2–3) and §Platform-Specific Notes:
  - `viola run` prints nothing once the child starts.
  - `viola hook` writes nothing to stderr and always exits 0.
  - `viola mcp` writes only MCP frames on stdout.
  - So hook, mcp and post-spawn run paths get no print `#[allow]`. This matches the scope's "println! in a hook/run path fails" proof.
- design-system §Surface: cli → 5 (`run`) and the Exit-code phraseology row for exit 1: `viola run` does print its exit-1 start-refusal line and `hint:` before the child starts. Those lines must go through an allowed output sink, not an inline print in `run`. Research must check whether HEAD prints them inline in `run`, because that would trip the new ban.
- design-system §Surface: cli → 5 (`ui`) requires the launch line to go to stderr exactly once, unstyled. It is the only place the token or URL is ever printed. The `ui` launch-line `#[allow]` site must stay a single site.
- design-system §Surface: cli → Platform-Specific Notes ("Stack traces never print") requires errors as fixed messages, with full detail only in `instances/<name>/diagnostics/`. The panic hook's terminal output, if any, must not be a stack trace. On the hook path it must not be stderr at all.
- design-system §Brand Identity (expression table, 0.0 row) requires no colour, no glyphs, no cursor control and no spinner under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb` and `viola run` passthrough. Lint-driven refactors of the output modules must keep this gating.

## Patterns to follow
- design-system §Surface: cli → Toolkit: styling is "a small hand-written SGR module in the `viola` bin" with no dependency. Output is already expected to go through a small set of named output modules, which suits narrow `#[allow]` placement.
- design-system §Surface: cli → Exit-code phraseology fixes the first word on human stderr for each typed exit (`error: internal error`, `unable  <reason>  <detail>`, `error: wrapper fault  <code>`). Code that routes prints into an allowed sink should keep these exact strings.
- design-system §Surface: cli → Streams (`--json` bullet): under `--json` no `hint:` line is printed and the typed fields are the whole answer. The `--json` sink and the human-stderr sink stay separate.

## Anti-patterns to avoid
- design-system §Anti-Patterns → Per-Surface Bans (cli), "NEVER mix data and messages": moving prints into allowed modules must not swap stdout and stderr roles.
- design-system §Anti-Patterns → Per-Surface Bans (cli), "NEVER print stack traces": no panic output or `dbg!`-style diagnostics on the terminal. Detail goes to `diagnostics/` only.
- design-system §Anti-Patterns → Per-Surface Bans (cli):
  - "NEVER print the token or launch URL anywhere but the single `viola ui` launch line".
  - "NEVER print upstream text, paths, pids or anyhow chains with serde sources in errors or hints".
  - So a blanket crate-level print allow on a product module, which would hide new print sites, is out. This does not apply to the fake-agent `[[bin]]`, which is not a product surface.

## Contract bindings
- design ↔ obs §11 Logs / §3 `obs-ci-gate-wire`: the design cli Streams list decides which output modules may carry the local print `#[allow]` (`--json`, human CLI stderr, hook decision body, `ui` launch line). The design names no hook stdout body. The hook decision body's stdout sink comes from obs and arch, and design only requires that hook never writes stderr.
- design ↔ security NEVER-log floor ↔ obs §9 secret scan: the ban on printing the token or URL outside the launch line, and on paths, pids and upstream text in errors and hints, is the terminal side of the floor the secret scan enforces over `target/agent-run/*` and diagnostics files. Captured terminal output in `target/agent-run/*` falls under both.
- design ↔ obs panic hook / G2 (§10 panic SLO): design needs a panic to show no stack trace and to write nothing to stderr on `viola hook`. obs needs the hook first in `main` and zero `event:"panic"` lines. The gate witnesses order, and design constrains what the hook may print.

## Acceptance criteria contributions
- Every product-code `#[allow(clippy::print_stdout)]` / `#[allow(clippy::print_stderr)]` site owns a stream named in the cli Streams list (`--json` document, human stdout result, stderr context/refusal/hint/error line, `ui` launch line). No such allow covers a `viola hook` stderr path, `viola mcp` non-frame output, or `viola run` after the child starts (per design-system §Surface: cli → Streams; §Platform-Specific Notes).
- With the print bans in place, `viola run`'s exit-1 start refusal still prints its fixed-message line plus `hint:` on stderr, and nothing prints once the child runs (per design-system §Surface: cli → 5 `run`; Exit-code phraseology exit 1).
- The launch URL/token string appears in exactly one print site, the `viola ui` launch line, which is unstyled and on stderr (per design-system §Surface: cli → 5 `ui`; Per-Surface Bans cli).
- A forced panic prints no stack trace to the terminal, and under `viola hook` writes nothing to stderr (per design-system §Surface: cli → Platform-Specific Notes; Per-Surface Bans cli "NEVER print stack traces").

## Relevant amendment history
(none). The sidecar `D:/dev/projects/viola/.andromeda/design-system-amendments.md` does not exist, so the history is empty.
