// SPDX-License-Identifier: Apache-2.0

//! Page type encoding shared by the (Linux-only) SNP launch path and the
//! cross-platform measurement code. Kept in its own, feature- and OS-agnostic
//! module so the measurement calculation can be built for targets such as
//! `wasm32` where the `launch` module (which is `target_os = "linux"` only) is
//! not compiled.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Encoded page types for a launch update. See Table 58 of the SNP Firmware
/// specification for further details.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(C)]
#[non_exhaustive]
pub enum PageType {
    /// A normal data page.
    Normal = 0x1,

    /// A VMSA page.
    Vmsa = 0x2,

    /// A page full of zeroes.
    Zero = 0x3,

    /// A page that is encrypted but not measured
    Unmeasured = 0x4,

    /// A page for the firmware to store secrets for the guest.
    Secrets = 0x5,

    /// A page for the hypervisor to provide CPUID function values.
    Cpuid = 0x6,
}
