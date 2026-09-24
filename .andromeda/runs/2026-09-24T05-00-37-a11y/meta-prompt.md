## Output Protocol

You are an iteration agent improving the viola a11y plan draft at `D:/dev/projects/viola/.andromeda/runs/2026-09-24T05-00-37-a11y/a11y-plan-draft.md`. Check it against these upstream sources in the same run directory: `upstream-context.md` (binding excerpts), `a11y-scope.md` (Phase 1), `a11y-research.md` (Phase 2 tool catalog) and `review-feedback-1.md` (overseer decisions, which are binding).

Follow these rules:

1. **Output patches and a changelog only.** Never reproduce the full document or a full section.
2. **Patch format:**

   ### Patch N: <one-line description naming the section, e.g. "§4 State strips row — invalid `<p>` placement inside table">
   **Old:**
   ```
   <exact text copied from the draft, long enough to be unique>
   ```
   **New:**
   ```
   <replacement text>
   ```

   - Write at most 8 patches per iteration. Spend them in this order:
     1. downstream-blocking defects (dimensions 1, 2 and 3: downstream readiness, scope faithfulness, tool anchoring);
     2. implementation-misleading defects (dimensions 4, 5 and 6: tier calibration, WCAG mapping, design-token binding), only if budget remains;
     3. anti-pattern relevance (dimension 7), only if the patch is one line and cannot be deferred.
   - Every **Old:** block must match the draft byte for byte.
   - Keep token names, SC IDs, versions and paths exactly as upstream spells them.
3. **Changelog:** write exactly ONE line for the whole iteration, never one line per patch:
   `[Iteration N] [substantive|cosmetic] <description summarising all patches>`
4. **Prohibited:**
   - reproducing the full document;
   - restructuring or renumbering sections without a defect-driven reason;
   - labelling wording, formatting or synonym changes "substantive";
   - reopening decisions that `review-feedback-1.md` or Decisions Log entries marked "accepted (overseer)" already settled. This covers D-A11Y-02/03/04/06/09/14/19, the SC 1.4.10 < 760 CSS px exception, the 401 copy binding, the CSS-only ellipsis, the ESLint lockfile pin and the deferral of the v1.x brakes;
   - adding content owned by other specialists:
     - test code blocks over 5 lines (`test(...)`, `describe(...)`, `#[test]`, `fn test_*`);
     - instrumentation code;
     - threat-model or auth-flow detail;
     - raw design token values (hex, px, ms);
     - a11y implementation code blocks over 5 lines (`onKeyDown` handlers, `element.focus()` / FocusTrap blocks, `axe.run` / `axe.configure` blocks, `aria-*` injection markup);
   - recommending ARIA on non-semantic hosts (e.g. `role="row"` on `<viola-session-row>`). Semantic HTML comes first;
   - recommending Lighthouse, pa11y, WAVE, Stark or any manual-only or vendor-locked method as primary verification.
5. **If you find no issues:** output "No patches" followed by `[Iteration N] [cosmetic] No defects found along dimensions 1–7`.

## Analysis Protocol

Work through these steps in order. Do not start writing patches until step 4.

1. **Read once, without noting anything.** Read the whole draft (1341 lines, 12 sections) end to end. Use `offset` / `limit` reads, and re-read any section before you quote it in an **Old:** block.

