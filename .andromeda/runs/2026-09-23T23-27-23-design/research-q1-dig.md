## Personality Directions

_Seven new directions. None of them repeat the three from research-q1 (signal box, conn handover, calibration bench)._

### Direction 1: "Strip-bay handoff, read back"

**Physical-world metaphor:** An air-traffic-control tower strip bay. Each aircraft is a paper flight-progress strip in a holder on a rack. The controller slides a strip to the next sector at handoff and "cocks" it out of line when it needs attention. Every clearance is read back by the pilot before it counts.

**Domain anchor:** Each strip is a `web-spa` session row with a fixed set of fields: name, liveness `live`/`stale`, status `idle`/`busy`/`unknown`, wheel `human`/`driver`, CLI version + verified flag. A sector handoff is the `driver → driven` link with its `since`. A cocked strip is `dialog_pending`, which matches the creator's MulmoTerminal reference to "needs you". The readback rule is viola's own principle "every send is confirmed after the fact, never presumed". The pilot-in-command rule (the pilot may always deviate) is "the human always wins the wheel". The controller does not fly the aircraft, and viola is "mechanism, not policy".

**Voice:** Standard phraseology. It is short, fixed and read back ("Session builder, wheel human, dialog pending"), with no filler and no guessing.

### Direction 2: "Gray board, deviation-only colour"

**Physical-world metaphor:** A process-plant control room built to ISA-101. Wide consoles show a mostly gray, low-saturation plant overview. Colour appears only when a value leaves its normal band (amber warning, red alarm, blue operator action). Beside the console sits a handwritten shift log.

**Domain anchor:** For the `personal` founder audience, viola's GUI is a supervisory display that stays open for hours next to two terminals. It should be quiet until something departs from normal: `stale`, `budget_paused`, `dialog_pending`, an `unverified-cli`. Budget windows (`five_hour` / `seven_day` used %, `resets_at`, reading age) behave like process variables drifting toward limits. The negative anchor "Overclaiming" becomes an alarm-rationalization rule: nothing gets colour unless a hook or the wrapper actually observed it.

**Voice:** Restrained and calm under pressure. Normal states are said plainly and in gray, and only a real deviation earns an adjective or a colour.

### Direction 3: "Calling the book"

**Physical-world metaphor:** A theatre stage manager at the prompt-corner desk with the prompt book open. On headset they call "Standby sound 12... sound 12, GO" at exactly the right beat. The operators run the cues, the SM never rewrites the play, and anyone can call "HOLD" to stop the show.

**Domain anchor:** viola types into the driven session only at turn boundaries, which works like calling a cue on the beat and never mid-line. The prompt book is the ndjson audit trail, with every cue, hold and deviation noted against time. The SM does not direct the show, which is "mechanism, not policy". The "hold" is the human taking the wheel, and the typed refusals (`human-typing`, `budget-paused`) are the SM refusing to call a cue while the stage is unsafe. The v1.x brake (pause, unlink) is literally a show stop.

**Voice:** Crisp standby/go cadence. The warning comes before the action, confirmation comes after it, and everything is said in a low, even backstage voice.

### Direction 4: "Relay office, acknowledged tape"

**Physical-world metaphor:** A telegraph relay station. An operator receives a message on one line and re-keys it verbatim onto the next. Paper tape runs out of the teleprinter, every message gets a serial number, and nothing counts as delivered until the far end sends its acknowledgement.

**Domain anchor:** The founder is currently the manual "transport and operator" between an overseer and a builder session, which is the relay clerk's job. viola takes over that job through "our own buffer". The clerk never edits the message: the security plan renders every event field as plain text and never renders Markdown. `not-delivered` and the confirm-after-send rule are the acknowledgement protocol. The live SSE feed and `skipped` counts are like the tape itself, with unreadable groups marked rather than guessed.

**Voice:** Telegraphic and literal. Messages are numbered and timestamped, "ACK" and "NOT DELIVERED" appear exactly as sent, and there is no punctuation beyond what the facts need.

### Direction 5: "Dual-control lesson car"

**Physical-world metaphor:** A driving-school car with dual controls. The learner holds the wheel, and the instructor in the passenger seat has a second brake pedal. A mirror lets the instructor see what the learner sees, and the lesson logbook records every session.

**Domain anchor:** The creator's own words are "the wheel" and the v1.x "brake". The driver LLM session is the learner at the controls. The human is the instructor, who never grabs control without reason but always overrides instantly ("the human always wins the wheel", "human keystrokes are never blocked"). The v1 view-only GUI is the instructor's mirror. The budget governor ("ordinary, individual usage") is the speed limit the instructor watches. This is the warmest of the seven directions and suits the later audience of individual Claude Code subscribers, who hand the wheel to an agent for the first time.

**Voice:** Patient and steady, like an instructor. Short calm statements ("Driver has the wheel. Budget 42% of five-hour."), and it never sounds alarmed, even when braking.

