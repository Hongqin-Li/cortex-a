// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2021 by the author(s)
//
// Author(s):
//   - Hongqin Li <ihongqinli@gmail.com>

//! Architectural Feature Access Control Register - EL1
//!
//! Controls access to trace, SVE, and Advanced SIMD and
//! floating-point functionality.

use register::{cpu::RegisterReadWrite, register_bitfields};

register_bitfields! {u64,
    pub CPACR_EL1 [
        /// Traps EL0 and EL1 System register accesses to all implemented trace registers from both
        /// Execution states to EL1, or to EL2 when it is implemented and enabled in the current
        /// Security state and HCR_EL2.TGE is 1, as follows:
        ///
        /// - In AArch64 state, accesses to trace registers are trapped, reported using ESR_ELx.EC
        ///   value 0x18.
        /// - In AArch32 state, MRC and MCR accesses to trace registers are trapped, reported using
        /// ESR_ELx.EC value 0x05.
        /// - In AArch32 state, MRRC and MCRR accesses to trace registers are trapped, reported
        /// using ESR_ELx.EC value 0x0C.
        ///
        /// 0 This control does not cause any instructions to be trapped.
        ///
        /// 1 This control causes EL0 and EL1 System register accesses to all implemented trace
        ///   registers to be trapped.
        ///
        /// System register accesses to the trace registers can have side-effects. When a System
        /// register access is trapped, any side-effects that are normally associated with the
        /// access do not occur before the exception is taken.
        TTA OFFSET(28) NUMBITS(1) [
            Enable = 0b0,
            Disable = 0b1
        ],

        /// Traps execution at EL1 and EL0 of instructions that access the Advanced SIMD and
        /// floating-point registers from both Execution states to EL1, reported using ESR_ELx.EC
        /// value 0x07, or to EL2 reported using ESR_ELx.EC value 0x00 when EL2 is implemented and
        /// enabled in the current Security state and HCR_EL2.TGE is 1, as follows:
        ///
        /// - In AArch64 state, accesses to FPCR, FPSR, any of the SIMD and floating-point registers
        ///   V0-V31, including their views as D0-D31 registers or S0-31 registers.
        ///
        /// - In AArch32 state, FPSCR, and any of the SIMD and floating-point registers Q0-15,
        ///   including their views as D0-D31 registers or S0-31 registers.
        ///
        /// Traps execution at EL1 and EL0 of SVE instructions to EL1, or to EL2 when EL2 is
        /// implemented and enabled for the current Security state and HCR_EL2.TGE is 1.
        /// The exception is reported using ESR_ELx.EC value 0x07.
        ///
        /// A trap taken as a result of CPACR_EL1.ZEN has precedence over a trap taken as a result
        /// of CPACR_EL1.FPEN.
        ///
        /// 00 This control causes execution of these instructions at EL1 and EL0 to be trapped.
        ///
        /// 01 This control causes execution of these instructions at EL0 to be trapped, but does
        ///    not cause execution of any instructions at EL1 to be trapped.
        ///
        /// 10 This control causes execution of these instructions at EL1 and EL0 to be trapped.
        ///
        /// 11 This control does not cause execution of any instructions to be trapped.
        ///
        /// Writes to MVFR0, MVFR1 and MVFR2 from EL1 or higher are CONSTRAINED UNPREDICTABLE and
        /// whether these accesses can be trapped by this control depends on implemented CONSTRAINED
        /// UNPREDICTABLE behavior.
        FPEN OFFSET(20) NUMBITS(2) [
            Disable = 0b00,
            EnableAtEL1 = 0b01,
            Disable2 = 0b10,
            Enable = 0b11
        ],

        /// Traps execution at EL1 and EL0 of SVE instructions and instructions that directly access
        /// the ZCR_EL1 System register to EL1, or to EL2 when EL2 is implemented and enabled in the
        /// current Security state and HCR_EL2.TGE is 1.
        ///
        /// The exception is reported using ESR_ELx.EC value 0x19.
        ///
        /// A trap taken as a result of CPACR_EL1.ZEN has precedence over a trap taken as a result
        /// of CPACR_EL1.FPEN.
        ///
        /// 00 This control causes execution of these instructions at EL1 and EL0 to be trapped.
        ///
        /// 01 This control causes execution of these instructions at EL0 to be trapped, but does
        ///    not cause execution of any instructions at EL1 to be trapped.
        ///
        /// 10 This control causes execution of these instructions at EL1 and EL0 to be trapped.
        ///
        /// 11 This control does not cause execution of any instructions to be trapped.
        ZEN OFFSET(16) NUMBITS(2) [
            Disable = 0b00,
            EnableAtEL1 = 0b01,
            Disable2 = 0b10,
            Enable = 0b11
        ]
    ]
}

pub struct Reg;

impl RegisterReadWrite<u64, CPACR_EL1::Register> for Reg {
    sys_coproc_read_raw!(u64, "CPACR_EL1", "x");
    sys_coproc_write_raw!(u64, "CPACR_EL1", "x");
}

pub static CPACR_EL1: Reg = Reg {};
