# Review feedback 1 — 2026-09-24 (overseer, founder-delegated; pasted by the user)

(1) Tier Comprehensive stays, thresholds as in the tier table.
(2) Accept viola verify against the FAKE agent in CI; arch "only locally" means the real CLI. Log it as an arch amendment request in the Decisions Log.
(3) Keep 1.0 s provisional; log send-confirmation window and `viola ui --port 0` as arch requests.
(4) Node on the test side only: accepted.
(5) Add coverage (cross-plan audit of design vs arch/security):
  (a) driver-facing hints for human-typing / budget-paused must never suggest `viola release` (drivers are refused it, -32602 exit 20);
  (b) exit 21 and exit 1 each have several causes (strict-mode/server-verify fail; squatted name, SHA-256 mismatch, .cmd/.bat child): one negative test per cause, and the printed hint must match the cause;
  (c) budget.paused with a per-instance override;
  (d) unwrapped names containing \n or \t keep the `list` row fixed-width.
Then proceed.
