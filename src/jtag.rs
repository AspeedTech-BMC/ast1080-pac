#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    jtag000: Jtag000,
    jtag004: Jtag004,
    jtag008: Jtag008,
    jtag00c: Jtag00c,
    jtag010: Jtag010,
    jtag014: Jtag014,
    jtag018: Jtag018,
    _reserved7: [u8; 0x04],
    jtag020: Jtag020,
    jtag024: Jtag024,
    jtag028: Jtag028,
    jtag02c: Jtag02c,
    jtag030: Jtag030,
    jtag034: Jtag034,
    jtag038: Jtag038,
    jtag03c: Jtag03c,
}
impl RegisterBlock {
    #[doc = "0x00 - Data Port register"]
    #[inline(always)]
    pub const fn jtag000(&self) -> &Jtag000 {
        &self.jtag000
    }
    #[doc = "0x04 - Data Port register"]
    #[inline(always)]
    pub const fn jtag004(&self) -> &Jtag004 {
        &self.jtag004
    }
    #[doc = "0x08 - Engine control"]
    #[inline(always)]
    pub const fn jtag008(&self) -> &Jtag008 {
        &self.jtag008
    }
    #[doc = "0x0c - Interrupt status and enable"]
    #[inline(always)]
    pub const fn jtag00c(&self) -> &Jtag00c {
        &self.jtag00c
    }
    #[doc = "0x10 - Software mode and status"]
    #[inline(always)]
    pub const fn jtag010(&self) -> &Jtag010 {
        &self.jtag010
    }
    #[doc = "0x14 - TCK Control"]
    #[inline(always)]
    pub const fn jtag014(&self) -> &Jtag014 {
        &self.jtag014
    }
    #[doc = "0x18 - Engine Control 1"]
    #[inline(always)]
    pub const fn jtag018(&self) -> &Jtag018 {
        &self.jtag018
    }
    #[doc = "0x20 - Data Port register"]
    #[inline(always)]
    pub const fn jtag020(&self) -> &Jtag020 {
        &self.jtag020
    }
    #[doc = "0x24 - Data Port register"]
    #[inline(always)]
    pub const fn jtag024(&self) -> &Jtag024 {
        &self.jtag024
    }
    #[doc = "0x28 - Padding control 0"]
    #[inline(always)]
    pub const fn jtag028(&self) -> &Jtag028 {
        &self.jtag028
    }
    #[doc = "0x2c - Padding control 1"]
    #[inline(always)]
    pub const fn jtag02c(&self) -> &Jtag02c {
        &self.jtag02c
    }
    #[doc = "0x30 - Shift control"]
    #[inline(always)]
    pub const fn jtag030(&self) -> &Jtag030 {
        &self.jtag030
    }
    #[doc = "0x34 - Global control"]
    #[inline(always)]
    pub const fn jtag034(&self) -> &Jtag034 {
        &self.jtag034
    }
    #[doc = "0x38 - Interrupt control"]
    #[inline(always)]
    pub const fn jtag038(&self) -> &Jtag038 {
        &self.jtag038
    }
    #[doc = "0x3c - Status"]
    #[inline(always)]
    pub const fn jtag03c(&self) -> &Jtag03c {
        &self.jtag03c
    }
}
#[doc = "JTAG000 (rw) register accessor: Data Port register\n\nYou can [`read`](crate::Reg::read) this register and get [`jtag000::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jtag000::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jtag000`] module"]
#[doc(alias = "JTAG000")]
pub type Jtag000 = crate::Reg<jtag000::Jtag000Spec>;
#[doc = "Data Port register"]
pub mod jtag000;
#[doc = "JTAG004 (rw) register accessor: Data Port register\n\nYou can [`read`](crate::Reg::read) this register and get [`jtag004::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jtag004::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jtag004`] module"]
#[doc(alias = "JTAG004")]
pub type Jtag004 = crate::Reg<jtag004::Jtag004Spec>;
#[doc = "Data Port register"]
pub mod jtag004;
#[doc = "JTAG008 (rw) register accessor: Engine control\n\nYou can [`read`](crate::Reg::read) this register and get [`jtag008::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jtag008::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jtag008`] module"]
#[doc(alias = "JTAG008")]
pub type Jtag008 = crate::Reg<jtag008::Jtag008Spec>;
#[doc = "Engine control"]
pub mod jtag008;
#[doc = "JTAG00C (rw) register accessor: Interrupt status and enable\n\nYou can [`read`](crate::Reg::read) this register and get [`jtag00c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jtag00c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jtag00c`] module"]
#[doc(alias = "JTAG00C")]
pub type Jtag00c = crate::Reg<jtag00c::Jtag00cSpec>;
#[doc = "Interrupt status and enable"]
pub mod jtag00c;
#[doc = "JTAG010 (rw) register accessor: Software mode and status\n\nYou can [`read`](crate::Reg::read) this register and get [`jtag010::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jtag010::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jtag010`] module"]
#[doc(alias = "JTAG010")]
pub type Jtag010 = crate::Reg<jtag010::Jtag010Spec>;
#[doc = "Software mode and status"]
pub mod jtag010;
#[doc = "JTAG014 (rw) register accessor: TCK Control\n\nYou can [`read`](crate::Reg::read) this register and get [`jtag014::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jtag014::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jtag014`] module"]
#[doc(alias = "JTAG014")]
pub type Jtag014 = crate::Reg<jtag014::Jtag014Spec>;
#[doc = "TCK Control"]
pub mod jtag014;
#[doc = "JTAG018 (rw) register accessor: Engine Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`jtag018::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jtag018::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jtag018`] module"]
#[doc(alias = "JTAG018")]
pub type Jtag018 = crate::Reg<jtag018::Jtag018Spec>;
#[doc = "Engine Control 1"]
pub mod jtag018;
#[doc = "JTAG020 (rw) register accessor: Data Port register\n\nYou can [`read`](crate::Reg::read) this register and get [`jtag020::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jtag020::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jtag020`] module"]
#[doc(alias = "JTAG020")]
pub type Jtag020 = crate::Reg<jtag020::Jtag020Spec>;
#[doc = "Data Port register"]
pub mod jtag020;
#[doc = "JTAG024 (rw) register accessor: Data Port register\n\nYou can [`read`](crate::Reg::read) this register and get [`jtag024::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jtag024::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jtag024`] module"]
#[doc(alias = "JTAG024")]
pub type Jtag024 = crate::Reg<jtag024::Jtag024Spec>;
#[doc = "Data Port register"]
pub mod jtag024;
#[doc = "JTAG028 (rw) register accessor: Padding control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`jtag028::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jtag028::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jtag028`] module"]
#[doc(alias = "JTAG028")]
pub type Jtag028 = crate::Reg<jtag028::Jtag028Spec>;
#[doc = "Padding control 0"]
pub mod jtag028;
#[doc = "JTAG02C (rw) register accessor: Padding control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`jtag02c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jtag02c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jtag02c`] module"]
#[doc(alias = "JTAG02C")]
pub type Jtag02c = crate::Reg<jtag02c::Jtag02cSpec>;
#[doc = "Padding control 1"]
pub mod jtag02c;
#[doc = "JTAG030 (rw) register accessor: Shift control\n\nYou can [`read`](crate::Reg::read) this register and get [`jtag030::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jtag030::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jtag030`] module"]
#[doc(alias = "JTAG030")]
pub type Jtag030 = crate::Reg<jtag030::Jtag030Spec>;
#[doc = "Shift control"]
pub mod jtag030;
#[doc = "JTAG034 (rw) register accessor: Global control\n\nYou can [`read`](crate::Reg::read) this register and get [`jtag034::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jtag034::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jtag034`] module"]
#[doc(alias = "JTAG034")]
pub type Jtag034 = crate::Reg<jtag034::Jtag034Spec>;
#[doc = "Global control"]
pub mod jtag034;
#[doc = "JTAG038 (rw) register accessor: Interrupt control\n\nYou can [`read`](crate::Reg::read) this register and get [`jtag038::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jtag038::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jtag038`] module"]
#[doc(alias = "JTAG038")]
pub type Jtag038 = crate::Reg<jtag038::Jtag038Spec>;
#[doc = "Interrupt control"]
pub mod jtag038;
#[doc = "JTAG03C (rw) register accessor: Status\n\nYou can [`read`](crate::Reg::read) this register and get [`jtag03c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jtag03c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jtag03c`] module"]
#[doc(alias = "JTAG03C")]
pub type Jtag03c = crate::Reg<jtag03c::Jtag03cSpec>;
#[doc = "Status"]
pub mod jtag03c;
