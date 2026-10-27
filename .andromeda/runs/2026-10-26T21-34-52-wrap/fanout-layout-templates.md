proposals:
  - detector: D-design-cli-wording
    severity: warning
    section: Surface: cli → Component — Primary content block 2: refusal lines and the `unable` column (Line form)
    change: The `viola link` / `viola unlink` exit-21 strict-modes clause is widened to the six frame verbs: `viola send`, `wait`, `last`, `answer`, `pause` and `release` (and `link` / `unlink`, the driven instance) run the instance's strict-modes check first (`send` after its text read), before any frame, opening nothing under the home and writing no line to any log; a refusal is exit 21 with `revive`'s instance pair on stderr through `human::refuse` (`unable: <name>'s state files can be written by another user` / `hint: viola will not read them; make the viola home and its files owner-only`), stdout empty, the directory's mode left as found; under `--json` `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty (chunk 2026-10-26-home-and-code-bearing-file-integrity; the founder's own answers, P4 Q2 and Q3, `inputs#I3`).
    sidecar: Block 2's exit-21 strict-modes instance pair now names the six frame verbs beside link / unlink (chunk 2026-10-26-home-and-code-bearing-file-integrity, founder P4 Q3).
    rationale: Report Changes → Symbols (root bin `src/cmd/client.rs` `refuse_untrusted`, called first by send, wait, last, answer, pause, release; link_call moved onto it) and Counts 8 (instance-pair, layout-templates.md:329 @c1806/@c2291/@c2326 → extend (six verbs)); Expected amendments names «Block 2's fixed-form list gain the six frame verbs' instance pair».
    basis: src/cmd/client.rs:20-41
  - detector: D-design-cli-wording
    severity: warning
    section: Surface: cli → Component — Primary content block 2: refusal lines and the `unable` column (Exit codes as the typed tail)
    change: Exit 21's parenthesis reads that for `viola list`, `viola plugin install` and the strict-modes refusal of `viola send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` the fixed state pair above prints instead of the `unable  instance-unreachable` line.
    sidecar: Exit 21's fixed-state-pair exception gains the six frame verbs' strict-modes refusal (chunk 2026-10-26-home-and-code-bearing-file-integrity).
    rationale: Same moved fact as the primary (report Counts 8, Symbols `refuse_untrusted`); the exit-code row restates which verbs print the state pair at exit 21 and still names only list, plugin install and link / unlink; the report's Expected amendments names «the exit-21 row».
    basis: src/cmd/client.rs:20-41
    dependent-of: D-design-cli-wording
  - detector: D-design-cli-wording
    severity: warning
    section: Surface: cli → Component — Primary content block 2: refusal lines and the `unable` column (Line form)
    change: The fixed-message exit-1 exceptions gain the home-tree refusals: `viola verify` checks its home tree (the home, `ledger/`, `bin/`, `plugin/`) before its log and on a refusal prints the home pair (`unable: the viola home is not private to you` / `hint: the viola home must be readable only by you; viola does not change its permissions`), exit 1, stdout empty, no step line and no summary, and a stamps write its strict-modes check refuses prints the same pair; `viola run`'s start checks the home tree before its wrapper log and on a refusal prints the home pair for a home-level refusal or `revive`'s instance pair for an instance-level one, exit 1, no log opened, no line, no child.
    sidecar: Block 2's exit-1 fixed forms gain verify's and run's home-tree refusals (home pair; run's instance pair for an instance-level refusal) (chunk 2026-10-26-home-and-code-bearing-file-integrity).
    rationale: Report Changes → Symbols (`src/cmd/verify.rs` check_home_tree before its cli log → refuse_home, home pair, exit 1, no step line, no summary; `measure`'s StampsError::StrictModes → same pair; `src/cmd/run.rs` checked_exe / refuse_tree: HomeLevel → home pair, InstanceLevel → instance pair, before open_wrapper_log, exit 1) and Counts 9 (run and verify print the home pair, run the instance pair); Block 2 enumerates verify's exit-1 refusals and names none of these.
    basis: src/cmd/verify.rs:64-70
  - detector: D-design-cli-wording
    severity: warning
    section: Surface: cli → Output structure — `viola revive`
    change: The start's refusal list gains its home-tree check (made before the wrapper log, as `run`'s): a home-level refusal prints the home pair `unable: the viola home is not private to you` / `hint: the viola home must be readable only by you; viola does not change its permissions`, an instance-level one the state-files pair, each exit 1, no log opened, no child, and under `--json` the `strict-modes-failed` document.
    sidecar: viola revive's start refusals gain the home-tree check's home pair (chunk 2026-10-26-home-and-code-bearing-file-integrity).
    rationale: Report Changes → Symbols (`src/cmd/run.rs`: run (166-172) and revive's start (revive.rs:125-135) call checked_exe / refuse_tree before open_wrapper_log: exit 1, no log, no child, revive --json keeps strict-modes-failed; HomeLevel → human::home_state_pair()); the section lists revive's refusals as its pairs and holds no home pair.
    basis: src/cmd/revive.rs:125-135
note: D-layout-surface — no new surface or region (the chunk adds refusal paths on existing verbs only); D-retired — layout-templates states none of the eight retired dated strict-modes gaps (its one gap mention, line 331's macOS server-check gap, is owned elsewhere and not retired); the home-pair rows at layout-templates.md:102 / :284 / :288 read true per the report's Counts 9 disposition.
