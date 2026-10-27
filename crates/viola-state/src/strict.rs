//! The strict-modes check (security-plan §Authentication & Authorization, `~/.viola/` access control),
//! first wired on `run`'s `ledger/stamps.json` read: a trusted file, or a folder holding one, that
//! another user could write is refused before it is read (ssh StrictModes style); a folder whose
//! content another user must not read (the home, an instance's directory and its `diagnostics/`) is
//! held to the read rule as well. Unix reads the owner and the mode; Windows reads the owner and the
//! DACL. The decisions are plain functions tested on every OS; only the readers are per OS.

use std::io;
use std::path::{Path, PathBuf};

use viola_core::ViolaName;

use crate::events::EVENTS;
use crate::snapshot::SNAPSHOT;
use crate::stamps::{STAMPS, ledger_dir};

/// Why a path failed the check: fixed messages only (security-plan §Error Handling).
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum Refused {
    #[error("a trusted state path is not owned by this user")]
    NotOwner,
    #[error("a trusted state path is writable by another user")]
    Writable,
    #[error("a trusted state path is readable by another user")]
    Readable,
    #[error("a trusted state path is a symbolic link")]
    Symlink,
    #[error("a trusted state path's permissions could not be read")]
    Unreadable,
    #[error("a trusted state path has a NULL DACL")]
    NullDacl,
    #[error("a trusted state path is on a volume without persistent ACLs")]
    NoPersistentAcls,
}

/// The rule a path is held to: no write by another user, or no access by another user at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Rule {
    Write,
    Read,
}

/// The instance's `diagnostics/` folder.
pub const DIAGNOSTICS: &str = "diagnostics";

/// `events.ndjson` on Windows takes the read rule: appended across sessions and never replaced, an
/// explicit ACE on it survives, and bypass-traverse reaches it by its predictable path whatever its
/// folders grant. On Unix its folders' `0o077` keeps it from other users.
#[cfg(windows)]
const EVENTS_RULE: Rule = Rule::Read;
#[cfg(not(windows))]
const EVENTS_RULE: Rule = Rule::Write;

/// A diagnostics file on Unix takes the read rule (it is created 0600); on Windows the write rule,
/// its folder's read rule covering what it inherits.
#[cfg(unix)]
const DIAGNOSTICS_FILE_RULE: Rule = Rule::Read;
#[cfg(not(unix))]
const DIAGNOSTICS_FILE_RULE: Rule = Rule::Write;

/// `ledger/` and `ledger/stamps.json`, each that exists, in that order: the folder first, since an
/// entry written by another user between the checks would land through it.
pub fn check_stamps(home: &Path) -> Result<(), Refused> {
    let dir = ledger_dir(home);
    check_existing(&[(dir.clone(), Rule::Write), (dir.join(STAMPS), Rule::Write)])
}

/// The home, `instances/`, the instance directory, its `diagnostics/`, its `snapshot.json` and its
/// `events.ndjson`, each that exists, a folder before what it holds: the check a process runs
/// before it uses a value read from either file or opens anything under the instance. The home, the
/// instance directory and `diagnostics/` take the read rule, the rest the write rule.
pub fn check_instance(home: &Path, instance_dir: &Path) -> Result<(), Refused> {
    check_existing(&[
        (home.to_path_buf(), Rule::Read),
        (home.join("instances"), Rule::Write),
        (instance_dir.to_path_buf(), Rule::Read),
        (instance_dir.join(DIAGNOSTICS), Rule::Read),
        (instance_dir.join(SNAPSHOT), Rule::Write),
        (instance_dir.join(EVENTS), EVENTS_RULE),
    ])
}

/// Which side of the home tree [`check_home_tree`] refused: the caller prints the home's pair or
/// the instance's.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TreeRefused {
    HomeLevel(Refused),
    InstanceLevel(Refused),
}

/// The tree a starting process trusts, each path that exists: the home (the read rule); with a
/// name, the instance's set as [`check_instance`] reads it; then `ledger/`, `bin/` and `plugin/`
/// and, with a pinned copy's key, `bin/<key>/`, the pinned `viola(.exe)`, `plugin/<key>/` and its
/// `hooks/` and `.claude-plugin/` (the write rule).
pub fn check_home_tree(
    home: &Path,
    name: Option<&ViolaName>,
    key: Option<&str>,
) -> Result<(), TreeRefused> {
    check_existing(&[(home.to_path_buf(), Rule::Read)]).map_err(TreeRefused::HomeLevel)?;
    if let Some(name) = name {
        let instance_dir = home.join("instances").join(AsRef::<str>::as_ref(name));
        check_instance(home, &instance_dir).map_err(TreeRefused::InstanceLevel)?;
    }
    let mut code = vec![
        (ledger_dir(home), Rule::Write),
        (home.join("bin"), Rule::Write),
        (home.join("plugin"), Rule::Write),
    ];
    if let Some(key) = key {
        let bin = home.join("bin").join(key);
        let plugin = home.join("plugin").join(key);
        code.extend([
            (bin.clone(), Rule::Write),
            (
                bin.join(format!("viola{}", std::env::consts::EXE_SUFFIX)),
                Rule::Write,
            ),
            (plugin.clone(), Rule::Write),
            (plugin.join("hooks"), Rule::Write),
            (plugin.join(".claude-plugin"), Rule::Write),
        ]);
    }
    check_existing(&code).map_err(TreeRefused::HomeLevel)
}

/// A diagnostics folder or file, before it is created or opened: a symlink is refused (an open
/// through it lands wherever it points), then a folder takes the read rule and a file the write
/// rule, and on Unix no group or other bit. An absent path passes.
pub fn check_diagnostics(path: &Path) -> Result<(), Refused> {
    let meta = match std::fs::symlink_metadata(path) {
        Ok(meta) => meta,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(()),
        Err(_) => return Err(Refused::Unreadable),
    };
    if meta.file_type().is_symlink() {
        return Err(Refused::Symlink);
    }
    let rule = if meta.is_dir() {
        Rule::Read
    } else {
        DIAGNOSTICS_FILE_RULE
    };
    check_rule(path, rule)
}

/// Each path that exists, in order, under its rule; an absent one is skipped, and one that cannot
/// be statted is refused.
fn check_existing(paths: &[(PathBuf, Rule)]) -> Result<(), Refused> {
    for (path, rule) in paths {
        match std::fs::symlink_metadata(path) {
            Ok(_) => check_rule(path, *rule)?,
            Err(e) if e.kind() == io::ErrorKind::NotFound => {}
            Err(_) => return Err(Refused::Unreadable),
        }
    }
    Ok(())
}

/// The write rule on `path` when it exists; one that cannot be statted is refused.
pub fn check_present(path: &Path) -> Result<(), Refused> {
    check_existing(&[(path.to_path_buf(), Rule::Write)])
}

fn check_rule(path: &Path, rule: Rule) -> Result<(), Refused> {
    match rule {
        Rule::Write => check_path(path),
        Rule::Read => check_read(path),
    }
}

/// One path through this OS's reader and the write rule.
pub fn check_path(path: &Path) -> Result<(), Refused> {
    #[cfg(unix)]
    {
        let (mode, uid) = unix::reading(path).map_err(|_| Refused::Unreadable)?;
        unix_verdict(mode, uid, unix::euid())
    }
    #[cfg(windows)]
    {
        win::check(path, windows_verdict)
    }
}

