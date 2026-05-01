#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    vic010: Vic010,
    vic014: Vic014,
    vic018: Vic018,
    _reserved3: [u8; 0x04],
    vic020: Vic020,
    vic024: Vic024,
    vic028: Vic028,
    _reserved6: [u8; 0x04],
    vic030: Vic030,
    vic034: Vic034,
    vic038: Vic038,
    _reserved9: [u8; 0x04],
    vic040: Vic040,
    vic044: Vic044,
    _reserved11: [u8; 0x18],
    vic060: Vic060,
    vic064: Vic064,
    _reserved13: [u8; 0x18],
    vic080: Vic080,
    vic084: Vic084,
    vic088: Vic088,
    vic08c: Vic08c,
    vic090: Vic090,
    vic094: Vic094,
    _reserved19: [u8; 0x0168],
    vic200: Vic200,
    vic204: Vic204,
    vic208: Vic208,
    vic20c: Vic20c,
    vic210: Vic210,
    vic214: Vic214,
    vic218: Vic218,
    _reserved26: [u8; 0xe4],
    vic300: Vic300,
    vic304: Vic304,
    vic308: Vic308,
    vic30c: Vic30c,
    vic310: Vic310,
    vic314: Vic314,
    vic318: Vic318,
    _reserved33: [u8; 0xe4],
    vic400: Vic400,
    vic404: Vic404,
    vic408: Vic408,
    vic40c: Vic40c,
    vic410: Vic410,
    vic414: Vic414,
    vic418: Vic418,
    _reserved40: [u8; 0xe4],
    vic500: Vic500,
    _reserved41: [u8; 0x02fc],
    vic800: Vic800,
    vic804: Vic804,
    vic808: Vic808,
    vic80c: Vic80c,
    vic810: Vic810,
    vic814: Vic814,
    vic818: Vic818,
    vic81c: Vic81c,
    _reserved49: [u8; 0x60],
    vic880: Vic880,
    vic884: Vic884,
    vic888: Vic888,
    vic88c: Vic88c,
    vic890: Vic890,
    vic894: Vic894,
    vic898: Vic898,
    vic89c: Vic89c,
}
impl RegisterBlock {
    #[doc = "0x10 - PSP Software Interrupt Enable"]
    #[inline(always)]
    pub const fn vic010(&self) -> &Vic010 {
        &self.vic010
    }
    #[doc = "0x14 - PSP Software Interrupt Status"]
    #[inline(always)]
    pub const fn vic014(&self) -> &Vic014 {
        &self.vic014
    }
    #[doc = "0x18 - PSP Software Interrupt Set"]
    #[inline(always)]
    pub const fn vic018(&self) -> &Vic018 {
        &self.vic018
    }
    #[doc = "0x20 - Caliptra SSMCU Software Interrupt Enable"]
    #[inline(always)]
    pub const fn vic020(&self) -> &Vic020 {
        &self.vic020
    }
    #[doc = "0x24 - Caliptra SSMCU Software Interrupt Status"]
    #[inline(always)]
    pub const fn vic024(&self) -> &Vic024 {
        &self.vic024
    }
    #[doc = "0x28 - Caliptra SSMCU Software Interrupt Set"]
    #[inline(always)]
    pub const fn vic028(&self) -> &Vic028 {
        &self.vic028
    }
    #[doc = "0x30 - MCU Software Interrupt Enable"]
    #[inline(always)]
    pub const fn vic030(&self) -> &Vic030 {
        &self.vic030
    }
    #[doc = "0x34 - MCU Software Interrupt Status"]
    #[inline(always)]
    pub const fn vic034(&self) -> &Vic034 {
        &self.vic034
    }
    #[doc = "0x38 - MCU Software Interrupt Set"]
    #[inline(always)]
    pub const fn vic038(&self) -> &Vic038 {
        &self.vic038
    }
    #[doc = "0x40 - Register Protection"]
    #[inline(always)]
    pub const fn vic040(&self) -> &Vic040 {
        &self.vic040
    }
    #[doc = "0x44 - External Master ID"]
    #[inline(always)]
    pub const fn vic044(&self) -> &Vic044 {
        &self.vic044
    }
    #[doc = "0x60 - HeartBeat Control"]
    #[inline(always)]
    pub const fn vic060(&self) -> &Vic060 {
        &self.vic060
    }
    #[doc = "0x64 - HeartBeat Output Enable"]
    #[inline(always)]
    pub const fn vic064(&self) -> &Vic064 {
        &self.vic064
    }
    #[doc = "0x80 - MCU Interrupt Raw 0"]
    #[inline(always)]
    pub const fn vic080(&self) -> &Vic080 {
        &self.vic080
    }
    #[doc = "0x84 - MCU Interrupt Raw 1"]
    #[inline(always)]
    pub const fn vic084(&self) -> &Vic084 {
        &self.vic084
    }
    #[doc = "0x88 - MCU Interrupt Raw 2"]
    #[inline(always)]
    pub const fn vic088(&self) -> &Vic088 {
        &self.vic088
    }
    #[doc = "0x8c - MCU Interrupt Raw 3"]
    #[inline(always)]
    pub const fn vic08c(&self) -> &Vic08c {
        &self.vic08c
    }
    #[doc = "0x90 - MCU Interrupt Raw 4"]
    #[inline(always)]
    pub const fn vic090(&self) -> &Vic090 {
        &self.vic090
    }
    #[doc = "0x94 - MCU Interrupt Raw 5"]
    #[inline(always)]
    pub const fn vic094(&self) -> &Vic094 {
        &self.vic094
    }
    #[doc = "0x200 - Int Routing Select 0"]
    #[inline(always)]
    pub const fn vic200(&self) -> &Vic200 {
        &self.vic200
    }
    #[doc = "0x204 - Int Routing Select 1"]
    #[inline(always)]
    pub const fn vic204(&self) -> &Vic204 {
        &self.vic204
    }
    #[doc = "0x208 - Int Routing Select 2"]
    #[inline(always)]
    pub const fn vic208(&self) -> &Vic208 {
        &self.vic208
    }
    #[doc = "0x20c - Int Routing Select 3"]
    #[inline(always)]
    pub const fn vic20c(&self) -> &Vic20c {
        &self.vic20c
    }
    #[doc = "0x210 - Int Routing Select 4"]
    #[inline(always)]
    pub const fn vic210(&self) -> &Vic210 {
        &self.vic210
    }
    #[doc = "0x214 - Int Routing Select 5"]
    #[inline(always)]
    pub const fn vic214(&self) -> &Vic214 {
        &self.vic214
    }
    #[doc = "0x218 - Int Routing Select 6"]
    #[inline(always)]
    pub const fn vic218(&self) -> &Vic218 {
        &self.vic218
    }
    #[doc = "0x300 - Int Routing Select2 0"]
    #[inline(always)]
    pub const fn vic300(&self) -> &Vic300 {
        &self.vic300
    }
    #[doc = "0x304 - Int Routing Select2 1"]
    #[inline(always)]
    pub const fn vic304(&self) -> &Vic304 {
        &self.vic304
    }
    #[doc = "0x308 - Int Routing Select2 2"]
    #[inline(always)]
    pub const fn vic308(&self) -> &Vic308 {
        &self.vic308
    }
    #[doc = "0x30c - Int Routing Select2 3"]
    #[inline(always)]
    pub const fn vic30c(&self) -> &Vic30c {
        &self.vic30c
    }
    #[doc = "0x310 - Int Routing Select2 4"]
    #[inline(always)]
    pub const fn vic310(&self) -> &Vic310 {
        &self.vic310
    }
    #[doc = "0x314 - Int Routing Select2 5"]
    #[inline(always)]
    pub const fn vic314(&self) -> &Vic314 {
        &self.vic314
    }
    #[doc = "0x318 - Int Routing Select2 6"]
    #[inline(always)]
    pub const fn vic318(&self) -> &Vic318 {
        &self.vic318
    }
    #[doc = "0x400 - Int Routing Select3 0"]
    #[inline(always)]
    pub const fn vic400(&self) -> &Vic400 {
        &self.vic400
    }
    #[doc = "0x404 - Int Routing Select3 1"]
    #[inline(always)]
    pub const fn vic404(&self) -> &Vic404 {
        &self.vic404
    }
    #[doc = "0x408 - Int Routing Select3 2"]
    #[inline(always)]
    pub const fn vic408(&self) -> &Vic408 {
        &self.vic408
    }
    #[doc = "0x40c - Int Routing Select3 3"]
    #[inline(always)]
    pub const fn vic40c(&self) -> &Vic40c {
        &self.vic40c
    }
    #[doc = "0x410 - Int Routing Select3 4"]
    #[inline(always)]
    pub const fn vic410(&self) -> &Vic410 {
        &self.vic410
    }
    #[doc = "0x414 - Int Routing Select3 5"]
    #[inline(always)]
    pub const fn vic414(&self) -> &Vic414 {
        &self.vic414
    }
    #[doc = "0x418 - Int Routing Select3 6"]
    #[inline(always)]
    pub const fn vic418(&self) -> &Vic418 {
        &self.vic418
    }
    #[doc = "0x500 - Reset INTC Select"]
    #[inline(always)]
    pub const fn vic500(&self) -> &Vic500 {
        &self.vic500
    }
    #[doc = "0x800 - MCU Interrupt Event 0"]
    #[inline(always)]
    pub const fn vic800(&self) -> &Vic800 {
        &self.vic800
    }
    #[doc = "0x804 - MCU Interrupt Event 1"]
    #[inline(always)]
    pub const fn vic804(&self) -> &Vic804 {
        &self.vic804
    }
    #[doc = "0x808 - MCU Interrupt Event 2"]
    #[inline(always)]
    pub const fn vic808(&self) -> &Vic808 {
        &self.vic808
    }
    #[doc = "0x80c - MCU Interrupt Event 3"]
    #[inline(always)]
    pub const fn vic80c(&self) -> &Vic80c {
        &self.vic80c
    }
    #[doc = "0x810 - MCU Interrupt Event 4"]
    #[inline(always)]
    pub const fn vic810(&self) -> &Vic810 {
        &self.vic810
    }
    #[doc = "0x814 - MCU Interrupt Event 5"]
    #[inline(always)]
    pub const fn vic814(&self) -> &Vic814 {
        &self.vic814
    }
    #[doc = "0x818 - MCU Interrupt Event 6"]
    #[inline(always)]
    pub const fn vic818(&self) -> &Vic818 {
        &self.vic818
    }
    #[doc = "0x81c - MCU Interrupt Event 7"]
    #[inline(always)]
    pub const fn vic81c(&self) -> &Vic81c {
        &self.vic81c
    }
    #[doc = "0x880 - MCU Interrupt Enable 0"]
    #[inline(always)]
    pub const fn vic880(&self) -> &Vic880 {
        &self.vic880
    }
    #[doc = "0x884 - MCU Interrupt Enable 1"]
    #[inline(always)]
    pub const fn vic884(&self) -> &Vic884 {
        &self.vic884
    }
    #[doc = "0x888 - MCU Interrupt Enable 2"]
    #[inline(always)]
    pub const fn vic888(&self) -> &Vic888 {
        &self.vic888
    }
    #[doc = "0x88c - MCU Interrupt Enable 3"]
    #[inline(always)]
    pub const fn vic88c(&self) -> &Vic88c {
        &self.vic88c
    }
    #[doc = "0x890 - MCU Interrupt Enable 4"]
    #[inline(always)]
    pub const fn vic890(&self) -> &Vic890 {
        &self.vic890
    }
    #[doc = "0x894 - MCU Interrupt Enable 5"]
    #[inline(always)]
    pub const fn vic894(&self) -> &Vic894 {
        &self.vic894
    }
    #[doc = "0x898 - MCU Interrupt Enable 6"]
    #[inline(always)]
    pub const fn vic898(&self) -> &Vic898 {
        &self.vic898
    }
    #[doc = "0x89c - MCU Interrupt Enable 7"]
    #[inline(always)]
    pub const fn vic89c(&self) -> &Vic89c {
        &self.vic89c
    }
}
#[doc = "VIC010 (rw) register accessor: PSP Software Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`vic010::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic010::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vic010`] module"]
#[doc(alias = "VIC010")]
pub type Vic010 = crate::Reg<vic010::Vic010Spec>;
#[doc = "PSP Software Interrupt Enable"]
pub mod vic010;
#[doc = "VIC014 (rw) register accessor: PSP Software Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`vic014::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic014::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vic014`] module"]
#[doc(alias = "VIC014")]
pub type Vic014 = crate::Reg<vic014::Vic014Spec>;
#[doc = "PSP Software Interrupt Status"]
pub mod vic014;
#[doc = "VIC018 (rw) register accessor: PSP Software Interrupt Set\n\nYou can [`read`](crate::Reg::read) this register and get [`vic018::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic018::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vic018`] module"]
#[doc(alias = "VIC018")]
pub type Vic018 = crate::Reg<vic018::Vic018Spec>;
#[doc = "PSP Software Interrupt Set"]
pub mod vic018;
#[doc = "VIC020 (rw) register accessor: Caliptra SSMCU Software Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`vic020::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic020::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vic020`] module"]
#[doc(alias = "VIC020")]
pub type Vic020 = crate::Reg<vic020::Vic020Spec>;
#[doc = "Caliptra SSMCU Software Interrupt Enable"]
pub mod vic020;
#[doc = "VIC024 (rw) register accessor: Caliptra SSMCU Software Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`vic024::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic024::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vic024`] module"]
#[doc(alias = "VIC024")]
pub type Vic024 = crate::Reg<vic024::Vic024Spec>;
#[doc = "Caliptra SSMCU Software Interrupt Status"]
pub mod vic024;
#[doc = "VIC028 (rw) register accessor: Caliptra SSMCU Software Interrupt Set\n\nYou can [`read`](crate::Reg::read) this register and get [`vic028::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic028::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vic028`] module"]
#[doc(alias = "VIC028")]
pub type Vic028 = crate::Reg<vic028::Vic028Spec>;
#[doc = "Caliptra SSMCU Software Interrupt Set"]
pub mod vic028;
#[doc = "VIC030 (rw) register accessor: MCU Software Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`vic030::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic030::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vic030`] module"]
#[doc(alias = "VIC030")]
pub type Vic030 = crate::Reg<vic030::Vic030Spec>;
#[doc = "MCU Software Interrupt Enable"]
pub mod vic030;
#[doc = "VIC034 (rw) register accessor: MCU Software Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`vic034::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic034::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vic034`] module"]
#[doc(alias = "VIC034")]
pub type Vic034 = crate::Reg<vic034::Vic034Spec>;
#[doc = "MCU Software Interrupt Status"]
pub mod vic034;
#[doc = "VIC038 (rw) register accessor: MCU Software Interrupt Set\n\nYou can [`read`](crate::Reg::read) this register and get [`vic038::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic038::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vic038`] module"]
#[doc(alias = "VIC038")]
pub type Vic038 = crate::Reg<vic038::Vic038Spec>;
#[doc = "MCU Software Interrupt Set"]
pub mod vic038;
#[doc = "VIC040 (rw) register accessor: Register Protection\n\nYou can [`read`](crate::Reg::read) this register and get [`vic040::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic040::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vic040`] module"]
#[doc(alias = "VIC040")]
pub type Vic040 = crate::Reg<vic040::Vic040Spec>;
#[doc = "Register Protection"]
pub mod vic040;
#[doc = "VIC044 (rw) register accessor: External Master ID\n\nYou can [`read`](crate::Reg::read) this register and get [`vic044::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic044::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vic044`] module"]
#[doc(alias = "VIC044")]
pub type Vic044 = crate::Reg<vic044::Vic044Spec>;
#[doc = "External Master ID"]
pub mod vic044;
#[doc = "VIC060 (rw) register accessor: HeartBeat Control\n\nYou can [`read`](crate::Reg::read) this register and get [`vic060::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic060::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vic060`] module"]
#[doc(alias = "VIC060")]
pub type Vic060 = crate::Reg<vic060::Vic060Spec>;
#[doc = "HeartBeat Control"]
pub mod vic060;
#[doc = "VIC064 (rw) register accessor: HeartBeat Output Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`vic064::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic064::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vic064`] module"]
#[doc(alias = "VIC064")]
pub type Vic064 = crate::Reg<vic064::Vic064Spec>;
#[doc = "HeartBeat Output Enable"]
pub mod vic064;
#[doc = "VIC080 (rw) register accessor: MCU Interrupt Raw 0\n\nYou can [`read`](crate::Reg::read) this register and get [`vic080::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic080::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vic080`] module"]
#[doc(alias = "VIC080")]
pub type Vic080 = crate::Reg<vic080::Vic080Spec>;
#[doc = "MCU Interrupt Raw 0"]
pub mod vic080;
#[doc = "VIC084 (rw) register accessor: MCU Interrupt Raw 1\n\nYou can [`read`](crate::Reg::read) this register and get [`vic084::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic084::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vic084`] module"]
#[doc(alias = "VIC084")]
pub type Vic084 = crate::Reg<vic084::Vic084Spec>;
#[doc = "MCU Interrupt Raw 1"]
pub mod vic084;
#[doc = "VIC088 (rw) register accessor: MCU Interrupt Raw 2\n\nYou can [`read`](crate::Reg::read) this register and get [`vic088::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic088::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vic088`] module"]
#[doc(alias = "VIC088")]
pub type Vic088 = crate::Reg<vic088::Vic088Spec>;
#[doc = "MCU Interrupt Raw 2"]
pub mod vic088;
#[doc = "VIC08C (rw) register accessor: MCU Interrupt Raw 3\n\nYou can [`read`](crate::Reg::read) this register and get [`vic08c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic08c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vic08c`] module"]
#[doc(alias = "VIC08C")]
pub type Vic08c = crate::Reg<vic08c::Vic08cSpec>;
#[doc = "MCU Interrupt Raw 3"]
pub mod vic08c;
#[doc = "VIC090 (rw) register accessor: MCU Interrupt Raw 4\n\nYou can [`read`](crate::Reg::read) this register and get [`vic090::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic090::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vic090`] module"]
#[doc(alias = "VIC090")]
pub type Vic090 = crate::Reg<vic090::Vic090Spec>;
#[doc = "MCU Interrupt Raw 4"]
pub mod vic090;
#[doc = "VIC094 (rw) register accessor: MCU Interrupt Raw 5\n\nYou can [`read`](crate::Reg::read) this register and get [`vic094::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic094::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vic094`] module"]
#[doc(alias = "VIC094")]
pub type Vic094 = crate::Reg<vic094::Vic094Spec>;
#[doc = "MCU Interrupt Raw 5"]
pub mod vic094;
#[doc = "VIC200 (rw) register accessor: Int Routing Select 0\n\nYou can [`read`](crate::Reg::read) this register and get [`vic200::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic200::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vic200`] module"]
#[doc(alias = "VIC200")]
pub type Vic200 = crate::Reg<vic200::Vic200Spec>;
#[doc = "Int Routing Select 0"]
pub mod vic200;
#[doc = "VIC204 (rw) register accessor: Int Routing Select 1\n\nYou can [`read`](crate::Reg::read) this register and get [`vic204::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic204::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vic204`] module"]
#[doc(alias = "VIC204")]
pub type Vic204 = crate::Reg<vic204::Vic204Spec>;
#[doc = "Int Routing Select 1"]
pub mod vic204;
#[doc = "VIC208 (rw) register accessor: Int Routing Select 2\n\nYou can [`read`](crate::Reg::read) this register and get [`vic208::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic208::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vic208`] module"]
#[doc(alias = "VIC208")]
pub type Vic208 = crate::Reg<vic208::Vic208Spec>;
#[doc = "Int Routing Select 2"]
pub mod vic208;
#[doc = "VIC20C (rw) register accessor: Int Routing Select 3\n\nYou can [`read`](crate::Reg::read) this register and get [`vic20c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic20c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vic20c`] module"]
#[doc(alias = "VIC20C")]
pub type Vic20c = crate::Reg<vic20c::Vic20cSpec>;
#[doc = "Int Routing Select 3"]
pub mod vic20c;
#[doc = "VIC210 (rw) register accessor: Int Routing Select 4\n\nYou can [`read`](crate::Reg::read) this register and get [`vic210::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic210::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vic210`] module"]
#[doc(alias = "VIC210")]
pub type Vic210 = crate::Reg<vic210::Vic210Spec>;
#[doc = "Int Routing Select 4"]
pub mod vic210;
#[doc = "VIC214 (rw) register accessor: Int Routing Select 5\n\nYou can [`read`](crate::Reg::read) this register and get [`vic214::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic214::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vic214`] module"]
#[doc(alias = "VIC214")]
pub type Vic214 = crate::Reg<vic214::Vic214Spec>;
#[doc = "Int Routing Select 5"]
pub mod vic214;
#[doc = "VIC218 (rw) register accessor: Int Routing Select 6\n\nYou can [`read`](crate::Reg::read) this register and get [`vic218::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic218::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vic218`] module"]
#[doc(alias = "VIC218")]
pub type Vic218 = crate::Reg<vic218::Vic218Spec>;
#[doc = "Int Routing Select 6"]
pub mod vic218;
#[doc = "VIC300 (rw) register accessor: Int Routing Select2 0\n\nYou can [`read`](crate::Reg::read) this register and get [`vic300::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic300::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vic300`] module"]
#[doc(alias = "VIC300")]
pub type Vic300 = crate::Reg<vic300::Vic300Spec>;
#[doc = "Int Routing Select2 0"]
pub mod vic300;
#[doc = "VIC304 (rw) register accessor: Int Routing Select2 1\n\nYou can [`read`](crate::Reg::read) this register and get [`vic304::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic304::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vic304`] module"]
#[doc(alias = "VIC304")]
pub type Vic304 = crate::Reg<vic304::Vic304Spec>;
#[doc = "Int Routing Select2 1"]
pub mod vic304;
#[doc = "VIC308 (rw) register accessor: Int Routing Select2 2\n\nYou can [`read`](crate::Reg::read) this register and get [`vic308::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic308::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vic308`] module"]
#[doc(alias = "VIC308")]
pub type Vic308 = crate::Reg<vic308::Vic308Spec>;
#[doc = "Int Routing Select2 2"]
pub mod vic308;
#[doc = "VIC30C (rw) register accessor: Int Routing Select2 3\n\nYou can [`read`](crate::Reg::read) this register and get [`vic30c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic30c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vic30c`] module"]
#[doc(alias = "VIC30C")]
pub type Vic30c = crate::Reg<vic30c::Vic30cSpec>;
#[doc = "Int Routing Select2 3"]
pub mod vic30c;
#[doc = "VIC310 (rw) register accessor: Int Routing Select2 4\n\nYou can [`read`](crate::Reg::read) this register and get [`vic310::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic310::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vic310`] module"]
#[doc(alias = "VIC310")]
pub type Vic310 = crate::Reg<vic310::Vic310Spec>;
#[doc = "Int Routing Select2 4"]
pub mod vic310;
#[doc = "VIC314 (rw) register accessor: Int Routing Select2 5\n\nYou can [`read`](crate::Reg::read) this register and get [`vic314::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic314::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vic314`] module"]
#[doc(alias = "VIC314")]
pub type Vic314 = crate::Reg<vic314::Vic314Spec>;
#[doc = "Int Routing Select2 5"]
pub mod vic314;
#[doc = "VIC318 (rw) register accessor: Int Routing Select2 6\n\nYou can [`read`](crate::Reg::read) this register and get [`vic318::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic318::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vic318`] module"]
#[doc(alias = "VIC318")]
pub type Vic318 = crate::Reg<vic318::Vic318Spec>;
#[doc = "Int Routing Select2 6"]
pub mod vic318;
#[doc = "VIC400 (rw) register accessor: Int Routing Select3 0\n\nYou can [`read`](crate::Reg::read) this register and get [`vic400::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic400::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vic400`] module"]
#[doc(alias = "VIC400")]
pub type Vic400 = crate::Reg<vic400::Vic400Spec>;
#[doc = "Int Routing Select3 0"]
pub mod vic400;
#[doc = "VIC404 (rw) register accessor: Int Routing Select3 1\n\nYou can [`read`](crate::Reg::read) this register and get [`vic404::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic404::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vic404`] module"]
#[doc(alias = "VIC404")]
pub type Vic404 = crate::Reg<vic404::Vic404Spec>;
#[doc = "Int Routing Select3 1"]
pub mod vic404;
#[doc = "VIC408 (rw) register accessor: Int Routing Select3 2\n\nYou can [`read`](crate::Reg::read) this register and get [`vic408::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic408::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vic408`] module"]
#[doc(alias = "VIC408")]
pub type Vic408 = crate::Reg<vic408::Vic408Spec>;
#[doc = "Int Routing Select3 2"]
pub mod vic408;
#[doc = "VIC40C (rw) register accessor: Int Routing Select3 3\n\nYou can [`read`](crate::Reg::read) this register and get [`vic40c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic40c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vic40c`] module"]
#[doc(alias = "VIC40C")]
pub type Vic40c = crate::Reg<vic40c::Vic40cSpec>;
#[doc = "Int Routing Select3 3"]
pub mod vic40c;
#[doc = "VIC410 (rw) register accessor: Int Routing Select3 4\n\nYou can [`read`](crate::Reg::read) this register and get [`vic410::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic410::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vic410`] module"]
#[doc(alias = "VIC410")]
pub type Vic410 = crate::Reg<vic410::Vic410Spec>;
#[doc = "Int Routing Select3 4"]
pub mod vic410;
#[doc = "VIC414 (rw) register accessor: Int Routing Select3 5\n\nYou can [`read`](crate::Reg::read) this register and get [`vic414::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic414::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vic414`] module"]
#[doc(alias = "VIC414")]
pub type Vic414 = crate::Reg<vic414::Vic414Spec>;
#[doc = "Int Routing Select3 5"]
pub mod vic414;
#[doc = "VIC418 (rw) register accessor: Int Routing Select3 6\n\nYou can [`read`](crate::Reg::read) this register and get [`vic418::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic418::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vic418`] module"]
#[doc(alias = "VIC418")]
pub type Vic418 = crate::Reg<vic418::Vic418Spec>;
#[doc = "Int Routing Select3 6"]
pub mod vic418;
#[doc = "VIC500 (rw) register accessor: Reset INTC Select\n\nYou can [`read`](crate::Reg::read) this register and get [`vic500::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic500::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vic500`] module"]
#[doc(alias = "VIC500")]
pub type Vic500 = crate::Reg<vic500::Vic500Spec>;
#[doc = "Reset INTC Select"]
pub mod vic500;
#[doc = "VIC800 (rw) register accessor: MCU Interrupt Event 0\n\nYou can [`read`](crate::Reg::read) this register and get [`vic800::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic800::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vic800`] module"]
#[doc(alias = "VIC800")]
pub type Vic800 = crate::Reg<vic800::Vic800Spec>;
#[doc = "MCU Interrupt Event 0"]
pub mod vic800;
#[doc = "VIC804 (rw) register accessor: MCU Interrupt Event 1\n\nYou can [`read`](crate::Reg::read) this register and get [`vic804::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic804::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vic804`] module"]
#[doc(alias = "VIC804")]
pub type Vic804 = crate::Reg<vic804::Vic804Spec>;
#[doc = "MCU Interrupt Event 1"]
pub mod vic804;
#[doc = "VIC808 (rw) register accessor: MCU Interrupt Event 2\n\nYou can [`read`](crate::Reg::read) this register and get [`vic808::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic808::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vic808`] module"]
#[doc(alias = "VIC808")]
pub type Vic808 = crate::Reg<vic808::Vic808Spec>;
#[doc = "MCU Interrupt Event 2"]
pub mod vic808;
#[doc = "VIC80C (rw) register accessor: MCU Interrupt Event 3\n\nYou can [`read`](crate::Reg::read) this register and get [`vic80c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic80c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vic80c`] module"]
#[doc(alias = "VIC80C")]
pub type Vic80c = crate::Reg<vic80c::Vic80cSpec>;
#[doc = "MCU Interrupt Event 3"]
pub mod vic80c;
#[doc = "VIC810 (rw) register accessor: MCU Interrupt Event 4\n\nYou can [`read`](crate::Reg::read) this register and get [`vic810::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic810::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vic810`] module"]
#[doc(alias = "VIC810")]
pub type Vic810 = crate::Reg<vic810::Vic810Spec>;
#[doc = "MCU Interrupt Event 4"]
pub mod vic810;
#[doc = "VIC814 (rw) register accessor: MCU Interrupt Event 5\n\nYou can [`read`](crate::Reg::read) this register and get [`vic814::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic814::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vic814`] module"]
#[doc(alias = "VIC814")]
pub type Vic814 = crate::Reg<vic814::Vic814Spec>;
#[doc = "MCU Interrupt Event 5"]
pub mod vic814;
#[doc = "VIC818 (rw) register accessor: MCU Interrupt Event 6\n\nYou can [`read`](crate::Reg::read) this register and get [`vic818::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic818::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vic818`] module"]
#[doc(alias = "VIC818")]
pub type Vic818 = crate::Reg<vic818::Vic818Spec>;
#[doc = "MCU Interrupt Event 6"]
pub mod vic818;
#[doc = "VIC81C (rw) register accessor: MCU Interrupt Event 7\n\nYou can [`read`](crate::Reg::read) this register and get [`vic81c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic81c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vic81c`] module"]
#[doc(alias = "VIC81C")]
pub type Vic81c = crate::Reg<vic81c::Vic81cSpec>;
#[doc = "MCU Interrupt Event 7"]
pub mod vic81c;
#[doc = "VIC880 (rw) register accessor: MCU Interrupt Enable 0\n\nYou can [`read`](crate::Reg::read) this register and get [`vic880::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic880::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vic880`] module"]
#[doc(alias = "VIC880")]
pub type Vic880 = crate::Reg<vic880::Vic880Spec>;
#[doc = "MCU Interrupt Enable 0"]
pub mod vic880;
#[doc = "VIC884 (rw) register accessor: MCU Interrupt Enable 1\n\nYou can [`read`](crate::Reg::read) this register and get [`vic884::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic884::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vic884`] module"]
#[doc(alias = "VIC884")]
pub type Vic884 = crate::Reg<vic884::Vic884Spec>;
#[doc = "MCU Interrupt Enable 1"]
pub mod vic884;
#[doc = "VIC888 (rw) register accessor: MCU Interrupt Enable 2\n\nYou can [`read`](crate::Reg::read) this register and get [`vic888::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic888::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vic888`] module"]
#[doc(alias = "VIC888")]
pub type Vic888 = crate::Reg<vic888::Vic888Spec>;
#[doc = "MCU Interrupt Enable 2"]
pub mod vic888;
#[doc = "VIC88C (rw) register accessor: MCU Interrupt Enable 3\n\nYou can [`read`](crate::Reg::read) this register and get [`vic88c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic88c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vic88c`] module"]
#[doc(alias = "VIC88C")]
pub type Vic88c = crate::Reg<vic88c::Vic88cSpec>;
#[doc = "MCU Interrupt Enable 3"]
pub mod vic88c;
#[doc = "VIC890 (rw) register accessor: MCU Interrupt Enable 4\n\nYou can [`read`](crate::Reg::read) this register and get [`vic890::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic890::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vic890`] module"]
#[doc(alias = "VIC890")]
pub type Vic890 = crate::Reg<vic890::Vic890Spec>;
#[doc = "MCU Interrupt Enable 4"]
pub mod vic890;
#[doc = "VIC894 (rw) register accessor: MCU Interrupt Enable 5\n\nYou can [`read`](crate::Reg::read) this register and get [`vic894::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic894::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vic894`] module"]
#[doc(alias = "VIC894")]
pub type Vic894 = crate::Reg<vic894::Vic894Spec>;
#[doc = "MCU Interrupt Enable 5"]
pub mod vic894;
#[doc = "VIC898 (rw) register accessor: MCU Interrupt Enable 6\n\nYou can [`read`](crate::Reg::read) this register and get [`vic898::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic898::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vic898`] module"]
#[doc(alias = "VIC898")]
pub type Vic898 = crate::Reg<vic898::Vic898Spec>;
#[doc = "MCU Interrupt Enable 6"]
pub mod vic898;
#[doc = "VIC89C (rw) register accessor: MCU Interrupt Enable 7\n\nYou can [`read`](crate::Reg::read) this register and get [`vic89c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic89c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vic89c`] module"]
#[doc(alias = "VIC89C")]
pub type Vic89c = crate::Reg<vic89c::Vic89cSpec>;
#[doc = "MCU Interrupt Enable 7"]
pub mod vic89c;
