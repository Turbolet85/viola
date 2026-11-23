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

use serde_json::{Value, json};
use tracing::instrument;
use viola_agent_claude::agents::{AgentStatus, AgentsList};
use viola_core::UnreachableCause;
use viola_state::board::{self, Board, Observation, Scan, ScanRefused, SessionStatus};

use super::client;
use crate::{human, run};

#[derive(clap::Args)]
pub(crate) struct ListArgs {
    /// Print one JSON document on stdout
    #[arg(long)]
    json: bool,
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
    let board = read(&home, &scan, args.json);
    if args.json {
        client::document(&document(&board));
    } else {
        // The terminal's columns from stdout itself, never `COLUMNS`: a piped board reads none and
        // cuts nothing.
        let width = viola_pty::host_size().map(|size| usize::from(size.cols));
        let colour = human::read_colour(false, false);
        let _ = human::write_board(&mut io::stdout().lock(), &board, width, colour.depth);
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
            let board = read(home, &scan, true);
            document(&board)
        }
        Err(_) => refused_document(),
    })
}

/// Exit 21's `--json` document: a home or instance another user could write refuses the whole board.
fn refused_document() -> Value {
    let cause = UnreachableCause::StrictModes.as_str();
    json!({"v": 1, "error": "instance-unreachable", "detail": cause})
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

/// The board, judged now.
#[instrument(skip_all, name = "cli.list", fields(json = json))]
fn read(home: &Path, scan: &Scan, json: bool) -> Board {
    let mut board = board::read(scan, SystemTime::now());
    if let Ok(list) = std::env::current_dir()
        .ok()
        .ok_or(())
        .and_then(|cwd| run::agents::read_here(home, &cwd).map_err(|_| ()))
    {
        board::join(&mut board, observations(list));
    }
    board
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

/// `{"v":1,"ok":{"items":[…],"links":[…],"skipped":{…}}}`, `links` absent when the board holds
/// none.
fn document(board: &Board) -> Value {
    if board.links.is_empty() {
        return json!({"v": 1, "ok": {"items": board.items, "skipped": board.skipped}});
    }
    json!({"v": 1, "ok": {"items": board.items, "links": board.links, "skipped": board.skipped}})
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser as _;
    use viola_agent_claude::agents;
    use viola_state::board::{BoardLink, Item, JoinKeys, RowLiveness, SkippedTotal};
    use viola_state::snapshot::Wheel;

    fn board() -> Board {
        Board {
            items: vec![Item {
                name: "builder".to_owned(),
                wrapped: true,
                liveness: RowLiveness::Live,
                status: SessionStatus::Unknown,
                wheel: Some(Wheel::Driver),
                dialog_pending: false,
                cli_version: Some("2.1.287".to_owned()),
                cli_verified: Some(true),
                dialog_kind: None,
                join: JoinKeys::default(),
                parent: None,
                role: None,
                model: None,
                effort: None,
            }],
            skipped: SkippedTotal {
                unknown_kinds: 1,
                unknown_fields: 0,
                torn_lines: 2,
            },
            links: Vec::new(),
        }
    }

    #[test]
    fn list_document_is_the_contract_envelope() {
        assert_eq!(
            document(&board()).to_string(),
            r#"{"v":1,"ok":{"items":[{"name":"builder","wrapped":true,"liveness":"live","status":"unknown","wheel":"driver","dialog_pending":false,"cli_version":"2.1.287","cli_verified":true}],"skipped":{"unknown_kinds":1,"unknown_fields":0,"torn_lines":2}}}"#
        );
    }

    #[test]
    fn list_document_carries_parent_role_and_links() {
        let mut board = board();
        board.items[0].parent = Some("overseer".to_owned());
        board.items[0].role = Some("builder".to_owned());
        board.links = vec![BoardLink {
            driver: "tester".to_owned(),
            driven: "builder".to_owned(),
            since: "2026-11-22T01:02:03.000Z".to_owned(),
        }];
        assert_eq!(
            document(&board).to_string(),
            r#"{"v":1,"ok":{"items":[{"name":"builder","wrapped":true,"liveness":"live","status":"unknown","wheel":"driver","dialog_pending":false,"cli_version":"2.1.287","cli_verified":true,"parent":"overseer","role":"builder"}],"links":[{"driver":"tester","driven":"builder","since":"2026-11-22T01:02:03.000Z"}],"skipped":{"unknown_kinds":1,"unknown_fields":0,"torn_lines":2}}}"#
        );
    }

    #[test]
    fn list_document_carries_the_launch_pair() {
        let mut board = board();
        board.items[0].role = Some("builder".to_owned());
        board.items[0].model = Some("haiku".to_owned());
        board.items[0].effort = Some("low".to_owned());
        assert_eq!(
            document(&board).to_string(),
            r#"{"v":1,"ok":{"items":[{"name":"builder","wrapped":true,"liveness":"live","status":"unknown","wheel":"driver","dialog_pending":false,"cli_version":"2.1.287","cli_verified":true,"role":"builder","model":"haiku","effort":"low"}],"skipped":{"unknown_kinds":1,"unknown_fields":0,"torn_lines":2}}}"#
        );
    }

    #[test]
    fn list_document_of_an_empty_board_holds_no_item() {
        let empty = Board {
            items: Vec::new(),
            skipped: SkippedTotal::default(),
            links: Vec::new(),
        };
        assert_eq!(
            document(&empty).to_string(),
            r#"{"v":1,"ok":{"items":[],"skipped":{"unknown_kinds":0,"unknown_fields":0,"torn_lines":0}}}"#
        );
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
