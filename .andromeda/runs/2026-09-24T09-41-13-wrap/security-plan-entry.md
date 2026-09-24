
## 2026-09-24-supply-chain-and-workflow-gates — trust boundary, CI jobs, deny.toml additions, nightly.yml
**Section:** §Threat Model Summary → Supply chain (Trust boundary) · §Architecture Overview → CI/CD · §Dependency Security (`deny.toml` additions, CI integration)
**Change:**
- Trust boundary: now names `deny.toml` with its four families plus the sole-root `deny-sync.toml` tokio ban.
- CI/CD: adds `nightly.yml` and the ubuntu supply-chain job.
- `deny.toml` additions: the heading no longer says arch lists only licences and bans. The arch-bans bullet places the tokio ban in `deny-sync.toml` per sync crate as sole root, and records that `scripts/deny-probes.sh` proves every ban live.
- CI integration: the weekly advisory run is a separate workflow, `nightly.yml` (weekly `schedule` + `workflow_dispatch`, no cache), not a trigger on `ci.yml`.

**Why:** chunk 2026-09-24-supply-chain-and-workflow-gates. The fan-out had 0 proposals from this doc's detectors, which all held. Its return flagged these sites as restatements of the claims the pass retires, so the orchestrator raised them as routine cascade dependents. Sweep: see architecture-amendments.md, same entry heading. For this master, 5 sites were amended (:134, :161, :308, :314, :327). The pinned-action set was left unchanged, because no new action was added.
