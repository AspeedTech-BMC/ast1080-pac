#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    timer000: Timer000,
    timer004: Timer004,
    timer008: Timer008,
    timer00c: Timer00c,
    timer010: Timer010,
    _reserved5: [u8; 0x08],
    timer01c: Timer01c,
}
impl RegisterBlock {
    #[doc = "0x00 - Counter Status Register"]
    #[inline(always)]
    pub const fn timer000(&self) -> &Timer000 {
        &self.timer000
    }
    #[doc = "0x04 - Counter Reload Value Register"]
    #[inline(always)]
    pub const fn timer004(&self) -> &Timer004 {
        &self.timer004
    }
    #[doc = "0x08 - Counter First Matching Register"]
    #[inline(always)]
    pub const fn timer008(&self) -> &Timer008 {
        &self.timer008
    }
    #[doc = "0x0c - Counter Second Matching Register"]
    #[inline(always)]
    pub const fn timer00c(&self) -> &Timer00c {
        &self.timer00c
    }
    #[doc = "0x10 - Counter Control and Interrupt Status Register"]
    #[inline(always)]
    pub const fn timer010(&self) -> &Timer010 {
        &self.timer010
    }
    #[doc = "0x1c - Conter Write Protection Register"]
    #[inline(always)]
    pub const fn timer01c(&self) -> &Timer01c {
        &self.timer01c
    }
}
#[doc = "TIMER000 (rw) register accessor: Counter Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timer000::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer000::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer000`] module"]
#[doc(alias = "TIMER000")]
pub type Timer000 = crate::Reg<timer000::Timer000Spec>;
#[doc = "Counter Status Register"]
pub mod timer000;
#[doc = "TIMER004 (rw) register accessor: Counter Reload Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timer004::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer004::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer004`] module"]
#[doc(alias = "TIMER004")]
pub type Timer004 = crate::Reg<timer004::Timer004Spec>;
#[doc = "Counter Reload Value Register"]
pub mod timer004;
#[doc = "TIMER008 (rw) register accessor: Counter First Matching Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timer008::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer008::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer008`] module"]
#[doc(alias = "TIMER008")]
pub type Timer008 = crate::Reg<timer008::Timer008Spec>;
#[doc = "Counter First Matching Register"]
pub mod timer008;
#[doc = "TIMER00C (rw) register accessor: Counter Second Matching Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timer00c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer00c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer00c`] module"]
#[doc(alias = "TIMER00C")]
pub type Timer00c = crate::Reg<timer00c::Timer00cSpec>;
#[doc = "Counter Second Matching Register"]
pub mod timer00c;
#[doc = "TIMER010 (rw) register accessor: Counter Control and Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timer010::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer010::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer010`] module"]
#[doc(alias = "TIMER010")]
pub type Timer010 = crate::Reg<timer010::Timer010Spec>;
#[doc = "Counter Control and Interrupt Status Register"]
pub mod timer010;
#[doc = "TIMER01C (rw) register accessor: Conter Write Protection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timer01c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer01c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer01c`] module"]
#[doc(alias = "TIMER01C")]
pub type Timer01c = crate::Reg<timer01c::Timer01cSpec>;
#[doc = "Conter Write Protection Register"]
pub mod timer01c;
