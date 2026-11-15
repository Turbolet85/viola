//! The test-only `viola-hypr-tap` spawned as it runs, against a planted `#!/bin/sh` stub `hyprctl`
//! that replays the committed `fixtures/hyprland/0.56.2/` replies (test-plan §7 Fixture hygiene;
//! security-plan · the Hyprland recorder's string allowlist): `record` and `record-dispatch` each
//! write their set, and every malformed argument form is refused exit 2 with nothing written. Every
//! child is owned and killed at a stated deadline; `HOME` is the case's own temp dir, so the scrub
//! has a home and a user word to strip.

use std::fs;
use std::io::Write as _;
use std::os::unix::fs::PermissionsExt as _;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::time::{Duration, Instant};

use serde_json::Value;

const TAP: &str = env!("CARGO_BIN_EXE_viola-hypr-tap");
const VERSION: &str = "0.56.2";
const WITHIN: Duration = Duration::from_secs(10);

/// The committed reply set the stub replays.
fn fixtures() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/hyprland")
        .join(VERSION)
}

/// The child's output once it exits, killed and failing the case past `WITHIN`.
fn finished(mut child: Child) -> Output {
    let until = Instant::now() + WITHIN;
    loop {
        if child.try_wait().expect("try_wait").is_some() {
            return child.wait_with_output().expect("output");
        }
        if Instant::now() >= until {
            let _ = child.kill();
            let _ = child.wait();
            panic!("the tap did not exit within {WITHIN:?}");
        }
        std::thread::sleep(Duration::from_millis(10));
    }
}

/// A stub `hyprctl` planted 0700 in `dir`: `-j <word>` prints the committed reply's `stdout` for
/// that word, a `dispatch` prints `ok`; every handle is closed before any spawn.
fn stub(dir: &Path) -> PathBuf {
    for (word, file) in [
        ("version", "Version.recorded-1.json"),
        ("clients", "Clients.recorded-1.json"),
        ("activewindow", "ActiveWindow.recorded-1.json"),
        ("workspaces", "Workspaces.recorded-1.json"),
    ] {
        let doc: Value =
            serde_json::from_slice(&fs::read(fixtures().join(file)).expect("a committed reply"))
                .expect("a reply document");
        let stdout = doc["stdout"].as_str().expect("a recorded stdout");
        fs::write(dir.join(format!("reply-{word}")), stdout).expect("the reply file");
    }
    let path = dir.join("hyprctl");
    let body = format!(
        "#!/bin/sh\ncase \"$1\" in\n  -j) cat '{}/reply-'\"$2\" ;;\n  dispatch) echo ok ;;\n  *) exit 1 ;;\nesac\n",
        dir.display()
    );
    let mut file = fs::File::create(&path).expect("create");
    file.write_all(body.as_bytes()).expect("write");
    file.sync_all().expect("sync");
    drop(file);
    fs::set_permissions(&path, fs::Permissions::from_mode(0o700)).expect("chmod");
    path
}

/// `viola-hypr-tap <args>` with `HOME` set to `home`.
fn tap(args: &[&str], home: &Path) -> Output {
    let child = Command::new(TAP)
        .args(args)
        .env("HOME", home)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("the tap");
    finished(child)
}

/// The case's own dir, its stub, and an empty out dir.
struct Rig {
    dir: tempfile::TempDir,
    program: String,
    out: PathBuf,
}

impl Rig {
    fn new() -> Self {
        let dir = tempfile::tempdir().expect("tempdir");
        let program = stub(dir.path()).to_str().expect("utf-8").to_owned();
        let out = dir.path().join("out");
        fs::create_dir(&out).expect("the out dir");
        Self { dir, program, out }
    }

    fn out_arg(&self) -> &str {
        self.out.to_str().expect("utf-8")
    }

    fn run(&self, args: &[&str]) -> Output {
        tap(args, self.dir.path())
    }

    /// Each of `names` written under `<out>/0.56.2/` and byte-identical to the committed reply it
    /// replays.
    fn assert_written(&self, names: &[&str], same_as_committed: bool) {
        let set = self.out.join(VERSION);
        for name in names {
            let written = fs::read(set.join(name)).unwrap_or_else(|_| panic!("{name} written"));
            if same_as_committed {
                let committed = fs::read(fixtures().join(name)).expect("committed");
                assert_eq!(written, committed, "{name}");
            }
        }
    }
}

#[test]
fn hypr_tap_record_writes_the_five_replies_from_a_stub_hyprctl() {
    let rig = Rig::new();
    let out = rig.run(&["record", "--program", &rig.program, "--out", rig.out_arg()]);
    assert_eq!(out.status.code(), Some(0), "{:?}", out.stderr);
    rig.assert_written(
        &[
            "Version.recorded-1.json",
            "Clients.recorded-1.json",
            "ActiveWindow.recorded-1.json",
            "Workspaces.recorded-1.json",
        ],
        true,
    );
    rig.assert_written(&["Version.absent-1.json"], false);
}

#[test]
fn hypr_tap_record_dispatch_writes_the_focus_and_workspace_replies() {
    let rig = Rig::new();
    let out = rig.run(&[
        "record-dispatch",
        "--program",
        &rig.program,
        "--out",
        rig.out_arg(),
    ]);
    assert_eq!(out.status.code(), Some(0), "{:?}", out.stderr);
    rig.assert_written(
        &["FocusWindow.recorded-1.json", "Workspace.recorded-1.json"],
        true,
    );
}

#[test]
fn hypr_tap_refuses_each_malformed_argument_form_with_exit_2_and_writes_nothing() {
    let rig = Rig::new();
    let program = rig.program.as_str();
    let out = rig.out_arg();
    let cases: [(&str, Vec<&str>); 6] = [
        (
            "--prog for --program",
            vec!["record", "--prog", program, "--out", out],
        ),
        (
            "--o for --out",
            vec!["record", "--program", program, "--o", out],
        ),
        ("both wrong", vec!["record", "--prog", program, "--o", out]),
        (
            "an unknown verb",
            vec!["recorded", "--program", program, "--out", out],
        ),
        (
            "a relative program",
            vec!["record", "--program", "hyprctl", "--out", out],
        ),
        (
            "four arguments",
            vec!["record", "--program", program, "--out"],
        ),
    ];
    for (label, args) in cases {
        let ran = rig.run(&args);
        assert_eq!(ran.status.code(), Some(2), "{label}");
        let written = fs::read_dir(&rig.out).expect("the out dir").count();
        assert_eq!(written, 0, "{label}");
    }
}
