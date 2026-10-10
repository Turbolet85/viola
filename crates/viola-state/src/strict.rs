//! The strict-modes check (security-plan §Authentication & Authorization, `~/.viola/` access control),
//! first wired on `run`'s `ledger/stamps.json` read: a trusted file, or a folder holding one, that
//! another user could write is refused before it is read (ssh StrictModes style). Unix reads the
//! owner and the mode; Windows reads the owner and the DACL. The decisions are plain functions
//! tested on every OS; only the readers are per OS.

use std::io;
use std::path::{Path, PathBuf};

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
    #[error("a trusted state path's permissions could not be read")]
    Unreadable,
    #[error("a trusted state path has a NULL DACL")]
    NullDacl,
    #[error("a trusted state path is on a volume without persistent ACLs")]
    NoPersistentAcls,
}

/// `ledger/` and `ledger/stamps.json`, each that exists, in that order: the folder first, since an
/// entry written by another user between the checks would land through it.
pub fn check_stamps(home: &Path) -> Result<(), Refused> {
    let dir = ledger_dir(home);
    check_existing(&[dir.clone(), dir.join(STAMPS)])
}

/// The home, `instances/`, the instance directory, its `snapshot.json` and its `events.ndjson`,
/// each that exists, a folder before what it holds: the check a process runs before it uses a
/// value read from either file.
pub fn check_instance(home: &Path, instance_dir: &Path) -> Result<(), Refused> {
    check_existing(&[
        home.to_path_buf(),
        home.join("instances"),
        instance_dir.to_path_buf(),
        instance_dir.join(SNAPSHOT),
        instance_dir.join(EVENTS),
    ])
}

/// Each path that exists, in order; an absent one is skipped, and one that cannot be statted is
/// refused.
fn check_existing(paths: &[PathBuf]) -> Result<(), Refused> {
    for path in paths {
        match std::fs::symlink_metadata(path) {
            Ok(_) => check_path(path)?,
            Err(e) if e.kind() == io::ErrorKind::NotFound => {}
            Err(_) => return Err(Refused::Unreadable),
        }
    }
    Ok(())
}

/// One path through this OS's reader and rule.
pub fn check_path(path: &Path) -> Result<(), Refused> {
    #[cfg(unix)]
    {
        let (mode, uid) = unix::reading(path).map_err(|_| Refused::Unreadable)?;
        unix_verdict(mode, uid, unix::euid())
    }
    #[cfg(windows)]
    {
        win::check(path)
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

/// The write rights the Windows rule refuses to any other SID: `FILE_WRITE_DATA` / `FILE_ADD_FILE`
/// (0x2), `FILE_APPEND_DATA` / `FILE_ADD_SUBDIRECTORY` (0x4), `FILE_DELETE_CHILD` (0x40), `DELETE`
/// (0x1_0000), `WRITE_DAC` (0x4_0000), `WRITE_OWNER` (0x8_0000), `GENERIC_ALL` (0x1000_0000) and
/// `GENERIC_WRITE` (0x4000_0000), written as one value (the Windows test pins it to the names).
pub const WRITE_RIGHTS: u32 = 0x500D_0046;

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

    use super::{Allow, Refused, volume_keeps_acls, windows_verdict};

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

    pub(super) fn check(path: &Path) -> Result<(), Refused> {
        let user = user_sid().ok_or(Refused::Unreadable)?;
        let persistent = persistent_acls(path).ok_or(Refused::Unreadable)?;
        let (owner, dacl) = owner_and_dacl(path).ok_or(Refused::Unreadable)?;
        windows_verdict(&owner, &user, dacl.as_deref(), persistent)
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
            assert_eq!(check(&path), Err(Refused::Unreadable), "{unmounted}");
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
            let verdict = check(path);
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
            links: Vec::new(),
            child_pid: None,
            pending_dialog: None,
            cwd: None,
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
}
