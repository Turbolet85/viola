@leaf_group group=architecture-2
@leaf_read leaf=.claude/docs/services/viola-pty.md lines=47
last: - `doc:architecture` [PTY] · `.claude/docs/gotchas.md` (ConPTY, p
@leaf_read leaf=.claude/docs/services/viola-state.md lines=49
last: - `doc:architecture` §Standard Contracts (snapshot envelope, in
@leaf_read leaf=.claude/docs/services/viola.md lines=58
last: - `doc:architecture` · `.claude/rules/api.md` · `.claude/rules
@leaf_edit n=1 leaf=.claude/docs/services/viola-pty.md line=14 class=stale
leaf-text: `viola verify`'s five interactive PTY runs
source: dec-uczlwirj «viola verify makes one print-mode probe, six interactive PTY runs under the R8 strip at a fixed 80×24, and one agents --json read inside Run B, pasting only compiled texts.»
replace-with: `viola verify`'s six interactive PTY runs
@leaf_edit n=2 leaf=.claude/docs/services/viola.md line=6 class=stale
leaf-text: then the five interactive PTY runs (`src/cmd/verify/typed.rs`
source: dec-uczlwirj «viola verify makes one print-mode probe, six interactive PTY runs under the R8 strip at a fixed 80×24, and one agents --json read inside Run B, pasting only compiled texts.»
replace-with: then the six interactive PTY runs (`src/cmd/verify/typed.rs`
@leaf_edit n=3 leaf=.claude/docs/services/viola.md line=6 class=stale
leaf-text: then last Run E trusted in `-resume/`
source: dec-tvibhd5b «The resume ledger row checks Run E's first SessionStart for source resume and that same id, and the six runs execute in order A to F.»
replace-with: then Run E trusted in `-resume/`
@leaf_edit n=4 leaf=.claude/docs/services/viola.md line=6 class=omission
leaf-text: empty input, no `--settings`, ended by a kill)
source: dec-f67jf527 «The trust-record relied ledger row asks, per CLI version, whether a trust record written for an untrusted folder opens it with no modal; verify's Run F, after Run E, passes it only when the record was written»
replace-with: empty input, no `--settings`, ended by a kill), then last Run F, after Run E, starting in an untrusted folder for which verify wrote a trust record (the `trust-record` row passing only when the record was written, Run A showed the trust modal, and Run F's start shows no modal and the input box; verify writes that record for its own temp folder only into the Claude config file its child reads, located through `CLAUDE_CONFIG_DIR` only when `claude_env_keep` keeps it, never creating an absent file, and removes that one record after; `run` and `revive` write no trust record)
@leaf_edit n=5 leaf=.claude/docs/services/viola.md line=6 class=omission
leaf-text: its drop guards removing only the dirs that run created
source: dec-wc3ygufa «viola verify removes its probe folders whole, and its own trust record from the Claude config file, when it passes, fails or is stopped by Ctrl-C or TERM, only what that run created, never by a pattern or a listing.»
replace-with: its drop guards removing only the dirs, and its own trust record from the Claude config file, that run created
@leaf_edit n=6 leaf=.claude/docs/services/viola.md line=35 class=stale
leaf-text: each of the five interactive PTY runs (Run A, Run B, Run C, Run D, then Run E)
source: dec-uczlwirj «viola verify makes one print-mode probe, six interactive PTY runs under the R8 strip at a fixed 80×24, and one agents --json read inside Run B, pasting only compiled texts.»
replace-with: each of the six interactive PTY runs (Run A, Run B, Run C, Run D, Run E, then Run F)
@leaf_edit n=7 leaf=.claude/docs/services/viola.md line=35 class=stale
leaf-text: `verify`'s eight child spawns are each logged
source: dec-uczlwirj «viola verify makes one print-mode probe, six interactive PTY runs under the R8 strip at a fixed 80×24, and one agents --json read inside Run B, pasting only compiled texts.»
replace-with: `verify`'s nine child spawns are each logged