/// One path through this OS's reader and the read rule.
pub fn check_read(path: &Path) -> Result<(), Refused> {
    #[cfg(unix)]
    {
        let (mode, uid) = unix::reading(path).map_err(|_| Refused::Unreadable)?;
        unix_read_verdict(mode, uid, unix::euid())
    }
    #[cfg(windows)]
    {
        win::check(path, windows_read_verdict)
    }
}

/// Owned by the effective user, and no group or other write bit.
pub fn unix_verdict(mode: u32, uid: u32, euid: u32) -> Result<(), Refused> {
    if uid != euid {
        return Err(Refused::NotOwner);
    }
    if mode & 0o022 != 0 {
        return Err(Refused::Writable);
    }
    Ok(())
}

/// The write rule, then no group or other bit at all (security-plan detail
/// authentication-authorization.md, the read rule).
pub fn unix_read_verdict(mode: u32, uid: u32, euid: u32) -> Result<(), Refused> {
    unix_verdict(mode, uid, euid)?;
    if mode & 0o077 != 0 {
        return Err(Refused::Readable);
    }
    Ok(())
}

/// The write rights the Windows rule refuses to any other SID: `FILE_WRITE_DATA` / `FILE_ADD_FILE`
/// (0x2), `FILE_APPEND_DATA` / `FILE_ADD_SUBDIRECTORY` (0x4), `FILE_DELETE_CHILD` (0x40), `DELETE`
/// (0x1_0000), `WRITE_DAC` (0x4_0000), `WRITE_OWNER` (0x8_0000), `GENERIC_ALL` (0x1000_0000) and
/// `GENERIC_WRITE` (0x4000_0000), written as one value (the Windows test pins it to the names).
pub const WRITE_RIGHTS: u32 = 0x500D_0046;

/// The read rights the Windows read rule refuses to any other SID: `FILE_READ_DATA` /
/// `FILE_LIST_DIRECTORY` (0x1), `GENERIC_ALL` (0x1000_0000) and `GENERIC_READ` (0x8000_0000), written
/// as one value (the Windows test pins it to the names).
pub const READ_RIGHTS: u32 = 0x9000_0001;

/// `SYSTEM`, `Administrators` and `CREATOR OWNER`, the SIDs trusted beside the user's own.
const TRUSTED_SIDS: [&str; 3] = ["S-1-5-18", "S-1-5-32-544", "S-1-3-0"];

/// The user's SID or one of [`TRUSTED_SIDS`].
pub fn trusted_sid(sid: &str, user: &str) -> bool {
    sid == user || TRUSTED_SIDS.contains(&sid)
}

/// `FILE_PERSISTENT_ACLS` among a volume's flags.
const PERSISTENT_ACLS: u32 = 0x8;

/// Whether a volume with these flags keeps ACLs (the flags `GetVolumeInformationW` reads).
pub fn volume_keeps_acls(flags: u32) -> bool {
    flags & PERSISTENT_ACLS != 0
}

/// One DACL entry as the Windows reader sees it: an allow ACE's trustee, access mask (an
/// inherit-only ACE included, its generic bits unmapped) and `AceFlags` (inheritance).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Allow {
    pub sid: String,
    pub mask: u32,
    pub flags: u8,
}

/// The owner is the user or Administrators; no allow ACE grants a write right to another SID. A
/// NULL DACL and a volume without persistent ACLs are refused before the entries are read.
pub fn windows_verdict(
    owner: &str,
    user: &str,
    dacl: Option<&[Allow]>,
    persistent_acls: bool,
) -> Result<(), Refused> {
    if !persistent_acls {
        return Err(Refused::NoPersistentAcls);
    }
    if owner != user && owner != "S-1-5-32-544" {
        return Err(Refused::NotOwner);
    }
    let dacl = dacl.ok_or(Refused::NullDacl)?;
    if dacl
        .iter()
        .any(|ace| ace.mask & WRITE_RIGHTS != 0 && !trusted_sid(&ace.sid, user))
    {
        return Err(Refused::Writable);
    }
    Ok(())
}

/// The write rule, then no allow ACE grants a read right to another SID, an inherit-only ACE's
/// unmapped generic bits included.
pub fn windows_read_verdict(
    owner: &str,
    user: &str,
    dacl: Option<&[Allow]>,
    persistent_acls: bool,
) -> Result<(), Refused> {
    windows_verdict(owner, user, dacl, persistent_acls)?;
    if dacl
        .unwrap_or_default()
        .iter()
        .any(|ace| ace.mask & READ_RIGHTS != 0 && !trusted_sid(&ace.sid, user))
    {
        return Err(Refused::Readable);
    }
    Ok(())
}

#[cfg(unix)]
mod unix {
    use std::io;
    use std::os::unix::fs::MetadataExt as _;
    use std::path::Path;

    /// The mode bits and the owner uid; a symlink is followed, as `stat` does.
    pub(super) fn reading(path: &Path) -> io::Result<(u32, u32)> {
        let meta = std::fs::metadata(path)?;
        Ok((meta.mode(), meta.uid()))
    }

    pub(super) fn euid() -> u32 {
        // SAFETY: `geteuid` takes no arguments, cannot fail and touches no memory.
        unsafe { libc::geteuid() }
    }
}

#[cfg(windows)]
pub(crate) mod win {
    use std::os::windows::ffi::OsStrExt as _;
    use std::path::Path;
    use std::ptr;

