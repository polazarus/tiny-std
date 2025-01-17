use crate::platform::FilesystemType;
use crate::string::unix_str::AsUnixStr;
use crate::Result;
use sc::syscall;

/// Mount a device.
/// Attempt to mount a device from `source` to `target` specifying a `FilesystemType` and `flags`.
/// Some filesystems allow providing additional data, which goes in `data`.
/// See the [linux docs for details](https://man7.org/linux/man-pages/man2/mount.2.html).
/// # Errors
/// See above
pub fn mount<SRC, TGT, DATA>(
    source: SRC,
    target: TGT,
    fs_type: FilesystemType,
    flags: u64,
    data: Option<DATA>,
) -> Result<()>
where
    SRC: AsUnixStr,
    TGT: AsUnixStr,
    DATA: AsUnixStr,
{
    unsafe {
        source.exec_with_self_as_ptr(|src| {
            target.exec_with_self_as_ptr(|tgt| {
                if let Some(data) = data {
                    data.exec_with_self_as_ptr(|data| {
                        let res =
                            syscall!(MOUNT, src, tgt, fs_type.label().0.as_ptr(), flags, data);
                        bail_on_below_zero!(res, "`MOUNT` syscall failed");
                        Ok(res)
                    })
                } else {
                    let res = syscall!(MOUNT, src, tgt, fs_type.label().0.as_ptr(), flags, 0);
                    bail_on_below_zero!(res, "`MOUNT` syscall failed");
                    Ok(res)
                }
            })
        })?;
    };
    Ok(())
}

/// Unmount a device.
/// Attempts to unmount the device at `target`.
/// See the [linux docs for details](https://man7.org/linux/man-pages/man2/umount.2.html).
/// # Errors
/// See above.
pub fn unmount<TGT>(target: TGT) -> Result<()>
where
    TGT: AsUnixStr,
{
    target.exec_with_self_as_ptr(|ptr| {
        unsafe {
            let res = syscall!(UMOUNT2, ptr, 0);
            bail_on_below_zero!(res, "`UNMOUNT2` syscall failed");
        }
        Ok(())
    })
}
