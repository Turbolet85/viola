//! A wrapper held stopped until the guard drops, so its pid and start time still match while its
//! heartbeat ages: the one freeze every root test that needs a stale wrapper takes. Unix stops it
//! by `SIGSTOP`, Windows by a debugger attach.

#[cfg(unix)]
use std::process::Command;

/// A wrapper held stopped by `SIGSTOP` until the guard drops: the Unix twin of the Windows guard.
#[cfg(unix)]
pub struct Frozen(String);

#[cfg(unix)]
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

#[cfg(unix)]
impl Drop for Frozen {
    fn drop(&mut self) {
        self.signal("-CONT");
    }
}

/// A wrapper frozen under a debugger attach, the Windows form of `SIGSTOP`: every thread of the
/// process stays suspended while its first debug event is unanswered. Detached on drop, so a failing
/// test never leaves the wrapper frozen, and never killed: the attach is set not to kill on exit.
#[cfg(windows)]
pub struct Frozen(u32);

#[cfg(windows)]
impl Frozen {
    pub fn attach(pid: u32) -> Self {
        use windows_sys::Win32::Foundation::CloseHandle;
        use windows_sys::Win32::System::Diagnostics::Debug::{
            CREATE_PROCESS_DEBUG_EVENT, DEBUG_EVENT, DebugActiveProcess, DebugSetProcessKillOnExit,
            WaitForDebugEvent,
        };

        // SAFETY: plain values cross; no pointer is kept.
        assert_ne!(unsafe { DebugActiveProcess(pid) }, 0, "debugger attach");
        let frozen = Self(pid);
        // SAFETY: a flag value for this thread's debuggees.
        assert_ne!(unsafe { DebugSetProcessKillOnExit(0) }, 0, "keep on exit");
        let wait_ms = u32::try_from(super::watch::WITHIN.as_millis()).expect("ms");
        // SAFETY: `event` is a writable DEBUG_EVENT for the call's duration.
        let mut event: DEBUG_EVENT = unsafe { std::mem::zeroed() };
        // SAFETY: as above; the event is read only after the call reported it written.
        assert_ne!(
            unsafe { WaitForDebugEvent(&mut event, wait_ms) },
            0,
            "first debug event"
        );
        assert_eq!(event.dwProcessId, pid);
        assert_eq!(event.dwDebugEventCode, CREATE_PROCESS_DEBUG_EVENT);
        // SAFETY: the code read above says this union member is the one written; the file handle
        // is the debugger's to close.
        unsafe {
            let file = event.u.CreateProcessInfo.hFile;
            if !file.is_null() {
                CloseHandle(file);
            }
        }
        frozen
    }
}

#[cfg(windows)]
impl Drop for Frozen {
    fn drop(&mut self) {
        // SAFETY: a plain pid; the same thread that attached detaches.
        unsafe {
            windows_sys::Win32::System::Diagnostics::Debug::DebugActiveProcessStop(self.0);
        }
    }
}