2. **Walk these cross-references.** Each one is a place this plan has already drifted or could drift:
   - **a11y-scope entity → Sections 4 / 5 / 6 / 7.** Each of the 11 assertable web-spa entities in Section 1 needs a Section 4 catalog row and at least one concrete verification path. The tui boundary-only entity has three boundary clauses (zero viola bytes; keystrokes never blocked past the atomic paste; focus/mouse/resize not counted as editing). Each clause needs its own harness assertion. The not-assertable entities (403 page, v1.x brakes, `/api/*`, plugin/IPC, internal crates, phone view) must not be asserted as v1 requirements anywhere.
   - **Surface → Section 4 reach table → Section 9 stage row.** web-spa maps to the Lint and E2E stages, and cli/tui map to the Unit/integration stage. Every tool in the reach table appears in a Section 9 stage.
   - **Path P1–P6 → Section 4 scenario.** Roles, focus order and SC list agree with Section 1. The accepted "1.4.10 (≥ 760)" qualifier is the only allowed difference.
   - **Trigger → owning section.** visual-discrimination → Section 6 contrast pairs + state colour tokens. keyboard-only → Section 5. screen-reader-priority (scoped) → Section 7, with `aria-live="off"` on the log.
   - **Tool/version at three sites:** the Section 1 copy block, the body (Sections 2, 3, 4 and 9), and the Decisions Log. All three agree with `a11y-research.md`.
   - **Section 6 tokens ↔ `upstream-context.md` Section 3 A11y-Relevant Design Tokens** (binding). No invented token, no invented pairing, no raw value.
   - **Section 3 violation row ↔ obs Log Format schema reproduced in Section 1** (binding). Field names are unrenamed, an absent key means null (never a literal `null`), and `event` / `process` are closed enums.
   - **Section 3 `violation_type` closed list ↔ every id used in Sections 5–10.** Every id used must be in the list, and each id is used for one meaning only.
   - **Section 9 ↔ the tests' 5-command discipline** (`boot`, `run`, `status`, `cleanup`, `logs`). There is no second driver and no new `suite` value.
   - **Pointers:** every `[resolved: …]`, `(See § X)`, "see the Decisions Log" and "(Section N)" reference lands on a real subsection. Where a D-A11Y ID exists, the reference names it.

3. **Check each dimension below** with its anchor example in mind. Anchors are verified ground truth. Look for the same *kind* of defect elsewhere, not only the quoted instance.

4. **Out-of-scope discipline.** Do NOT patch a finding if fixing it would require any of the following:
   - specific test code (`it(...)` / `test(...)` / `describe(...)` / `#[test]` blocks over 5 lines — tests / `/andromeda-implement` domain);
   - instrumentation code (OTel/tracing/Sentry init, span or metric blocks over 5 lines — obs / `/andromeda-implement` domain);
   - a11y implementation code over 5 lines (`aria-*` injection markup, focus-management JS, keyboard handlers, `axe.run` / `axe.configure` / AxeBuilder chains, pa11y scripts — `/andromeda-implement` domain);
   - threat-model, auth-flow or CSP-policy design (security domain);
   - raw token values (hex, px, ms — design domain).

   Instead, check that the plan states the boundary requirement: which SC must be verified, by which named tool, producing which JSON. Patch only if that boundary is missing. WCAG SC IDs, ARIA role names, design token NAMES and harness picks are a11y's own domain and are in scope.

   **Section 1 rule (D-A11Y-15):** Section 1 is verbatim from `a11y-scope.md`. When Section 1 and a later section disagree, patch the later section or a `[resolved: …]` pointer. Never patch Section 1 prose.

5. **Prioritize** by downstream breakage. Patch first anything that makes `setup-project` materialize a failing gate, makes `route` mis-order phases, or makes CI green while an SC goes unverified. Wording fixes come last and are labelled cosmetic.

## Analysis Dimensions

### 1. Downstream Readiness [priority: high]

- **Closed vocabularies are internally consistent.** Check every `violation_type` and `severity` use against the Section 3 rules: non-axe checks emit `serious`, and each closed-list id has exactly one meaning. `csp-console` is listed as its own check, and Section 10 lists "A CSP / Trusted Types console violation" and "An unscrubbed token…" as two separate failure conditions. Does a scrub failure borrow `csp-console` with a different severity? Does it need its own closed-list id (for example a scrub-leak id) so the `jq` triage query can tell the two apart?
- **Structural placement can be materialized.** Section 4 catalog rows must name positions that are valid HTML inside the stated parents. The racks are `<table>` + `<caption>` (holding the `<h2>`), and the tape is `role="log"` → `<ol>` → `<li>`. For each state strip, can `setup-project` tell whether the `<p>` goes before the `<table>`, after it, or inside the tape `<ol>` as an `<li>`? html-validate content-model rules gate the rendered DOM, so an ambiguous position becomes a CI failure.
- **CI wiring inputs are defined.** Section 3 "Bootstrap phases" and the Section 9 Lint row say "over the Lit sources" / "the Lit `html` templates" and "the embedded `assets/index.html`". Neither names the crate or source directory, and neither names where the `eslint.config.js` that `npx --prefix e2e-web eslint` resolves lives. Searches of the draft for `viola-ui`, `crates/`, `eslint.config` and `frontend` found no path. Can `setup-project` write the lint step without guessing? Is the html-validate `elements` declaration specific enough (permitted parent/content for `display: contents` hosts inside `<tbody>`) that `no-unknown-elements` and the content-model rules do not misfire (research: "Declare `viola-*` elements in its `elements` config")?
- **The coverage gate is derivable.** Section 10 requires "every applicable SC in the Section 3 per-SC map" to have ≥ 1 passing `@sc-*` test. The map's "Applies in v1" column mixes `yes`, `no media`, `no inputs`, `single-page exception`, `single page`, `no surface` and `indeterminate-language content`. Does the plan say which of these count as "applicable" in `sc-coverage.json`? For example, does `surface-absence` carry `@sc-1.2.x` tags? Section 9 places Aggregation "after `gate`" and never states its run condition when `gate --require playwright` has already failed the job.

