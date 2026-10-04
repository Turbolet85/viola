
## 2026-10-04-windows-boundary-mutation-workflow — the third workflow under CI integration
**Section:** §Dependency Security → CI integration (the workflow list, a new `windows-mutants.yml` bullet, the concurrency note, the event-payload bullet)
**Change:**
- The CI-integration list gains `.github/workflows/windows-mutants.yml`.
- New bullet: `workflow_dispatch` only, no `inputs:` (no event-payload input class), at the epoch-boundary audit, never a gate or a `ci.yml` dependency. Workflow `permissions: {}`, job `contents: read`. Checkout and install-action are SHA-pinned with `persist-credentials: false` and ci.yml's tool pins; `matrix.*` reaches the step only via `env:`. No secret, cache, `needs:` or upload: the job log carries repo-relative documents only, and the unscanned-upload list is unchanged.
- Concurrency note: `concurrency-limits` was "(2 low findings)"; now 3 low, one per workflow (zizmor 1.30.1, dev host, 2026-10-04). "CI runs no mutation job" is now: no push or pull-request run carries one, and the workflow uploads nothing.
- Event-payload bullet: no workflow reads a `github.event` value; `ci.yml` runs no mutation job; the workflow's `--package` arm reads no base.
**Why:** founder ruling C2 (2026-10-04). Not a boundary widening: no permission, secret, input class, upload or action pin is added.
**Ref:** .andromeda/runs/2026-10-04T04-08-06-wrap/
