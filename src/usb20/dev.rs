#[repr(C)]
#[doc = "DEV\\[%s\\]"]
#[doc(alias = "DEV")]
pub struct Dev {
    dev00: Dev00,
    dev04: Dev04,
    dev08: Dev08,
    dev0c: Dev0c,
}
impl Dev {
    #[doc = "0x00 - Downstream Device Function Enable Control Register"]
    #[inline(always)]
    pub const fn dev00(&self) -> &Dev00 {
        &self.dev00
    }
    #[doc = "0x04 - Interrupt Status"]
    #[inline(always)]
    pub const fn dev04(&self) -> &Dev04 {
        &self.dev04
    }
    #[doc = "0x08 - Endpoint 0 Control/Status Register"]
    #[inline(always)]
    pub const fn dev08(&self) -> &Dev08 {
        &self.dev08
    }
    #[doc = "0x0c - Base Address of Endpoint 0 IN/OUT Data Buffer Register"]
    #[inline(always)]
    pub const fn dev0c(&self) -> &Dev0c {
        &self.dev0c
    }
}
#[doc = "DEV00 (rw) register accessor: Downstream Device Function Enable Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dev00::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev00::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev00`] module"]
#[doc(alias = "DEV00")]
pub type Dev00 = crate::Reg<dev00::Dev00Spec>;
#[doc = "Downstream Device Function Enable Control Register"]
pub mod dev00;
#[doc = "DEV04 (rw) register accessor: Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`dev04::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev04::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev04`] module"]
#[doc(alias = "DEV04")]
pub type Dev04 = crate::Reg<dev04::Dev04Spec>;
#[doc = "Interrupt Status"]
pub mod dev04;
#[doc = "DEV08 (rw) register accessor: Endpoint 0 Control/Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dev08::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev08::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev08`] module"]
#[doc(alias = "DEV08")]
pub type Dev08 = crate::Reg<dev08::Dev08Spec>;
#[doc = "Endpoint 0 Control/Status Register"]
pub mod dev08;
#[doc = "DEV0C (rw) register accessor: Base Address of Endpoint 0 IN/OUT Data Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dev0c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev0c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev0c`] module"]
#[doc(alias = "DEV0C")]
pub type Dev0c = crate::Reg<dev0c::Dev0cSpec>;
#[doc = "Base Address of Endpoint 0 IN/OUT Data Buffer Register"]
pub mod dev0c;
