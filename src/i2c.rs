#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    i2c00: I2c00,
    i2c04: I2c04,
    i2c08: I2c08,
    i2c0c: I2c0c,
    i2c10: I2c10,
    i2c14: I2c14,
    i2c18: I2c18,
    i2c1c: I2c1c,
    i2c20: I2c20,
    i2c24: I2c24,
    i2c28: I2c28,
    i2c2c: I2c2c,
    i2c30: I2c30,
    i2c34: I2c34,
    i2c38: I2c38,
    i2c3c: I2c3c,
    i2c40: I2c40,
    i2c44: I2c44,
    i2c48: I2c48,
    i2c4c: I2c4c,
    i2c50: I2c50,
    i2c54: I2c54,
    _reserved22: [u8; 0x08],
    i2c60: I2c60,
    i2c64: I2c64,
    i2c68: I2c68,
    i2c6c: I2c6c,
    i2c70: I2c70,
    i2c74: I2c74,
    i2c78: I2c78,
    i2c7c: I2c7c,
    i2c80: I2c80,
    i2c84: I2c84,
    i2c88: I2c88,
    i2c8c: I2c8c,
    i2c90: I2c90,
    i2c94: I2c94,
    i2c98: I2c98,
    i2c9c: I2c9c,
}
impl RegisterBlock {
    #[doc = "0x00 - Master/Slave Function Control Register"]
    #[inline(always)]
    pub const fn i2c00(&self) -> &I2c00 {
        &self.i2c00
    }
    #[doc = "0x04 - Master/Slave Clock and AC Timing Control Register"]
    #[inline(always)]
    pub const fn i2c04(&self) -> &I2c04 {
        &self.i2c04
    }
    #[doc = "0x08 - Master/Slave Transmit/Receive Byte Buffer Register"]
    #[inline(always)]
    pub const fn i2c08(&self) -> &I2c08 {
        &self.i2c08
    }
    #[doc = "0x0c - I2CC\\_BUFCTL"]
    #[inline(always)]
    pub const fn i2c0c(&self) -> &I2c0c {
        &self.i2c0c
    }
    #[doc = "0x10 - Master Interrupt Control Register"]
    #[inline(always)]
    pub const fn i2c10(&self) -> &I2c10 {
        &self.i2c10
    }
    #[doc = "0x14 - Master Interrupt Status Register"]
    #[inline(always)]
    pub const fn i2c14(&self) -> &I2c14 {
        &self.i2c14
    }
    #[doc = "0x18 - Master Command/Status Register"]
    #[inline(always)]
    pub const fn i2c18(&self) -> &I2c18 {
        &self.i2c18
    }
    #[doc = "0x1c - Master DMA Buffer Length Register"]
    #[inline(always)]
    pub const fn i2c1c(&self) -> &I2c1c {
        &self.i2c1c
    }
    #[doc = "0x20 - Slave Interrupt Control Register"]
    #[inline(always)]
    pub const fn i2c20(&self) -> &I2c20 {
        &self.i2c20
    }
    #[doc = "0x24 - Slave Interrupt Status Register"]
    #[inline(always)]
    pub const fn i2c24(&self) -> &I2c24 {
        &self.i2c24
    }
    #[doc = "0x28 - Slave Command/Status Register"]
    #[inline(always)]
    pub const fn i2c28(&self) -> &I2c28 {
        &self.i2c28
    }
    #[doc = "0x2c - Slave DMA Buffer Length Register"]
    #[inline(always)]
    pub const fn i2c2c(&self) -> &I2c2c {
        &self.i2c2c
    }
    #[doc = "0x30 - Master DMA Mode Tx Buffer Base Address"]
    #[inline(always)]
    pub const fn i2c30(&self) -> &I2c30 {
        &self.i2c30
    }
    #[doc = "0x34 - Master DMA Mode Rx Buffer Base Address"]
    #[inline(always)]
    pub const fn i2c34(&self) -> &I2c34 {
        &self.i2c34
    }
    #[doc = "0x38 - Slave DMA Mode Tx Buffer Base Address"]
    #[inline(always)]
    pub const fn i2c38(&self) -> &I2c38 {
        &self.i2c38
    }
    #[doc = "0x3c - Slave DMA Mode Rx Buffer Base Address"]
    #[inline(always)]
    pub const fn i2c3c(&self) -> &I2c3c {
        &self.i2c3c
    }
    #[doc = "0x40 - Slave Device Address Register"]
    #[inline(always)]
    pub const fn i2c40(&self) -> &I2c40 {
        &self.i2c40
    }
    #[doc = "0x44 - Slave Device Address Register (extra)"]
    #[inline(always)]
    pub const fn i2c44(&self) -> &I2c44 {
        &self.i2c44
    }
    #[doc = "0x48 - Master DMA Length Status Register"]
    #[inline(always)]
    pub const fn i2c48(&self) -> &I2c48 {
        &self.i2c48
    }
    #[doc = "0x4c - Slave DMA Length Status Register"]
    #[inline(always)]
    pub const fn i2c4c(&self) -> &I2c4c {
        &self.i2c4c
    }
    #[doc = "0x50 - I2CC\\_DMA\\_CFG"]
    #[inline(always)]
    pub const fn i2c50(&self) -> &I2c50 {
        &self.i2c50
    }
    #[doc = "0x54 - I2CC\\_DMA\\_STA"]
    #[inline(always)]
    pub const fn i2c54(&self) -> &I2c54 {
        &self.i2c54
    }
    #[doc = "0x60 - Master DMA Mode Tx Buffer Base Address\\[39:32\\]"]
    #[inline(always)]
    pub const fn i2c60(&self) -> &I2c60 {
        &self.i2c60
    }
    #[doc = "0x64 - Master DMA Mode Rx Buffer Base Address\\[39:32\\]"]
    #[inline(always)]
    pub const fn i2c64(&self) -> &I2c64 {
        &self.i2c64
    }
    #[doc = "0x68 - Slave DMA Mode Tx Buffer Base Address\\[39:32\\]"]
    #[inline(always)]
    pub const fn i2c68(&self) -> &I2c68 {
        &self.i2c68
    }
    #[doc = "0x6c - Slave DMA Mode Rx Buffer Base Address\\[39:32\\]"]
    #[inline(always)]
    pub const fn i2c6c(&self) -> &I2c6c {
        &self.i2c6c
    }
    #[doc = "0x70 - MISC configuration for AC timing0"]
    #[inline(always)]
    pub const fn i2c70(&self) -> &I2c70 {
        &self.i2c70
    }
    #[doc = "0x74 - MISC configuration for AC timing1"]
    #[inline(always)]
    pub const fn i2c74(&self) -> &I2c74 {
        &self.i2c74
    }
    #[doc = "0x78 - Debug information for device"]
    #[inline(always)]
    pub const fn i2c78(&self) -> &I2c78 {
        &self.i2c78
    }
    #[doc = "0x7c - I2CC\\_HS\\_ACTIME"]
    #[inline(always)]
    pub const fn i2c7c(&self) -> &I2c7c {
        &self.i2c7c
    }
    #[doc = "0x80 - Recorder information for Slave address matching"]
    #[inline(always)]
    pub const fn i2c80(&self) -> &I2c80 {
        &self.i2c80
    }
    #[doc = "0x84 - Recorder information for Byte transfer"]
    #[inline(always)]
    pub const fn i2c84(&self) -> &I2c84 {
        &self.i2c84
    }
    #[doc = "0x88 - I2CC\\_MIRQ\\_LOG"]
    #[inline(always)]
    pub const fn i2c88(&self) -> &I2c88 {
        &self.i2c88
    }
    #[doc = "0x8c - I2CC\\_SIRQ\\_LOG"]
    #[inline(always)]
    pub const fn i2c8c(&self) -> &I2c8c {
        &self.i2c8c
    }
    #[doc = "0x90 - I2CC\\_WLOCK"]
    #[inline(always)]
    pub const fn i2c90(&self) -> &I2c90 {
        &self.i2c90
    }
    #[doc = "0x94 - I2CC\\_VERSION\\_CTL"]
    #[inline(always)]
    pub const fn i2c94(&self) -> &I2c94 {
        &self.i2c94
    }
    #[doc = "0x98 - I2CM\\_MCMDQ"]
    #[inline(always)]
    pub const fn i2c98(&self) -> &I2c98 {
        &self.i2c98
    }
    #[doc = "0x9c - I2CC\\_MISC2"]
    #[inline(always)]
    pub const fn i2c9c(&self) -> &I2c9c {
        &self.i2c9c
    }
}
#[doc = "I2C00 (rw) register accessor: Master/Slave Function Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c00::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c00::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c00`] module"]
#[doc(alias = "I2C00")]
pub type I2c00 = crate::Reg<i2c00::I2c00Spec>;
#[doc = "Master/Slave Function Control Register"]
pub mod i2c00;
#[doc = "I2C04 (rw) register accessor: Master/Slave Clock and AC Timing Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c04::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c04::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c04`] module"]
#[doc(alias = "I2C04")]
pub type I2c04 = crate::Reg<i2c04::I2c04Spec>;
#[doc = "Master/Slave Clock and AC Timing Control Register"]
pub mod i2c04;
#[doc = "I2C08 (rw) register accessor: Master/Slave Transmit/Receive Byte Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c08::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c08::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c08`] module"]
#[doc(alias = "I2C08")]
pub type I2c08 = crate::Reg<i2c08::I2c08Spec>;
#[doc = "Master/Slave Transmit/Receive Byte Buffer Register"]
pub mod i2c08;
#[doc = "I2C0C (rw) register accessor: I2CC\\_BUFCTL\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c0c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c0c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c0c`] module"]
#[doc(alias = "I2C0C")]
pub type I2c0c = crate::Reg<i2c0c::I2c0cSpec>;
#[doc = "I2CC\\_BUFCTL"]
pub mod i2c0c;
#[doc = "I2C10 (rw) register accessor: Master Interrupt Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c10`] module"]
#[doc(alias = "I2C10")]
pub type I2c10 = crate::Reg<i2c10::I2c10Spec>;
#[doc = "Master Interrupt Control Register"]
pub mod i2c10;
#[doc = "I2C14 (rw) register accessor: Master Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c14`] module"]
#[doc(alias = "I2C14")]
pub type I2c14 = crate::Reg<i2c14::I2c14Spec>;
#[doc = "Master Interrupt Status Register"]
pub mod i2c14;
#[doc = "I2C18 (rw) register accessor: Master Command/Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c18`] module"]
#[doc(alias = "I2C18")]
pub type I2c18 = crate::Reg<i2c18::I2c18Spec>;
#[doc = "Master Command/Status Register"]
pub mod i2c18;
#[doc = "I2C1C (rw) register accessor: Master DMA Buffer Length Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c1c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c1c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c1c`] module"]
#[doc(alias = "I2C1C")]
pub type I2c1c = crate::Reg<i2c1c::I2c1cSpec>;
#[doc = "Master DMA Buffer Length Register"]
pub mod i2c1c;
#[doc = "I2C20 (rw) register accessor: Slave Interrupt Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c20`] module"]
#[doc(alias = "I2C20")]
pub type I2c20 = crate::Reg<i2c20::I2c20Spec>;
#[doc = "Slave Interrupt Control Register"]
pub mod i2c20;
#[doc = "I2C24 (rw) register accessor: Slave Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c24`] module"]
#[doc(alias = "I2C24")]
pub type I2c24 = crate::Reg<i2c24::I2c24Spec>;
#[doc = "Slave Interrupt Status Register"]
pub mod i2c24;
#[doc = "I2C28 (rw) register accessor: Slave Command/Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c28`] module"]
#[doc(alias = "I2C28")]
pub type I2c28 = crate::Reg<i2c28::I2c28Spec>;
#[doc = "Slave Command/Status Register"]
pub mod i2c28;
#[doc = "I2C2C (rw) register accessor: Slave DMA Buffer Length Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c2c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c2c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c2c`] module"]
#[doc(alias = "I2C2C")]
pub type I2c2c = crate::Reg<i2c2c::I2c2cSpec>;
#[doc = "Slave DMA Buffer Length Register"]
pub mod i2c2c;
#[doc = "I2C30 (rw) register accessor: Master DMA Mode Tx Buffer Base Address\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c30`] module"]
#[doc(alias = "I2C30")]
pub type I2c30 = crate::Reg<i2c30::I2c30Spec>;
#[doc = "Master DMA Mode Tx Buffer Base Address"]
pub mod i2c30;
#[doc = "I2C34 (rw) register accessor: Master DMA Mode Rx Buffer Base Address\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c34::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c34::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c34`] module"]
#[doc(alias = "I2C34")]
pub type I2c34 = crate::Reg<i2c34::I2c34Spec>;
#[doc = "Master DMA Mode Rx Buffer Base Address"]
pub mod i2c34;
#[doc = "I2C38 (rw) register accessor: Slave DMA Mode Tx Buffer Base Address\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c38::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c38::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c38`] module"]
#[doc(alias = "I2C38")]
pub type I2c38 = crate::Reg<i2c38::I2c38Spec>;
#[doc = "Slave DMA Mode Tx Buffer Base Address"]
pub mod i2c38;
#[doc = "I2C3C (rw) register accessor: Slave DMA Mode Rx Buffer Base Address\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c3c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c3c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c3c`] module"]
#[doc(alias = "I2C3C")]
pub type I2c3c = crate::Reg<i2c3c::I2c3cSpec>;
#[doc = "Slave DMA Mode Rx Buffer Base Address"]
pub mod i2c3c;
#[doc = "I2C40 (rw) register accessor: Slave Device Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c40::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c40::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c40`] module"]
#[doc(alias = "I2C40")]
pub type I2c40 = crate::Reg<i2c40::I2c40Spec>;
#[doc = "Slave Device Address Register"]
pub mod i2c40;
#[doc = "I2C44 (rw) register accessor: Slave Device Address Register (extra)\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c44::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c44::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c44`] module"]
#[doc(alias = "I2C44")]
pub type I2c44 = crate::Reg<i2c44::I2c44Spec>;
#[doc = "Slave Device Address Register (extra)"]
pub mod i2c44;
#[doc = "I2C48 (rw) register accessor: Master DMA Length Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c48::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c48::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c48`] module"]
#[doc(alias = "I2C48")]
pub type I2c48 = crate::Reg<i2c48::I2c48Spec>;
#[doc = "Master DMA Length Status Register"]
pub mod i2c48;
#[doc = "I2C4C (rw) register accessor: Slave DMA Length Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c4c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c4c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c4c`] module"]
#[doc(alias = "I2C4C")]
pub type I2c4c = crate::Reg<i2c4c::I2c4cSpec>;
#[doc = "Slave DMA Length Status Register"]
pub mod i2c4c;
#[doc = "I2C50 (rw) register accessor: I2CC\\_DMA\\_CFG\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c50::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c50::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c50`] module"]
#[doc(alias = "I2C50")]
pub type I2c50 = crate::Reg<i2c50::I2c50Spec>;
#[doc = "I2CC\\_DMA\\_CFG"]
pub mod i2c50;
#[doc = "I2C54 (rw) register accessor: I2CC\\_DMA\\_STA\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c54::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c54::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c54`] module"]
#[doc(alias = "I2C54")]
pub type I2c54 = crate::Reg<i2c54::I2c54Spec>;
#[doc = "I2CC\\_DMA\\_STA"]
pub mod i2c54;
#[doc = "I2C60 (rw) register accessor: Master DMA Mode Tx Buffer Base Address\\[39:32\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c60::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c60::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c60`] module"]
#[doc(alias = "I2C60")]
pub type I2c60 = crate::Reg<i2c60::I2c60Spec>;
#[doc = "Master DMA Mode Tx Buffer Base Address\\[39:32\\]"]
pub mod i2c60;
#[doc = "I2C64 (rw) register accessor: Master DMA Mode Rx Buffer Base Address\\[39:32\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c64::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c64::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c64`] module"]
#[doc(alias = "I2C64")]
pub type I2c64 = crate::Reg<i2c64::I2c64Spec>;
#[doc = "Master DMA Mode Rx Buffer Base Address\\[39:32\\]"]
pub mod i2c64;
#[doc = "I2C68 (rw) register accessor: Slave DMA Mode Tx Buffer Base Address\\[39:32\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c68::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c68::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c68`] module"]
#[doc(alias = "I2C68")]
pub type I2c68 = crate::Reg<i2c68::I2c68Spec>;
#[doc = "Slave DMA Mode Tx Buffer Base Address\\[39:32\\]"]
pub mod i2c68;
#[doc = "I2C6C (rw) register accessor: Slave DMA Mode Rx Buffer Base Address\\[39:32\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c6c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c6c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c6c`] module"]
#[doc(alias = "I2C6C")]
pub type I2c6c = crate::Reg<i2c6c::I2c6cSpec>;
#[doc = "Slave DMA Mode Rx Buffer Base Address\\[39:32\\]"]
pub mod i2c6c;
#[doc = "I2C70 (rw) register accessor: MISC configuration for AC timing0\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c70::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c70::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c70`] module"]
#[doc(alias = "I2C70")]
pub type I2c70 = crate::Reg<i2c70::I2c70Spec>;
#[doc = "MISC configuration for AC timing0"]
pub mod i2c70;
#[doc = "I2C74 (rw) register accessor: MISC configuration for AC timing1\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c74::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c74::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c74`] module"]
#[doc(alias = "I2C74")]
pub type I2c74 = crate::Reg<i2c74::I2c74Spec>;
#[doc = "MISC configuration for AC timing1"]
pub mod i2c74;
#[doc = "I2C78 (rw) register accessor: Debug information for device\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c78::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c78::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c78`] module"]
#[doc(alias = "I2C78")]
pub type I2c78 = crate::Reg<i2c78::I2c78Spec>;
#[doc = "Debug information for device"]
pub mod i2c78;
#[doc = "I2C7C (rw) register accessor: I2CC\\_HS\\_ACTIME\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c7c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c7c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c7c`] module"]
#[doc(alias = "I2C7C")]
pub type I2c7c = crate::Reg<i2c7c::I2c7cSpec>;
#[doc = "I2CC\\_HS\\_ACTIME"]
pub mod i2c7c;
#[doc = "I2C80 (rw) register accessor: Recorder information for Slave address matching\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c80::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c80::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c80`] module"]
#[doc(alias = "I2C80")]
pub type I2c80 = crate::Reg<i2c80::I2c80Spec>;
#[doc = "Recorder information for Slave address matching"]
pub mod i2c80;
#[doc = "I2C84 (rw) register accessor: Recorder information for Byte transfer\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c84::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c84::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c84`] module"]
#[doc(alias = "I2C84")]
pub type I2c84 = crate::Reg<i2c84::I2c84Spec>;
#[doc = "Recorder information for Byte transfer"]
pub mod i2c84;
#[doc = "I2C88 (rw) register accessor: I2CC\\_MIRQ\\_LOG\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c88::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c88::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c88`] module"]
#[doc(alias = "I2C88")]
pub type I2c88 = crate::Reg<i2c88::I2c88Spec>;
#[doc = "I2CC\\_MIRQ\\_LOG"]
pub mod i2c88;
#[doc = "I2C8C (rw) register accessor: I2CC\\_SIRQ\\_LOG\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c8c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c8c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c8c`] module"]
#[doc(alias = "I2C8C")]
pub type I2c8c = crate::Reg<i2c8c::I2c8cSpec>;
#[doc = "I2CC\\_SIRQ\\_LOG"]
pub mod i2c8c;
#[doc = "I2C90 (rw) register accessor: I2CC\\_WLOCK\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c90::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c90::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c90`] module"]
#[doc(alias = "I2C90")]
pub type I2c90 = crate::Reg<i2c90::I2c90Spec>;
#[doc = "I2CC\\_WLOCK"]
pub mod i2c90;
#[doc = "I2C94 (rw) register accessor: I2CC\\_VERSION\\_CTL\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c94::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c94::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c94`] module"]
#[doc(alias = "I2C94")]
pub type I2c94 = crate::Reg<i2c94::I2c94Spec>;
#[doc = "I2CC\\_VERSION\\_CTL"]
pub mod i2c94;
#[doc = "I2C98 (rw) register accessor: I2CM\\_MCMDQ\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c98::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c98::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c98`] module"]
#[doc(alias = "I2C98")]
pub type I2c98 = crate::Reg<i2c98::I2c98Spec>;
#[doc = "I2CM\\_MCMDQ"]
pub mod i2c98;
#[doc = "I2C9C (rw) register accessor: I2CC\\_MISC2\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c9c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c9c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c9c`] module"]
#[doc(alias = "I2C9C")]
pub type I2c9c = crate::Reg<i2c9c::I2c9cSpec>;
#[doc = "I2CC\\_MISC2"]
pub mod i2c9c;
