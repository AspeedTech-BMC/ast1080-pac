#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    rtc000: Rtc000,
    rtc004: Rtc004,
    rtc008: Rtc008,
    _reserved3: [u8; 0x04],
    rtc010: Rtc010,
    rtc014: Rtc014,
}
impl RegisterBlock {
    #[doc = "0x00 - Counter Status Register \\#1"]
    #[inline(always)]
    pub const fn rtc000(&self) -> &Rtc000 {
        &self.rtc000
    }
    #[doc = "0x04 - Counter Status Register \\#2"]
    #[inline(always)]
    pub const fn rtc004(&self) -> &Rtc004 {
        &self.rtc004
    }
    #[doc = "0x08 - Clock Alarm Register"]
    #[inline(always)]
    pub const fn rtc008(&self) -> &Rtc008 {
        &self.rtc008
    }
    #[doc = "0x10 - Control Register"]
    #[inline(always)]
    pub const fn rtc010(&self) -> &Rtc010 {
        &self.rtc010
    }
    #[doc = "0x14 - Alarm Status Register"]
    #[inline(always)]
    pub const fn rtc014(&self) -> &Rtc014 {
        &self.rtc014
    }
}
#[doc = "RTC000 (rw) register accessor: Counter Status Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc000::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc000::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc000`] module"]
#[doc(alias = "RTC000")]
pub type Rtc000 = crate::Reg<rtc000::Rtc000Spec>;
#[doc = "Counter Status Register \\#1"]
pub mod rtc000;
#[doc = "RTC004 (rw) register accessor: Counter Status Register \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc004::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc004::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc004`] module"]
#[doc(alias = "RTC004")]
pub type Rtc004 = crate::Reg<rtc004::Rtc004Spec>;
#[doc = "Counter Status Register \\#2"]
pub mod rtc004;
#[doc = "RTC008 (rw) register accessor: Clock Alarm Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc008::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc008::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc008`] module"]
#[doc(alias = "RTC008")]
pub type Rtc008 = crate::Reg<rtc008::Rtc008Spec>;
#[doc = "Clock Alarm Register"]
pub mod rtc008;
#[doc = "RTC010 (rw) register accessor: Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc010::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc010::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc010`] module"]
#[doc(alias = "RTC010")]
pub type Rtc010 = crate::Reg<rtc010::Rtc010Spec>;
#[doc = "Control Register"]
pub mod rtc010;
#[doc = "RTC014 (rw) register accessor: Alarm Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc014::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc014::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc014`] module"]
#[doc(alias = "RTC014")]
pub type Rtc014 = crate::Reg<rtc014::Rtc014Spec>;
#[doc = "Alarm Status Register"]
pub mod rtc014;
