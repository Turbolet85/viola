# Founder forks — chunk 2026-11-15-api-error-turn-end (P4), for his return

The founder is away (inputs#I2). The overseer answered both forks below PROVISIONALLY, as the founder's delegate. Each
is a boundary widening (playbook «Boundary widening», `rul-r74hshif` / `rul-p2zl2kgl`), owed his own word. Nothing
here is answered by him yet.

## F1 — The hook surface admits `StopFailure` (a new event on the hook stdin surface)
The chunk registers the CLI's `StopFailure` hook (`plugin/hooks/hooks.json`) as `viola hook stop-failure`. Its payload
is read by `viola-agent-claude` and becomes a `turn-ended` line that clears the stuck running turn. The payload's
`error` value is classified; its `last_assistant_message` and `error_details` (the API-error text) are dropped at the
parse.
- Taken provisionally: register it (the route entry's own purpose), with no ledger row and no verify probe. One scratch
  live reading (`claude -p` under a model name that does not exist, a capture plugin, a private 0700 dir) reads the
  payload on 2.1.325. The connection-lost end is not read live: a named exception owned by «The 2.1.325 recorded
  fixture set».
- Options he may pick instead, in order:
  1. (as taken) register, no row, one scratch reading.
  2. Register with a 22nd relied ledger row `stop-failure` and a `viola verify` probe (a print-mode start under an
     unknown model). Cost: one full verify round per CLI version (7 session starts on 2.1.325), every older stamp
     reading `cli_verified:false` until re-verified.
  3. Do not register StopFailure: the stuck turn stays a human `pause` + `release` (today's behaviour).

## F2 — The `cause` word crossing into `events.ndjson`, `wait --json` and MCP `wait`
- Taken provisionally: `turn-ended` carries `cause: "api-error"` on an API-error end only; a Stop's line is unchanged.
  The closed set lives in one place in viola-core. The CLI's `error` class is computed and tested inside
  `viola-agent-claude` but crosses into no event, snapshot or log line.
- The granular list, the overseer's lean for him (marked recommended by the phase at P4):
  `rate-limited` (rate_limit) · `overloaded` · `auth-failed` (authentication_failed, oauth_org_not_allowed,
  account_on_hold, verification_required, cloud_credential_error) · `billing` (billing_error) · `invalid-request` ·
  `model-not-found` · `server-error` (connection lost included) · `output-limit` (max_output_tokens) · `api-error`
  (unknown, and any value outside the map).
  - Its cost if he takes it later: an additive change. New members join the one viola-core set; the mapping already
    exists in `viola-agent-claude` and only stops collapsing to `api-error`. The wrapper's re-validation admits the
    new words, and the obs/architecture rows naming the closed list are amended. No reader breaks: an older reader
    counts no unknown key (the key is the same), and a word it does not know is a plain string it carries.
  - Its gain: a driver can tell «wait and retry» (rate-limited, overloaded, server-error) from «needs a human»
    (auth-failed, billing, model-not-found).
- Other options shown at P4: every turn end carries a cause (`completed` on a Stop). Not taken: it changes every Stop
  line and pinned key set.
