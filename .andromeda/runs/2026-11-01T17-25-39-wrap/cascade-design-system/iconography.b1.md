### iconography#1.1 · values · icons · ## Iconography · lines 8–8
- **Transfer arrows.** On the web, `→` (U+2192) marks the outbound marker (blue, with the name), `link` / `unlink` tape lines, and the target of a `send` tape line (`send  → builder`). In tape lines the arrow is buff and only link names are blue. `←` (U+2190) marks only the inbound twin (`← overseer`, arrow and name in blue). The CLI uses ASCII `->` and has no inbound form. `viola list` carries no link column in v1, so the six columns shared with the web strip stay fixed. Links print only as `link` / `unlink` verb output (`overseer -> builder`).
### iconography#1.2 · values · icons · ## Iconography · lines 9–9
- **Readback strike.** A CSS `linear-gradient` inside the box.
### iconography#1.3 · values · icons · ## Iconography · lines 10–12
- **Separator.** Middle dot `·` on the web. The CLI is ASCII only:
  - Two spaces separate columns, and a refusal's reason from its detail (they are separate fields, as in the exit-code table).
  - ` - ` (space, hyphen, space) replaces `·` inside one field or label: `driver - budget-paused`, `skipped 0 - 0 - 0`, `-- UNWRAPPED - READ-ONLY --`.
