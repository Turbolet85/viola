## Expression Level

### Base level

**Recommended base:** 0.2

**Reasoning:** viola is a hybrid local developer tool. Its audience is one founder working as the operator between two Claude Code sessions, and its main automated callers are LLM driver sessions. Both want state they can read at a glance and predict. Neither wants choreography, so the base goes below the ≤ 0.4 dev-tool ceiling. The "Strip-bay handoff, read back" personality is standard phraseology: short, fixed, read back, no filler. In a strip bay the only motion is physical and it means something. A strip slides once at handoff, or it gets cocked out of line. Nothing pulses, and the Q3 mood rule "nothing glows unless a lamp is on it" says the same. So motion is allowed only when it carries a state change, and it plays once and stops.

The security tier is Minimal (0). There is no credential entry (the one-time `?t=` token is exchanged for a cookie, with no login screen), so security adds no motion ceiling of its own. The CSP does limit how motion can be built: no `style="…"` attributes, no inline `<style>`, and no vendored animation library, because cargo-deny cannot audit JS. All motion is therefore CSS `transition` on class or attribute toggles in `/assets/app.css` or Lit `static styles`. Scale intent is personal: loopback only in v1, a public release in v1.x, and later a phone view of the same page. The page stays open for hours beside two terminals, so a low base also rules out any ambient or looping motion that would wear on the eye over a long watch.

### Per-surface adjustments

| Surface | Expression | Delta vs base | Reasoning |
|---|---|---|---|
| web-spa (`viola ui`, 127.0.0.1, view-only v1) | 0.3 | +0.1 | This is the only surface where the metaphor's two physical motions can show, so it gets one step above base. (1) When `dialog_pending` turns on, the whole `<viola-session-row>` cocks: a single `translate` of about 12px over about 160ms ease-out, with the amber `#D97706` holder mark and the text label. The row keeps its rack slot and there is no loop or pulse. (2) A new `driver → driven` link marker or a new feed line appears with at most a 120ms opacity fade. The ceiling is Linear-level 0.3 and never reaches 0.5. There are no skeletons (`unknown` is printed in its box, following the anti-"Overclaiming" anchor), no spinner or pulse for `busy`, no staggered entrances, and no re-sort animation, because order is stable. The readback close is instant: a `prompt-submitted` pair changes from outline to filled strip with no transition when its `turn-ended` or typed refusal lands, because acknowledgement is a discrete fact, not a gradual one. `stale` dims instantly. Hover effects are limited to focus rings and link underline, since v1 has no controls. Under `prefers-reduced-motion: reduce` every duration goes to 0 and the offset position and amber mark remain, so the state is still fully readable. |
| cli, human TTY output (`list · last · wait · verify · link …`) | 0.2 | 0.0 | Follows k9s-style fixed-column rows but stays below the cargo/gh 0.3 tier. Status words get colour only as a second cue (amber `DIALOG`, dimmed `stale` rows) and always keep the text word. There is a one-line static context header (budget `five_hour`/`seven_day` %, reading age). No live-redrawing dashboard. `viola wait` prints one static "waiting: <session>" line and a result line instead of a spinner, following the readback rule of stating the outcome once. |
| cli, `--json` / non-TTY / `NO_COLOR` / `viola run` passthrough | 0.0 | -0.2 | LLM driver sessions are the main callers, and `viola run` must print nothing while the wrapped `claude` TUI runs. Any ANSI escape or spinner frame here is noise that corrupts the parse or the child's screen. This output uses no colour, no spinner, no cursor control, and only typed exit codes. |

## Recommended

**Final expression map:**
- Base: 0.2
- web-spa (`viola ui`): 0.3 (0.0 under `prefers-reduced-motion`; the offset and amber state remain)
- cli, human TTY: 0.2
- cli, `--json` / non-TTY / `NO_COLOR` / `viola run`: 0.0

**Reasoning:** This is a personal, Minimal-tier local developer tool watched for hours by one developer and read by LLM callers. The "Strip-bay handoff, read back" personality means motion must stand for a state change (a strip cocking for `dialog_pending`, a link or feed line arriving) and play once. The same thinking rules out motion that implies progress or success before it is confirmed: no spinners, no pulses, no animated "done". Implement should add no motion dependency (no framer-motion, GSAP, Lottie or Motion One). Everything is two or three CSS transitions of 200ms or less under hand-written CSS, which also keeps the page clean under the CSP (`style-src 'self'`, no inline styles).

**Research basis:** WebSearch 2026-09-23:
- 2026 motion trend toward restraint and purpose; 150-200ms for small state changes; 8-12px offsets over 20px slides. Sources: [MotionKit, "Web Animation Trends 2026"](https://motionkit.io/blog/web-animation-trends-2026), [Vibe Coder Blog, "Animation Patterns With Framer Motion and AI in 2026"](https://blog.vibecoder.me/animation-patterns-framer-motion-ai), [WebPeak, "CSS / JS Animation Trends 2026"](https://webpeak.org/blog/css-js-animation-trends), [Acodez, "Micro-Interactions & Motion Design 2026"](https://acodez.in/micro-interactions-motion-design/).
- `prefers-reduced-motion` treated as table stakes and applied to dashboard layouts by default. Sources: [MDN prefers-reduced-motion](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/At-rules/@media/prefers-reduced-motion), [W3C WCAG 2.2 Technique C39](https://www.w3.org/WAI/WCAG22/Techniques/css/C39), [schemavaults/ui PR #117](https://github.com/schemavaults/ui/pull/117).
- Agent-friendly CLIs drop colour, spinners and decoration when not attached to a TTY, and honour `NO_COLOR` (several GitHub issues from Sept 2026). Sources: [Trevin Chow, "7 Principles for Agent-Friendly CLIs"](https://trevinsays.com/p/7-principles-for-agent-friendly-clis), [Better CLI, colours in output](https://bettercli.org/design/using-colors-in-cli/), [Heroku CLI Style Guide](https://devcenter.heroku.com/articles/cli-style-guide), [kelp issue #68](https://github.com/Mic52M/kelp/issues/68), [Vex issue #36](https://github.com/wavefnd/Vex/issues/36).
- The strip-bay motion vocabulary (the cocked-strip offset, the single slide at handoff, no re-sort) comes from the run's research-q2.md, citing the vNAS vStrips manual and FAA TFDM EFS. The exact ms and px values are my own calibration, based on training data (2026).
