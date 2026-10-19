
<!-- detail: values · dec-hook-contract -->

- At the hold's end the wrapper also appends one `dialog-expired` line (`source: wrapper`, `{dialog_id, dialog_kind}`, no question, plan or tool text) through `WaitFeed::appending`, so a parked `wait` wakes and the driver learns the dialog is gone; a hand-back or an answer appends none, and a failed append changes nothing else (its log lines: obs-plan §4, Scenario: Dialog → `answer`).
- A dialog hook held 671 s returned the driver's `viola answer` decision into claude 2.1.287 on the Linux dev host, as measured at `viola-0.1.0/chunks/2026-10-19-dialog-hold-for-the-driver/evidence/live-round.md`.

<!-- /detail -->

<!-- detail: pending · dec-hook-contract -->

- A hold near the 3600 s ceiling is first read live at "Overseer switch to viola", and the long hold's Windows leg at "Windows-only live measurements"; the ceiling stays PROVISIONAL until then.

<!-- /detail -->

