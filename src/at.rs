#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    at004: At004,
    _reserved1: [u8; 0x08],
    at010: At010,
    at014: At014,
    at018: At018,
    at01c: At01c,
    at020: At020,
    at024: At024,
    at028: At028,
    at02c: At02c,
    at030: At030,
    _reserved10: [u8; 0x08],
    at03c: At03c,
    _reserved11: [u8; 0x04],
    at044: At044,
    at048: At048,
    _reserved13: [u8; 0x04],
    at050: At050,
    _reserved14: [u8; 0x10],
    at064: At064,
    at068: At068,
    _reserved16: [u8; 0x04],
    at070: At070,
    at074: At074,
    _reserved18: [u8; 0x04],
    at07c: At07c,
    _reserved19: [u8; 0x04],
    at084: At084,
    at088: At088,
    _reserved21: [u8; 0x08],
    at094: At094,
    at098: At098,
    at09c: At09c,
    _reserved24: [u8; 0x58],
    at0f8: At0f8,
    at0fc: At0fc,
}
impl RegisterBlock {
    #[doc = "0x04 - Anti-Tamper Interrupt Status"]
    #[inline(always)]
    pub const fn at004(&self) -> &At004 {
        &self.at004
    }
    #[doc = "0x10 - Clock Attack Monitor Control"]
    #[inline(always)]
    pub const fn at010(&self) -> &At010 {
        &self.at010
    }
    #[doc = "0x14 - Clock Attack Monitor Enable Set"]
    #[inline(always)]
    pub const fn at014(&self) -> &At014 {
        &self.at014
    }
    #[doc = "0x18 - Clock Attack Monitor Enable Clear"]
    #[inline(always)]
    pub const fn at018(&self) -> &At018 {
        &self.at018
    }
    #[doc = "0x1c - Clock Attack Monitor Status 0"]
    #[inline(always)]
    pub const fn at01c(&self) -> &At01c {
        &self.at01c
    }
    #[doc = "0x20 - Clock Attack Monitor Status 1"]
    #[inline(always)]
    pub const fn at020(&self) -> &At020 {
        &self.at020
    }
    #[doc = "0x24 - Clock Attack Monitor Interrupt Status"]
    #[inline(always)]
    pub const fn at024(&self) -> &At024 {
        &self.at024
    }
    #[doc = "0x28 - Clock Attack Monitor Interrupt Enable"]
    #[inline(always)]
    pub const fn at028(&self) -> &At028 {
        &self.at028
    }
    #[doc = "0x2c - Clock Attack Monitor Master"]
    #[inline(always)]
    pub const fn at02c(&self) -> &At02c {
        &self.at02c
    }
    #[doc = "0x30 - Glitch Detection Control"]
    #[inline(always)]
    pub const fn at030(&self) -> &At030 {
        &self.at030
    }
    #[doc = "0x3c - Glitch Detection Status"]
    #[inline(always)]
    pub const fn at03c(&self) -> &At03c {
        &self.at03c
    }
    #[doc = "0x44 - Glitch Detection Interrupt Status"]
    #[inline(always)]
    pub const fn at044(&self) -> &At044 {
        &self.at044
    }
    #[doc = "0x48 - Glitch Detection Interrupt Enable"]
    #[inline(always)]
    pub const fn at048(&self) -> &At048 {
        &self.at048
    }
    #[doc = "0x50 - IR Drop Control"]
    #[inline(always)]
    pub const fn at050(&self) -> &At050 {
        &self.at050
    }
    #[doc = "0x64 - IR Drop Interrupt Status"]
    #[inline(always)]
    pub const fn at064(&self) -> &At064 {
        &self.at064
    }
    #[doc = "0x68 - IR Drop Interrupt Enable"]
    #[inline(always)]
    pub const fn at068(&self) -> &At068 {
        &self.at068
    }
    #[doc = "0x70 - TSENSE Control"]
    #[inline(always)]
    pub const fn at070(&self) -> &At070 {
        &self.at070
    }
    #[doc = "0x74 - TSENSE Clock Divisor"]
    #[inline(always)]
    pub const fn at074(&self) -> &At074 {
        &self.at074
    }
    #[doc = "0x7c - TSENSE Status"]
    #[inline(always)]
    pub const fn at07c(&self) -> &At07c {
        &self.at07c
    }
    #[doc = "0x84 - TSENSE Interrupt Status"]
    #[inline(always)]
    pub const fn at084(&self) -> &At084 {
        &self.at084
    }
    #[doc = "0x88 - TSENSE Interrupt Enable"]
    #[inline(always)]
    pub const fn at088(&self) -> &At088 {
        &self.at088
    }
    #[doc = "0x94 - Glitch Detection Master"]
    #[inline(always)]
    pub const fn at094(&self) -> &At094 {
        &self.at094
    }
    #[doc = "0x98 - IR Drop Master"]
    #[inline(always)]
    pub const fn at098(&self) -> &At098 {
        &self.at098
    }
    #[doc = "0x9c - TSENSE Master"]
    #[inline(always)]
    pub const fn at09c(&self) -> &At09c {
        &self.at09c
    }
    #[doc = "0xf8 - Write Protection 1"]
    #[inline(always)]
    pub const fn at0f8(&self) -> &At0f8 {
        &self.at0f8
    }
    #[doc = "0xfc - Write Protection 2"]
    #[inline(always)]
    pub const fn at0fc(&self) -> &At0fc {
        &self.at0fc
    }
}
#[doc = "AT004 (rw) register accessor: Anti-Tamper Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`at004::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`at004::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@at004`] module"]
#[doc(alias = "AT004")]
pub type At004 = crate::Reg<at004::At004Spec>;
#[doc = "Anti-Tamper Interrupt Status"]
pub mod at004;
#[doc = "AT010 (rw) register accessor: Clock Attack Monitor Control\n\nYou can [`read`](crate::Reg::read) this register and get [`at010::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`at010::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@at010`] module"]
#[doc(alias = "AT010")]
pub type At010 = crate::Reg<at010::At010Spec>;
#[doc = "Clock Attack Monitor Control"]
pub mod at010;
#[doc = "AT014 (rw) register accessor: Clock Attack Monitor Enable Set\n\nYou can [`read`](crate::Reg::read) this register and get [`at014::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`at014::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@at014`] module"]
#[doc(alias = "AT014")]
pub type At014 = crate::Reg<at014::At014Spec>;
#[doc = "Clock Attack Monitor Enable Set"]
pub mod at014;
#[doc = "AT018 (rw) register accessor: Clock Attack Monitor Enable Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`at018::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`at018::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@at018`] module"]
#[doc(alias = "AT018")]
pub type At018 = crate::Reg<at018::At018Spec>;
#[doc = "Clock Attack Monitor Enable Clear"]
pub mod at018;
#[doc = "AT01C (rw) register accessor: Clock Attack Monitor Status 0\n\nYou can [`read`](crate::Reg::read) this register and get [`at01c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`at01c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@at01c`] module"]
#[doc(alias = "AT01C")]
pub type At01c = crate::Reg<at01c::At01cSpec>;
#[doc = "Clock Attack Monitor Status 0"]
pub mod at01c;
#[doc = "AT020 (rw) register accessor: Clock Attack Monitor Status 1\n\nYou can [`read`](crate::Reg::read) this register and get [`at020::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`at020::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@at020`] module"]
#[doc(alias = "AT020")]
pub type At020 = crate::Reg<at020::At020Spec>;
#[doc = "Clock Attack Monitor Status 1"]
pub mod at020;
#[doc = "AT024 (rw) register accessor: Clock Attack Monitor Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`at024::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`at024::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@at024`] module"]
#[doc(alias = "AT024")]
pub type At024 = crate::Reg<at024::At024Spec>;
#[doc = "Clock Attack Monitor Interrupt Status"]
pub mod at024;
#[doc = "AT028 (rw) register accessor: Clock Attack Monitor Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`at028::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`at028::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@at028`] module"]
#[doc(alias = "AT028")]
pub type At028 = crate::Reg<at028::At028Spec>;
#[doc = "Clock Attack Monitor Interrupt Enable"]
pub mod at028;
#[doc = "AT02C (rw) register accessor: Clock Attack Monitor Master\n\nYou can [`read`](crate::Reg::read) this register and get [`at02c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`at02c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@at02c`] module"]
#[doc(alias = "AT02C")]
pub type At02c = crate::Reg<at02c::At02cSpec>;
#[doc = "Clock Attack Monitor Master"]
pub mod at02c;
#[doc = "AT030 (rw) register accessor: Glitch Detection Control\n\nYou can [`read`](crate::Reg::read) this register and get [`at030::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`at030::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@at030`] module"]
#[doc(alias = "AT030")]
pub type At030 = crate::Reg<at030::At030Spec>;
#[doc = "Glitch Detection Control"]
pub mod at030;
#[doc = "AT03C (rw) register accessor: Glitch Detection Status\n\nYou can [`read`](crate::Reg::read) this register and get [`at03c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`at03c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@at03c`] module"]
#[doc(alias = "AT03C")]
pub type At03c = crate::Reg<at03c::At03cSpec>;
#[doc = "Glitch Detection Status"]
pub mod at03c;
#[doc = "AT044 (rw) register accessor: Glitch Detection Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`at044::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`at044::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@at044`] module"]
#[doc(alias = "AT044")]
pub type At044 = crate::Reg<at044::At044Spec>;
#[doc = "Glitch Detection Interrupt Status"]
pub mod at044;
#[doc = "AT048 (rw) register accessor: Glitch Detection Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`at048::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`at048::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@at048`] module"]
#[doc(alias = "AT048")]
pub type At048 = crate::Reg<at048::At048Spec>;
#[doc = "Glitch Detection Interrupt Enable"]
pub mod at048;
#[doc = "AT050 (rw) register accessor: IR Drop Control\n\nYou can [`read`](crate::Reg::read) this register and get [`at050::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`at050::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@at050`] module"]
#[doc(alias = "AT050")]
pub type At050 = crate::Reg<at050::At050Spec>;
#[doc = "IR Drop Control"]
pub mod at050;
#[doc = "AT064 (rw) register accessor: IR Drop Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`at064::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`at064::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@at064`] module"]
#[doc(alias = "AT064")]
pub type At064 = crate::Reg<at064::At064Spec>;
#[doc = "IR Drop Interrupt Status"]
pub mod at064;
#[doc = "AT068 (rw) register accessor: IR Drop Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`at068::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`at068::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@at068`] module"]
#[doc(alias = "AT068")]
pub type At068 = crate::Reg<at068::At068Spec>;
#[doc = "IR Drop Interrupt Enable"]
pub mod at068;
#[doc = "AT070 (rw) register accessor: TSENSE Control\n\nYou can [`read`](crate::Reg::read) this register and get [`at070::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`at070::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@at070`] module"]
#[doc(alias = "AT070")]
pub type At070 = crate::Reg<at070::At070Spec>;
#[doc = "TSENSE Control"]
pub mod at070;
#[doc = "AT074 (rw) register accessor: TSENSE Clock Divisor\n\nYou can [`read`](crate::Reg::read) this register and get [`at074::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`at074::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@at074`] module"]
#[doc(alias = "AT074")]
pub type At074 = crate::Reg<at074::At074Spec>;
#[doc = "TSENSE Clock Divisor"]
pub mod at074;
#[doc = "AT07C (rw) register accessor: TSENSE Status\n\nYou can [`read`](crate::Reg::read) this register and get [`at07c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`at07c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@at07c`] module"]
#[doc(alias = "AT07C")]
pub type At07c = crate::Reg<at07c::At07cSpec>;
#[doc = "TSENSE Status"]
pub mod at07c;
#[doc = "AT084 (rw) register accessor: TSENSE Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`at084::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`at084::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@at084`] module"]
#[doc(alias = "AT084")]
pub type At084 = crate::Reg<at084::At084Spec>;
#[doc = "TSENSE Interrupt Status"]
pub mod at084;
#[doc = "AT088 (rw) register accessor: TSENSE Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`at088::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`at088::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@at088`] module"]
#[doc(alias = "AT088")]
pub type At088 = crate::Reg<at088::At088Spec>;
#[doc = "TSENSE Interrupt Enable"]
pub mod at088;
#[doc = "AT094 (rw) register accessor: Glitch Detection Master\n\nYou can [`read`](crate::Reg::read) this register and get [`at094::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`at094::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@at094`] module"]
#[doc(alias = "AT094")]
pub type At094 = crate::Reg<at094::At094Spec>;
#[doc = "Glitch Detection Master"]
pub mod at094;
#[doc = "AT098 (rw) register accessor: IR Drop Master\n\nYou can [`read`](crate::Reg::read) this register and get [`at098::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`at098::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@at098`] module"]
#[doc(alias = "AT098")]
pub type At098 = crate::Reg<at098::At098Spec>;
#[doc = "IR Drop Master"]
pub mod at098;
#[doc = "AT09C (rw) register accessor: TSENSE Master\n\nYou can [`read`](crate::Reg::read) this register and get [`at09c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`at09c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@at09c`] module"]
#[doc(alias = "AT09C")]
pub type At09c = crate::Reg<at09c::At09cSpec>;
#[doc = "TSENSE Master"]
pub mod at09c;
#[doc = "AT0F8 (rw) register accessor: Write Protection 1\n\nYou can [`read`](crate::Reg::read) this register and get [`at0f8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`at0f8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@at0f8`] module"]
#[doc(alias = "AT0F8")]
pub type At0f8 = crate::Reg<at0f8::At0f8Spec>;
#[doc = "Write Protection 1"]
pub mod at0f8;
#[doc = "AT0FC (rw) register accessor: Write Protection 2\n\nYou can [`read`](crate::Reg::read) this register and get [`at0fc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`at0fc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@at0fc`] module"]
#[doc(alias = "AT0FC")]
pub type At0fc = crate::Reg<at0fc::At0fcSpec>;
#[doc = "Write Protection 2"]
pub mod at0fc;
