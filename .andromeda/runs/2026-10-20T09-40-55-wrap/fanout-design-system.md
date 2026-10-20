proposals:
  - detector: D-retired
    severity: warning
    section: Brand Identity
    change: "The ATIS anchor drops «This is the `<viola-atis>` header.»; it reads: every budget reading carries its age (`read 4m ago`), on the cli the BAY line's age cell; old information is shown as old (`expired`), never as current."
    sidecar: "§Brand Identity ATIS anchor: the cut page component `<viola-atis>` removed (chunk 2026-10-20-epoch-5-cleanup, F12); the superseded sentence kept here."
    rationale: "Report Retired: design-system §Brand Identity cites cut page components, F12 row `:16` (`<viola-atis>`); Expected amendments: the web-surface references at lines 16, 21, 41, 45 cut, the cli anchors kept."
    basis: ".andromeda/design-system.md:16"
  - detector: D-retired
    severity: warning
    section: Brand Identity
    change: "The Signature element names the readback box by its cli mirror and its drawn states, without «every send line in `<viola-event-feed>` and … every outbound transfer marker»: the box is the `[RB]` / `[  ]` / `[/ ]` column of every `viola send` line; a 0.2.0 frontend's rendering of it is that version's to design (its technology not chosen)."
    sidecar: "§Brand Identity Signature element: the cut page component `<viola-event-feed>` and the web transfer-marker placement removed (chunk 2026-10-20-epoch-5-cleanup, F12); the superseded paragraph kept here."
    rationale: "Report Retired: F12 row `:21` (`<viola-event-feed>`); Expected amendments: the web-surface reference at line 21 cut."
    basis: ".andromeda/design-system.md:21"
  - detector: D-retired
    severity: warning
    section: Brand Identity
    change: "The per-surface expression table drops the `web-spa (`viola ui`)` 0.3 row; the two cli rows (human TTY 0.2; `--json` / non-TTY / `NO_COLOR` / `TERM=dumb` / passthrough 0.0) stand, with one line under the table: the frontend is deferred to 0.2.0, its technology not chosen, and its expression value is that version's to set."
    sidecar: "§Brand Identity per-surface table: the web-spa (`viola ui`) 0.3 row and its two motions removed (chunk 2026-10-20-epoch-5-cleanup, F12); the superseded row kept here."
    rationale: "Report Retired: F12 row `:41` (`web-spa (`viola ui`)` row), the web motion row; Expected amendments: line 41 cut, the cli expression rows kept."
    basis: ".andromeda/design-system.md:41"
  - detector: D-retired
    severity: warning
    section: Brand Identity
    change: "The single-source paragraph drops «viola's web page is 0.3, but its only two motions are the ones in the table above, and Motion's … override the 0.3–0.4 row»; it reads: at 0.2 the duration scale's 0.0–0.2 row applies (0–100ms, no entrance), and viola's cli (0.2 on a TTY, 0.0 otherwise) has no motion at all; 0.1.0 renders no page."
    sidecar: "§Brand Identity expression paragraph: the web page's 0.3 sentence removed (chunk 2026-10-20-epoch-5-cleanup, read by hand beside F12); the superseded sentence kept here."
    rationale: "Report Retired: «Line `:45` («viola's web page is 0.3…») states the same in other words and was read by hand: F12 does not reach it»; Expected amendments: line 45 cut."
    basis: ".andromeda/design-system.md:45"
  - detector: D-retired
    severity: warning
    section: Brand Identity
    change: "The Supporting attention element states the cocked strip as the cli renders it (the amber `DIALOG` word plus the dialog kind, the row keeping its place); the 12px / 160ms translate and the 4px holder band are named as a 0.2.0 frontend design input, not a standing motion."
    sidecar: "§Brand Identity Supporting attention element: the web translate motion re-framed as a 0.2.0 design input (chunk 2026-10-20-epoch-5-cleanup); the superseded paragraph kept here."
    rationale: "Sweep hit of the retired web motion row (report Retired, F12 `:41`): this paragraph restates motion (1) of that row (12px over 160ms ease-out) as standing; the report's Expected amendments name the supporting-attention paragraph as the sweep's to read."
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: Brand Identity
    change: "The Design direction's opening «The page is a control-tower strip bay after dark.» is re-framed as the direction a 0.2.0 frontend inherits (0.1.0 renders no page); the paragraph's cli-relevant direction (fixed fields, printed words, nothing announced before it is confirmed) stands."
    sidecar: "§Brand Identity Design direction: «The page is …» re-framed as a 0.2.0 frontend design input (chunk 2026-10-20-epoch-5-cleanup); the superseded sentence kept here."
    rationale: "Sweep hit of the retired web surface (report Retired, F12 `:41`): the paragraph describes the page as standing; the report's Expected amendments name the design-direction paragraph as the sweep's to read."
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: Color Palette → Semantic Colors
    change: "The DialogPending::true row's ink cell drops «see web-spa component 1»; it reads «#9C9278 when the strip is also `stale`, per the `stale` `--ink` swap»."
    sidecar: "§Semantic Colors DialogPending::true: the pointer to the cut web-spa component 1 removed (chunk 2026-10-20-epoch-5-cleanup, F12); the superseded cell kept here."
    rationale: "Report Retired: F12 row `:136` names a cut page component (web-spa component 1); §Surface: web-spa holds only the deferral stub."
    basis: ".andromeda/design-system.md:136"
  - detector: D-retired
    severity: warning
    section: Color Palette → Semantic Colors
    change: "The Problem::unauthorized (401) row drops «Access strip (see web-spa component 6)»; it reads: an access refusal, never showing the token, the URL or the file path."
    sidecar: "§Semantic Colors Problem::unauthorized: the pointer to the cut web-spa component 6 removed (chunk 2026-10-20-epoch-5-cleanup, F12); the superseded cell kept here."
    rationale: "Report Retired: F12 row `:208` names a cut page component (web-spa component 6)."
    basis: ".andromeda/design-system.md:208"
  - detector: D-retired
    severity: warning
    section: Color Palette → Semantic Colors
    change: "The TapeConnection::closed row's printed word drops the cut verb name: `TAPE stopped · not answering` (or the 0.2.0 frontend's server, unnamed), since no `viola ui` verb stands and the frontend's technology is not chosen."
    sidecar: "§Semantic Colors TapeConnection::closed: the cut `viola ui` verb name removed (chunk 2026-10-20-epoch-5-cleanup); the superseded cell kept here."
    rationale: "Sweep hit of the retired `web-spa (`viola ui`)` row (report Retired, F12 `:41`): this row still names `viola ui` as a standing verb."
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: Typography
    change: "The Display row's usage drops «at the start of the `<viola-atis>` header … rendered as the page's one `<h1>`»; it reads: `VIOLA` as the board's display word, the BAY / Data split as in the ATIS anatomy, its page markup a 0.2.0 frontend's to decide."
    sidecar: "§Typography Display row: the cut page component `<viola-atis>` and its `<h1>` removed (chunk 2026-10-20-epoch-5-cleanup, F12); the superseded cell kept here."
    rationale: "Report Retired: F12 row `:247` names the cut page component `<viola-atis>`."
    basis: ".andromeda/design-system.md:247"
  - detector: D-retired
    severity: warning
    section: Motion
    change: "The heading reads `## Motion (calibrated to the base expression level `0.2`)`, without «`0.3` on web-spa»."
    sidecar: "§Motion heading: the web-spa 0.3 calibration removed (chunk 2026-10-20-epoch-5-cleanup, F12); the superseded heading kept here."
    rationale: "Report Retired: F12 row `:335` (§Motion's `0.3 on web-spa`), the web motion calibration; Expected amendments name it among F12's rows."
    basis: ".andromeda/design-system.md:335"
  - detector: D-retired
    severity: warning
    section: "Surface: web-spa"
    change: "The heading over the one-line deferral stub is renamed to the deferred frontend (e.g. `## Surface: frontend (deferred to 0.2.0)`), so no `web-spa` surface stands; the stub sentence «The frontend is deferred to 0.2.0; its technology is not chosen.» stays."
    sidecar: "§Surface: web-spa heading renamed to the deferred frontend (chunk 2026-10-20-epoch-5-cleanup, F12); the superseded heading kept here."
    rationale: "Report Retired: F12 row `:370` (`## Surface: web-spa`) stands over a deferral stub, a standing web leftover like obs-plan §4's GUI heading."
    basis: ".andromeda/design-system.md:370"
  - detector: D-retired
    severity: warning
    section: Self-Validation Protocol → 3. Signature Test
    change: "The Signature Test points to the readback box in its one 0.1.0 place, the `[RB]` / `[  ]` / `[/ ]` column of CLI `viola send`; places (1) `<viola-event-feed>` and (2) `<viola-transfer>`, the `<viola-readback>` element and its `.rb` rule, and «per web-spa component 3» are removed, a 0.2.0 frontend's test being that version's to write."
    sidecar: "§Signature Test: the cut page components `<viola-event-feed>`, `<viola-transfer>`, `<viola-readback>` and web-spa component 3 removed (chunk 2026-10-20-epoch-5-cleanup, F12); the superseded paragraph kept here."
    rationale: "Report Retired: F12 row `:584` (cut page components); Expected amendments name §Self-Validation's Signature Test among F12's rows."
    basis: ".andromeda/design-system.md:584"
note: "D-design-tokens holds: every new surface in the report's Coverage reads `tokens n/a` (the headroom keepers, the census `--case` form, the fake agent's process-group spawn); the chunk renders no UI. Sections framed «A 0.2.0 frontend design input; 0.1.0 renders no page» (Domain status rows other than those above, Iconography's «On the web», the Token Test's CSS tokens) were read and left: they describe deferred inputs, not standing components."
