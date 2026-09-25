//! The module ↔ PTY boundary (test-plan §5): the fake agent spawned for real through `viola-pty`.
//! Exit is read on the process handle while the output is still held open, and a resize reaches
//! the child as a `size` receipt.

#[allow(dead_code)]
mod support;

use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::time::Instant;

use rstest::rstest;
use serde_json::json;
use support::fake::{self, FAKE, of_kind};
use support::home::{TestHome, home};
use support::outer_pty::{EXIT_WITHIN, OuterPty};
use viola_pty::Size;

fn fake_in_pty(receipt: &Path, extra: &[&str], size: Size) -> OuterPty {
    let mut args: Vec<OsString> = vec!["--receipt".into(), receipt.into()];
    args.extend(extra.iter().map(OsString::from));
    OuterPty::spawn_sized(Path::new(FAKE), &args, &[], size)
}

fn receipt_in(tmp: &TestHome) -> PathBuf {
    tmp.scratch().join("seam.receipt.ndjson")
}

#[rstest]
fn pty_exit_is_read_on_the_handle_while_the_output_is_held(#[from(home)] tmp: TestHome) {
    let receipt = receipt_in(&tmp);
    let mut pty = fake_in_pty(&receipt, &["--exit-no-eof"], Size::DEFAULT);
    fake::wait_for(&receipt, "start", |l| !of_kind(l, "start").is_empty());
    pty.write(b"\x03");
    let deadline = Instant::now() + EXIT_WITHIN;
    let code = loop {
        if let Some(code) = pty.try_wait() {
            break code;
        }
        assert!(Instant::now() < deadline, "exit never read on the handle");
        std::thread::yield_now();
    };
    assert_eq!(code, 0);
    // The holder's own receipt splits the two reds: it never started, or it started and the
    // output still ended.
    fake::wait_for(&receipt, "hold", |l| !of_kind(l, "hold").is_empty());
    assert!(
        !pty.drained(),
        "the output ended although the holder started (hold receipt present)"
    );
}

#[rstest]
fn pty_resize_reaches_the_child(#[from(home)] tmp: TestHome) {
    let receipt = receipt_in(&tmp);
    let mut pty = fake_in_pty(&receipt, &[], Size { cols: 80, rows: 24 });
    fake::wait_for(&receipt, "first size", |l| {
        of_kind(l, "size").contains(&&json!({"v": 1, "kind": "size", "cols": 80, "rows": 24}))
    });
    pty.resize(Size {
        cols: 100,
        rows: 30,
    });
    pty.write(b"a");
    fake::wait_for(&receipt, "resized", |l| {
        of_kind(l, "size").contains(&&json!({"v": 1, "kind": "size", "cols": 100, "rows": 30}))
    });
    pty.write(b"\x03");
    assert_eq!(pty.wait_exit(EXIT_WITHIN), 0);
}