**Anchor example:** Section 3 A11y Assertion Harness Contract → Structured violation JSON schema → Scrubbing

> "any string that matches the token, `Cookie`, `?t=`, an absolute path or `CLAUDE*` fails the test with `violation_type:"csp-console"`-class severity `critical`. The raw axe JSON attachment is the scrubbed object."

**Issue:** Two bullets earlier in the same subsection, the plan says "`severity` uses the axe impact vocabulary (`minor|moderate|serious|critical`). Non-axe checks emit `serious`." A scrub failure is a non-axe check, yet it emits `critical`. It also reuses `csp-console`, which the closed list and Section 10 reserve for CSP / Trusted Types console violations. "`csp-console`-class" is not a value that a row writer can emit.

**Why this matters:** `setup-project` materializes the violation-row writer from this text. It must either break the `serious` rule or emit a mislabelled id, so the Section 3 `jq` triage and any SC/type aggregation conflate secret leaks with CSP errors. The fix is plan-level: add a closed-list id and state its severity, with no code.

**Anchor example (second):** Section 4 ARIA Patterns & Roles → Per-component pattern catalog → State strips row

> "native `<p>` directly after the rack `<caption>`-bearing table's heading position (rack errors / 401) or as the first tape line (`TAPE stopped`)"

**Issue:** The rack heading sits inside `<caption>`, so "directly after the … heading position" reads as a `<p>` inside `<table>`, which is invalid. "The first tape line" inside an `<ol>` of `<li>` does not say whether the `<p>` is wrapped in an `<li>`. P5 then says "state strips are `<p>` text in `main`" without resolving either position.

**Why this matters:** html-validate 11.16.0 gates the rendered DOM (SC 4.1.1 row), so a literal implementation fails CI. A guessed placement changes the `ariaSnapshotJSON` region order that SC 1.3.2 and 3.2.3 compare across states.

**Adversarial:** If the scrubber fails the test on a leaked token but the row it writes is indistinguishable from a CSP console error, and the Aggregation stage (which runs "after `gate`") never runs because the job already failed, what machine-readable artifact tells the triage agent that a secret leaked into an a11y attachment?

### 2. A11y Scope Faithfulness [priority: high]

- **Every boundary clause has a harness.** The tui entity in Section 1 has three boundary clauses. Trace each one to a concrete assertion in Section 3 (Keyboard test harness / tooling), Section 4 P4 and the Section 9 Unit/integration row. The same applies to the cli required behaviours: C0/C1 escaping except `\n`/`\t`, static output with no spinner, and word beside every SGR. Is each one pinned by a named trycmd / assert_cmd / portable-pty check on all three OS legs, or only restated?
- **Entity coverage is complete, and not-assertable entities stay excluded.** Check that each of these appears in the Section 4 catalog and has a verification path:
  - the `<viola-atis>` states (`budget-paused`, `skipped N · N · N`, `expired`, `TAPE connecting|live|stopped`);
  - the trim notice;
  - the `cli_verified:false` column word;
  - the `unconfirmable` readback state.

  Does any Section 7 / Section 8 text about the v1.x brakes ("Per WCAG SC 3.3.2 + SC 4.1.2 when built") read as a v1 requirement that `sc-coverage.json` would pick up?
- **Paths and pointers.** Do Section 4 P1–P6 SC lists match Section 1 P1–P6? Section 4 P2 adds SC 3.2.4. Is that addition reflected anywhere Section 1-derived lists are consumed? Does each `[resolved: …]` pointer land on the named subsection? For example, "Section 4 → landmarks" should land on "Per-surface landmark roles inventory".

