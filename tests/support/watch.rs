//! The root-watch recorder: a test waiting on a child streams what it polls, one line per change, to
//! a known file under the temp dir, so a test the runner kills still leaves its evidence
//! (testing.md 2026-09-24, extended 2026-09-27). Lines carry codes, counts and booleans only, never
//! file or screen content, and the dir is outside every home's `diagnostics/` (obs-plan §9 G4).

use std::cell::RefCell;
use std::fs::{self, File, OpenOptions};
use std::io::Write as _;
use std::path::PathBuf;
use std::time::{Duration, Instant};

/// Strictly below the nextest `mutants` profile's 10 s kill, so a stuck wait fails the test itself
/// with the report in its message, before the runner kills it and loses that dump.
pub const WITHIN: Duration = Duration::from_secs(7);

pub struct Watch {
    report: PathBuf,
    last: RefCell<Option<String>>,
}

impl Watch {
    /// `<temp dir>/viola-root-watch/<test name, :: → .>.<label>.report`, created empty.
    pub fn start(label: &str) -> Self {
        let dir = std::env::temp_dir().join("viola-root-watch");
        fs::create_dir_all(&dir).expect("watch dir");
        let test = std::thread::current()
            .name()
            .unwrap_or("unnamed")
            .replace("::", ".");
        let report = dir.join(format!("{test}.{label}.report"));
        File::create(&report).expect("watch report");
        Self {
            report,
            last: RefCell::new(None),
        }
    }

    /// Appends `line` unless it repeats the last one noted.
    pub fn note(&self, line: &str) {
        let mut last = self.last.borrow_mut();
        if last.as_deref() == Some(line) {
            return;
        }
        if let Ok(mut file) = OpenOptions::new().append(true).open(&self.report) {
            let _ = file.write_all(format!("{line}\n").as_bytes());
        }
        *last = Some(line.to_owned());
    }

    /// Past `deadline`: panics with `what` and every line reported so far.
    pub fn deadline_check(&self, deadline: Instant, what: &str) {
        if Instant::now() >= deadline {
            let report = fs::read_to_string(&self.report).unwrap_or_default();
            panic!("{what}\nwatch report:\n{report}");
        }
    }
}

/// A passing wait removes its report; a failed one keeps it for the post-mortem.
impl Drop for Watch {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            let _ = fs::remove_file(&self.report);
        }
    }
}
