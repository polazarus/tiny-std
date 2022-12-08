use sc::syscall;

use crate::compat::unix_str::AsUnixStr;
use crate::platform::{Fd, AT_FDCWD};

/// Openflags accepted by the [open syscall](https://man7.org/linux/man-pages/man2/open.2.html)
transparent_bitflags! {
    pub struct OpenFlags: i32 {
        const O_RDONLY = linux_rust_bindings::O_RDONLY;
        const O_WRONLY = linux_rust_bindings::O_WRONLY;
        const O_RDWR = linux_rust_bindings::O_RDWR;
        const O_APPEND = linux_rust_bindings::O_APPEND;
        const O_ASYNC = 0x2000;
        const O_CLOEXEC = linux_rust_bindings::O_CLOEXEC;
        const O_CREAT = linux_rust_bindings::O_CREAT;
        const O_DIRECT = linux_rust_bindings::O_DIRECT;
        const O_DIRECTORY = linux_rust_bindings::O_DIRECTORY;
        const O_DSYNC = linux_rust_bindings::O_DSYNC;
        const O_EXCL = linux_rust_bindings::O_EXCL;
        const O_LARGEFILE = linux_rust_bindings::O_LARGEFILE;
        const O_NOATIME = linux_rust_bindings::O_NOATIME;
        const O_NOCTTY = linux_rust_bindings::O_NOCTTY;
        const O_NOFOLLOW = linux_rust_bindings::O_NOFOLLOW;
        const O_NONBLOCK = linux_rust_bindings::O_NONBLOCK;
        const O_NDELAY = linux_rust_bindings::O_NDELAY;
        const O_PATH = linux_rust_bindings::O_PATH;
        const O_SYNC = linux_rust_bindings::O_SYNC;
        const O_TMPFILE = linux_rust_bindings::O_TMPFILE;
        const O_TRUNC = linux_rust_bindings::O_TRUNC;
    }
}

/// Mode accepted by the [open syscall](https://man7.org/linux/man-pages/man2/open.2.html)
transparent_bitflags! {
    pub struct Mode: i32 {
        const S_IRWXU = linux_rust_bindings::S_IRWXU; // 00700 user read write exec
        const S_IRUSR = linux_rust_bindings::S_IRUSR; // 00400 user Read
        const S_IWUSR = linux_rust_bindings::S_IWUSR; // 00200 user write
        const S_IXUSR = linux_rust_bindings::S_IXUSR;  // 00100 user execute
        const S_IRWXG = linux_rust_bindings::S_IRWXG;  // 00070 group read write exec
        const S_IRGRP = linux_rust_bindings::S_IRGRP;  // 00040 group read
        const S_IWGRP = linux_rust_bindings::S_IWGRP;  // 00020 group write
        const S_IXGRP = linux_rust_bindings::S_IXGRP;   // 00010 group exec
        const S_IRWXO = linux_rust_bindings::S_IRWXO;   // 00007 other read write exec
        const S_IROTH = linux_rust_bindings::S_IROTH;   // 00004 other read
        const S_IWOTH = linux_rust_bindings::S_IWOTH;   // 00002 other write
        const S_IXOTH = linux_rust_bindings::S_IXOTH;   // 00001 other execute

        // Linux specific bits
        const S_ISUID = linux_rust_bindings::S_ISUID; // 0004000 set-user-ID bit
        const S_ISGID = linux_rust_bindings::S_ISGID; // 0002000 set-group-ID bit
        const S_ISVTX = linux_rust_bindings::S_ISVTX; // 0001000 set-sticky bit

        // File specific bits
        const S_IFIFO  = linux_rust_bindings::S_IFIFO;
        const S_IFCHR  = linux_rust_bindings::S_IFCHR;
        const S_IFDIR  = linux_rust_bindings::S_IFDIR;
        const S_IFBLK  = linux_rust_bindings::S_IFBLK;
        const S_IFREG  = linux_rust_bindings::S_IFREG;
        const S_IFLNK  = linux_rust_bindings::S_IFLNK;
        const S_IFSOCK = linux_rust_bindings::S_IFSOCK;
        const S_IFMT   = linux_rust_bindings::S_IFMT;
    }
}

