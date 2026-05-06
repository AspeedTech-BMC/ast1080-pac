#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    i2cglobal000: I2cglobal000,
    i2cglobal004: I2cglobal004,
    _reserved2: [u8; 0x04],
    i2cglobal00c: I2cglobal00c,
    i2cglobal010: I2cglobal010,
    i2cglobal014: I2cglobal014,
    i2cglobal018: I2cglobal018,
    i2cglobal01c: I2cglobal01c,
    i2cglobal020: I2cglobal020,
}
impl RegisterBlock {
    #[doc = "0x00 - Device Master Mode Interrupt Status Register"]
    #[inline(always)]
    pub const fn i2cglobal000(&self) -> &I2cglobal000 {
        &self.i2cglobal000
    }
    #[doc = "0x04 - Device Slave Mode Interrupt Status Register"]
    #[inline(always)]
    pub const fn i2cglobal004(&self) -> &I2cglobal004 {
        &self.i2cglobal004
    }
    #[doc = "0x0c - Global Control Register"]
    #[inline(always)]
    pub const fn i2cglobal00c(&self) -> &I2cglobal00c {
        &self.i2cglobal00c
    }
    #[doc = "0x10 - New Clock Divider Control Register"]
    #[inline(always)]
    pub const fn i2cglobal010(&self) -> &I2cglobal010 {
        &self.i2cglobal010
    }
    #[doc = "0x14 - I2CG\\_FIFO\\_CFG0"]
    #[inline(always)]
    pub const fn i2cglobal014(&self) -> &I2cglobal014 {
        &self.i2cglobal014
    }
    #[doc = "0x18 - I2CG\\_FIFO\\_CFG1"]
    #[inline(always)]
    pub const fn i2cglobal018(&self) -> &I2cglobal018 {
        &self.i2cglobal018
    }
    #[doc = "0x1c - I2CG\\_MARB\\_CFG"]
    #[inline(always)]
    pub const fn i2cglobal01c(&self) -> &I2cglobal01c {
        &self.i2cglobal01c
    }
    #[doc = "0x20 - Write lock protection Register for Security"]
    #[inline(always)]
    pub const fn i2cglobal020(&self) -> &I2cglobal020 {
        &self.i2cglobal020
    }
}
#[doc = "I2CGLOBAL000 (rw) register accessor: Device Master Mode Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cglobal000::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cglobal000::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cglobal000`] module"]
#[doc(alias = "I2CGLOBAL000")]
pub type I2cglobal000 = crate::Reg<i2cglobal000::I2cglobal000Spec>;
#[doc = "Device Master Mode Interrupt Status Register"]
pub mod i2cglobal000;
#[doc = "I2CGLOBAL004 (rw) register accessor: Device Slave Mode Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cglobal004::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cglobal004::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cglobal004`] module"]
#[doc(alias = "I2CGLOBAL004")]
pub type I2cglobal004 = crate::Reg<i2cglobal004::I2cglobal004Spec>;
#[doc = "Device Slave Mode Interrupt Status Register"]
pub mod i2cglobal004;
#[doc = "I2CGLOBAL00C (rw) register accessor: Global Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cglobal00c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cglobal00c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cglobal00c`] module"]
#[doc(alias = "I2CGLOBAL00C")]
pub type I2cglobal00c = crate::Reg<i2cglobal00c::I2cglobal00cSpec>;
#[doc = "Global Control Register"]
pub mod i2cglobal00c;
#[doc = "I2CGLOBAL010 (rw) register accessor: New Clock Divider Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cglobal010::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cglobal010::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cglobal010`] module"]
#[doc(alias = "I2CGLOBAL010")]
pub type I2cglobal010 = crate::Reg<i2cglobal010::I2cglobal010Spec>;
#[doc = "New Clock Divider Control Register"]
pub mod i2cglobal010;
#[doc = "I2CGLOBAL014 (rw) register accessor: I2CG\\_FIFO\\_CFG0\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cglobal014::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cglobal014::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cglobal014`] module"]
#[doc(alias = "I2CGLOBAL014")]
pub type I2cglobal014 = crate::Reg<i2cglobal014::I2cglobal014Spec>;
#[doc = "I2CG\\_FIFO\\_CFG0"]
pub mod i2cglobal014;
#[doc = "I2CGLOBAL018 (rw) register accessor: I2CG\\_FIFO\\_CFG1\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cglobal018::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cglobal018::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cglobal018`] module"]
#[doc(alias = "I2CGLOBAL018")]
pub type I2cglobal018 = crate::Reg<i2cglobal018::I2cglobal018Spec>;
#[doc = "I2CG\\_FIFO\\_CFG1"]
pub mod i2cglobal018;
#[doc = "I2CGLOBAL01C (rw) register accessor: I2CG\\_MARB\\_CFG\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cglobal01c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cglobal01c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cglobal01c`] module"]
#[doc(alias = "I2CGLOBAL01C")]
pub type I2cglobal01c = crate::Reg<i2cglobal01c::I2cglobal01cSpec>;
#[doc = "I2CG\\_MARB\\_CFG"]
pub mod i2cglobal01c;
#[doc = "I2CGLOBAL020 (rw) register accessor: Write lock protection Register for Security\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cglobal020::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cglobal020::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cglobal020`] module"]
#[doc(alias = "I2CGLOBAL020")]
pub type I2cglobal020 = crate::Reg<i2cglobal020::I2cglobal020Spec>;
#[doc = "Write lock protection Register for Security"]
pub mod i2cglobal020;
