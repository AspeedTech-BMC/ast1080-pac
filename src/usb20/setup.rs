#[repr(C)]
#[doc = "SETUP\\[%s\\]"]
#[doc(alias = "SETUP")]
pub struct Setup {
    setupbuff0: Setupbuff0,
    setupbuff1: Setupbuff1,
}
impl Setup {
    #[doc = "0x00 - Root Device SETUP Data Buffer"]
    #[inline(always)]
    pub const fn setupbuff0(&self) -> &Setupbuff0 {
        &self.setupbuff0
    }
    #[doc = "0x04 - Root Device SETUP Data Buffer"]
    #[inline(always)]
    pub const fn setupbuff1(&self) -> &Setupbuff1 {
        &self.setupbuff1
    }
}
#[doc = "SETUPBUFF0 (rw) register accessor: Root Device SETUP Data Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`setupbuff0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`setupbuff0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@setupbuff0`] module"]
#[doc(alias = "SETUPBUFF0")]
pub type Setupbuff0 = crate::Reg<setupbuff0::Setupbuff0Spec>;
#[doc = "Root Device SETUP Data Buffer"]
pub mod setupbuff0;
#[doc = "SETUPBUFF1 (rw) register accessor: Root Device SETUP Data Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`setupbuff1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`setupbuff1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@setupbuff1`] module"]
#[doc(alias = "SETUPBUFF1")]
pub type Setupbuff1 = crate::Reg<setupbuff1::Setupbuff1Spec>;
#[doc = "Root Device SETUP Data Buffer"]
pub mod setupbuff1;
