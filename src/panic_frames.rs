//! A panic's call stack as raw frames, never symbolised on the panic path (obs-plan §7: symbolising
//! 72 frames took 351 of the hook's 403 ms against its 1.0 s spine bound). Each frame carries its
//! instruction address, the module that holds it and that module's load base, so an offline
//! resolver can symbolise it later against the pinned copy (the offset is the address less the
//! base).

/// The frames kept.
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

fn raw_frames() -> Vec<usize> {
    let mut buf = [std::ptr::null_mut(); MAX_FRAMES];
    let size = libc::c_int::try_from(MAX_FRAMES).unwrap_or(0);
    // SAFETY: backtrace writes at most `size` pointers into the buffer.
    let n = unsafe { libc::backtrace(buf.as_mut_ptr(), size) };
    let n = usize::try_from(n).unwrap_or(0).min(MAX_FRAMES);
    buf[..n].iter().map(|p| p.addr()).collect()
}

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
}
