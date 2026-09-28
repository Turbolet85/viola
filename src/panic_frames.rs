//! A panic's call stack as raw frames, never symbolised on the panic path (obs-plan §7; measured on
//! the Windows runner: symbolising 72 frames took 351 of the hook's 403 ms against its 1.0 s spine
//! bound). Each frame carries its instruction address, the module that holds it and that module's
//! load base, so an offline resolver can symbolise it later against the pinned copy (the offset is
//! the address less the base). Only the two OS readers differ per OS; the rendering is shared.

/// The frames kept: `RtlCaptureStackBackTrace` takes fewer than 63 on older Windows.
const MAX_FRAMES: usize = 62;

/// One detail-line backtrace string: `0x<ip> <module> base=0x<base> +0x<offset>`, or `0x<ip> ?` for
/// an address no loaded module holds.
fn render(ip: usize, module: Option<(String, usize)>) -> String {
    match module {
        Some((path, base)) => format!(
            "0x{ip:016x} {path} base=0x{base:016x} +0x{:x}",
            ip.wrapping_sub(base)
        ),
        None => format!("0x{ip:016x} ?"),
    }
}

/// The calling thread's stack, innermost first, one rendered string per frame.
pub(crate) fn capture() -> Vec<String> {
    raw_frames()
        .into_iter()
        .map(|ip| render(ip, module_of(ip)))
        .collect()
}

/// `GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT`, one
/// literal (pinned by a test): the address is read as an address, and no reference is taken.
#[cfg(windows)]
const MODULE_BY_ADDRESS: u32 = 6;

#[cfg(windows)]
fn raw_frames() -> Vec<usize> {
    use windows_sys::Win32::System::Diagnostics::Debug::RtlCaptureStackBackTrace;
    let mut buf = [std::ptr::null_mut(); MAX_FRAMES];
    let size = u32::try_from(MAX_FRAMES).unwrap_or(0);
    // SAFETY: the buffer holds MAX_FRAMES pointers and the hash out-pointer may be null.
    let n = unsafe { RtlCaptureStackBackTrace(0, size, buf.as_mut_ptr(), std::ptr::null_mut()) };
    buf[..usize::from(n).min(MAX_FRAMES)]
        .iter()
        .map(|p| p.addr())
        .collect()
}

#[cfg(windows)]
fn module_of(ip: usize) -> Option<(String, usize)> {
    use windows_sys::Win32::System::LibraryLoader::{GetModuleFileNameW, GetModuleHandleExW};
    let mut module = std::ptr::null_mut();
    let address = std::ptr::without_provenance::<u16>(ip);
    // SAFETY: with FROM_ADDRESS the name argument is read as an address inside a module, never
    // dereferenced as a string; UNCHANGED_REFCOUNT leaves no reference to release.
    if unsafe { GetModuleHandleExW(MODULE_BY_ADDRESS, address, &mut module) } == 0 {
        return None;
    }
    let mut name = [0u16; 1024];
    let size = u32::try_from(name.len()).unwrap_or(0);
    // SAFETY: `module` is a loaded module's handle and the buffer holds `size` UTF-16 units.
    let len = unsafe { GetModuleFileNameW(module, name.as_mut_ptr(), size) };
    // The count is at most `size`; a failed read (0) leaves an empty path, the address still kept.
    let path = name.get(..usize::try_from(len).ok()?)?;
    Some((String::from_utf16_lossy(path), module.addr()))
}

#[cfg(unix)]
fn raw_frames() -> Vec<usize> {
    let mut buf = [std::ptr::null_mut(); MAX_FRAMES];
    let size = libc::c_int::try_from(MAX_FRAMES).unwrap_or(0);
    // SAFETY: backtrace writes at most `size` pointers into the buffer.
    let n = unsafe { libc::backtrace(buf.as_mut_ptr(), size) };
    let n = usize::try_from(n).unwrap_or(0).min(MAX_FRAMES);
    buf[..n].iter().map(|p| p.addr()).collect()
}

#[cfg(unix)]
fn module_of(ip: usize) -> Option<(String, usize)> {
    // SAFETY: Dl_info is plain pointers, for which all-zero is a valid value.
    let mut info: libc::Dl_info = unsafe { std::mem::zeroed() };
    // SAFETY: dladdr only reads the address and fills `info`.
    if unsafe { libc::dladdr(std::ptr::without_provenance(ip), &mut info) } == 0 {
        return None;
    }
    if info.dli_fname.is_null() {
        return None;
    }
    // SAFETY: a non-null dli_fname is a NUL-terminated path owned by the loader.
    let path = unsafe { std::ffi::CStr::from_ptr(info.dli_fname) };
    Some((path.to_string_lossy().into_owned(), info.dli_fbase.addr()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_writes_the_address_the_module_its_base_and_the_offset() {
        assert_eq!(
            render(
                0x7ff6_1234,
                Some(("C:/h/bin/k/viola.exe".to_owned(), 0x7ff6_0000))
            ),
            "0x000000007ff61234 C:/h/bin/k/viola.exe base=0x000000007ff60000 +0x1234"
        );
        assert_eq!(render(0x10, None), "0x0000000000000010 ?");
    }

    /// The innermost frames are this test binary's own code: its path and a base at or below
    /// the address, and every frame rendered.
    #[test]
    fn capture_names_this_binary_for_its_own_frames() {
        let frames = capture();
        assert!(!frames.is_empty() && frames.len() <= MAX_FRAMES);
        let exe = std::env::current_exe().expect("exe");
        let exe = exe.to_string_lossy().to_lowercase();
        assert!(
            frames.iter().any(|f| f.to_lowercase().contains(&exe)),
            "{frames:?}"
        );
        assert!(frames.iter().all(|f| f.starts_with("0x")));
        let ip = raw_frames()[0];
        let (path, base) = module_of(ip).expect("a module holds this code");
        assert!(base <= ip, "base {base:x} above ip {ip:x}");
        assert!(!path.is_empty());
    }

    #[test]
    fn module_of_an_address_no_module_holds_is_none() {
        assert_eq!(module_of(0x10), None);
    }

    #[cfg(windows)]
    #[test]
    fn module_by_address_is_the_two_flags() {
        use windows_sys::Win32::System::LibraryLoader::{
            GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS, GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
        };
        assert_eq!(
            MODULE_BY_ADDRESS,
            GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT
        );
    }
}
