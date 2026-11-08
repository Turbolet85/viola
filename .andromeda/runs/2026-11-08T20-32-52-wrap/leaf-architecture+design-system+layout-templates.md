@leaf_group group=architecture+design-system+layout-templates
@leaf_read leaf=.claude/rules/frontend.md lines=29
last: _This section is owned by `/wrap-session`. setup-project pre
@leaf_edit n=1 leaf=.claude/rules/frontend.md line=8 class=stale
leaf-text: the web-spa frontend is deferred to 0.2.0, its technology is not chosen, and 0.1.0 renders no page.
source: fct-ll5xgfku «No web surface: viola renders no page; the keyboard-driven terminal fleet console tiled on Hyprland (matrix#v2-14) takes its own surface section at the route entry «Console scaffold».»
replace-with: viola renders no page and has no web-spa surface; the keyboard-driven terminal fleet console tiled on Hyprland (matrix#v2-14) takes its own surface section at the route entry «Console scaffold».
@leaf_edit n=2 leaf=.claude/rules/frontend.md line=10 class=retired
leaf-text: ## Tokens (the Token Test; a 0.2.0 frontend design input, 0.1.0 renders no page)
source: fct-k6sgov4c «N/A — viola renders no page, so no stylesheet tokens to trace.»
replace-with: ## Palette (viola renders no page, so no stylesheet tokens to trace)
@leaf_edit n=3 leaf=.claude/rules/frontend.md line=11 class=retired
leaf-text: - Exactly eight hex values (`--c-*`), read only where a role alias is assigned; components use role aliases (`--surface-*`, `--ink*`, `--rule-*`, `--attention`, `--handoff`, `--focus-ring`, `--rb-*`). Every length is a token or a `ch` track in `--strip-cols` / `--strip-cols-wrapped` / `--tape-cols`; only the `760px` / `1024px` breakpoints are literal.
source: fct-5lo2eq3p «N/A — viola renders no page, so no stylesheet token to trace.»
replace-with: - The palette holds eight values; viola renders no page, so no `--c-*` value, role alias, length token or breakpoint is traced.
@leaf_edit n=4 leaf=.claude/rules/frontend.md line=12 class=retired
leaf-text: Lamp-off, amber and blue text never sit on holder `#2A2E33` (they fail AA there).
source: fct-ashdaqa2 «The quiz limits CLI colour to amber and dim, so link output (`overseer -> builder`) is plain at every depth.»
replace-with: On the CLI handoff blue is never used: link output (`overseer -> builder`) is plain at every depth.
@leaf_edit n=5 leaf=.claude/rules/frontend.md line=13 class=retired
leaf-text: - Borders-only depth: no `box-shadow`, `text-shadow`, glow or `drop-shadow`; radius 0 everywhere; dark only.
source: fct-cwga6qbn «N/A — viola renders no page, so no depth tokens; the cli draws no depth.»
replace-with: - viola renders no page, so no depth or radius tokens; the cli draws no depth.
@leaf_edit n=6 leaf=.claude/rules/frontend.md line=14 class=retired
leaf-text: - Fonts: installed stacks only (`--font-label` Bahnschrift…, `--font-field` Cascadia Mono…); every contrast and render assertion must hold with the Linux DejaVu families resolved (the resolved family, not only the declared stack); 0.1.0 renders no page, so no CI step renders these stacks.
source: fct-qwc3u462 «**cli:** Terminal monospace (the user's configured font).»
replace-with: - Fonts: the cli's type is the terminal monospace (the user's configured font); viola renders no page, so no font stack is declared.
@leaf_edit n=7 leaf=.claude/rules/frontend.md line=16 class=stale
leaf-text: ## Motion (base expression 0.2; the frontend's level deferred to 0.2.0)
source: fct-wuobflqs «N/A — viola has no web or graphical frontend, so no frontend expression level.»
replace-with: ## Motion (base expression 0.2; no web or graphical frontend, so no frontend expression level)
@leaf_edit n=8 leaf=.claude/rules/frontend.md line=17 class=retired
leaf-text: the duration scale's 0.0–0.2 row applies (0–100ms, no entrance), and the cli (0.2 on a TTY, 0.0 otherwise) has no motion at all; the cocked strip's 12px / 160ms translate is a 0.2.0 frontend design input.
source: fct-4m7ag2lh «At 0.2 viola's cli (0.2 on a TTY, 0.0 otherwise) has no motion at all.»
replace-with: the cli (0.2 on a TTY, 0.0 otherwise) has no motion at all; viola renders no page, so no duration scale applies.
@leaf_edit n=9 leaf=.claude/rules/frontend.md line=20 class=stale
leaf-text: The web components are 0.2.0 frontend design; 0.1.0 renders no page.
source: fct-ll5xgfku «No web surface: viola renders no page; the keyboard-driven terminal fleet console tiled on Hyprland (matrix#v2-14) takes its own surface section at the route entry «Console scaffold».»
replace-with: viola renders no page, so there are no web components.
@leaf_edit n=10 leaf=.claude/rules/frontend.md line=20 class=stale
leaf-text: and a future page reuses them
source: fct-mpm2axrl «The six captions, words and slot order are fixed, and the terminal fleet console (matrix#v2-14) reuses them.»
replace-with: and the terminal fleet console (matrix#v2-14) reuses them
