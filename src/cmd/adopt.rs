//! `viola adopt <session-id> --name <n>`: a plain `claude` session, started outside viola and
//! already ended, resumed under a wrapper in the terminal the verb is typed in. It is `revive`'s
//! start without a log to read: the id is the operator's, in the closed session-id shape, and a
//! session a live `claude` still holds is refused, never ended or taken over.

use std::ffi::OsString;
use std::path::Path;
use std::process::ExitCode;

use viola_agent_claude::{PROGRAM, resume_args};
use viola_core::ViolaName;
use viola_state::replay::read_snapshot_or_replay;
use viola_state::snapshot::Lineage;
use viola_state::strict::check_instance;

use super::client;
use super::revive::{live_elsewhere, parse_session_id};
use super::run::{
    Launch, Started, checked_exe, collision_check, open_wrapper_log, parse_effort, parse_model,
    pump_child, refuse_tree, refused, start,
};
use crate::human;

#[derive(clap::Args)]
pub(crate) struct AdoptArgs {
    /// The id of the ended claude session to resume
    #[arg(value_name = "SESSION_ID", value_parser = parse_session_id)]
    id: String,
    /// Instance name: [a-z0-9-], 1-32 characters, starting with a letter
    #[arg(long, value_name = "NAME", value_parser = super::send::parse_name)]
    pub(super) name: ViolaName,
    /// The instance this session works under
    #[arg(long, value_name = "NAME", value_parser = super::send::parse_name)]
    parent: Option<ViolaName>,
    /// Resume on this model instead of the default one
    #[arg(long, value_name = "MODEL", value_parser = parse_model)]
    model: Option<String>,
    /// Resume on this effort instead of the default one: low, medium, high, xhigh or max
    #[arg(long, value_name = "EFFORT", value_parser = parse_effort)]
    effort: Option<String>,
    /// Print one JSON document on stdout instead of a refusal's lines
    #[arg(long)]
    json: bool,
}

const SESSION_LIVE_UNABLE: &str = "that session is open in another claude";
const SESSION_LIVE_HINT: &str = "close it there first, then adopt it again";

/// A refusal adopt words itself. A name held by a live or stale wrapper is `run`'s own refusal,
/// written by its collision check.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Refusal {
    StrictModes,
    SessionLive,
}

impl Refusal {
    /// The closed `process-exit` detail. A refused strict-modes check writes no line.
    const fn detail(self) -> &'static str {
        match self {
            Self::StrictModes => "strict-modes-failed",
            Self::SessionLive => "session-live",
        }
    }

    /// The `unable:` and `hint:` texts: fixed words and the instance's name, never the id.
    fn pair(self, name: &str) -> (String, String) {
        match self {
            Self::StrictModes => human::instance_state_pair(name),
            Self::SessionLive => (SESSION_LIVE_UNABLE.to_owned(), SESSION_LIVE_HINT.to_owned()),
        }
    }

    /// The stderr pair, or with `json` the one document naming the closed code.
    fn write(self, name: &ViolaName, json: bool) {
        if json {
            client::refused_document(self.detail());
        } else {
            let (unable, hint) = self.pair(name.as_ref());
            human::refuse(&unable, &hint);
        }
    }
}

/// The arguments clap cannot check alone: a `--parent` naming the session itself is a usage error.
fn checked(args: AdoptArgs) -> Result<AdoptArgs, clap::Error> {
    if args.parent.as_ref() == Some(&args.name) {
        return Err(clap::Error::raw(
            clap::error::ErrorKind::ArgumentConflict,
            "the argument '--parent <NAME>' cannot name the session itself\n",
        ));
    }
    Ok(args)
}

pub(crate) fn adopt(home: &Path, args: AdoptArgs) -> anyhow::Result<ExitCode> {
    let args = checked(args).unwrap_or_else(|usage| usage.exit());
    let json = args.json;
    // The error still reaches the catch site, for its chain and its line. A `pump_child` error
    // prints no document: the child has held the terminal, and viola writes nothing past it.
    let started = started(home, args).inspect_err(|_| {
        if json {
            client::refused_document("internal-error");
        }
    })?;
    match started {
        Started::Launched(launched) => pump_child(*launched),
        Started::Refused(code) => Ok(code),
    }
}

