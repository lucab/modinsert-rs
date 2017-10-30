# modinsert

[![Build Status](https://travis-ci.org/lucab/modinsert-rs.svg?branch=master)](https://travis-ci.org/lucab/modinsert-rs)
[![crates.io](https://img.shields.io/crates/v/modinsert.svg)](https://crates.io/crates/modinsert)
[![LoC](https://tokei.rs/b1/github/lucab/modinsert-rs?category=code)](https://github.com/lucab/modinsert-rs)
[![Documentation](https://docs.rs/modinsert/badge.svg)](https://docs.rs/modinsert)

A dirty module-loading library for the Linux kernel.

`modinsert` provides support for loading Linux kernel modules, in an "alternative" way.

This abuses Linux modules auto-loading mechanism to trick
the kernel into shelling out to the userspace `modprobe`
helper.

A [side-effect][dev_load] of `SIOCGIFINDEX` ioctl results in
the kernel looking up and loading arbitrary modules by name.
This isn't strictly a privilege escalation as the caller must
have `CAP_SYS_MODULE` capability; however it allows containerized
process to load modules in the host namespace.

This is a dirty mechanism, as the ioctl syscall will induce a
context-switch back from kernel-space to user-space to run
a host binary outside of caller context.

Typically this results in `modprobe` being called in the host,
however arbitrary binaries can be run by tweaking the usermode
helper sysctl at `/proc/sys/kernel/modprobe`.

See `modprobe(7)` for more documentation on kernel modules.

[dev_load]: https://github.com/torvalds/linux/blob/v4.12/net/core/dev_ioctl.c#L372

## Example

```rust
//! Run this, then check `dmesg` for the effects.
//! It works in containers too, as long as the process has `CAP_SYS_MODULE`.

extern crate modinsert;

use std::ffi::CString;

fn main() {
    let modname = CString::new("rbd").unwrap();
    modinsert::try_load(&modname);
}
```

Some more examples are available under [examples](examples).

## License

Licensed under either of

 * MIT license - <http://opensource.org/licenses/MIT>
 * Apache License, Version 2.0 - <http://www.apache.org/licenses/LICENSE-2.0>

at your option.
