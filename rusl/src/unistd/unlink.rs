use sc::syscall;

use crate::platform::{Fd, NonNegativeI32, AT_FDCWD, AT_REMOVEDIR};
use crate::string::unix_str::AsUnixStr;

/// Taking the liberty of using `unlinkat` for both implementations, effectively meaning
/// that `CWD` is the base if the path isn't absolute
/// "unlink, unlinkat - delete a name and possibly the file it refers to"[docs](https://man7.org/linux/man-pages/man2/unlink.2.html)
/// # Errors
/// See above docs
#[inline]
pub fn unlink(path: impl AsUnixStr) -> crate::Result<()> {
    unlink_flags(path, NonNegativeI32::ZERO)
}

/// Unlink with a path from `CWD` and the provided flags. Flags are either 0 or `AT_REMOVEDIR`.
/// "unlink, unlinkat - delete a name and possibly the file it refers to"[docs](https://man7.org/linux/man-pages/man2/unlink.2.html)
/// # Errors
/// See above docs
#[inline]
pub fn unlink_flags(path: impl AsUnixStr, flags: NonNegativeI32) -> crate::Result<()> {
    do_unlink(AT_FDCWD, path, flags.0)
}

/// Taking the liberty of using `unlinkat` for both implementations, effectively meaning
/// that `CWD` is the base if the path isn't absolute. Flags are either 0 or `AT_REMOVEDIR`
/// "unlink, unlinkat - delete a name and possibly the file it refers to"[docs](https://man7.org/linux/man-pages/man2/unlink.2.html)
/// # Errors
/// See above docs
#[inline]
pub fn unlink_at(dir_fd: Fd, path: impl AsUnixStr, flags: i32) -> crate::Result<()> {
    do_unlink(dir_fd.0, path, flags)
}

#[inline(always)]
#[allow(clippy::inline_always)]
fn do_unlink(dir_fd: i32, path: impl AsUnixStr, flags: i32) -> crate::Result<()> {
    path.exec_with_self_as_ptr(|ptr| {
        let res = unsafe { syscall!(UNLINKAT, dir_fd, ptr, flags) };
        bail_on_below_zero!(res, "`UNLINKAT` syscall failed");
        Ok(())
    })
}

/// Taking the liberty of using `unlinkat` for both implementations, effectively meaning
/// that `CWD` is the base if the path isn't absolute. Flags are either 0 or `AT_REMOVEDIR`
/// "unlink, unlinkat - delete a name and possibly the file it refers to"[docs](https://man7.org/linux/man-pages/man2/unlink.2.html)
/// # Errors
/// See above docs
#[inline]
pub fn rmdir(dir_fd: Fd) -> crate::Result<()> {
    let res = unsafe { syscall!(UNLINKAT, dir_fd.0, 0, AT_REMOVEDIR.0) };
    bail_on_below_zero!(res, "`UNLINKAT` syscall failed");
    Ok(())
}
