# a11y-plan · the rows 2026-11-23-seam-requests-for-viola-open touches — picked 1 of 238 · rule any · pin 4310

## 11-a11y-anti-patterns-never-do-these · ## 11. A11y Anti-Patterns (NEVER do these)
decision/a11y-fail-fast-gate · A11y is a fail-fast gate — A11y is never skipped on CI failure: it is a FAIL-fast gate, not a warning. · r1
decision/codes-beside-plain-word · Kebab-case codes beside a plain word — A kebab-case reason or detail code (`not-delivered`, `human-typing`, `unverified-cli`) on a refusal line never appears without the plain word `unable`; the board's cells show status codes such as `unverified-cli` without it. · why: no jargon without a plain alternative; SC 3.1.5 AAA is not claimed · r1
decision/compiled-shapes-over-signatures · Compiled shapes over the screen signatures — The compiled shapes the rendered `claude` TUI screen is parsed for over its compiled signatures are a modal label row's option shape and the box's rows below it; no row text is kept. · r1
decision/list-captions-fixed · Board captions fixed by their literal — The caption set and order of `viola list` (NAME · LIVE · STATUS · WHEEL · DIALOG · CLI) never drift from its `CAPTION` literal (SC 3.2.4). · r1
decision/machine-output-a11y-review · No human-only a11y review — No a11y review is human-only without machine output. · why: the canonical agent-runnable principle · r1
decision/manual-checks-supplemental-only · Manual checks are supplemental only — Manual screen reader, keyboard or contrast checks are never the ONLY method; as a supplement to automated tools they are OK. · r1
decision/no-colour-on-failure-words · Failure words are never coloured — The CLI words `unable` and `fail` are never coloured. · r1
decision/no-information-by-colour-alone · No information by colour alone — Colour is only a second cue: every state indicator has a text or icon beside it (SC 1.4.1). · r1
decision/no-screen-content-parsing · No screen-content parsing beyond signatures — The rendered `claude` TUI screen is parsed only for compiled signatures (footer literals, the input box's rule and prompt rows, modal literals, the compiled shapes over them), the prompt row only for its opening char, never the text after it; screenshots are never taken as a11y evidence (R7). · why: the rendered TUI is vendor content · except: the test-only rig reads closed words, never text: trust selector's place, input box empty, holds-text or absent, modals, box-row cell attributes before any send or after a turn end, send or KR resize · r8
decision/no-soft-a11y-slo · No soft a11y SLO — An a11y SLO is never soft: it triggers a build or deploy failure on its threshold. · r1
decision/no-spinners · No spinners — CLI output is static and linear, with no spinner or redraw. · why: design ban · r1
decision/no-stack-trace-only-error · No stack trace as the only error text — The CLI never renders a stack trace as the only error text. · r1
decision/no-unwarned-timeouts · No harsh timeouts without warning — No harsh timeout is set without a warn-extend pattern (SC 2.2.1). · r1 · stale
decision/no-vendor-locked-a11y-tool · No vendor-locked a11y tool as only method — A vendor-locked a11y tool without machine output or API (Stark Visual Studio plugin, Tenable.io UI-only) is never the ONLY method. · r1
decision/non-editing-sequences · Terminal sequences are not human editing — Focus, mouse and resize sequences and terminal replies (the F-W2 closed list) pass through and never count as human editing; every other byte does (P4). · r1
decision/refusal-ends-with-hint · Every refusal ends with a hint — Every human-mode CLI refusal ends with a `hint:` recovery line (SC 3.3.3). · except: `unknown` (exit 14), whose detail is a code word or nothing · r1
