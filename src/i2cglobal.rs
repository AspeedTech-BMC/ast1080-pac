#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    i2cg00: I2cg00,
    i2cg04: I2cg04,
    _reserved2: [u8; 0x04],
    i2cg0c: I2cg0c,
    i2cg10: I2cg10,
}
impl RegisterBlock {
    #[doc = "0x00 - Device Master Mode Interrupt Status Register"]
    #[inline(always)]
    pub const fn i2cg00(&self) -> &I2cg00 {
        &self.i2cg00
    }
    #[doc = "0x04 - Device Slave Mode Interrupt Status Register"]
    #[inline(always)]
    pub const fn i2cg04(&self) -> &I2cg04 {
        &self.i2cg04
    }
    #[doc = "0x0c - Global Control Register"]
    #[inline(always)]
    pub const fn i2cg0c(&self) -> &I2cg0c {
        &self.i2cg0c
    }
    #[doc = "0x10 - New Clock Divider Control Register"]
    #[inline(always)]
    pub const fn i2cg10(&self) -> &I2cg10 {
        &self.i2cg10
    }
}
#[doc = "I2CG00 (rw) register accessor: Device Master Mode Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cg00::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cg00::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cg00`] module"]
#[doc(alias = "I2CG00")]
pub type I2cg00 = crate::Reg<i2cg00::I2cg00Spec>;
#[doc = "Device Master Mode Interrupt Status Register"]
pub mod i2cg00;
#[doc = "I2CG04 (rw) register accessor: Device Slave Mode Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cg04::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cg04::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cg04`] module"]
#[doc(alias = "I2CG04")]
pub type I2cg04 = crate::Reg<i2cg04::I2cg04Spec>;
#[doc = "Device Slave Mode Interrupt Status Register"]
pub mod i2cg04;
#[doc = "I2CG0C (rw) register accessor: Global Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cg0c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cg0c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cg0c`] module"]
#[doc(alias = "I2CG0C")]
pub type I2cg0c = crate::Reg<i2cg0c::I2cg0cSpec>;
#[doc = "Global Control Register"]
pub mod i2cg0c;
#[doc = "I2CG10 (rw) register accessor: New Clock Divider Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cg10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cg10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cg10`] module"]
#[doc(alias = "I2CG10")]
pub type I2cg10 = crate::Reg<i2cg10::I2cg10Spec>;
#[doc = "New Clock Divider Control Register"]
pub mod i2cg10;