/// Attempts to open the fd at the path specified by a null terminated string, with the provided `OpenFlags`
/// See the [linux docs here](https://man7.org/linux/man-pages/man2/open.2.html)
/// # Errors
/// See above, errors are converted into an Err with the corresponding error code
pub fn open(path: impl AsUnixStr, flags: OpenFlags) -> crate::Result<Fd> {
    path.exec_with_self_as_ptr(|ptr| {
        let res = unsafe { syscall!(OPENAT, AT_FDCWD, ptr, flags.bits()) } as i32;
        bail_on_below_zero!(res, "`OPENAT` syscall failed");
        Ok(res)
    })
}

/// Attempts to open the fd at the path specified by a null terminated string, with the provided `OpenFlags` and `Mode`
/// See the [linux docs here](https://man7.org/linux/man-pages/man2/open.2.html)
/// # Errors
/// see above
#[inline]
pub fn open_mode(path: impl AsUnixStr, flags: OpenFlags, mode: Mode) -> crate::Result<Fd> {
    path.exec_with_self_as_ptr(|ptr| {
        let res = unsafe { syscall!(OPENAT, AT_FDCWD, ptr, flags.bits(), mode.bits()) } as i32;
        bail_on_below_zero!(res, "`OPENAT` syscall failed");
        Ok(res)
    })
}

/// Attempts to open a file at the specified path from the opened directory (`Fd`) with the specified `OpenFlags`
/// See the [linux docs here](https://man7.org/linux/man-pages/man2/open.2.html)
/// # Errors
/// see above
pub fn open_at(dir: Fd, path: impl AsUnixStr, flags: OpenFlags) -> crate::Result<Fd> {
    path.exec_with_self_as_ptr(|ptr| {
        let res = unsafe { syscall!(OPENAT, dir, ptr, flags.bits()) } as i32;
        bail_on_below_zero!(res, "`OPENAT` syscall failed");
        Ok(res)
    })
}

/// Attempts to open a file at the specified path from the opened directory (`Fd`) with the specified `OpenFlags` and `Mode`
/// See the [linux docs here](https://man7.org/linux/man-pages/man2/open.2.html)
/// # Errors
/// see above
pub fn open_at_mode(
    dir: Fd,
    path: impl AsUnixStr,
    flags: OpenFlags,
    mode: Mode,
) -> crate::Result<Fd> {
    path.exec_with_self_as_ptr(|ptr| {
        let res = unsafe { syscall!(OPENAT, dir, ptr, flags.bits(), mode.bits()) } as i32;
        bail_on_below_zero!(res, "`OPENAT` syscall failed");
        Ok(res)
    })
}

#[cfg(test)]
mod tests {
    use crate::unistd::open::{open, OpenFlags};

    #[test]
    fn try_open() {
        // TODO: fix
        #[cfg(feature = "alloc")]
        let path = "test-files/can_open.txt";
        #[cfg(not(feature = "alloc"))]
        let path = "test-files/can_open.txt\0";
        try_open_at_path(path).unwrap();
    }

    fn try_open_at_path(path: &str) -> crate::Result<()> {
        let _fd = open(path, OpenFlags::O_RDONLY)?;
        let _fd = open(path, OpenFlags::O_WRONLY)?;
        let _fd = open(path, OpenFlags::O_RDWR)?;
        Ok(())
    }

    // Differences between tmp-file creating on `x86_64` and `aarch64`, pretty interesting
    // seems that we can't just name a dir on `aarch64` because it produces `EISDIR`
    #[test]
    #[cfg(target_arch = "x86_64")]
    fn try_open_temp() {
        // TODO: fix
        #[cfg(feature = "alloc")]
        let path = "test-files";
        #[cfg(not(feature = "alloc"))]
        let path = "test-files\0";
        try_open_temp_at_path(path).unwrap();
    }

    #[cfg(target_arch = "x86_64")]
    fn try_open_temp_at_path(path: &str) -> crate::Result<()> {
        use crate::unistd::open::{open_mode, Mode};
        let _fd = open_mode(
            path,
            OpenFlags::O_WRONLY | OpenFlags::O_TMPFILE,
            Mode::S_IRUSR | Mode::S_IWUSR,
        )?;
        let _fd = open_mode(
            path,
            OpenFlags::O_RDWR | OpenFlags::O_TMPFILE,
            Mode::S_IRGRP | Mode::S_IWGRP,
        )?;
        Ok(())
    }
}
