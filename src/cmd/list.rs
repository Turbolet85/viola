//! `viola list`: the board of sessions (architecture §Standard Contracts, `list` payload). It is no
//! channel method: this process reads the home's `instances/` itself and contacts no wrapper. Every
//! path is checked before anything under the home is opened, its own log included, and one that
//! another user could write refuses the whole board. Then one `claude agents --json` read gives each
//! wrapped row its idle or busy status and lists the sessions viola does not wrap; a failed read
//! leaves the board as the disk gave it.

use std::io;
use std::path::Path;
use std::process::ExitCode;
use std::time::SystemTime;

use chrono::{DateTime, Utc};
use serde_json::{Value, json};
use tracing::instrument;
use viola_agent_claude::agents::{AgentStatus, AgentsList};
use viola_state::board::{self, Board, Observation, Scan, ScanRefused, SessionStatus};
use viola_state::budget::{StampedReading, read_budget_stamped};

use super::client;
use crate::human::{self, BayBudget};
use crate::run::budget::{self, Thresholds};
use crate::{obs, run};

#[derive(clap::Args)]
pub(crate) struct ListArgs {
    /// Print one JSON document on stdout
    #[arg(long)]
    json: bool,
}

/// The newest usage reading and whether the gate's rule pauses on it.
struct Budget {
    stamped: StampedReading,
    paused: bool,
}

pub(crate) fn list(home: &Path, args: &ListArgs) -> anyhow::Result<ExitCode> {
    let home = std::path::absolute(home)?;
    // Before the log opens: opening it narrows the home to its owner, which would erase the very
    // state this check refuses. A refused check opens nothing under the home and writes no line.
    let scan = match board::scan(&home)? {
        Ok(scan) => scan,
        Err(refused) => {
            refuse(args.json, &refused);
            return Ok(ExitCode::from(21));
        }
    };
    if let Some(name) = client::own_name() {
        client::start(&home, &name)?;
    }
    let (board, budget, now) = read(&home, &scan, args.json);
    if args.json {
        client::document(&document(&board, budget.as_ref()));
    } else {
        let bay = budget.as_ref().map(|budget| BayBudget {
            reading: &budget.stamped,
            paused: budget.paused,
        });
        // The terminal's columns from stdout itself, never `COLUMNS`: a piped board reads none and
        // cuts nothing. `colour` holds the Windows VT enable until the board is written.
        let width = viola_pty::host_size().map(|size| usize::from(size.cols));
        let colour = human::read_colour(false, false);
        let _ = human::write_board(
            &mut io::stdout().lock(),
            bay.as_ref(),
            &board,
            now,
            width,
            colour.depth,
        );
    }
    run::log_self_exit(0, None);
    Ok(ExitCode::SUCCESS)
}

/// The board of `home` as `list --json` prints it, read now: the scan, then the read, then the one
/// document, or the refusal document when the scan refuses. The `mcp` process's `list` tool reads
/// it; CLI `list` runs the same three pieces with its own log opened between the scan and the read.
pub(crate) fn board_document(home: &Path) -> anyhow::Result<Value> {
    Ok(match board::scan(home)? {
        Ok(scan) => {
            let (board, budget, _) = read(home, &scan, true);
            document(&board, budget.as_ref())
        }
        Err(_) => refused_document(),
    })
}

/// Exit 21's `--json` document: a home or instance another user could write refuses the whole board.
fn refused_document() -> Value {
    json!({"v": 1, "error": "instance-unreachable", "detail": null})
}

/// Exit 21's output: the one `--json` document, or the pair for what failed.
fn refuse(json: bool, refused: &ScanRefused) {
    if json {
        client::document(&refused_document());
        return;
    }
    match refused {
        ScanRefused::Home => {
            let (unable, hint) = human::home_state_pair();
            human::refuse(unable, hint);
        }
        ScanRefused::Instance(name) => {
            let (unable, hint) = human::instance_state_pair(name.as_ref());
            human::refuse(&unable, &hint);
        }
    }
}

/// The board, the budget and the one instant both were judged at.
#[instrument(skip_all, name = "cli.list", fields(json = json))]
fn read(home: &Path, scan: &Scan, json: bool) -> (Board, Option<Budget>, DateTime<Utc>) {
    let at = SystemTime::now();
    let now = DateTime::<Utc>::from(at);
    let mut board = board::read(scan, at);
    if let Ok(list) = std::env::current_dir()
        .ok()
        .ok_or(())
        .and_then(|cwd| run::agents::read_here(home, &cwd).map_err(|_| ()))
    {
        board::join(&mut board, observations(list));
    }
    let stamped = read_budget_stamped(home);
    let (thresholds, rejections) = budget::read_thresholds(home);
    for rejection in rejections {
        obs::log_config_rejection(rejection);
    }
    (board, budget_of(stamped, thresholds, now), now)
}

/// Each listed session as the board reads it: the name is the session id, never the CLI's title.
fn observations(list: AgentsList) -> Vec<Observation> {
    list.rows
        .into_iter()
        .map(|row| Observation {
            pid: row.pid,
            session_id: row.session_id,
            status: match row.status {
                AgentStatus::Idle => SessionStatus::Idle,
                AgentStatus::Busy => SessionStatus::Busy,
                AgentStatus::Unknown => SessionStatus::Unknown,
            },
        })
        .collect()
}

/// `paused` as the wrapper's gate reads the same reading at `now` with no override.
fn budget_of(
    stamped: Option<StampedReading>,
    thresholds: Thresholds,
    now: DateTime<Utc>,
) -> Option<Budget> {
    stamped.map(|stamped| {
        let paused = budget::decide(Some(&stamped.reading), thresholds, None, now)
            .paused
            .is_some();
        Budget { stamped, paused }
    })
}

