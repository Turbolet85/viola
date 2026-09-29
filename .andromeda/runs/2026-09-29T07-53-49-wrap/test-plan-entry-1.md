
## 2026-09-29-verify-stamped-test-homes-and-harness — stamped root seam, stop waits for the endpoint, boot readiness line 3
**Section:** §3 `boot` Readiness signal (staged checks; `events.ndjson` bullet); §3 `run` step 2 (the root fixture chain; `WITHIN`)
**Change:**
- Root `stamped_home` runs `viola --home <h> verify -- <fake agent> --cli-version 2.1.283 --fixtures <root>/fixtures/claude`; `stamped` is true only on exit 0 and a last line `stamped 2.1.283  <n> pass  0 fail`, else the fixture panics with codes. Was "an interim seam (`stamped:false`) until `viola verify` exists". `StampedHome::unstamped(TestHome)` serves the unverified-path tests; `Wrapper::boot` always passes `--cli-version 2.1.283`, `--fixtures` stays per-test.
- `Wrapper::stop` / `stop_keep` wait for the exit and then for the recorded endpoint to be unconnectable (`wait_endpoint_gone`, the harness `endpoint_gone` rule); 9 root waits share `WITHIN` (was 8). An exit code is not the endpoint gone: on Windows the stopped wrapper's own pipe took a hook 25 ms after its exit line, as measured at ci#36532038635; why is recorded, not established.
- `boot` readiness checks `events.ndjson` lines 1-3 through `start_records` after snapshot, endpoint and heartbeat (missing `<name>:events`); was "`boot` checks lines 1–2 today".
**Why:** stamps come only from `viola verify` against the fake agent (§7 Seed strategies); the folded case_08 red showed "stopped" meant an exit code, not the endpoint gone. The pipe-name-collision hypothesis was falsified (the name carries the home).
**Ref:** .andromeda/runs/2026-09-29T07-53-49-wrap/
