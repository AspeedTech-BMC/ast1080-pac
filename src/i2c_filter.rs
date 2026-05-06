#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    i2c_filter000: I2cFilter000,
    _reserved1: [u8; 0x04],
    i2c_filter008: I2cFilter008,
    i2c_filter00c: I2cFilter00c,
}
impl RegisterBlock {
    #[doc = "0x00 - I2CFLT\\_RST"]
    #[inline(always)]
    pub const fn i2c_filter000(&self) -> &I2cFilter000 {
        &self.i2c_filter000
    }
    #[doc = "0x08 - I2CFLT\\_IRQEN"]
    #[inline(always)]
    pub const fn i2c_filter008(&self) -> &I2cFilter008 {
        &self.i2c_filter008
    }
    #[doc = "0x0c - I2CFLT\\_IRQSTA"]
    #[inline(always)]
    pub const fn i2c_filter00c(&self) -> &I2cFilter00c {
        &self.i2c_filter00c
    }
}
#[doc = "I2C_FILTER000 (rw) register accessor: I2CFLT\\_RST\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_filter000::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_filter000::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_filter000`] module"]
#[doc(alias = "I2C_FILTER000")]
pub type I2cFilter000 = crate::Reg<i2c_filter000::I2cFilter000Spec>;
#[doc = "I2CFLT\\_RST"]
pub mod i2c_filter000;
#[doc = "I2C_FILTER008 (rw) register accessor: I2CFLT\\_IRQEN\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_filter008::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_filter008::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_filter008`] module"]
#[doc(alias = "I2C_FILTER008")]
pub type I2cFilter008 = crate::Reg<i2c_filter008::I2cFilter008Spec>;
#[doc = "I2CFLT\\_IRQEN"]
pub mod i2c_filter008;
#[doc = "I2C_FILTER00C (rw) register accessor: I2CFLT\\_IRQSTA\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_filter00c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_filter00c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_filter00c`] module"]
#[doc(alias = "I2C_FILTER00C")]
pub type I2cFilter00c = crate::Reg<i2c_filter00c::I2cFilter00cSpec>;
#[doc = "I2CFLT\\_IRQSTA"]
pub mod i2c_filter00c;
