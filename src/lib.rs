//! A dirty module-loading library for the Linux kernel.
//!
//! This abuses Linux modules auto-loading mechanism to trick
//! the kernel into shelling out to the userspace `modprobe`
//! helper.
//!
//! A [side-effect][dev_load] of `SIOCGIFINDEX` ioctl results in
//! the kernel looking up and loading arbitrary modules by name.
//! This isn't strictly a privilege escalation as the caller must
//! have `CAP_SYS_MODULE` capability; however it allows containerized
//! process to load modules in the host namespace.
//!
//! This is a dirty mechanism, as the ioctl syscall will induce a
//! context-switch back from kernel-space to user-space to run
//! a host binary outside of caller context.
//!
//! Typically this results in `modprobe` being called in the host,
//! however arbitrary binaries can be run by tweaking the usermode
//! helper sysctl at `/proc/sys/kernel/modprobe`.
//!
//! [dev_load]: https://github.com/torvalds/linux/blob/v4.12/net/core/dev_ioctl.c#L372

extern crate nix;

use std::ffi::CStr;
use nix::net::if_::if_nametoindex;

/// Try to load a host kernel module via the modprobe userspace helper.
///
/// This triggers a `modprobe` execution in the host namespace.
/// Caller must have the `CAP_SYS_MODULE` capability in order to
/// load a module. There is no way to know if the module has been
/// properly loaded, as the kernel does not return such information.
pub fn try_load(module: &CStr) -> () {
    let _ = if_nametoindex(module);
}