**Anchor example:** Section 1 A11y Scope Summary → `viola run` TUI passthrough entity, versus Section 4 P4

> "viola writes zero bytes to the terminal except the child's output;"

and Section 4 → Must-be-accessible path scenarios → P4:

> "**tui:** the portable-pty boundary asserts that human keystrokes are never blocked and that focus/mouse/resize sequences do not move the wheel."

**Issue:** The first boundary clause (zero viola bytes) has no harness assertion.

**Search evidence:** the draft was searched for `zero viola bytes`, `zero terminal bytes`, `bytes of its own` and `writes zero`. Every hit is a restatement, not an assertion spec: Section 1 (line 97 above, and the surface note "They check that viola emits zero terminal bytes of its own") and the Section 4 reach-table cell "zero viola bytes, keystrokes unblocked". Neither the Section 3 Keyboard test harness tui tooling nor P4 nor the Section 9 Unit/integration row specifies it. Section 6 asserts only "zero SGR under … `viola run`", which is a subset of bytes, not all viola-originated output.

**Why this matters:** R7 bans parsing the child screen, so the byte-level wrapper boundary is the only automated tui guarantee. Without a spec, `setup-project` wires keystroke tests only, and a stray viola diagnostic line on the terminal goes undetected.

**Adversarial:** If a future edit moves the Section 1 surface note "They check that viola emits zero terminal bytes of its own" into a Decisions Log rationale, would any Section 3 / 4 / 9 artifact still fail CI when viola prints to the child's terminal? How would the iteration agent notice that nothing gates it?

### 3. Tool Anchoring (Catalog ↔ Plan) [priority: high]

- **Behavioural claims about tools are anchored in research.** For each claim the plan makes about how a tool behaves, check whether `a11y-research.md` says it or the plan labels it an assumption with a machine check. Claims to check:
  - option ordering;
  - rules surviving `withTags()`;
  - VSR Trusted Types safety (D-A11Y-11 already flags this);
  - `target-size` default state.
- **Versions agree across the Section 1 copy, the body and the Decisions Log.** Check:
  - `@axe-core/playwright` 4.13.0 / axe-core 4.13.x (research: "it ships axe-core 4.13.x");
  - Playwright Test 1.63.0; colorjs.io 0.7.1; tabbable 6.5.0; @guidepup/virtual-screen-reader 0.33.0;
  - html-validate 11.16.0; eslint-plugin-lit-a11y 5.1.1; Lit 3.3.3;
  - assert_cmd 2.2.2 / predicates 3.1.4 / trycmd 1.2.1; NVDA 2026.2;
  - `actions/upload-artifact` v7.0.1; portable-pty `=0.8.1` vs research 0.9.0 (D-A11Y-13).

  Section 2's Robust row says "axe 4.1.2 rules" beside "html-validate 11.16.0". Could a downstream reader take "4.1.2" as an axe version rather than SC 4.1.2? `jq` (Section 9 Aggregation) does not appear in `a11y-research.md`; a whole-word `jq` search returned no hits. Is it stated as catalog-less transport?
- **Tool SC credits match research `**WCAG SCs covered:**`.** Research says the best-practice rules "carry no SC tag", and it lists SC 2.4.6 under "Not covered by any axe rule". Section 3 says the five rules "close the SC 1.3.1 / 2.4.6 landmark and heading gaps", while `wcag_criterion` maps them only to `"1.3.1"`. Is SC 2.4.6 credit coming from tools research assigns it to (html-validate, aria snapshot), and never from axe?

**Anchor example:** Section 3 A11y Assertion Harness Contract → A11y testing tool pick → Configuration

> "**Illustrative anchor** (`options()` first, then `withTags()`, so the tag filter is not overwritten):"

**Issue:** The claim that `.options({rules})` before `.withTags()` keeps the five best-practice rules enabled under a tag-type `runOnly` is not anchored in research.

**Search evidence:** research's `@axe-core/playwright` block defines the fixture as `.withTags([...])` only. Searching `a11y-research.md` for `.options`, `options(` and `best-practice` finds only item (d), which ends "Phase 3 decides" and says nothing about ordering or `rules` overriding `runOnly`.

