# Live starts — 2026-11-16-survey-key-keeps-the-wheel (P3's end, before P4; `inputs#I1` item 2)

**Ask: 4 live `claude` 2.1.325 starts, every key typed only by the rig into its own pty.**

**How the survey is made to appear.** Static read of the installed 2.1.325 package, not yet seen live (research
Measured facts 5–7):
- The env var `CLAUDE_FORCE_DISPLAY_SURVEY=1` shows the survey once in a config dir that has never shown it. Each
  start uses a fresh private config dir, so each start can show it once.
- The first prompt is given as a command-line argument (`claude "<synthetic prompt>"`), so the rig types no
  prompt.
- What still stands outside this control: the account's `allow_product_feedback` policy, and further gates in the
  bundle I could not read.
- **If it does not appear:** after 120 s with no survey literal on screen, the rig stops the session with Ctrl-C and
  types no key. The start still counts against the cap. No literal joins the list, `v2-21` stays unclaimed, and the
  chunk stops and reports, asking nothing more.

**The rig.** The headless pty host from 2026-11-15 (`live-pty.py`'s form: its own pty, no compositor or window, a
declared environment with no inherited `CLAUDE*` or `VIOLA*`).
- **Folders:** one 0700 folder per start under `/tmp` (`/tmp/viola-survey-<pid>/`), outside every repository. It
  holds the start's working folder, a private `CLAUDE_CONFIG_DIR` and, for S4, a scratch viola home.
- **Screens:** the screen stays local in memory. What lands is closed words per start: shown yes/no, the prompt and
  options-row literals each present or absent, `ms` from turn end to the survey appearing, the input box holding the
  digit yes/no, cleared yes/no, a new turn yes/no. Nothing of his status line or config enters a commit.
- **The login premise:** a fresh private config dir starts **logged out**, because the CLI keeps its login in
  `<config dir>/.credentials.json`. It would also show first-run screens. One of these is his word (fork L below).

| start | what runs | key the rig types | what it reads |
|---|---|---|---|
| S1 | `claude "<synthetic prompt>"`, not wrapped | `0` alone, once, after the survey has stood 1 s | the pause path: the digit shows in the input box, the survey closes, the input box clears, no turn starts |
| S2 | the same | `0` then Enter | the Enter path: the same reads, and whether Enter submits an empty prompt |
| S3 | the same | `3` (his V158 key), then nothing | a rating's path, and whether the transcript follow-up (`y/n/d`) or the free-text follow-up appears; left to its timeout, no further key |
| S4 | after implement, on this chunk's build: `viola run` in a scratch home whose `claude_env_keep` lists `CLAUDE_FORCE_DISPLAY_SURVEY` and `CLAUDE_CONFIG_DIR` (neither is on the identity floor); the prompt sent by `viola send` | `0` alone | `events.ndjson`: no `wheel{human,human-input}` after the key; the driver's next `viola send` is confirmed, not refused `human-typing` |

**Keys the rig types, all his word:**
- `0` (S1, S2, S4), Enter (S2), `3` (S3).
- Fork L may add a login (his own) or trust and first-run keys.
- No key is ever typed into a modal or into the transcript follow-up.

**Residuals:**
- A dismissed survey (`0`) and a rating (`3`, S3) are reported to Anthropic as product feedback from his account.
  S3 sends a real "good" rating.
- Each start writes the CLI's own files only inside its private config dir, which the rig deletes after the start.
- No `viola verify` live run is added here: the 23rd ledger row's live re-stamp rides the CARRY already standing on
  «The 2.1.325 recorded fixture set».

**Fork L, the login (his call):**
- (a) The rig copies `~/.claude/.credentials.json` into each private config dir (0600, on tmpfs) and deletes it
  with the folder. It also seeds the folder's trust and `hasCompletedOnboarding` in that dir's own `.claude.json`,
  so no trust or first-run key is typed.
- (b) He logs in once per start himself.
- (c) The rig uses his real config dir, which is not private (against the directive's letter).

**Fork R, how far to go (his call):**
- S1 + S2 + S4 measures the Dismiss key on both paths and proves the wheel. Only `0` then joins the list.
- Adding S3 lets `1`–`3` join too, and shows what follows a rating.

**What the chunk loses without the starts.** Nothing can join the list (his rule, 2026-11-08: only once measured
live), so `v2-21` stays unclaimed. At most the chunk lands the dormant mechanism: the screen-state bridge to the
wheel and its fail-toward-the-human tests on the fake agent, with an empty survey signature set, so no key changes
behaviour. Without S4 the end-to-end wheel proof is the fake agent's only, which is never proof of the real CLI
(`decision/fake-agent-not-proof-of-real-cli`).

**Everything else is planned on the fake agent and the recorded payloads meanwhile:** unit tables, outer-PTY
`tui_wheel` cases, the ledger-row check, each with bounded waits.