    use windows_sys::Win32::Foundation::{CloseHandle, HANDLE, LocalFree};
    use windows_sys::Win32::Security::Authorization::{
        ConvertSidToStringSidW, GetNamedSecurityInfoW, SE_FILE_OBJECT,
    };
    use windows_sys::Win32::Security::{
        ACCESS_ALLOWED_ACE, ACE_HEADER, ACL, GetAce, GetTokenInformation, PSECURITY_DESCRIPTOR,
        PSID, TOKEN_QUERY, TOKEN_USER, TokenUser,
    };
    use windows_sys::Win32::Storage::FileSystem::{GetVolumeInformationW, GetVolumePathNameW};
    use windows_sys::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};
    use windows_sys::core::PWSTR;

    use super::{Allow, Refused, volume_keeps_acls};

    /// `ACCESS_ALLOWED_ACE_TYPE`: the one ACE type that grants (its windows-sys home,
    /// `Win32_System_SystemServices`, is a feature this workspace does not enable).
    const ACCESS_ALLOWED: u8 = 0;

    /// What `owner_and_dacl` reads: `OWNER_SECURITY_INFORMATION` (0x1) and
    /// `DACL_SECURITY_INFORMATION` (0x4), written as one value (the Windows test pins it to the
    /// names).
    const OWNER_AND_DACL: u32 = 0x5;

    fn wide(path: &Path) -> Vec<u16> {
        path.as_os_str().encode_wide().chain(Some(0)).collect()
    }

    /// The decision a rule makes over one path's reading.
    pub(super) type Verdict = fn(&str, &str, Option<&[Allow]>, bool) -> Result<(), Refused>;

    pub(super) fn check(path: &Path, verdict: Verdict) -> Result<(), Refused> {
        let user = user_sid().ok_or(Refused::Unreadable)?;
        let persistent = persistent_acls(path).ok_or(Refused::Unreadable)?;
        let (owner, dacl) = owner_and_dacl(path).ok_or(Refused::Unreadable)?;
        verdict(&owner, &user, dacl.as_deref(), persistent)
    }

    fn persistent_acls(path: &Path) -> Option<bool> {
        let name = wide(path);
        let mut root = vec![0u16; 1024];
        let len = u32::try_from(root.len()).ok()?;
        // SAFETY: `name` is NUL-terminated; `root` holds `len` u16s and outlives the call.
        if unsafe { GetVolumePathNameW(name.as_ptr(), root.as_mut_ptr(), len) } == 0 {
            return None;
        }
        let mut flags = 0u32;
        // SAFETY: `root` now holds a NUL-terminated volume path; the out-buffers are null with
        // zero sizes, and `flags` is a local out-pointer.
        let read = unsafe {
            GetVolumeInformationW(
                root.as_ptr(),
                ptr::null_mut(),
                0,
                ptr::null_mut(),
                ptr::null_mut(),
                &mut flags,
                ptr::null_mut(),
                0,
            )
        };
        (read != 0).then_some(volume_keeps_acls(flags))
    }

    /// The owner SID and the allow ACEs (`None` for a NULL DACL).
    fn owner_and_dacl(path: &Path) -> Option<(String, Option<Vec<Allow>>)> {
        let name = wide(path);
        let mut owner: PSID = ptr::null_mut();
        let mut dacl: *mut ACL = ptr::null_mut();
        let mut sd: PSECURITY_DESCRIPTOR = ptr::null_mut();
        // SAFETY: `name` is NUL-terminated; every out-pointer is a local; `sd` is LocalAlloc'd by
        // the call and freed below, and `owner` / `dacl` point into it.
        let status = unsafe {
            GetNamedSecurityInfoW(
                name.as_ptr(),
                SE_FILE_OBJECT,
                OWNER_AND_DACL,
                &mut owner,
                ptr::null_mut(),
                &mut dacl,
                ptr::null_mut(),
                &mut sd,
            )
        };
        if status != 0 {
            return None;
        }
        let read = sid_string(owner).map(|owner| (owner, aces(dacl)));
        // SAFETY: `sd` was allocated by the call above and is freed exactly once.
        unsafe { LocalFree(sd) };
        match read {
            Some((owner, Some(aces))) => Some((owner, aces)),
            _ => None,
        }
    }

    /// `Some(None)` for a NULL DACL; `None` when an entry cannot be read.
    fn aces(dacl: *mut ACL) -> Option<Option<Vec<Allow>>> {
        if dacl.is_null() {
            return Some(None);
        }
        // SAFETY: a non-null DACL inside the security descriptor still held by the caller.
        let count = unsafe { (*dacl).AceCount };
        let mut allows = Vec::new();
        for i in 0..u32::from(count) {
            let mut ace: *mut core::ffi::c_void = ptr::null_mut();
            // SAFETY: `i` is below the DACL's own count; `ace` is a local out-pointer.
            if unsafe { GetAce(dacl, i, &mut ace) } == 0 {
                return None;
            }
            // SAFETY: every ACE starts with an ACE_HEADER.
            let header = unsafe { &*ace.cast::<ACE_HEADER>() };
            if header.AceType != ACCESS_ALLOWED {
                continue;
            }
            // SAFETY: an ACCESS_ALLOWED ACE is an ACCESS_ALLOWED_ACE; its SID starts at
            // `SidStart` and lies inside the ACE.
            let allowed = unsafe { &*ace.cast::<ACCESS_ALLOWED_ACE>() };
            let sid = ptr::addr_of!(allowed.SidStart).cast_mut().cast();
            allows.push(Allow {
                sid: sid_string(sid)?,
                mask: allowed.Mask,
                flags: header.AceFlags,
            });
        }
        Some(Some(allows))
    }

    fn sid_string(sid: PSID) -> Option<String> {
        let mut text: PWSTR = ptr::null_mut();
        // SAFETY: `sid` points into a live security descriptor or token buffer; `text` receives a
        // LocalAlloc'd string freed below.
        if unsafe { ConvertSidToStringSidW(sid, &mut text) } == 0 {
            return None;
        }
        // SAFETY: `text` is the NUL-terminated string the call above allocated.
        let len = (0..).take_while(|&i| unsafe { *text.add(i) } != 0).count();
        // SAFETY: `len` u16s were just read from `text`.
        let out = String::from_utf16(unsafe { std::slice::from_raw_parts(text, len) }).ok();
        // SAFETY: allocated by ConvertSidToStringSidW with LocalAlloc; freed once.
        unsafe { LocalFree(text.cast()) };
        out
    }

    /// The allow ACEs of `path`'s DACL (`None` when unreadable or NULL).
    #[cfg(test)]
    pub(crate) fn allows(path: &Path) -> Option<Vec<Allow>> {
        owner_and_dacl(path).and_then(|(_, dacl)| dacl)
    }

    /// The process token's user SID, as `S-1-…`.
    pub(crate) fn user_sid() -> Option<String> {
        let mut token: HANDLE = ptr::null_mut();
        // SAFETY: the current-process pseudo-handle needs no close; `token` is a local out-pointer.
        if unsafe { OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token) } == 0 {
            return None;
        }
        let mut len = 0u32;
        // SAFETY: a size query: a null buffer of length 0, `len` a local out-pointer.
        unsafe { GetTokenInformation(token, TokenUser, ptr::null_mut(), 0, &mut len) };
        // u64 words: the buffer must be aligned for TOKEN_USER's pointer field.
        let mut buf = vec![0u64; usize::try_from(len).unwrap_or(0).div_ceil(8)];
        // SAFETY: `buf` holds at least `len` bytes and outlives the call.
        let filled = unsafe {
            GetTokenInformation(token, TokenUser, buf.as_mut_ptr().cast(), len, &mut len)
        };
        let sid = (filled != 0)
            .then(|| {
                // SAFETY: the call above wrote a TOKEN_USER at the start of the aligned buffer.
                let user = unsafe { &*buf.as_ptr().cast::<TOKEN_USER>() };
                sid_string(user.User.Sid)
            })
            .flatten();
        // SAFETY: `token` was opened above and is closed exactly once.
        unsafe { CloseHandle(token) };
        sid
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn write_rights_are_the_named_flags() {
            use windows_sys::Win32::Foundation::{GENERIC_ALL, GENERIC_WRITE};
            use windows_sys::Win32::Storage::FileSystem::{
                DELETE, FILE_APPEND_DATA, FILE_DELETE_CHILD, FILE_WRITE_DATA, WRITE_DAC,
                WRITE_OWNER,
            };
            let named = FILE_WRITE_DATA
                | FILE_APPEND_DATA
                | FILE_DELETE_CHILD
                | DELETE
                | WRITE_DAC
                | WRITE_OWNER
                | GENERIC_ALL
                | GENERIC_WRITE;
            assert_eq!(super::super::WRITE_RIGHTS, named);
        }

        #[test]
        fn read_rights_are_the_named_flags() {
            use windows_sys::Win32::Foundation::{GENERIC_ALL, GENERIC_READ};
            use windows_sys::Win32::Storage::FileSystem::FILE_READ_DATA;
            assert_eq!(
                super::super::READ_RIGHTS,
                FILE_READ_DATA | GENERIC_ALL | GENERIC_READ
            );
        }

        #[test]
        fn owner_and_dacl_is_the_named_flags() {
            use windows_sys::Win32::Security::{
                DACL_SECURITY_INFORMATION, OWNER_SECURITY_INFORMATION,
            };
            assert_eq!(
                OWNER_AND_DACL,
                OWNER_SECURITY_INFORMATION | DACL_SECURITY_INFORMATION
            );
        }

        /// A drive letter no volume is mounted on: its flags cannot be read, so the reader has no
        /// answer and the check refuses the path as unreadable rather than assuming ACLs.
        #[test]
        fn persistent_acls_of_a_volume_that_cannot_be_read_is_none() {
            let unmounted = ('D'..='Z')
                .rev()
                .map(|letter| format!(r"{letter}:\"))
                .find(|root| !Path::new(root).exists())
                .expect("a drive letter with no volume");
            let path = Path::new(&unmounted).join("viola").join("ledger");
            assert_eq!(persistent_acls(&path), None, "{unmounted}");
            assert_eq!(
                check(&path, super::super::windows_verdict),
                Err(Refused::Unreadable),
                "{unmounted}"
            );
        }

        #[test]
        fn user_sid_is_a_user_sid() {
            let sid = user_sid().expect("sid");
            assert!(sid.starts_with("S-1-5-"), "{sid}");
        }

        /// `INHERITED_ACE` in `AceFlags`.
        const INHERITED: u8 = 0x10;

        /// Every reading the verdict rests on, for one path: the verdict, owner, user, volume and
        /// each allow ACE (SID, mask, flags, inherited, whether it carries a write right). SIDs and
        /// codes only, never a path.
        fn reading_of(label: &str, path: &Path) -> String {
            let verdict = check(path, super::super::windows_verdict);
            let user = user_sid();
            let persistent = persistent_acls(path);
            let mut text = format!(
                "{label}: verdict {verdict:?} user {user:?} persistent_acls {persistent:?}"
            );
            match owner_and_dacl(path) {
                None => text.push_str(" owner/dacl unreadable"),
                Some((owner, None)) => text.push_str(&format!(" owner {owner} dacl NULL")),
                Some((owner, Some(aces))) => {
                    text.push_str(&format!(" owner {owner} aces {}", aces.len()));
                    for ace in aces {
                        text.push_str(&format!(
                            "\n    allow {} mask 0x{:08X} flags 0x{:02X} inherited {} write {} trusted {}",
                            ace.sid,
                            ace.mask,
                            ace.flags,
                            ace.flags & INHERITED != 0,
                            ace.mask & super::super::WRITE_RIGHTS != 0,
                            user.as_deref()
                                .is_some_and(|u| super::super::trusted_sid(&ace.sid, u)),
                        ));
                    }
                }
            }
            text
        }

        /// A home where the CI test homes live, under the workspace's `target/e2e-home` (outside
        /// `%USERPROFILE%`): a ledger `viola verify` would write there must pass the check. A
        /// refusal names the variant and every ACE of the ledger folder and the stamps file
        /// (chunk 2026-10-04-dialog-answers-by-dialog-id, ci#37213772796 `test (windows-2025)`).
        #[test]
        fn check_stamps_of_a_home_under_the_workspace_target_passes() {
            let (_tmp, home) = home_viola_wrote();
            assert_eq!(
                super::super::check_stamps(&home),
                Ok(()),
                "{}",
                readings(&home)
            );
        }

        /// A home with a ledger and a stamps file viola wrote, under the workspace's
        /// `target/e2e-home`.
        fn home_viola_wrote() -> (tempfile::TempDir, std::path::PathBuf) {
            let base = Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("..")
                .join("..")
                .join("target")
                .join("e2e-home");
            std::fs::create_dir_all(&base).expect("e2e-home");
            let tmp = tempfile::tempdir_in(&base).expect("tempdir");
            let home = tmp.path().join("home");
            crate::stamps::update_stamps(&home, |_| b"{}".to_vec()).expect("written");
            (tmp, home)
        }

        fn readings(home: &Path) -> String {
            let ledger = crate::stamps::ledger_dir(home);
            format!(
                "\n{}\n{}\nhome {}",
                reading_of("ledger", &ledger),
                reading_of("stamps", &ledger.join(super::super::STAMPS)),
                reading_of("home", home)
            )
        }

        /// A protected DACL that keeps the user's and SYSTEM's full access, inherited below it as
        /// `inherit` says, and lets Everyone write data (`FILE_WRITE_DATA`, 0x2) on this one
        /// object. The user's access stays whole, so the home is still removable.
        fn everyone_may_write(inherit: &str) -> String {
            let user = user_sid().expect("user sid");
            format!("D:P(A;{inherit};FA;;;{user})(A;{inherit};FA;;;SY)(A;;0x2;;;WD)")
        }

        /// The ledger folder alone is widened: the check refuses the folder, and so the home.
        #[test]
        fn check_stamps_refuses_a_ledger_folder_everyone_may_write() {
            let (_tmp, home) = home_viola_wrote();
            let ledger = crate::stamps::ledger_dir(&home);
            assert_eq!(
                super::super::check_stamps(&home),
                Ok(()),
                "{}",
                readings(&home)
            );
            crate::fs::win::set_dacl(&ledger, &everyone_may_write("OICI")).expect("widened");
            assert_eq!(
                super::super::check_path(&ledger),
                Err(Refused::Writable),
                "{}",
                readings(&home)
            );
            assert_eq!(
                super::super::check_stamps(&home),
                Err(Refused::Writable),
                "{}",
                readings(&home)
            );
        }

        /// The stamps file alone is widened: its folder still passes, the file is refused, and so
        /// is the home, whose check goes on from the folder to the file.
        #[test]
        fn check_stamps_refuses_a_stamps_file_everyone_may_write() {
            let (_tmp, home) = home_viola_wrote();
            let ledger = crate::stamps::ledger_dir(&home);
            let stamps = ledger.join(super::super::STAMPS);
            assert_eq!(
                super::super::check_stamps(&home),
                Ok(()),
                "{}",
                readings(&home)
            );
            crate::fs::win::set_dacl(&stamps, &everyone_may_write("")).expect("widened");
            assert_eq!(
                super::super::check_path(&ledger),
                Ok(()),
                "{}",
                readings(&home)
            );
            assert_eq!(
                super::super::check_path(&stamps),
                Err(Refused::Writable),
                "{}",
                readings(&home)
            );
            assert_eq!(
                super::super::check_stamps(&home),
                Err(Refused::Writable),
                "{}",
                readings(&home)
            );
        }

        /// `icacls <path> <args…>`, its exit asserted: a grant the test relies on has landed.
        fn icacls(path: &Path, args: &[&str]) {
            let root = std::env::var_os("SystemRoot").expect("SystemRoot");
            let status = std::process::Command::new(
                std::path::PathBuf::from(root)
                    .join("System32")
                    .join("icacls.exe"),
            )
            .arg(path)
            .args(args)
            .stdout(std::process::Stdio::null())
            .status()
            .expect("icacls");
            assert!(status.success(), "icacls {args:?}: {status}");
        }

        /// An instance tree made through `create_private_dir` under the workspace's
        /// `target/e2e-home` (outside `%USERPROFILE%`, so the protected DACL), with its
        /// `diagnostics/`, a snapshot-shaped file and a log-shaped file.
        fn instance_tree() -> (tempfile::TempDir, std::path::PathBuf, std::path::PathBuf) {
            let base = Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("..")
                .join("..")
                .join("target")
                .join("e2e-home");
            std::fs::create_dir_all(&base).expect("e2e-home");
            let tmp = tempfile::tempdir_in(&base).expect("tempdir");
            let home = tmp.path().join("home");
            let instance = home.join("instances").join("builder");
            crate::fs::create_private_dir(&instance.join(super::super::DIAGNOSTICS))
                .expect("the tree");
            crate::fs::replace_private(&instance.join(crate::snapshot::SNAPSHOT), b"{}", 0o600)
                .expect("snapshot");
            crate::fs::open_private_append(&instance.join(crate::events::EVENTS)).expect("log");
            (tmp, home, instance)
        }

        /// The allow ACEs of `home` that the check refuses it for: inherited, granting a write
        /// right, to a SID it does not trust.
        fn inherited_untrusted_writes(home: &Path) -> Vec<Allow> {
            let user = user_sid().expect("user sid");
            allows(home)
                .expect("the home's DACL")
                .into_iter()
                .filter(|ace| {
                    ace.mask & super::super::WRITE_RIGHTS != 0
                        && ace.flags & INHERITED != 0
                        && !super::super::trusted_sid(&ace.sid, &user)
                })
                .collect()
        }

        /// A home a test makes with a plain `create_dir_all` under the workspace's `target/e2e-home`
        /// (the seeded-home question, CARRY 13): on the runner it inherits a write grant from the
        /// drive, so the check refuses it `Writable` — a home made before viola runs is one the
        /// product must refuse. The oracle reads that grant by its right, whichever SID holds it:
        /// the runners differ (Users held the write on `ci#40403389629`'s host, Authenticated Users
        /// `S-1-5-11` on `ci#40425866096`'s, both image `windows-2025-vs2026` 20261021.278.1). Its
        /// control: a home viola made (the protected DACL) holds no such grant, so the oracle
        /// cannot pass vacuously. A failure prints every ACE of the home.
        #[test]
        fn strict_instance_refuses_a_plain_home_made_before_viola_and_names_the_entry() {
            let base = Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("..")
                .join("..")
                .join("target")
                .join("e2e-home");
            std::fs::create_dir_all(&base).expect("e2e-home");
            let tmp = tempfile::tempdir_in(&base).expect("tempdir");
            let home = tmp.path().join("home");
            let instance = home.join("instances").join("builder");
            std::fs::create_dir_all(&instance).expect("a plain tree");
            let reading = reading_of("plain home", &home);
            assert_eq!(
                super::super::check_instance(&home, &instance),
                Err(Refused::Writable),
                "{reading}"
            );
            assert!(!inherited_untrusted_writes(&home).is_empty(), "{reading}");
            let (_made_tmp, made_home, _made) = instance_tree();
            assert_eq!(
                inherited_untrusted_writes(&made_home),
                [],
                "{}",
                reading_of("viola-made home", &made_home)
            );
        }

        /// The tree viola made passes; a sibling tree granted Users write through `icacls` on its
        /// instance directory is refused `Writable`.
        #[test]
        fn strict_instance_refuses_a_tree_granted_users_write_beside_one_that_passes() {
            let (_tmp, home, instance) = instance_tree();
            assert_eq!(
                super::super::check_instance(&home, &instance),
                Ok(()),
                "{}",
                reading_of("instance", &instance)
            );
            let (_widened_tmp, widened_home, widened) = instance_tree();
            icacls(&widened, &["/grant", "*S-1-5-32-545:(OI)(CI)(W)"]);
            assert_eq!(
                super::super::check_instance(&widened_home, &widened),
                Err(Refused::Writable),
                "{}",
                reading_of("widened", &widened)
            );
        }

        /// A `diagnostics/` granted Users read is refused `Readable` by the read rule, where the
        /// write rule alone passes it.
        #[test]
        fn strict_instance_refuses_a_diagnostics_folder_granted_users_read() {
            let (_tmp, home, instance) = instance_tree();
            let diagnostics = instance.join(super::super::DIAGNOSTICS);
            icacls(&diagnostics, &["/grant", "*S-1-5-32-545:(OI)(CI)(R)"]);
            assert_eq!(
                super::super::check_path(&diagnostics),
                Ok(()),
                "{}",
                reading_of("diagnostics", &diagnostics)
            );
            assert_eq!(
                super::super::check_instance(&home, &instance),
                Err(Refused::Readable),
                "{}",
                reading_of("diagnostics", &diagnostics)
            );
            assert_eq!(
                super::super::check_diagnostics(&diagnostics),
                Err(Refused::Readable)
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    const USER: &str = "S-1-5-21-1-2-3-1001";

    fn allow(sid: &str, mask: u32) -> Allow {
        Allow {
            sid: sid.to_owned(),
            mask,
            flags: 0,
        }
    }

    #[rstest]
    #[case::owner_0600(0o100_600, 1000, 1000, Ok(()))]
    #[case::owner_0700_dir(0o040_700, 1000, 1000, Ok(()))]
    #[case::group_read_only(0o100_640, 1000, 1000, Ok(()))]
    #[case::group_write(0o100_620, 1000, 1000, Err(Refused::Writable))]
    #[case::other_write(0o040_702, 1000, 1000, Err(Refused::Writable))]
    #[case::other_owner(0o100_600, 0, 1000, Err(Refused::NotOwner))]
    #[case::other_owner_and_writable(0o100_666, 1001, 1000, Err(Refused::NotOwner))]
    fn unix_verdict_needs_the_owner_and_no_foreign_write(
        #[case] mode: u32,
        #[case] uid: u32,
        #[case] euid: u32,
        #[case] verdict: Result<(), Refused>,
    ) {
        assert_eq!(unix_verdict(mode, uid, euid), verdict);
    }

    #[rstest]
    #[case::user("S-1-5-21-1-2-3-1001", true)]
    #[case::system("S-1-5-18", true)]
    #[case::administrators("S-1-5-32-544", true)]
    #[case::creator_owner("S-1-3-0", true)]
    #[case::users("S-1-5-32-545", false)]
    #[case::authenticated_users("S-1-5-11", false)]
    #[case::everyone("S-1-1-0", false)]
    #[case::another_user("S-1-5-21-1-2-3-1002", false)]
    fn trusted_sid_is_the_user_and_the_three_named(#[case] sid: &str, #[case] trusted: bool) {
        assert_eq!(trusted_sid(sid, USER), trusted);
    }

    #[test]
    fn windows_verdict_takes_the_profile_inheritance() {
        let dacl = [
            allow(USER, 0x001F_01FF),
            allow("S-1-5-18", 0x001F_01FF),
            allow("S-1-5-32-544", 0x001F_01FF),
            allow("S-1-5-32-545", 0x0012_00A9),
        ];
        assert_eq!(windows_verdict(USER, USER, Some(&dacl), true), Ok(()));
        assert_eq!(
            windows_verdict("S-1-5-32-544", USER, Some(&dacl), true),
            Ok(())
        );
    }

    /// Each write right alone, granted to `Users`, is refused — the generic bits of an inherit-only
    /// ACE included.
    #[rstest]
    #[case::write_data(0x2)]
    #[case::append_data(0x4)]
    #[case::delete_child(0x40)]
    #[case::delete(0x1_0000)]
    #[case::write_dac(0x4_0000)]
    #[case::write_owner(0x8_0000)]
    #[case::generic_all(0x1000_0000)]
    #[case::generic_write(0x4000_0000)]
    fn windows_verdict_refuses_any_write_right_to_another_sid(#[case] mask: u32) {
        let dacl = [allow(USER, 0x001F_01FF), allow("S-1-5-32-545", mask)];
        assert_eq!(
            windows_verdict(USER, USER, Some(&dacl), true),
            Err(Refused::Writable)
        );
    }

    #[test]
    fn windows_verdict_refuses_a_foreign_owner_a_null_dacl_and_a_fat_volume() {
        let dacl = [allow(USER, 0x001F_01FF)];
        assert_eq!(
            windows_verdict("S-1-5-21-9-9-9-1002", USER, Some(&dacl), true),
            Err(Refused::NotOwner)
        );
        assert_eq!(
            windows_verdict(USER, USER, None, true),
            Err(Refused::NullDacl)
        );
        assert_eq!(
            windows_verdict(USER, USER, Some(&dacl), false),
            Err(Refused::NoPersistentAcls)
        );
    }

    /// `FILE_PERSISTENT_ACLS` is 0x8, whatever else the volume's flags hold.
    #[rstest]
    #[case::the_bit_alone(0x8, true)]
    #[case::the_bit_among_others(0x00C7_00FF, true)]
    #[case::other_bits_without_it(0x0002_0206, false)]
    #[case::every_other_bit(!0x8, false)]
    #[case::no_bit(0, false)]
    fn volume_keeps_acls_reads_the_persistent_acls_bit(#[case] flags: u32, #[case] keeps: bool) {
        assert_eq!(volume_keeps_acls(flags), keeps);
    }

    #[test]
    fn refused_messages_are_fixed() {
        let messages: Vec<String> = [
            Refused::NotOwner,
            Refused::Writable,
            Refused::Readable,
            Refused::Symlink,
            Refused::Unreadable,
            Refused::NullDacl,
            Refused::NoPersistentAcls,
        ]
        .iter()
        .map(ToString::to_string)
        .collect();
        assert_eq!(
            messages,
            [
                "a trusted state path is not owned by this user",
                "a trusted state path is writable by another user",
                "a trusted state path is readable by another user",
                "a trusted state path is a symbolic link",
                "a trusted state path's permissions could not be read",
                "a trusted state path has a NULL DACL",
                "a trusted state path is on a volume without persistent ACLs",
            ]
        );
    }

    #[test]
    fn check_stamps_of_a_home_without_a_ledger_passes() {
        let tmp = tempfile::tempdir().expect("tempdir");
        assert_eq!(check_stamps(&tmp.path().join("home")), Ok(()));
    }

    /// A path holding a NUL byte fails the stat with a kind other than `NotFound`, before any
    /// filesystem call: only an absent path is skipped, and a stat that fails is a refusal.
    #[test]
    fn check_stamps_of_a_path_that_cannot_be_statted_is_unreadable() {
        assert_eq!(
            check_stamps(Path::new("home\0dir")),
            Err(Refused::Unreadable)
        );
    }

    /// A ledger viola created, and the stamps file it wrote, pass on the OS the test runs on.
    #[test]
    fn check_stamps_of_a_home_viola_wrote_passes() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let home = tmp.path().join("home");
        crate::stamps::update_stamps(&home, |_| b"{}".to_vec()).expect("written");
        assert_eq!(check_stamps(&home), Ok(()));
    }

    #[cfg(unix)]
    #[rstest]
    #[case::stamps_group_writable("stamps", 0o620, Refused::Writable)]
    #[case::stamps_world_writable("stamps", 0o606, Refused::Writable)]
    #[case::ledger_group_writable("ledger", 0o770, Refused::Writable)]
    #[case::ledger_world_writable("ledger", 0o707, Refused::Writable)]
    fn check_stamps_refuses_a_widened_mode(
        #[case] which: &str,
        #[case] mode: u32,
        #[case] refused: Refused,
    ) {
        use std::os::unix::fs::PermissionsExt as _;
        let tmp = tempfile::tempdir().expect("tempdir");
        let home = tmp.path().join("home");
        crate::stamps::update_stamps(&home, |_| b"{}".to_vec()).expect("written");
        let path = match which {
            "stamps" => ledger_dir(&home).join(STAMPS),
            _ => ledger_dir(&home),
        };
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(mode)).expect("chmod");
        assert_eq!(check_stamps(&home), Err(refused));
    }

    /// A home and an instance laid down by the crate's own writers: the directory, a snapshot and
    /// one event line.
    fn instance_viola_wrote(root: &Path) -> (PathBuf, PathBuf) {
        use crate::events::{EventLine, Source, append_event};
        use crate::snapshot::{InstanceSnapshot, Wheel, write_snapshot};
        let home = root.join("home");
        let instance_dir = home.join("instances").join("builder");
        crate::fs::create_private_dir(&instance_dir).expect("the instance dir");
        let snapshot = InstanceSnapshot {
            endpoint: None,
            pid: 1,
            started_at: "s".to_owned(),
            pinned_bin: "b".to_owned(),
            cli_verified: false,
            cli_version: None,
            wheel: Wheel::Driver,
            budget_paused: false,
            budget_override_until: None,
            links: Vec::new(),
            child_pid: None,
            pending_dialog: None,
            cwd: None,
            statusline_command: None,
            agent_session_id: None,
        };
        write_snapshot(&instance_dir, &snapshot).expect("the snapshot");
        let name = viola_core::ViolaName::try_new("builder".to_owned()).expect("valid");
        let line = EventLine::new(
            &name,
            viola_core::EventKind::BudgetGate,
            Source::Wrapper,
            serde_json::json!({"paused": false}),
            chrono::Utc::now(),
        );
        append_event(&instance_dir, &line).expect("the log");
        (home, instance_dir)
    }

    #[test]
    fn strict_instance_of_a_tree_viola_wrote_passes() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let (home, instance_dir) = instance_viola_wrote(tmp.path());
        assert!(instance_dir.join(SNAPSHOT).is_file() && instance_dir.join(EVENTS).is_file());
        assert_eq!(check_instance(&home, &instance_dir), Ok(()));
    }

    #[test]
    fn strict_instance_of_a_home_with_no_instance_directory_passes() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let home = tmp.path().join("home");
        let instance_dir = home.join("instances").join("builder");
        assert_eq!(check_instance(&home, &instance_dir), Ok(()));
        crate::fs::create_private_dir(&home).expect("the home");
        assert_eq!(check_instance(&home, &instance_dir), Ok(()));
    }

    /// Only an absent path is skipped: a home that cannot be statted refuses, and so does an
    /// instance directory that cannot be under a home that passes.
    #[test]
    fn strict_instance_of_a_path_that_cannot_be_statted_is_unreadable() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let (home, _) = instance_viola_wrote(tmp.path());
        assert_eq!(
            check_instance(Path::new("home\0dir"), Path::new("absent")),
            Err(Refused::Unreadable)
        );
        assert_eq!(
            check_instance(&home, Path::new("instance\0dir")),
            Err(Refused::Unreadable)
        );
    }

    #[cfg(unix)]
    #[rstest]
    #[case::home_group_writable("home", 0o770)]
    #[case::instances_world_writable("instances", 0o707)]
    #[case::instance_dir_group_writable("instance", 0o770)]
    #[case::snapshot_group_writable("snapshot", 0o620)]
    #[case::events_world_writable("events", 0o606)]
    fn strict_instance_refuses_a_widened_mode(#[case] which: &str, #[case] mode: u32) {
        use std::os::unix::fs::PermissionsExt as _;
        let tmp = tempfile::tempdir().expect("tempdir");
        let (home, instance_dir) = instance_viola_wrote(tmp.path());
        let path = match which {
            "home" => home.clone(),
            "instances" => home.join("instances"),
            "instance" => instance_dir.clone(),
            "snapshot" => instance_dir.join(SNAPSHOT),
            _ => instance_dir.join(EVENTS),
        };
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(mode)).expect("chmod");
        assert_eq!(check_instance(&home, &instance_dir), Err(Refused::Writable));
    }

    #[rstest]
    #[case::owner_dir_0700(0o040_700, 1000, 1000, Ok(()))]
    #[case::owner_file_0600(0o100_600, 1000, 1000, Ok(()))]
    #[case::group_read_dir(0o040_740, 1000, 1000, Err(Refused::Readable))]
    #[case::other_read_dir(0o040_704, 1000, 1000, Err(Refused::Readable))]
    #[case::group_exec_dir(0o040_710, 1000, 1000, Err(Refused::Readable))]
    #[case::other_exec_dir(0o040_701, 1000, 1000, Err(Refused::Readable))]
    #[case::group_read_file(0o100_640, 1000, 1000, Err(Refused::Readable))]
    #[case::group_write_dir(0o040_770, 1000, 1000, Err(Refused::Writable))]
    #[case::other_owner(0o040_700, 0, 1000, Err(Refused::NotOwner))]
    fn unix_read_verdict_needs_the_write_rule_and_no_foreign_bit(
        #[case] mode: u32,
        #[case] uid: u32,
        #[case] euid: u32,
        #[case] verdict: Result<(), Refused>,
    ) {
        assert_eq!(unix_read_verdict(mode, uid, euid), verdict);
    }

    /// `OBJECT_INHERIT_ACE | CONTAINER_INHERIT_ACE | INHERIT_ONLY_ACE`, as `icacls (OI)(CI)(IO)`
    /// stores it.
    const INHERIT_ONLY: u8 = 0x0B;

    /// Each read right granted to `Users` is refused by the read rule, an inherit-only ACE's
    /// generic bits included, while the write rule passes every one that grants no write.
    #[rstest]
    #[case::read_data(0x1, 0, Err(Refused::Readable))]
    #[case::generic_read(0x8000_0000, 0, Err(Refused::Readable))]
    #[case::inherit_only_generic_read(0x8000_0000, INHERIT_ONLY, Err(Refused::Readable))]
    #[case::read_and_execute(0x0012_00A9, 0, Err(Refused::Readable))]
    #[case::inherit_only_generic_all(0x1000_0000, INHERIT_ONLY, Err(Refused::Writable))]
    #[case::read_attributes_only(0x80, 0, Ok(()))]
    fn windows_read_verdict_refuses_any_read_right_to_another_sid(
        #[case] mask: u32,
        #[case] flags: u8,
        #[case] verdict: Result<(), Refused>,
    ) {
        let dacl = [
            allow(USER, 0x001F_01FF),
            Allow {
                sid: "S-1-5-32-545".to_owned(),
                mask,
                flags,
            },
        ];
        assert_eq!(windows_read_verdict(USER, USER, Some(&dacl), true), verdict);
        if verdict == Err(Refused::Readable) {
            assert_eq!(windows_verdict(USER, USER, Some(&dacl), true), Ok(()));
        }
    }

    /// The profile's inheritance (the user, SYSTEM, Administrators) and `CREATOR OWNER` pass the
    /// read rule; the write rule's refusals come first.
    #[test]
    fn windows_read_verdict_takes_the_trusted_sids_and_keeps_the_write_rule() {
        let dacl = [
            allow(USER, 0x001F_01FF),
            allow("S-1-5-18", 0x001F_01FF),
            allow("S-1-5-32-544", 0x001F_01FF),
            Allow {
                sid: "S-1-3-0".to_owned(),
                mask: 0x1000_0000,
                flags: INHERIT_ONLY,
            },
        ];
        assert_eq!(windows_read_verdict(USER, USER, Some(&dacl), true), Ok(()));
        assert_eq!(
            windows_read_verdict("S-1-5-21-9-9-9-1002", USER, Some(&dacl), true),
            Err(Refused::NotOwner)
        );
        assert_eq!(
            windows_read_verdict(USER, USER, None, true),
            Err(Refused::NullDacl)
        );
        assert_eq!(
            windows_read_verdict(USER, USER, Some(&dacl), false),
            Err(Refused::NoPersistentAcls)
        );
    }

    /// A home with one instance, a ledger, a pinned copy and a plugin folder, laid down by the
    /// crate's own writers; its key.
    fn home_tree_viola_wrote(root: &Path) -> (PathBuf, PathBuf, String) {
        let (home, instance_dir) = instance_viola_wrote(root);
        crate::fs::create_private_dir(&instance_dir.join(DIAGNOSTICS)).expect("diagnostics");
        crate::stamps::update_stamps(&home, |_| b"{}".to_vec()).expect("stamps");
        let exe = root.join("source-exe");
        std::fs::write(&exe, b"abc").expect("source");
        let pinned = crate::pin::pin_exe(&home, &exe).expect("pinned");
        let plugin = home.join("plugin").join(&pinned.key);
        for dir in ["hooks", ".claude-plugin"] {
            crate::fs::create_private_dir(&plugin.join(dir)).expect("plugin");
        }
        (home, instance_dir, pinned.key)
    }

    fn builder() -> ViolaName {
        ViolaName::try_new("builder".to_owned()).expect("valid")
    }

    #[test]
    fn check_home_tree_of_a_tree_viola_wrote_passes() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let (home, _, key) = home_tree_viola_wrote(tmp.path());
        assert_eq!(check_home_tree(&home, Some(&builder()), Some(&key)), Ok(()));
        assert_eq!(check_home_tree(&home, None, None), Ok(()));
        assert_eq!(
            check_home_tree(&tmp.path().join("absent"), Some(&builder()), Some(&key)),
            Ok(())
        );
    }

    /// Each path of the tree widened in turn is refused on its own side; with no name the
    /// instance's set is not read, with no key the key's folders are not.
    #[cfg(unix)]
    #[rstest]
    #[case::home_group_readable("", 0o750, Some(TreeRefused::HomeLevel(Refused::Readable)))]
    #[case::home_group_writable("", 0o770, Some(TreeRefused::HomeLevel(Refused::Writable)))]
    #[case::instances_world_writable(
        "instances",
        0o707,
        Some(TreeRefused::InstanceLevel(Refused::Writable))
    )]
    #[case::instance_group_readable(
        "instances/builder",
        0o740,
        Some(TreeRefused::InstanceLevel(Refused::Readable))
    )]
    #[case::instance_diagnostics_other_readable(
        "instances/builder/diagnostics",
        0o704,
        Some(TreeRefused::InstanceLevel(Refused::Readable))
    )]
    #[case::ledger_group_writable("ledger", 0o770, Some(TreeRefused::HomeLevel(Refused::Writable)))]
    #[case::bin_group_writable("bin", 0o770, Some(TreeRefused::HomeLevel(Refused::Writable)))]
    #[case::plugin_world_writable("plugin", 0o707, Some(TreeRefused::HomeLevel(Refused::Writable)))]
    #[case::bin_key_group_writable(
        "bin/KEY",
        0o770,
        Some(TreeRefused::HomeLevel(Refused::Writable))
    )]
    #[case::pinned_copy_group_writable(
        "bin/KEY/viola",
        0o770,
        Some(TreeRefused::HomeLevel(Refused::Writable))
    )]
    #[case::plugin_key_group_writable(
        "plugin/KEY",
        0o770,
        Some(TreeRefused::HomeLevel(Refused::Writable))
    )]
    #[case::plugin_hooks_group_writable(
        "plugin/KEY/hooks",
        0o770,
        Some(TreeRefused::HomeLevel(Refused::Writable))
    )]
    #[case::plugin_manifest_dir_group_writable(
        "plugin/KEY/.claude-plugin",
        0o770,
        Some(TreeRefused::HomeLevel(Refused::Writable))
    )]
    #[case::ledger_group_readable_passes("ledger", 0o750, None)]
    fn check_home_tree_refuses_each_widened_path(
        #[case] rel: &str,
        #[case] mode: u32,
        #[case] refused: Option<TreeRefused>,
    ) {
        use std::os::unix::fs::PermissionsExt as _;
        let tmp = tempfile::tempdir().expect("tempdir");
        let (home, _, key) = home_tree_viola_wrote(tmp.path());
        let path = rel
            .split('/')
            .filter(|part| !part.is_empty())
            .fold(home.clone(), |path, part| {
                path.join(if part == "KEY" { key.as_str() } else { part })
            });
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(mode)).expect("chmod");
        assert_eq!(
            check_home_tree(&home, Some(&builder()), Some(&key)),
            refused.map_or(Ok(()), Err)
        );
        let instance_side = matches!(refused, Some(TreeRefused::InstanceLevel(_)));
        let key_side = rel.contains("KEY");
        if instance_side || key_side {
            assert_eq!(check_home_tree(&home, None, None), Ok(()), "{rel}");
        }
    }

    #[test]
    fn check_diagnostics_of_what_viola_wrote_or_of_nothing_passes() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let dir = tmp.path().join("home").join(DIAGNOSTICS);
        assert_eq!(check_diagnostics(&dir), Ok(()));
        crate::fs::create_private_dir(&dir).expect("dir");
        let file = dir.join("run-builder.ndjson");
        crate::fs::open_private_append(&file).expect("file");
        assert_eq!(check_diagnostics(&dir), Ok(()));
        assert_eq!(check_diagnostics(&file), Ok(()));
        assert_eq!(
            check_diagnostics(Path::new("diag\0dir")),
            Err(Refused::Unreadable)
        );
    }

    #[cfg(unix)]
    #[rstest]
    #[case::dir_group_readable(true, 0o750, Refused::Readable)]
    #[case::dir_group_writable(true, 0o770, Refused::Writable)]
    #[case::file_group_readable(false, 0o640, Refused::Readable)]
    #[case::file_other_writable(false, 0o602, Refused::Writable)]
    fn check_diagnostics_refuses_a_widened_mode(
        #[case] dir: bool,
        #[case] mode: u32,
        #[case] refused: Refused,
    ) {
        use std::os::unix::fs::PermissionsExt as _;
        let tmp = tempfile::tempdir().expect("tempdir");
        let diagnostics = tmp.path().join(DIAGNOSTICS);
        crate::fs::create_private_dir(&diagnostics).expect("dir");
        let file = diagnostics.join("run-builder.ndjson");
        crate::fs::open_private_append(&file).expect("file");
        let path = if dir { &diagnostics } else { &file };
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(mode)).expect("chmod");
        assert_eq!(check_diagnostics(path), Err(refused));
    }

    /// A role file reached through a symlink is refused, wherever the link points, and so is a
    /// symlinked `diagnostics/` folder.
    #[cfg(unix)]
    #[test]
    fn check_diagnostics_refuses_a_symlinked_role_file_and_folder() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let diagnostics = tmp.path().join(DIAGNOSTICS);
        crate::fs::create_private_dir(&diagnostics).expect("dir");
        let target = tmp.path().join("elsewhere.ndjson");
        crate::fs::open_private_append(&target).expect("target");
        let file = diagnostics.join("run-builder.ndjson");
        std::os::unix::fs::symlink(&target, &file).expect("symlink");
        assert_eq!(check_diagnostics(&target), Ok(()));
        assert_eq!(check_diagnostics(&file), Err(Refused::Symlink));
        let folder = tmp.path().join("linked-diagnostics");
        std::os::unix::fs::symlink(&diagnostics, &folder).expect("symlink");
        assert_eq!(check_diagnostics(&folder), Err(Refused::Symlink));
    }

    /// The read rule reaches the home, the instance directory and its `diagnostics/`; a
    /// group-readable `instances/` and snapshot stay under the write rule alone.
    #[cfg(unix)]
    #[rstest]
    #[case::home_group_readable("home", 0o750, Err(Refused::Readable))]
    #[case::instance_other_readable("instance", 0o705, Err(Refused::Readable))]
    #[case::diagnostics_group_readable("diagnostics", 0o740, Err(Refused::Readable))]
    #[case::instances_group_readable("instances", 0o750, Ok(()))]
    #[case::snapshot_group_readable("snapshot", 0o640, Ok(()))]
    #[case::events_group_readable("events", 0o640, Ok(()))]
    fn strict_instance_holds_the_content_folders_to_the_read_rule(
        #[case] which: &str,
        #[case] mode: u32,
        #[case] verdict: Result<(), Refused>,
    ) {
        use std::os::unix::fs::PermissionsExt as _;
        let tmp = tempfile::tempdir().expect("tempdir");
        let (home, instance_dir) = instance_viola_wrote(tmp.path());
        crate::fs::create_private_dir(&instance_dir.join(DIAGNOSTICS)).expect("diagnostics");
        let path = match which {
            "home" => home.clone(),
            "instances" => home.join("instances"),
            "instance" => instance_dir.clone(),
            "diagnostics" => instance_dir.join(DIAGNOSTICS),
            "snapshot" => instance_dir.join(SNAPSHOT),
            _ => instance_dir.join(EVENTS),
        };
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(mode)).expect("chmod");
        assert_eq!(check_instance(&home, &instance_dir), verdict);
    }
}
