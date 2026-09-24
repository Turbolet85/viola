//! The closed diagnostics vocabulary (obs-plan §3 Log format) and `obs_event!`, the one sanctioned
//! emitter. The macro expands to `::tracing::event!` at the caller, so this crate stays tracing-free.

use std::fmt;
use std::sync::OnceLock;

use crate::ViolaName;

/// The closed `event` enum; a new value needs an obs-plan Decisions Log entry and a
/// `schemas/diag-line.v1.json` update. `a11y-violation` is a harness-only row, never a variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObsEvent {
    ChannelRequest,
    ChannelResponse,
    DialogRaised,
    DialogAnswered,
    HookInvoked,
    HookDecision,
    SendIssued,
    SendConfirmed,
    SendRefused,
    ReleaseFromDriver,
    ProcessStart,
    ProcessExit,
    HttpRequest,
    Panic,
    LivenessChanged,
    StateRecovered,
    SseOpened,
    SseClosed,
    ParseRejected,
}

impl ObsEvent {
    pub const ALL: [Self; 19] = [
        Self::ChannelRequest,
        Self::ChannelResponse,
        Self::DialogRaised,
        Self::DialogAnswered,
        Self::HookInvoked,
        Self::HookDecision,
        Self::SendIssued,
        Self::SendConfirmed,
        Self::SendRefused,
        Self::ReleaseFromDriver,
        Self::ProcessStart,
        Self::ProcessExit,
        Self::HttpRequest,
        Self::Panic,
        Self::LivenessChanged,
        Self::StateRecovered,
        Self::SseOpened,
        Self::SseClosed,
        Self::ParseRejected,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ChannelRequest => "channel-request",
            Self::ChannelResponse => "channel-response",
            Self::DialogRaised => "dialog-raised",
            Self::DialogAnswered => "dialog-answered",
            Self::HookInvoked => "hook-invoked",
            Self::HookDecision => "hook-decision",
            Self::SendIssued => "send-issued",
            Self::SendConfirmed => "send-confirmed",
            Self::SendRefused => "send-refused",
            Self::ReleaseFromDriver => "release-from-driver",
            Self::ProcessStart => "process-start",
            Self::ProcessExit => "process-exit",
            Self::HttpRequest => "http-request",
            Self::Panic => "panic",
            Self::LivenessChanged => "liveness-changed",
            Self::StateRecovered => "state-recovered",
            Self::SseOpened => "sse-opened",
            Self::SseClosed => "sse-closed",
            Self::ParseRejected => "parse-rejected",
        }
    }
}

impl fmt::Display for ObsEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// The process role: it names the home-level role file and fills every line's `process`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObsProcess {
    Run,
    Hook,
    Mcp,
    Ui,
    Cli,
}

impl ObsProcess {
    pub const ALL: [Self; 5] = [Self::Run, Self::Hook, Self::Mcp, Self::Ui, Self::Cli];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Run => "run",
            Self::Hook => "hook",
            Self::Mcp => "mcp",
            Self::Ui => "ui",
            Self::Cli => "cli",
        }
    }
}

impl fmt::Display for ObsProcess {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Set once at init; every `obs_event!` reads its `process` and `instance` from here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessCtx {
    pub process: ObsProcess,
    pub instance: Option<ViolaName>,
}

static CTX: OnceLock<ProcessCtx> = OnceLock::new();

/// The first call wins; returns whether this one took.
pub fn set_ctx(ctx: ProcessCtx) -> bool {
    CTX.set(ctx).is_ok()
}

pub fn ctx() -> Option<&'static ProcessCtx> {
    CTX.get()
}

pub fn ctx_process() -> Option<&'static str> {
    ctx().map(|c| c.process.as_str())
}

pub fn ctx_instance() -> Option<&'static str> {
    ctx().and_then(|c| c.instance.as_ref()).map(name_str)
}

fn name_str(name: &ViolaName) -> &str {
    name.as_ref()
}

/// `obs_event!(INFO, ObsEvent::ProcessStart, subject = "self", pid = 7)`: one codes-only line
/// carrying `event`, `process` and `instance` from the process context (an absent value is an
/// absent key), with `message` equal to the event name. `corr` is passed as a typed field.
#[macro_export]
macro_rules! obs_event {
    ($level:ident, $event:expr $(, $key:ident = $value:expr)* $(,)?) => {{
        let event: $crate::obs::ObsEvent = $event;
        #[allow(clippy::disallowed_macros)]
        {
            ::tracing::event!(
                ::tracing::Level::$level,
                event = event.as_str(),
                process = $crate::obs::ctx_process(),
                instance = $crate::obs::ctx_instance(),
                $($key = $value,)*
                "{}",
                event.as_str()
            );
        }
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    /// test-plan §3 Log format + obs-plan D-01…D-05, written out as the oracle.
    const LOG_FORMAT_EVENTS: [&str; 19] = [
        "channel-request",
        "channel-response",
        "dialog-raised",
        "dialog-answered",
        "hook-invoked",
        "hook-decision",
        "send-issued",
        "send-confirmed",
        "send-refused",
        "release-from-driver",
        "process-start",
        "process-exit",
        "http-request",
        "panic",
        "liveness-changed",
        "state-recovered",
        "sse-opened",
        "sse-closed",
        "parse-rejected",
    ];

    #[test]
    fn obs_event_display_matches_the_log_format_literals() {
        let shown: Vec<String> = ObsEvent::ALL.iter().map(ToString::to_string).collect();
        assert_eq!(shown, LOG_FORMAT_EVENTS);
        for event in ObsEvent::ALL {
            assert_eq!(event.to_string(), event.as_str());
        }
        assert!(!shown.iter().any(|s| s == "a11y-violation"));
    }

    #[test]
    fn obs_process_display_is_the_five_roles() {
        let shown: Vec<String> = ObsProcess::ALL.iter().map(ToString::to_string).collect();
        assert_eq!(shown, ["run", "hook", "mcp", "ui", "cli"]);
    }

    #[test]
    fn obs_ctx_first_set_wins() {
        assert_eq!(ctx_process(), None);
        assert_eq!(ctx_instance(), None);
        let builder = ViolaName::try_new("builder".to_owned()).expect("valid");
        assert!(set_ctx(ProcessCtx {
            process: ObsProcess::Run,
            instance: Some(builder),
        }));
        assert!(!set_ctx(ProcessCtx {
            process: ObsProcess::Ui,
            instance: None,
        }));
        assert_eq!(ctx_process(), Some("run"));
        assert_eq!(ctx_instance(), Some("builder"));
        assert_eq!(ctx().map(|c| c.process), Some(ObsProcess::Run));
    }
}
