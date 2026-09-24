# design extract

## No domain coverage
This chunk is CI and supply-chain policy only: deny.toml, cargo-deny families, zizmor, the tokio-free check, the weekly advisory cron and workflow permissions. It renders nothing, so no design-system tokens, typography, motion, iconography or component patterns apply. Two points in the plan are close but belong elsewhere. The CLI styling needs no extra dependency (design-system §Toolkit / Framework, "hand-written SGR module… adds no dependency"). The CI image must provide the DejaVu fonts for the Linux GUI render (design-system §Typography, "Assertions hold on the Linux fallback"). Both are for the headless GUI and CLI chunks, not this gate chunk. There is no amendment history: design-system-amendments.md does not exist, which is normal.