The plan presents the ordering as fact, and it adds no machine check that the rules actually ran. Research line 8 confirms axe JSON carries `passes[]` / `incomplete[]` / `inapplicable[]`, so a boundary requirement is cheap: each of the five rule ids must appear in some result array, or the run fails.

**Why this matters:** if the semantics differ from the claim, the five rules silently do not run. The binary verdict stays `violations: []`, and the SC 1.3.1 landmark/heading gaps D-A11Y-01 was meant to close reopen with no signal.

**Adversarial:** If `@axe-core/playwright` changes `options()` from merge to replace in a patch release under the unpinned `4.13.x` engine range, which artifact in Section 9 would show that `region` / `landmark-one-main` stopped executing? Would anything fail CI?

### 4. Tier Calibration (Standard, web-spa only) [priority: high]

- **The SLO state list matches the path scenarios.** Section 10's "after every state render" list is the gating contract. Compare it state by state with:
  - the per-state axe verdicts in Section 4 P1–P5;
  - the Section 5 layouts (bay-steady-state, bay-narrow, bay-first-reading/empty, bay-degraded, tape-line-expanded, after skip-link activation);
  - P4's web WHEEL `human` state.

  Every state a path scenario says gets an "axe verdict" must be in the SLO list, or the SLO is weaker than the scenarios.
- **Depth stays at Standard.** Is Section 8 limited to what has a v1 surface: SC 2.2.1, 3.3.1, 3.3.3 and 3.2.1–3.2.4, plus absence for 3.3.7 / 3.3.8, with 2.2.6 / 3.1.5 / 3.3.9 not claimed? Is Section 7 limited to announcement correctness, as the scoped screen-reader-priority trigger says? Is gating on @guidepup/virtual-screen-reader 0.33.0 plus a MutationObserver fallback proportionate to that scope?
- **Budgets are measurable from named artifacts.** For "axe `analyze()` < 30 s per state render", "Total … < 10 min" and "> 50 % over the budgets", is each tied to a field the plan names (`duration_ms` in the axe attachment, Playwright JSON durations)? Is the "> 50 %" failure evaluated by a named stage or command?

**Anchor example:** Section 10 SLO Invariants & A11y Budgets → Always-required SLO invariant

> "axe `violations: []` after every state render (steady, bay-narrow, first-reading/empty, cocked, readback-refused, 503, `TAPE stopped`, 401 access strip);"

versus Section 4 → P5:

> "**Tests:** `a11y-path5` asserts, per state (503, `TAPE stopped`, stale, 401 after a `viola ui` restart under the same harness session):"

**Issue:** P5 requires an axe verdict in the `stale` state, but the SLO list omits `stale`. It also omits tape-line-expanded (Section 5), the P4 WHEEL `human` state, and a scrolled-up state in which the `N new lines below` anchor exists.

**Search evidence:** Section 10 (lines 1100–1131) was searched for `stale`, `scroll`, `expanded`, `human`, `wheel` and `focused`, with zero hits.

**Why this matters:** Section 10 is the build-failure contract that the SC coverage report and route read. States outside it can regress, for example a stale `--ink-dim` swap on a lit strip, and the Standard-tier "zero AA violations on must-be-accessible paths" claim is still reported as met.

**Adversarial:** The `N new lines below` anchor exists only "while scrolled up", and no listed state scrolls up. Can axe `link-name`, `color-contrast` or `target-size` ever evaluate the anchor? If it cannot, which SCs claimed for it (2.4.4, 2.5.8) rest on nothing but a `boundingBox()` check?

### 5. WCAG Mapping Discipline [trigger: a11y_tier=Standard OR a11y_tier=Comprehensive — fired: A11y Scope Summary declares "A11y tier: `Standard (1)`"] [priority: high]

- **The same SC is verified the same way everywhere.** For each SC with checks in more than one section, the trigger lists and thresholds must agree:
  - SC 2.4.11: the Section 3 row vs Section 5 Sticky header vs Section 6 Focus ring;
  - SC 2.5.8: the Section 3 row vs Section 6 Target size;
  - SC 2.2.2: the Section 3 row vs Section 5 Auto-follow vs Section 6 Motion.

  Does SC 2.5.8 say how the spacing exception is computed? Is the SC 2.4.11 check defined for the zero-height `#tape-end` focus target after skip-link activation?