/// Everything ahead of the child: the checks, the log, the preflight and `run`'s start.
fn started(home: &Path, args: AdoptArgs) -> anyhow::Result<Started> {
    // Before the log opens: opening it narrows the home to its owner, which would erase the very
    // state this check refuses.
    if !trusted(home, &args.name)? {
        Refusal::StrictModes.write(&args.name, args.json);
        return Ok(Started::Refused(ExitCode::from(1)));
    }
    let exe = match checked_exe(home, &args.name)? {
        Ok(exe) => exe,
        Err(_) if args.json => {
            Refusal::StrictModes.write(&args.name, true);
            return Ok(Started::Refused(ExitCode::from(1)));
        }
        Err(refused) => {
            refuse_tree(&args.name, refused);
            return Ok(Started::Refused(ExitCode::from(1)));
        }
    };
    let persistent = open_wrapper_log(home, &args.name)?;
    match preflight(home, args)? {
        Ok(launch) => start(home, &launch, &persistent, &exe),
        Err(refusal) => Ok(refusal),
    }
}

/// Whether the instance's home and files pass the strict-modes check.
fn trusted(home: &Path, name: &ViolaName) -> anyhow::Result<bool> {
    let home = std::path::absolute(home)?;
    let instance_dir = home.join("instances").join(AsRef::<str>::as_ref(name));
    Ok(check_instance(&home, &instance_dir).is_ok())
}

/// The two readings in their fixed order: the name's holder, then whether a live `claude` holds
/// the session. The second is taken only when the first passed, so the first refusal wins.
fn preflight_order<R>(
    holder: impl FnOnce() -> Option<R>,
    session_live: impl FnOnce() -> Option<R>,
) -> Option<R> {
    if let Some(refusal) = holder() {
        return Some(refusal);
    }
    session_live()
}

/// The launch a passed preflight yields, or its refusal, already written and logged.
fn preflight(home: &Path, args: AdoptArgs) -> anyhow::Result<Result<Launch, Started>> {
    let name = &args.name;
    let home = std::path::absolute(home)?;
    let instance_dir = home.join("instances").join(AsRef::<str>::as_ref(name));
    let json = args.json;
    let cwd = std::env::current_dir()?;
    let read = read_snapshot_or_replay(&instance_dir);
    let refusal = preflight_order(
        || collision_check(name, &instance_dir, read.as_ref().ok(), json),
        || {
            live_elsewhere(&home, &cwd, &args.id).then(|| {
                Refusal::SessionLive.write(name, json);
                refused(Refusal::SessionLive.detail())
            })
        },
    );
    if let Some(refusal) = refusal {
        return Ok(Err(refusal));
    }
    let lineage = lineage_of(args.parent, &args.name, client::own_name());
    Ok(Ok(Launch {
        name: args.name,
        program: OsString::from(PROGRAM),
        args: resume_args(&args.id, false),
        spawn_dir: cwd,
        json,
        lineage,
        given: (args.model, args.effort),
        window: None,
    }))
}