### Direction 6: "Heard at the pass"

**Physical-world metaphor:** A restaurant kitchen's pass. The expediter stands between the dining room and the line, pins tickets to the rail, calls orders to the cooks and waits for "Heard, chef!" before treating an order as fired. The head chef can step in at any moment.

**Domain anchor:** The expediter's job is to move orders between two teams without cooking. That matches viola sitting between the overseer (front of house) and the builder running the Andromeda pipeline (the line). The ticket rail is the live event feed (prompt-submitted, turn-ended, question, permission, plan). "Heard" is the confirmed send. A ticket with no "heard" is `not-delivered`. A ticket waiting on a question is `dialog_pending`. "No dancing with a tambourine" fits a good pass: no ritual, only tickets and acknowledgements.

**Voice:** Quick and call-and-answer, but controlled. Each line is short and names the order, the station and the acknowledgement, with no chatter. It stays professional even when busy.

### Direction 7: "Behind the talkback glass"

**Physical-world metaphor:** A recording-studio control room. The producer sits behind soundproof glass and watches the performer in the live room. A red tally light shows when tape is rolling, and a talkback button lets the producer speak into the room between takes. The engineer keeps a take sheet of every pass.

**Domain anchor:** The v1 GUI is literally behind glass: view-only, loopback, the session visible but not touched ("no control accepts input"). The tally light is status `busy`. Talkback only between takes is viola typing only at turn boundaries. The take sheet is the ndjson trail. The later phone view is the producer stepping out of the room while still watching. `viola run` passing the `claude` TUI through unchanged matches the rule that the control room never alters the performance.

**Voice:** Quiet and attentive. It speaks only between takes, logs rather than comments, and uses a hushed tone that marks when the room is live.

## Recommended

**Recommended direction:** 1 — "Strip-bay handoff, read back"

**Reasoning:**
- **Closest match to the product.** It is the only one of the seven that maps all four parts of `core_functionality` one-to-one: strip = session row, handoff = `driver → driven` link, cocked strip = `dialog_pending` ("needs you"), readback = confirmed-after-the-fact send. The pilot-in-command rule also matches "the human always wins the wheel".
- **Fits the tooling.** A flight strip is a fixed-field, text-only record in a rack. That suits the `family_chosen` Web Components / Lit with no component library: one `<viola-session-row>` is one strip, built with CSS grid, system fonts and no images. So it works within the CSP limits.
- **Fits scale and audience.** It stays credible for `scale_intent` personal now and for the later phone view (portable electronic strips are an established ATC idea). Unlike the ship's-bridge option, it does not clash with the brief's use of "bridge".
- **Fallback.** Direction 2 (ISA-101 gray board) is the best choice if the user wants an alarm-management discipline rather than a narrative metaphor. It fits the negative anchor "Overclaiming" most tightly.

**Research basis:** Web searches run 2026-09-23:
- **Agent-supervision dashboards.** Claude Code Agent View (launched May 11 2026) groups sessions by "Needs input / Ready for review / Working / Completed". The broader 2026 trend is the "supervision dashboard where the user intervenes only on friction points". Sources: [MindStudio](https://www.mindstudio.ai/blog/claude-code-agent-view-manage-multiple-agents), [pasqualepillitteri.it](https://pasqualepillitteri.it/en/news/2384/claude-code-agent-view-cli-dashboard-sessions-2026), [Fuselab Creative, "Dashboard Design Trends 2026"](https://fuselabcreative.com/top-dashboard-design-trends-2025/).
- **Calm, anti-theatrical interfaces.** Sources: [Envato, "calm interfaces, transparent AI and the end of visual theatrics"](https://elements.envato.com/learn/ux-ui-design-trends), [Lucky Graphics, "Calm Interfaces 2026"](https://lucky.graphics/learn/calm-interface-audit-2026-efficiency/).
- **Utilitarian, monospace, brutalist look for dev tools.** Sources: [Setproduct 2026 field guide](https://www.setproduct.com/blog/retro-brutalist-ui-design-2026), [Fireart Studio 2026](https://fireart.studio/blog/the-best-web-design-trends/), [StudioMeyer "what held up" 2026](https://studiomeyer.io/en/blog/webdesign-trends-2026-reality-check), [Tubik 2026](https://tubikstudio.com/blog/ui-design-trends-2026/).
- **ISA-101 grayscale HMI, with colour reserved for deviation.** Sources: [iFactory, "ISA-101 Principles for 2026"](https://ifactoryapp.com/blog/hmi-design-best-practices), [Malisko](https://malisko.com/isa-101/).
- **Flight progress strips and electronic strip handoff.** Sources: [SKYbrary](https://skybrary.aero/articles/flight-progress-strips), [ACM TOCHI "Is paper safer?"](https://dl.acm.org/doi/10.1145/331490.331491).
- The stage-manager, telegraph-relay, dual-control-car, kitchen-pass and studio-talkback metaphors come from training data (2026).