- **Levels are auditable.** The Section 3 per-SC map has columns `SC | Applies in v1 | Agent-runnable verification path (JSON)`, with no A/AA column and no marker of 2.2 origin beyond "(2.2)" on the last row. Should each row carry its level, so the "WCAG 2.1 AA + SC 2.4.11 + SC 2.5.8" label can be checked row by row? Section 6 subsections already write "(A)" / "(AA)".
- **Exception wording is consistent.** SC 2.4.5 is marked "single-page exception", SC 3.2.3 "single page", and Section 8 says 3.2.3 "is satisfied through the single-page exception". Is the same status stated the same way in Sections 3, 8 and 10? Does Section 10 "every applicable SC" include or exclude these rows? Generic "WCAG AA" wording without SC IDs (Section 10 "Zero WCAG AA violations", Section 11 § Strategy "NEVER skip POUR principle coverage") should cite specific SCs.

**Anchor example:** Section 3 → WCAG criteria mapping → per-SC table, row 2.4.11

> "| 2.4.11 Focus Not Obscured (Min) | yes | after each Tab, and after the header grows (`TAPE stopped` box), the focused element's rect is not fully covered by the `<viola-atis>` rect |"

versus Section 5 → Focus restoration → Sticky header:

> "The focused `<summary>` is checked against the header rect after each Tab, and again after the header gains a line for `expired`, `TAPE stopped` or a nonzero `skipped` (SC 2.4.11)."

**Issue:** The SC map names one header-growth trigger, and Section 5 names three. Also, the Section 6 `--rule-info` pair row lists "`budget-paused`, nonzero `skipped`, `TAPE stopped`, error strips" as boxed ATIS states, but `budget-paused` appears in neither growth list. Check whether that box also adds a header line. `@sc-2.4.11` coverage built from the Section 3 row would miss triggers that Section 5 requires.

**Why this matters:** `setup-project` and the SC coverage report key off the Section 3 map. SC 2.4.11 is one of the two 2.2 SCs the conformance label explicitly claims, so a partial trigger set makes the label's evidence incomplete.

**Adversarial:** Suppose the ATIS header grows by two lines at once (`expired` + `TAPE stopped`) while focus is on the newest `<summary>` near the top of the tape scroller. Does the `scroll-padding-top` "matching the sticky `<viola-atis>` block size" update before the assertion runs? Does any section require the check after the combined growth, or only after single triggers?

### 6. Visual Design Verification to Design Tokens [trigger: upstream-context Section 3 Design System Excerpt has explicit A11y-Relevant Design Tokens (NOT "N/A") — fired: upstream-context.md "### A11y-Relevant Design Tokens" lists color pairs, focus ring, target size, motion, state color and typography tokens] [priority: high]

- **Every rendered text/indicator has a pair.** Each visible text or state indicator in Sections 4 and 5 must map to a Section 6 pair row whose Context names it. Pairs must use upstream token names only, and the two derived pairs must trace to the D-A11Y-18 design text.
  - The focus ring pair `--focus-ring`/`--surface-strip` exists, but strips are "never tab stops". Which focusable element actually renders on `--surface-strip`? Is that pair asserted in a DOM state that exists?
- **Token-named checks can be implemented by name.** Section 6 Typography says "every element sized by `--line-h`, `--strip-h`, `--lh-display` or `--lh-label`". Does the plan say how a test identifies those elements, since computed style returns resolved values, not token references? `--strip-h` is "not asserted in v1" under Target size yet is listed for clipping under Typography. Which is it? The focus-ring tokens include `--radius`, but the SC 2.4.7 check compares only `outlineColor/Width/Offset`.
- **No raw values.** Scan Sections 3, 5, 6 and 10 for hex, `Npx`, `Nms`/`Ns`, rgb/hsl and cubic-bezier.
  - Allowed: WCAG thresholds 4.5:1, 3:1 and 24×24 / 44×44, and the SC 2.2.2 five-second threshold, framed as the SC threshold.
  - Viewport widths ("1024", "760–1023", "1536 CSS px at 200 %") must be attributed to tests/layout as test viewports.
  - The SC 1.4.12 override stays "exactly as SC 1.4.12 defines them", with no numbers.
  - Forced-colors expectations must match the design wording: borders survive; strike → dashed outline + `unable`; cock band → system highlight.