/// The lineage an adopted start records: the `--parent` given, else the inherited name of the
/// session it was typed in (read as the parent only), dropped when it is the adopted name; no role.
fn lineage_of(
    parent: Option<ViolaName>,
    name: &ViolaName,
    inherited: Option<ViolaName>,
) -> Lineage {
    let parent = parent.or_else(|| inherited.filter(|own| own != name));
    Lineage {
        parent: parent.map(|name| name.as_ref().to_owned()),
        role: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    use clap::Parser as _;
    use clap::error::ErrorKind;
    use rstest::rstest;

    const ID: &str = "c0f0cc23-690b-45dd-bbfb-06d6cfd44942";

    #[derive(clap::Parser)]
    struct Line {
        #[command(flatten)]
        args: AdoptArgs,
    }

    fn parse(words: &[&str]) -> Result<AdoptArgs, clap::Error> {
        Line::try_parse_from(std::iter::once("adopt").chain(words.iter().copied()))
            .map(|line| line.args)
            .and_then(checked)
    }

    fn name(word: &str) -> ViolaName {
        ViolaName::try_new(word.to_owned()).expect("valid")
    }

    #[test]
    fn adopt_args_take_an_id_a_name_a_parent_and_a_pair() {
        let args = parse(&[
            ID, "--name", "builder", "--parent", "overseer", "--model", "haiku", "--effort", "low",
            "--json",
        ])
        .expect("parsed");
        assert_eq!(args.id, ID);
        assert_eq!(AsRef::<str>::as_ref(&args.name), "builder");
        assert_eq!(args.parent, Some(name("overseer")));
        assert_eq!(args.model.as_deref(), Some("haiku"));
        assert_eq!(args.effort.as_deref(), Some("low"));
        assert!(args.json);
        let bare = parse(&[ID, "--name", "builder"]).expect("parsed");
        assert!(bare.parent.is_none() && bare.model.is_none() && bare.effort.is_none());
        assert!(!bare.json);
        // The shared session-id shape takes hex of either case, as revive's `--id` does.
        let upper =
            parse(&["C0F0CC23-690B-45DD-BBFB-06D6CFD44942", "--name", "builder"]).expect("parsed");
        assert_eq!(upper.id, "C0F0CC23-690B-45DD-BBFB-06D6CFD44942");
    }

    #[rstest]
    #[case::an_id_of_another_shape(&["s-1", "--name", "builder"], ErrorKind::ValueValidation)]
    #[case::an_id_that_is_an_option_word(
        &["--name", "builder", "--", "--dangerously-skip-permissions-00000"],
        ErrorKind::ValueValidation
    )]
    #[case::a_name_that_is_no_name(&[ID, "--name", "Not-A-Name"], ErrorKind::ValueValidation)]
    #[case::no_name(&[ID], ErrorKind::MissingRequiredArgument)]
    #[case::no_id(&["--name", "builder"], ErrorKind::MissingRequiredArgument)]
    #[case::a_parent_equal_to_the_name(
        &[ID, "--name", "builder", "--parent", "builder"],
        ErrorKind::ArgumentConflict
    )]
    #[case::a_bad_model(&[ID, "--name", "builder", "--model", "a/b"], ErrorKind::ValueValidation)]
    #[case::a_bad_effort(&[ID, "--name", "builder", "--effort", "huge"], ErrorKind::ValueValidation)]
    #[case::a_word_after_the_dashes(
        &[ID, "--name", "builder", "--", "--verbose"],
        ErrorKind::UnknownArgument
    )]
    fn adopt_args_refuse_as_usage(#[case] words: &[&str], #[case] kind: ErrorKind) {
        let error = parse(words).err().expect("refused");
        assert_eq!(error.kind(), kind);
        assert_eq!(error.exit_code(), 2);
    }

    #[rstest]
    #[case::the_pass([None, None], None, "holder live")]
    #[case::holder_alone([Some("holder"), None], Some("holder"), "holder")]
    #[case::live_alone([None, Some("live")], Some("live"), "holder live")]
    #[case::both([Some("holder"), Some("live")], Some("holder"), "holder")]
    fn adopt_preflight_order_the_first_refusal_wins(
        #[case] readings: [Option<&'static str>; 2],
        #[case] refusal: Option<&'static str>,
        #[case] taken: &str,
    ) {
        let log = RefCell::new(Vec::new());
        let read = |reading: &'static str, answer: Option<&'static str>| {
            log.borrow_mut().push(reading);
            answer
        };
        let [holder, live] = readings;
        let got = preflight_order(|| read("holder", holder), || read("live", live));
        assert_eq!(got, refusal);
        assert_eq!(log.borrow().join(" "), taken);
    }

    /// A refused holder takes no `claude agents --json` read.
    #[test]
    fn adopt_preflight_order_a_refused_holder_takes_no_agents_read() {
        let live_taken = RefCell::new(false);
        let got = preflight_order(
            || Some(Refusal::StrictModes),
            || {
                *live_taken.borrow_mut() = true;
                Some(Refusal::SessionLive)
            },
        );
        assert_eq!(got, Some(Refusal::StrictModes));
        assert!(!*live_taken.borrow());
    }

    #[rstest]
    #[case::strict_modes(
        Refusal::StrictModes,
        "strict-modes-failed",
        "builder's state files can be written by another user",
        "viola will not read them; make the viola home and its files owner-only"
    )]
    #[case::session_live(
        Refusal::SessionLive,
        "session-live",
        "that session is open in another claude",
        "close it there first, then adopt it again"
    )]
    fn adopt_refusals_have_their_fixed_pair_and_closed_detail(
        #[case] refusal: Refusal,
        #[case] detail: &str,
        #[case] unable: &str,
        #[case] hint: &str,
    ) {
        assert_eq!(refusal.detail(), detail);
        assert_eq!(
            refusal.pair("builder"),
            (unable.to_owned(), hint.to_owned())
        );
        assert!(!hint.contains("release"));
        assert!(!unable.contains(ID) && !hint.contains(ID));
    }

    #[rstest]
    #[case::given(Some("overseer"), Some("caller"), Some("overseer"))]
    #[case::inherited(None, Some("caller"), Some("caller"))]
    #[case::inherited_equal_to_the_name_is_dropped(None, Some("builder"), None)]
    #[case::none(None, None, None)]
    fn adopt_lineage_takes_the_given_parent_else_the_inherited_one(
        #[case] given: Option<&str>,
        #[case] inherited: Option<&str>,
        #[case] parent: Option<&str>,
    ) {
        let lineage = lineage_of(given.map(name), &name("builder"), inherited.map(name));
        assert_eq!(lineage.parent.as_deref(), parent);
        assert_eq!(lineage.role, None);
    }
}
