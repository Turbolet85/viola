# Forks — 2026-11-16-send-confirmation, phase P4

The operator is away (directive `inputs#I1`, item 6): no dialog is raised. Q1 is the founder's, a boundary widening
under the playbook's escalate rules `rul-r74hshif` and `rul-p2zl2kgl` (a validated surface admits a new input
class); a delegate's answer to it lands PROVISIONAL, the founder's own word owed. Q2 is technical, answered by relay
at once. The phase stops here until the answer arrives; it is appended below, snapped and cited.

## Q1 (founder's) — how far the send's claim tolerates whitespace

Today `SendSlot::claim` (`src/run/send/slot.rs:83`) relabels a `prompt-submitted` `driver` only when its text equals
the send's typed text exactly; the security plan states it as «no tolerance on the submitted side»
(security-plan · decision/delivery-confirmation-exact-match). On claude 2.1.296 a send ending in a space was
submitted trimmed, missed the claim, was filed `human`, took the wheel, ended exit 13 and stranded every later send
until a `release` (`inputs#I1`). No recorded fixture set shows what the CLI trims (none ends a prompt in whitespace;
no 2.1.325 set exists). Matrix `v2-18` asks that such a send is confirmed as the driver's own. Every option keeps
the typed text as it is (nothing stripped), refused characters refused, the local-command list exact, harness
classification on the raw start, and a human's own different prompt still taking the wheel.

1. **(Recommended) Whitespace at either end.** The claim takes `S` when `S == T`, or when `S` is not empty and `T`
   is `S` with only whitespace before and after it (Unicode whitespace, `char::is_whitespace`). Why it wins: it
   holds whatever end trim the CLI does on any build, the untrimmed text included, so no send relies on the trim
   and no ledger row or live reading is needed (architecture decision/no-probe-for-unrelied-shapes); a prompt
   differing anywhere inside still claims nothing. Trade-off: the new admit is a prompt equal to the in-flight
   send's text up to end whitespace, filed `driver` inside that send's 10 s window.
2. **Trailing whitespace only.** The claim takes `S` when `S == T`, or `S` is not empty and `T` is `S` followed by
   whitespace only. The measured 2.1.296 shape alone, the narrowest widening. Trade-off: a send opening with a
   space or a newline still misses if the CLI also trims the start (unmeasured), and v2-18's «whitespace the CLI
   trims» is then claimed for trailing whitespace only, the leading case named in its notes.
3. **Keep the exact match.** No widening; v2-18 stays pooled, unclaimed, and the chunk builds the race fix alone.
   Trade-off: the stranding instance in `inputs#I1` stays possible on every build that trims.

## Q2 (technical) — what v2-19 is claimed as

`v2-19`: «A turn that starts while a send waits on the readiness gate never receives the send's text: the send is
refused as a running turn or typed only once that turn is over, and a test reproduces the race.» The gate's own
wait is already covered (send.rs:121-130, cases at send.rs:606-635). The open window is after those reads: the link
record, `send-issued` and the paste follow with nothing re-reading the turn. The plan closes it by making the last
wheel and turn read, the link, `send-issued` and the paste one section under the flight lock, which the hook's turn
start also takes (slot.rs:79, :203), and reproduces the race with a forced-window parameter on a private function
(test-plan decision/forced-window-hook-is-test-parameter), held by a remove-the-guard control. What the wrapper
cannot see is a turn the CLI started whose `UserPromptSubmit` hook has not yet reached it (the gate reads the input
box, which a turn's screen keeps, screen.rs:664; that hook latency is unmeasured).

1. **(Recommended) Claim v2-19 concretized to what the wrapper can know**: «a turn whose `prompt-submitted` reaches
   the wrapper at any point before the paste refuses the send `not-delivered` / `turn-running` with nothing typed,
   no link and no `send-issued`; a unit case reproduces a turn starting after the post-gate reads and fails with
   the guard removed». A dated notes line names the residual (the hook-latency window) as unmeasured, with no owner
   minted. Why: it proves everything in-process; the residual is a property of the CLI's hook timing.
2. **Leave v2-19 pooled** and land the fix with the same test, unclaimed, until the hook-latency window is measured
   by a live reading. Trade-off: the closable part of the race is fixed and tested but the cap reads open.

## Answer

Relayed 2026-11-16 by the operator from `~/dev/projects/additional/viola-overseer/relay-send-confirmation-forks.md`,
the delegate's answers (the founder away), verbatim:

> # Relay — the two forks of «Send confirmation», the delegate's answers (the founder is away)
>
> - **Q1 = trailing whitespace only. PROVISIONAL.** It is the smallest widening that closes the MEASURED instance (a
>   send ending in a space, the CLI submitting it trimmed, 2026-10-19 on 2.1.296); what the CLI does to LEADING
>   whitespace is not measured on any version, and a tolerance for it is the founder's to give. Build it so the rule
>   lives in ONE place and «either end» is a one-line additive change. The text is validated as received and never
>   stripped: what is logged and compared is the sent text and the submitted text as they are; only the equality
>   tolerates the trailing run. Say in the plan which characters count as whitespace for this rule and why (the set
>   the CLI's trim is known or read to use — if that is not readable from the bundle, say so and take ASCII space,
>   tab, CR and LF, named as an assumption).
>   Write to founder-forks.md, for his return: «either end» as the candidate, with the overseer's lean for it once a
>   live reading shows what the CLI trims; and add a Beyond-the-plan Carry-forward on «Standing live rig with a screen
>   model»: read live, on the interactive input box, exactly which whitespace the CLI trims at submit (leading,
>   trailing, inside a paste).
> - **Q2 = claim v2-19 for what the wrapper can see, the hook-latency gap named as a residual** (your recommendation;
>   the overseer's technical answer). Name the residual's owner on the card.
> - The three decisions taken without a fork stand: no capability-ledger row, no new refusal for whitespace-only text,
>   the race closed with a lock rather than a new test seam. For the lock: say on the card what it holds across (link,
>   `send-issued`, paste), what a second sender sees while it is held, and the test that fails without it — bounded,
>   per the directive's item 4.
> - No live start: agreed; the cap for this chunk is 0.

Authority: Q1 `delegate overseer`, PROVISIONAL — a boundary widening no founder word covers; the founder's own word
owed. Q2 the overseer's technical answer.

## For the founder's return

- **Candidate: «either end».** The claim takes `S` when `T` is `S` with whitespace only before and after it, not
  only after. The overseer leans to it once a live reading shows what the CLI trims at submit. The plan builds the
  rule in one function so the change is one line, additive.
- **Owner of that live reading:** «Standing live rig with a screen model» (Beyond-the-plan row of the P5 card): read
  live, on the interactive input box, exactly which whitespace the CLI trims at submit (leading, trailing, inside a
  paste).
