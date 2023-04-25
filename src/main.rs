use std::env::consts::{ARCH, OS};

#[no_mangle]
pub fn main() -> &'static str {
    match (OS, ARCH) {
        ("windows", "x86_64") => "win-x64-msi",
        ("windows", "i686") => "win-x86-msi",
        ("windows", "i586") => "win-x86-msi",
        ("windows", "i386") => "win-x86-msi",
        ("macos", "x86_64") => "osx-x64-pkg",
        ("macos", "aarch64") => "osx-x64-pkg",
        ("macos", "i686") => "osx-x86-tar",
        ("linux", "x86_64") => "linux-x64",
        ("linux", "i686") => "linux-x86",
        ("linux", "aarch64") => "linux-aarch64",
        ("linux", "armv7l") => "linux-armv7",
        ("linux", "armv6l") => "linux-armv6",
        ("freebsd", "x86_64") => "freebsd-x64",
        ("freebsd", "i686") => "freebsd-x86",
        ("openbsd", "x86_64") => "openbsd-x64",
        ("openbsd", "i686") => "openbsd-x86",
        ("sunos", "x86_64") => "sunos-x64",
        ("sunos", "i686") => "sunos-x86",
        _ => panic!("Unsupported platform or architecture"),
    }
}
