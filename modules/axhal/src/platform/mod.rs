//! Platform-specific operations.

mod aarch64_common;
pub use self::aarch64_common::*;

mod qemu_virt_aarch64;
pub use self::qemu_virt_aarch64::*;