# Classified — arch · established-decisions.b2

## Answers
established-decisions#17.1 · yes · its line that every snapshot write goes through the wrapper's one in-process holder (src/run/snapshot.rs) states the only-the-wrapper-writes-snapshot.json behaviour the chunk re-reads
established-decisions#18.1 · no · the chunk changes no link recording, from passing or set restore, and its pre-frame refusals send no frame, so a refused send or answer records no link, as the entity already states
established-decisions#18.2 · yes · the check before client::start and ahead of any role-file or log open, refusing with exit 21, is the ordering the chunk now builds into start in client.rs
established-decisions#18.3 · no · the chunk adds no wrapper link line or span change; the wrapper's link and unlink channel lines and the cli.link / cli.unlink dispatch span are untouched
established-decisions#18.4 · no · LinkSet::line's Utc::now timestamp in src/run/links.rs is not reached, and the chunk adds no time stamp to a link line
established-decisions#19.1 · yes · read_budget reads through check_path, which the chunk edits, and the wrapper's start read follows run's new checks, so a refused check must still leave the gate fail-open
established-decisions#20.1 · yes · the run version gate's stamps read moves behind the strict check, so a refusal must read cli_verified:false and withhold dialog answers, and verify's stamps writes stay with the one writer update_stamps, which the chunk re-reads