/// `{"v":1,"ok":{"items":[…],"budget":…,"skipped":{…}}}`; `budget` is the word `"unknown"` when
/// there is no reading.
fn document(board: &Board, budget: Option<&Budget>) -> Value {
    let budget = match budget {
        Some(budget) => json!({
            "five_hour": budget.stamped.reading.five_hour,
            "seven_day": budget.stamped.reading.seven_day,
            "read_at": budget.stamped.read_at,
            "paused": budget.paused,
        }),
        None => json!("unknown"),
    };
    json!({"v": 1, "ok": {"items": board.items, "budget": budget, "skipped": board.skipped}})
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser as _;
    use viola_agent_claude::agents;
    use viola_core::{BudgetReading, BudgetWindow, Reading};
    use viola_state::board::{Item, JoinKeys, RowLiveness, SkippedTotal};
    use viola_state::snapshot::Wheel;

    fn at(text: &str) -> DateTime<Utc> {
        DateTime::parse_from_rfc3339(text)
            .expect("an instant")
            .with_timezone(&Utc)
    }

    const NOW: &str = "2025-03-01T12:00:00.000Z";

    fn stamped(five_hour: f64) -> StampedReading {
        StampedReading {
            reading: BudgetReading {
                five_hour: Reading::Known(BudgetWindow {
                    used_percentage: Reading::Known(five_hour),
                    resets_at: Reading::Known("2099-01-01T00:00:00.000Z".to_owned()),
                }),
                seven_day: Reading::UNKNOWN,
            },
            read_at: Reading::Known("2025-03-01T11:58:00.000Z".to_owned()),
        }
    }

    fn board() -> Board {
        Board {
            items: vec![Item {
                name: "builder".to_owned(),
                wrapped: true,
                liveness: RowLiveness::Live,
                status: SessionStatus::Unknown,
                wheel: Some(Wheel::Driver),
                budget_paused: Some(true),
                dialog_pending: false,
                cli_version: Some("2.1.287".to_owned()),
                cli_verified: Some(true),
                dialog_kind: None,
                join: JoinKeys::default(),
            }],
            skipped: SkippedTotal {
                unknown_kinds: 1,
                unknown_fields: 0,
                torn_lines: 2,
            },
        }
    }

    #[test]
    fn list_document_with_a_reading_is_the_contract_envelope() {
        let budget = budget_of(Some(stamped(91.0)), Thresholds::default(), at(NOW));
        assert_eq!(
            document(&board(), budget.as_ref()).to_string(),
            r#"{"v":1,"ok":{"items":[{"name":"builder","wrapped":true,"liveness":"live","status":"unknown","wheel":"driver","budget_paused":true,"dialog_pending":false,"cli_version":"2.1.287","cli_verified":true}],"budget":{"five_hour":{"used_percentage":91.0,"resets_at":"2099-01-01T00:00:00.000Z"},"seven_day":"unknown","read_at":"2025-03-01T11:58:00.000Z","paused":true},"skipped":{"unknown_kinds":1,"unknown_fields":0,"torn_lines":2}}}"#
        );
    }

    #[test]
    fn list_document_without_a_reading_has_the_budget_word_unknown() {
        let empty = Board {
            items: Vec::new(),
            skipped: SkippedTotal::default(),
        };
        let budget = budget_of(None, Thresholds::default(), at(NOW));
        assert!(budget.is_none());
        assert_eq!(
            document(&empty, None).to_string(),
            r#"{"v":1,"ok":{"items":[],"budget":"unknown","skipped":{"unknown_kinds":0,"unknown_fields":0,"torn_lines":0}}}"#
        );
    }

    #[test]
    fn list_document_paused_is_the_gates_rule_under_the_default_thresholds() {
        let paused = |used: f64| {
            budget_of(Some(stamped(used)), Thresholds::default(), at(NOW))
                .expect("a reading")
                .paused
        };
        assert!(paused(91.0));
        assert!(paused(90.0));
        assert!(!paused(89.0));
    }

    /// Each row's session id names it and its status word maps one to one; the CLI's title, its
    /// `cwd` and its `kind` reach no observation.
    #[test]
    fn list_observations_take_the_session_id_pid_and_status() {
        let list = agents::parse(
            br#"[{"pid":7,"sessionId":"s-1","status":"idle","name":"a title","cwd":"/w"},{"sessionId":"s-2","status":"busy"},{"pid":9,"sessionId":"s-3","status":"later"}]"#,
        )
        .expect("a list");
        assert_eq!(
            observations(list),
            [
                Observation {
                    pid: Some(7),
                    session_id: "s-1".to_owned(),
                    status: SessionStatus::Idle,
                },
                Observation {
                    pid: None,
                    session_id: "s-2".to_owned(),
                    status: SessionStatus::Busy,
                },
                Observation {
                    pid: Some(9),
                    session_id: "s-3".to_owned(),
                    status: SessionStatus::Unknown,
                },
            ]
        );
    }

    #[derive(clap::Parser)]
    struct Line {
        #[command(flatten)]
        args: ListArgs,
    }

    fn parse(words: &[&str]) -> Result<ListArgs, clap::error::ErrorKind> {
        Line::try_parse_from(std::iter::once("list").chain(words.iter().copied()))
            .map(|line| line.args)
            .map_err(|error| error.kind())
    }

    #[test]
    fn list_document_args_take_the_json_flag_and_nothing_else() {
        assert!(!parse(&[]).expect("bare").json);
        assert!(parse(&["--json"]).expect("--json").json);
        assert_eq!(
            parse(&["builder"]).err(),
            Some(clap::error::ErrorKind::UnknownArgument)
        );
    }
}
