#[repr(C)]
#[doc = "EPP\\[%s\\]"]
#[doc(alias = "EPP")]
pub struct Epp {
    epp00: Epp00,
    epp04: Epp04,
    epp08: Epp08,
    epp0c: Epp0c,
}
impl Epp {
    #[doc = "0x00 - Endpoint Configuration Register"]
    #[inline(always)]
    pub const fn epp00(&self) -> &Epp00 {
        &self.epp00
    }
    #[doc = "0x04 - DMA Descriptor List Control/Status Register"]
    #[inline(always)]
    pub const fn epp04(&self) -> &Epp04 {
        &self.epp04
    }
    #[doc = "0x08 - DMA Descriptor/Buffer Base Address"]
    #[inline(always)]
    pub const fn epp08(&self) -> &Epp08 {
        &self.epp08
    }
    #[doc = "0x0c - DMA Descriptor List Read(DMA)/Write(CPU) Pointer and Status"]
    #[inline(always)]
    pub const fn epp0c(&self) -> &Epp0c {
        &self.epp0c
    }
}
#[doc = "EPP00 (rw) register accessor: Endpoint Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`epp00::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`epp00::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epp00`] module"]
#[doc(alias = "EPP00")]
pub type Epp00 = crate::Reg<epp00::Epp00Spec>;
#[doc = "Endpoint Configuration Register"]
pub mod epp00;
#[doc = "EPP04 (rw) register accessor: DMA Descriptor List Control/Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`epp04::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`epp04::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epp04`] module"]
#[doc(alias = "EPP04")]
pub type Epp04 = crate::Reg<epp04::Epp04Spec>;
#[doc = "DMA Descriptor List Control/Status Register"]
pub mod epp04;
#[doc = "EPP08 (rw) register accessor: DMA Descriptor/Buffer Base Address\n\nYou can [`read`](crate::Reg::read) this register and get [`epp08::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`epp08::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epp08`] module"]
#[doc(alias = "EPP08")]
pub type Epp08 = crate::Reg<epp08::Epp08Spec>;
#[doc = "DMA Descriptor/Buffer Base Address"]
pub mod epp08;
#[doc = "EPP0C (rw) register accessor: DMA Descriptor List Read(DMA)/Write(CPU) Pointer and Status\n\nYou can [`read`](crate::Reg::read) this register and get [`epp0c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`epp0c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epp0c`] module"]
#[doc(alias = "EPP0C")]
pub type Epp0c = crate::Reg<epp0c::Epp0cSpec>;
#[doc = "DMA Descriptor List Read(DMA)/Write(CPU) Pointer and Status"]
pub mod epp0c;
