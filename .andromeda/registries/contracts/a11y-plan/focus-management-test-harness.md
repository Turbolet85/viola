### Focus management test harness

- **Driver:** the tests' Playwright Test 1.63.0 driver, per the upstream-context Section 5 Test Harness Contract Summary (binding). There is no separate harness. Each test boots `pw-<spec>-<test id>-<workerIndex>`, navigates `/?t=<token>`, runs `cleanup`, and uses `retries: 0`. Waits use auto-waiting locators and SSE event offsets only.
- **Library:** no runtime focus library. Native platform focus (`:focus-visible`, `<details>`/`<summary>`, fragment links) in the DOM the page's React components render. tabbable 6.5.0 is test-side only: it is injected with `page.evaluate(source)` and its `tabbable(document.body)` list is the expected-sequence oracle.
- **Pattern:**
  - **Tab walk:** scripted Tab / Shift+Tab traversal from `page.locator('body').focus()`, asserting `toBeFocused()` at each step against the oracle.
  - **Initial focus:** after load, `document.activeElement` is `<body>`.
  - **No focus theft:** for each transition (SSE arrival, auto-follow, cock/revert, readback refusal, 503, 401 strip, `TAPE stopped`, `liveness-changed` stale, E4 `Last-Event-ID` resume, E5 `state-recovered`), focus a mid-tape `<summary>` first, trigger through the harness/fake agent, wait on the DOM signal (`data-dialog`, `data-rb`, strip text), then assert the same element is still focused (`focus-lost` otherwise).
  - **Focus survives trimming:** push beyond the 2000-line DOM cap while the oldest `<summary>` is focused, then assert the same `<summary>` is still focused (`focus-lost` otherwise; Section 5 → Focus restoration). In this trimmed state also assert: the design trim-notice text (`older lines trimmed from view: N — the full tape is events.ndjson`) is a `<p>` inside the `role="log"` `<div>` before the `<ol>` (after the `TAPE stopped` `<p>` when present); it is not a tab stop; and the axe verdict holds (Section 10 state list).
  - **Modal patterns:** no focus trap entry, exit or restoration assertions, because v1 has no modals.
