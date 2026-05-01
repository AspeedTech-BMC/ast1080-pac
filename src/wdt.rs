#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    wdt000: Wdt000,
    wdt004: Wdt004,
    wdt008: Wdt008,
    wdt00c: Wdt00c,
    wdt010: Wdt010,
    wdt014: Wdt014,
    wdt018: Wdt018,
    _reserved7: [u8; 0x08],
    wdt024: Wdt024,
    wdt028: Wdt028,
    wdt02c: Wdt02c,
    wdt030: Wdt030,
    _reserved11: [u8; 0x08],
    wdt03c: Wdt03c,
    wdt040: Wdt040,
    wdt044: Wdt044,
    wdt048: Wdt048,
    wdt04c: Wdt04c,
    _reserved16: [u8; 0x08],
    wdt058: Wdt058,
    wdt05c: Wdt05c,
    wdt060: Wdt060,
    _reserved19: [u8; 0x08],
    wdt06c: Wdt06c,
    wdt070: Wdt070,
    wdt074: Wdt074,
    _reserved22: [u8; 0x04],
    wdt07c: Wdt07c,
}
impl RegisterBlock {
    #[doc = "0x00 - WDTn Counter Status Register"]
    #[inline(always)]
    pub const fn wdt000(&self) -> &Wdt000 {
        &self.wdt000
    }
    #[doc = "0x04 - WDTn Counter Reload Value Register"]
    #[inline(always)]
    pub const fn wdt004(&self) -> &Wdt004 {
        &self.wdt004
    }
    #[doc = "0x08 - WDTn Counter Restart Register"]
    #[inline(always)]
    pub const fn wdt008(&self) -> &Wdt008 {
        &self.wdt008
    }
    #[doc = "0x0c - WDTn Control Register"]
    #[inline(always)]
    pub const fn wdt00c(&self) -> &Wdt00c {
        &self.wdt00c
    }
    #[doc = "0x10 - WDTn Timeout Status Register"]
    #[inline(always)]
    pub const fn wdt010(&self) -> &Wdt010 {
        &self.wdt010
    }
    #[doc = "0x14 - WDTn Clear Timeout Status Register"]
    #[inline(always)]
    pub const fn wdt014(&self) -> &Wdt014 {
        &self.wdt014
    }
    #[doc = "0x18 - WDTn Reset Width Register"]
    #[inline(always)]
    pub const fn wdt018(&self) -> &Wdt018 {
        &self.wdt018
    }
    #[doc = "0x24 - WDTn Reset Mask Register \\#1"]
    #[inline(always)]
    pub const fn wdt024(&self) -> &Wdt024 {
        &self.wdt024
    }
    #[doc = "0x28 - WDTn Reset Mask Register \\#2"]
    #[inline(always)]
    pub const fn wdt028(&self) -> &Wdt028 {
        &self.wdt028
    }
    #[doc = "0x2c - WDTn Reset Mask Register \\#3"]
    #[inline(always)]
    pub const fn wdt02c(&self) -> &Wdt02c {
        &self.wdt02c
    }
    #[doc = "0x30 - WDTn Software Mode Reset Control Register"]
    #[inline(always)]
    pub const fn wdt030(&self) -> &Wdt030 {
        &self.wdt030
    }
    #[doc = "0x3c - WDTn Software Mode Reset Mask Register \\#1"]
    #[inline(always)]
    pub const fn wdt03c(&self) -> &Wdt03c {
        &self.wdt03c
    }
    #[doc = "0x40 - WDTn Software Mode Reset Mask Register \\#2"]
    #[inline(always)]
    pub const fn wdt040(&self) -> &Wdt040 {
        &self.wdt040
    }
    #[doc = "0x44 - WDTn Software Mode Reset Mask Register \\#3"]
    #[inline(always)]
    pub const fn wdt044(&self) -> &Wdt044 {
        &self.wdt044
    }
    #[doc = "0x48 - WDTn Funtion Disable Control Register"]
    #[inline(always)]
    pub const fn wdt048(&self) -> &Wdt048 {
        &self.wdt048
    }
    #[doc = "0x4c - WDTn Scratch Register"]
    #[inline(always)]
    pub const fn wdt04c(&self) -> &Wdt04c {
        &self.wdt04c
    }
    #[doc = "0x58 - WDTn Reset Mask Write Protection Register \\#1"]
    #[inline(always)]
    pub const fn wdt058(&self) -> &Wdt058 {
        &self.wdt058
    }
    #[doc = "0x5c - WDTn Reset Mask Write Protection Register \\#2"]
    #[inline(always)]
    pub const fn wdt05c(&self) -> &Wdt05c {
        &self.wdt05c
    }
    #[doc = "0x60 - WDTn Reset Mask Write Protection Register \\#3"]
    #[inline(always)]
    pub const fn wdt060(&self) -> &Wdt060 {
        &self.wdt060
    }
    #[doc = "0x6c - WDTn Reset Mask Write Protection Register \\#1"]
    #[inline(always)]
    pub const fn wdt06c(&self) -> &Wdt06c {
        &self.wdt06c
    }
    #[doc = "0x70 - WDTn Reset Mask Write Protection Register \\#2"]
    #[inline(always)]
    pub const fn wdt070(&self) -> &Wdt070 {
        &self.wdt070
    }
    #[doc = "0x74 - WDTn Reset Mask Write Protection Register \\#3"]
    #[inline(always)]
    pub const fn wdt074(&self) -> &Wdt074 {
        &self.wdt074
    }
    #[doc = "0x7c - WDTn Write Protection Register"]
    #[inline(always)]
    pub const fn wdt07c(&self) -> &Wdt07c {
        &self.wdt07c
    }
}
#[doc = "WDT000 (rw) register accessor: WDTn Counter Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdt000::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt000::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdt000`] module"]
#[doc(alias = "WDT000")]
pub type Wdt000 = crate::Reg<wdt000::Wdt000Spec>;
#[doc = "WDTn Counter Status Register"]
pub mod wdt000;
#[doc = "WDT004 (rw) register accessor: WDTn Counter Reload Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdt004::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt004::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdt004`] module"]
#[doc(alias = "WDT004")]
pub type Wdt004 = crate::Reg<wdt004::Wdt004Spec>;
#[doc = "WDTn Counter Reload Value Register"]
pub mod wdt004;
#[doc = "WDT008 (rw) register accessor: WDTn Counter Restart Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdt008::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt008::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdt008`] module"]
#[doc(alias = "WDT008")]
pub type Wdt008 = crate::Reg<wdt008::Wdt008Spec>;
#[doc = "WDTn Counter Restart Register"]
pub mod wdt008;
#[doc = "WDT00C (rw) register accessor: WDTn Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdt00c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt00c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdt00c`] module"]
#[doc(alias = "WDT00C")]
pub type Wdt00c = crate::Reg<wdt00c::Wdt00cSpec>;
#[doc = "WDTn Control Register"]
pub mod wdt00c;
#[doc = "WDT010 (rw) register accessor: WDTn Timeout Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdt010::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt010::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdt010`] module"]
#[doc(alias = "WDT010")]
pub type Wdt010 = crate::Reg<wdt010::Wdt010Spec>;
#[doc = "WDTn Timeout Status Register"]
pub mod wdt010;
#[doc = "WDT014 (rw) register accessor: WDTn Clear Timeout Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdt014::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt014::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdt014`] module"]
#[doc(alias = "WDT014")]
pub type Wdt014 = crate::Reg<wdt014::Wdt014Spec>;
#[doc = "WDTn Clear Timeout Status Register"]
pub mod wdt014;
#[doc = "WDT018 (rw) register accessor: WDTn Reset Width Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdt018::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt018::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdt018`] module"]
#[doc(alias = "WDT018")]
pub type Wdt018 = crate::Reg<wdt018::Wdt018Spec>;
#[doc = "WDTn Reset Width Register"]
pub mod wdt018;
#[doc = "WDT024 (rw) register accessor: WDTn Reset Mask Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`wdt024::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt024::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdt024`] module"]
#[doc(alias = "WDT024")]
pub type Wdt024 = crate::Reg<wdt024::Wdt024Spec>;
#[doc = "WDTn Reset Mask Register \\#1"]
pub mod wdt024;
#[doc = "WDT028 (rw) register accessor: WDTn Reset Mask Register \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`wdt028::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt028::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdt028`] module"]
#[doc(alias = "WDT028")]
pub type Wdt028 = crate::Reg<wdt028::Wdt028Spec>;
#[doc = "WDTn Reset Mask Register \\#2"]
pub mod wdt028;
#[doc = "WDT02C (rw) register accessor: WDTn Reset Mask Register \\#3\n\nYou can [`read`](crate::Reg::read) this register and get [`wdt02c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt02c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdt02c`] module"]
#[doc(alias = "WDT02C")]
pub type Wdt02c = crate::Reg<wdt02c::Wdt02cSpec>;
#[doc = "WDTn Reset Mask Register \\#3"]
pub mod wdt02c;
#[doc = "WDT030 (rw) register accessor: WDTn Software Mode Reset Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdt030::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt030::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdt030`] module"]
#[doc(alias = "WDT030")]
pub type Wdt030 = crate::Reg<wdt030::Wdt030Spec>;
#[doc = "WDTn Software Mode Reset Control Register"]
pub mod wdt030;
#[doc = "WDT03C (rw) register accessor: WDTn Software Mode Reset Mask Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`wdt03c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt03c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdt03c`] module"]
#[doc(alias = "WDT03C")]
pub type Wdt03c = crate::Reg<wdt03c::Wdt03cSpec>;
#[doc = "WDTn Software Mode Reset Mask Register \\#1"]
pub mod wdt03c;
#[doc = "WDT040 (rw) register accessor: WDTn Software Mode Reset Mask Register \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`wdt040::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt040::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdt040`] module"]
#[doc(alias = "WDT040")]
pub type Wdt040 = crate::Reg<wdt040::Wdt040Spec>;
#[doc = "WDTn Software Mode Reset Mask Register \\#2"]
pub mod wdt040;
#[doc = "WDT044 (rw) register accessor: WDTn Software Mode Reset Mask Register \\#3\n\nYou can [`read`](crate::Reg::read) this register and get [`wdt044::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt044::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdt044`] module"]
#[doc(alias = "WDT044")]
pub type Wdt044 = crate::Reg<wdt044::Wdt044Spec>;
#[doc = "WDTn Software Mode Reset Mask Register \\#3"]
pub mod wdt044;
#[doc = "WDT048 (rw) register accessor: WDTn Funtion Disable Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdt048::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt048::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdt048`] module"]
#[doc(alias = "WDT048")]
pub type Wdt048 = crate::Reg<wdt048::Wdt048Spec>;
#[doc = "WDTn Funtion Disable Control Register"]
pub mod wdt048;
#[doc = "WDT04C (rw) register accessor: WDTn Scratch Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdt04c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt04c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdt04c`] module"]
#[doc(alias = "WDT04C")]
pub type Wdt04c = crate::Reg<wdt04c::Wdt04cSpec>;
#[doc = "WDTn Scratch Register"]
pub mod wdt04c;
#[doc = "WDT058 (rw) register accessor: WDTn Reset Mask Write Protection Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`wdt058::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt058::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdt058`] module"]
#[doc(alias = "WDT058")]
pub type Wdt058 = crate::Reg<wdt058::Wdt058Spec>;
#[doc = "WDTn Reset Mask Write Protection Register \\#1"]
pub mod wdt058;
#[doc = "WDT05C (rw) register accessor: WDTn Reset Mask Write Protection Register \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`wdt05c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt05c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdt05c`] module"]
#[doc(alias = "WDT05C")]
pub type Wdt05c = crate::Reg<wdt05c::Wdt05cSpec>;
#[doc = "WDTn Reset Mask Write Protection Register \\#2"]
pub mod wdt05c;
#[doc = "WDT060 (rw) register accessor: WDTn Reset Mask Write Protection Register \\#3\n\nYou can [`read`](crate::Reg::read) this register and get [`wdt060::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt060::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdt060`] module"]
#[doc(alias = "WDT060")]
pub type Wdt060 = crate::Reg<wdt060::Wdt060Spec>;
#[doc = "WDTn Reset Mask Write Protection Register \\#3"]
pub mod wdt060;
#[doc = "WDT06C (rw) register accessor: WDTn Reset Mask Write Protection Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`wdt06c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt06c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdt06c`] module"]
#[doc(alias = "WDT06C")]
pub type Wdt06c = crate::Reg<wdt06c::Wdt06cSpec>;
#[doc = "WDTn Reset Mask Write Protection Register \\#1"]
pub mod wdt06c;
#[doc = "WDT070 (rw) register accessor: WDTn Reset Mask Write Protection Register \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`wdt070::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt070::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdt070`] module"]
#[doc(alias = "WDT070")]
pub type Wdt070 = crate::Reg<wdt070::Wdt070Spec>;
#[doc = "WDTn Reset Mask Write Protection Register \\#2"]
pub mod wdt070;
#[doc = "WDT074 (rw) register accessor: WDTn Reset Mask Write Protection Register \\#3\n\nYou can [`read`](crate::Reg::read) this register and get [`wdt074::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt074::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdt074`] module"]
#[doc(alias = "WDT074")]
pub type Wdt074 = crate::Reg<wdt074::Wdt074Spec>;
#[doc = "WDTn Reset Mask Write Protection Register \\#3"]
pub mod wdt074;
#[doc = "WDT07C (rw) register accessor: WDTn Write Protection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdt07c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt07c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdt07c`] module"]
#[doc(alias = "WDT07C")]
pub type Wdt07c = crate::Reg<wdt07c::Wdt07cSpec>;
#[doc = "WDTn Write Protection Register"]
pub mod wdt07c;
