//! mstatus register
// TODO: Virtualization, Memory Privilege and Extension Context Fields

use bit_field::BitField;

/// mstatus register
#[derive(Clone, Copy, Debug)]
pub struct Mstatus {
    bits: usize,
}

/// Additional extension state
pub enum XS {
    /// All off
    AllOff = 0,

    /// None dirty or clean, some on
    NoneDirtyOrClean = 1,

    /// None dirty, some clean
    NoneDirtySomeClean = 2,

    /// Some dirty
    SomeDirty = 3,
}

/// Floating-point extension state
pub enum FS {
    Off = 0,
    Initial = 1,
    Clean = 2,
    Dirty = 3,
}

/// Machine Previous Privilege Mode
pub enum MPP {
    Machine = 3,
    Supervisor = 1,
    User = 0,
}

/// Supervisor Previous Privilege Mode
pub enum SPP {
    Supervisor = 1,
    User = 0,
}

impl Mstatus {
    /// User Interrupt Enable
    #[inline]
    pub fn uie(&self) -> bool {
        self.bits & (1 << 0) == 1 << 0
    }

    /// Supervisor Interrupt Enable
    #[inline]
    pub fn sie(&self) -> bool {
        self.bits & (1 << 1) == 1 << 1
    }

    /// Machine Interrupt Enable
    #[inline]
    pub fn mie(&self) -> bool {
        self.bits & (1 << 3) == 1 << 3
    }

    /// User Previous Interrupt Enable
    #[inline]
    pub fn upie(&self) -> bool {
        self.bits & (1 << 4) == 1 << 4
    }

    /// Supervisor Previous Interrupt Enable
    #[inline]
    pub fn spie(&self) -> bool {
        self.bits & (1 << 5) == 1 << 5
    }

    /// User Previous Interrupt Enable
    #[inline]
    pub fn mpie(&self) -> bool {
        self.bits & (1 << 7) == 1 << 7
    }

    /// Supervisor Previous Privilege Mode
    #[inline]
    pub fn spp(&self) -> SPP {
        match self.bits & (1 << 8) == (1 << 8) {
            true => SPP::Supervisor,
            false => SPP::User,
        }
    }

    /// Machine Previous Privilege Mode
    #[inline]
    pub fn mpp(&self) -> MPP {
        match (self.bits & (0b11 << 11)) >> 11 {
            0b00 => MPP::User,
            0b01 => MPP::Supervisor,
            0b11 => MPP::Machine,
            _ => unreachable!(),
        }
    }

    /// Floating-point extension state
    ///
    /// Encodes the status of the floating-point unit,
    /// including the CSR `fcsr` and floating-point data registers `f0–f31`.
    #[inline]
    pub fn fs(&self) -> FS {
        match self.bits.get_bits(13..15) {
            0b00 => FS::Off,
            0b01 => FS::Initial,
            0b10 => FS::Clean,
            0b11 => FS::Dirty,
            _ => unreachable!(),
        }
    }

    /// Additional extension state
    ///
    /// Encodes the status of additional user-mode extensions and associated state.
    #[inline]
    pub fn xs(&self) -> XS {
        match self.bits.get_bits(15..17) {
            0b00 => XS::AllOff,
            0b01 => XS::NoneDirtyOrClean,
            0b10 => XS::NoneDirtySomeClean,
            0b11 => XS::SomeDirty,
            _ => unreachable!(),
        }
    }
}


read_csr_as!(Mstatus, 0x300, __read_mstatus);
write_csr!(0x300, __write_mstatus);
set!(0x300, __set_mstatus);
clear!(0x300, __clear_mstatus);

set_clear_csr!(
    /// User Interrupt Enable
    , set_uie, clear_uie, 1 << 0);
set_clear_csr!(
    /// Supervisor Interrupt Enable
    , set_sie, clear_sie, 1 << 1);
set_clear_csr!(
    /// Machine Interrupt Enable
    , set_mie, clear_mie, 1 << 3);
set_csr!(
    /// User Previous Interrupt Enable
    , set_upie, 1 << 4);
set_csr!(
    /// Supervisor Previous Interrupt Enable
    , set_spie, 1 << 5);
set_csr!(
    /// Machine Previous Interrupt Enable
    , set_mpie, 1 << 7);
/// Supervisor Previous Privilege Mode
#[inline]
pub unsafe fn set_spp(spp: SPP) {
    _set((spp as usize) << 8);
}
/// Machine Previous Privilege Mode
#[inline]
pub unsafe fn set_mpp(mpp: MPP) {
    _set((mpp as usize) << 11);
}

/// Floating-point extension state
#[inline]
pub unsafe fn set_fs(fs: FS) {
    let mut value = _read();
    value &= !(0b11 << 13);
    value |= (fs as usize) << 13;
    _write(value);
}
