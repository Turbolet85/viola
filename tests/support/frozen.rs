//! A wrapper held stopped until the guard drops, so its pid and start time still match while its
//! heartbeat ages: the one freeze every root test that needs a stale wrapper takes, by `SIGSTOP`.

use std::process::Command;

/// A wrapper held stopped by `SIGSTOP` until the guard drops.
pub struct Frozen(String);

impl Frozen {
    pub fn attach(pid: u32) -> Self {
        use std::time::Instant;

        let frozen = Self(pid.to_string());
        frozen.signal("-STOP");
        let watch = super::watch::Watch::start("stat");
        let deadline = Instant::now() + super::watch::WITHIN;
        loop {
            let stat = Command::new("ps")
                .args(["-o", "stat=", "-p", &frozen.0])
                .output()
                .expect("ps");
            let text = String::from_utf8_lossy(&stat.stdout);
            let state = text.trim_start();
            if state.starts_with('T') {
                return frozen;
            }
            watch.note(&format!("stat {}", state.chars().next().unwrap_or('-')));
            watch.deadline_check(deadline, "wrapper never stopped");
            std::thread::yield_now();
        }
    }

    fn signal(&self, sig: &str) {
        let ok = Command::new("kill")
            .args([sig, &self.0])
            .status()
            .expect("kill");
        assert!(ok.success(), "kill {sig}");
    }
}

impl Drop for Frozen {
    fn drop(&mut self) {
        self.signal("-CONT");
    }
}
