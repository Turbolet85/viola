# Phase 3.5 review feedback — round 1 (2026-09-23)

User decision, verbatim:

Accept items 1-5 and keeping the Auth & Authz section, with one precision and four answers.
Precision on item 3: multi-line text is normal (relays, plans, free-text answers are multi-line, brief 4.1 S1), so the refusal must allow LF, CR and TAB and refuse every other C0 control, DEL and C1 (ESC above all). Name that exact set in the plan.
Q1 Windows pipe ACL: the user SID + SYSTEM only, not the logon SID. A driver and a driven session in different logon sessions of the same user (an SSH or scheduled-task driver later) must still connect, and the boundary v1 promises is the OS user.
Q2 macOS: accept uid + folder permissions without a pid check. macOS is CI-only in v1 (O5 unmeasured), so record it as a known gap for the public version.
Q3 hash: make <hash> a truncated SHA-256 of the exe bytes and state it in the plan. It costs nothing, and it removes the accidental-only caveat. Tampering by the same user stays outside the v1 boundary.
Q4 16 MiB: keep it as the default and add a capability-ledger measurement of the largest hook payload seen, so the cap is checked against data rather than a guess.

## Changes to apply to security-plan-draft.md

1. Accepted as drafted: GUI per-launch cookie in v1, Unix per-user 0700 socket directory, `control-character` refusal detail, `release`-with-`from` refused, `MAX_FRAME` = 16 MiB; Auth & Authz section kept.
2. `validate_paste_text` exact set: allow LF (U+000A), CR (U+000D), TAB (U+0009); refuse every other C0 (U+0000–U+001F), DEL (U+007F) and C1 (U+0080–U+009F), ESC (U+001B) above all. Checked on decoded `char`s, not raw bytes.
3. Windows pipe SDDL: user SID + SYSTEM only; logon SID explicitly rejected (open question closed).
4. macOS client verification: euid + 0700 directory, no pid check — accepted; recorded as a known gap for the public version.
5. `<hash>` = truncated SHA-256 of the exe bytes; re-hash check on reuse. Same-user tampering stays outside the v1 boundary.
6. `MAX_FRAME` default 16 MiB kept; add a capability-ledger measurement (largest hook payload seen, recorded by `viola verify`) to check the cap against data.