**Anchor example:** Section 5 Keyboard Navigation → Skip links

> "`skip to tape` is the first focusable element, with `href="#tape-end"`. It is hidden until focused, then drawn over the header's inline start with the `--focus-ring` indicator (SC 2.4.1, 2.4.7)."

**Issue:** No Section 6 pair row's Context names the focused skip link's text or background, or the `N new lines below` anchor.

**Search evidence:** Section 6 (lines 833–925) was searched as follows:
- `skip` hits only the Target size verification line and the word `skipped`;
- `link` hits only the `--handoff` row ("names in `link`/`unlink` lines") and the Target size line;
- `anchor` / `new lines` hit only the Target size line;
- `header` / `ATIS` hit only the `--ink`/`--surface-bay` context "primary text on page, tape, ATIS, rack gap".

axe `color-contrast` runs only at the Section 10 state renders. There the skip link is hidden (not focused) and the anchor is absent (not scrolled up), so neither element's rendered contrast is ever machine-checked.

**Why this matters:** these are two of the three focusable elements in v1. The fix must not invent a token. Either extend the Context of an existing upstream pair, if design text supports it, or record a design question in the Decisions Log and add a focused-skip-link / scrolled-up state to the axe state list.

**Adversarial:** If the skip link is "drawn over the header's inline start" and the design later gives the header a different surface alias, would the token checker catch a skip-link contrast failure? It iterates only the listed pairs. Or would the failure surface only in the founder's manual NVDA pass, which the plan says is "never the gating evidence"?

### 7. Anti-Pattern Relevance [priority: medium]

- **Bans agree with the picks.** Does any Section 11 ban name or require a tool the plan dropped (D-A11Y-05: Lighthouse, pa11y, `@lhci/cli`, WAVE)? Every ban that names a tool should name the actual pick: `@axe-core/playwright` 4.13.0 inside `scripts/agent-run.* run --browser`.
- **No duplicates or generic bans.** Look for bans repeated across domains. § Visual and § Motion both open with "NEVER autoplay motion without `prefers-reduced-motion: reduce` respect". Look also for template bans with no viola grounding, such as § ARIA "NEVER overuse `aria-label` (use visible `<label>` first…)" in a plan with no inputs, and § Strategy "NEVER skip POUR principle coverage". Does each domain keep ≥ 3 bans grounded in viola pitfalls?
- **§ Universal holds only cross-stack bans.** § Universal should keep the ≥ 4 cross-stack bans: manual-only, vendor-locked, inaccessible captcha/modal, and conformance without machine evidence. Its viola-specific items ("NEVER parse the rendered `claude` TUI", "NEVER write a11y violation rows into `<home>/diagnostics/`", "NEVER overclaim in the GUI") belong under § Screen Reader, § CI or § Strategy. Verify each semantic-HTML-first ban (ARIA on custom-element hosts, no role via `setAttribute`, native `<details>`) is present and not weakened.

**Anchor example:** Section 11 A11y Anti-Patterns → § CI

> "- NEVER ship lint-only a11y without runtime axe-core / pa11y /"

versus § Strategy in the same section:

> "- NEVER add a second browser automation stack (Lighthouse, pa11y, `@lhci/cli`, Puppeteer `connect()`) beside the tests' Playwright Chromium (Overseer Direction 1)."

**Issue:** The § CI ban, which continues "Lighthouse (lint catches subset; runtime catches rest)", offers pa11y and Lighthouse as acceptable runtime checks. § Strategy and D-A11Y-05 ban both. A reader of § CI alone could add pa11y as the "runtime" half.

**Why this matters:** the phase loop reads Section 11 bans per domain. A ban that endorses a rejected tool undercuts Overseer Direction 1 and the "no second browser stack" contract. This is a one-line reword to `@axe-core/playwright` 4.13.0 runtime checks.

**Adversarial:** If an implementer reads only § CI and § Universal, where Universal says "axe-core JSON / pa11y JSON / Lighthouse JSON / equivalent", which bans in those two subsections alone would stop them from wiring `pa11y-ci` as a second gate? Does the iteration agent reconcile every ban that still names a dropped tool?

Read the document, follow the Analysis Protocol, analyze along all dimensions, output patches and changelog.
