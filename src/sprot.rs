#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sprot000: Sprot000,
    _reserved1: [u8; 0x08],
    sprot00c: Sprot00c,
    sprot010: Sprot010,
    sprot014: Sprot014,
    _reserved4: [u8; 0x68],
    sprot080: Sprot080,
    sprot084: Sprot084,
    sprot088: Sprot088,
    sprot08c: Sprot08c,
    sprot090: Sprot090,
    sprot094: Sprot094,
    sprot098: Sprot098,
    sprot09c: Sprot09c,
    sprot0a0: Sprot0a0,
    sprot0a4: Sprot0a4,
    sprot0a8: Sprot0a8,
    sprot0ac: Sprot0ac,
    sprot0b0: Sprot0b0,
    sprot0b4: Sprot0b4,
    sprot0b8: Sprot0b8,
    sprot0bc: Sprot0bc,
    sprot0c0: Sprot0c0,
    sprot0c4: Sprot0c4,
    sprot0c8: Sprot0c8,
    sprot0cc: Sprot0cc,
    sprot0d0: Sprot0d0,
    sprot0d4: Sprot0d4,
    sprot0d8: Sprot0d8,
    sprot0dc: Sprot0dc,
    sprot0e0: Sprot0e0,
    sprot0e4: Sprot0e4,
    sprot0e8: Sprot0e8,
    sprot0ec: Sprot0ec,
    sprot0f0: Sprot0f0,
    sprot0f4: Sprot0f4,
    sprot0f8: Sprot0f8,
    sprot0fc: Sprot0fc,
}
impl RegisterBlock {
    #[doc = "0x00 - SPROT\\_CFG"]
    #[inline(always)]
    pub const fn sprot000(&self) -> &Sprot000 {
        &self.sprot000
    }
    #[doc = "0x0c - SPROT\\_WLOCK"]
    #[inline(always)]
    pub const fn sprot00c(&self) -> &Sprot00c {
        &self.sprot00c
    }
    #[doc = "0x10 - SPROT\\_SIDG0"]
    #[inline(always)]
    pub const fn sprot010(&self) -> &Sprot010 {
        &self.sprot010
    }
    #[doc = "0x14 - SPROT\\_SIDG1"]
    #[inline(always)]
    pub const fn sprot014(&self) -> &Sprot014 {
        &self.sprot014
    }
    #[doc = "0x80 - SPROT\\_CTL00"]
    #[inline(always)]
    pub const fn sprot080(&self) -> &Sprot080 {
        &self.sprot080
    }
    #[doc = "0x84 - SPROT\\_CTL01"]
    #[inline(always)]
    pub const fn sprot084(&self) -> &Sprot084 {
        &self.sprot084
    }
    #[doc = "0x88 - SPROT\\_CTL02"]
    #[inline(always)]
    pub const fn sprot088(&self) -> &Sprot088 {
        &self.sprot088
    }
    #[doc = "0x8c - SPROT\\_CTL03"]
    #[inline(always)]
    pub const fn sprot08c(&self) -> &Sprot08c {
        &self.sprot08c
    }
    #[doc = "0x90 - SPROT\\_CTL04"]
    #[inline(always)]
    pub const fn sprot090(&self) -> &Sprot090 {
        &self.sprot090
    }
    #[doc = "0x94 - SPROT\\_CTL05"]
    #[inline(always)]
    pub const fn sprot094(&self) -> &Sprot094 {
        &self.sprot094
    }
    #[doc = "0x98 - SPROT\\_CTL06"]
    #[inline(always)]
    pub const fn sprot098(&self) -> &Sprot098 {
        &self.sprot098
    }
    #[doc = "0x9c - SPROT\\_CTL07"]
    #[inline(always)]
    pub const fn sprot09c(&self) -> &Sprot09c {
        &self.sprot09c
    }
    #[doc = "0xa0 - SPROT\\_CTL08"]
    #[inline(always)]
    pub const fn sprot0a0(&self) -> &Sprot0a0 {
        &self.sprot0a0
    }
    #[doc = "0xa4 - SPROT\\_CTL09"]
    #[inline(always)]
    pub const fn sprot0a4(&self) -> &Sprot0a4 {
        &self.sprot0a4
    }
    #[doc = "0xa8 - SPROT\\_CTL10"]
    #[inline(always)]
    pub const fn sprot0a8(&self) -> &Sprot0a8 {
        &self.sprot0a8
    }
    #[doc = "0xac - SPROT\\_CTL11"]
    #[inline(always)]
    pub const fn sprot0ac(&self) -> &Sprot0ac {
        &self.sprot0ac
    }
    #[doc = "0xb0 - SPROT\\_CTL12"]
    #[inline(always)]
    pub const fn sprot0b0(&self) -> &Sprot0b0 {
        &self.sprot0b0
    }
    #[doc = "0xb4 - SPROT\\_CTL13"]
    #[inline(always)]
    pub const fn sprot0b4(&self) -> &Sprot0b4 {
        &self.sprot0b4
    }
    #[doc = "0xb8 - SPROT\\_CTL14"]
    #[inline(always)]
    pub const fn sprot0b8(&self) -> &Sprot0b8 {
        &self.sprot0b8
    }
    #[doc = "0xbc - SPROT\\_CTL15"]
    #[inline(always)]
    pub const fn sprot0bc(&self) -> &Sprot0bc {
        &self.sprot0bc
    }
    #[doc = "0xc0 - SPROT\\_ADR00"]
    #[inline(always)]
    pub const fn sprot0c0(&self) -> &Sprot0c0 {
        &self.sprot0c0
    }
    #[doc = "0xc4 - SPROT\\_ADR01"]
    #[inline(always)]
    pub const fn sprot0c4(&self) -> &Sprot0c4 {
        &self.sprot0c4
    }
    #[doc = "0xc8 - SPROT\\_ADR02"]
    #[inline(always)]
    pub const fn sprot0c8(&self) -> &Sprot0c8 {
        &self.sprot0c8
    }
    #[doc = "0xcc - SPROT\\_ADR03"]
    #[inline(always)]
    pub const fn sprot0cc(&self) -> &Sprot0cc {
        &self.sprot0cc
    }
    #[doc = "0xd0 - SPROT\\_ADR04"]
    #[inline(always)]
    pub const fn sprot0d0(&self) -> &Sprot0d0 {
        &self.sprot0d0
    }
    #[doc = "0xd4 - SPROT\\_ADR05"]
    #[inline(always)]
    pub const fn sprot0d4(&self) -> &Sprot0d4 {
        &self.sprot0d4
    }
    #[doc = "0xd8 - SPROT\\_ADR06"]
    #[inline(always)]
    pub const fn sprot0d8(&self) -> &Sprot0d8 {
        &self.sprot0d8
    }
    #[doc = "0xdc - SPROT\\_ADR07"]
    #[inline(always)]
    pub const fn sprot0dc(&self) -> &Sprot0dc {
        &self.sprot0dc
    }
    #[doc = "0xe0 - SPROT\\_ADR08"]
    #[inline(always)]
    pub const fn sprot0e0(&self) -> &Sprot0e0 {
        &self.sprot0e0
    }
    #[doc = "0xe4 - SPROT\\_ADR09"]
    #[inline(always)]
    pub const fn sprot0e4(&self) -> &Sprot0e4 {
        &self.sprot0e4
    }
    #[doc = "0xe8 - SPROT\\_ADR10"]
    #[inline(always)]
    pub const fn sprot0e8(&self) -> &Sprot0e8 {
        &self.sprot0e8
    }
    #[doc = "0xec - SPROT\\_ADR11"]
    #[inline(always)]
    pub const fn sprot0ec(&self) -> &Sprot0ec {
        &self.sprot0ec
    }
    #[doc = "0xf0 - SPROT\\_ADR12"]
    #[inline(always)]
    pub const fn sprot0f0(&self) -> &Sprot0f0 {
        &self.sprot0f0
    }
    #[doc = "0xf4 - SPROT\\_ADR13"]
    #[inline(always)]
    pub const fn sprot0f4(&self) -> &Sprot0f4 {
        &self.sprot0f4
    }
    #[doc = "0xf8 - SPROT\\_ADR14"]
    #[inline(always)]
    pub const fn sprot0f8(&self) -> &Sprot0f8 {
        &self.sprot0f8
    }
    #[doc = "0xfc - SPROT\\_ADR15"]
    #[inline(always)]
    pub const fn sprot0fc(&self) -> &Sprot0fc {
        &self.sprot0fc
    }
}
#[doc = "SPROT000 (rw) register accessor: SPROT\\_CFG\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot000::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot000::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sprot000`] module"]
#[doc(alias = "SPROT000")]
pub type Sprot000 = crate::Reg<sprot000::Sprot000Spec>;
#[doc = "SPROT\\_CFG"]
pub mod sprot000;
#[doc = "SPROT00C (rw) register accessor: SPROT\\_WLOCK\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot00c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot00c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sprot00c`] module"]
#[doc(alias = "SPROT00C")]
pub type Sprot00c = crate::Reg<sprot00c::Sprot00cSpec>;
#[doc = "SPROT\\_WLOCK"]
pub mod sprot00c;
#[doc = "SPROT010 (rw) register accessor: SPROT\\_SIDG0\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot010::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot010::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sprot010`] module"]
#[doc(alias = "SPROT010")]
pub type Sprot010 = crate::Reg<sprot010::Sprot010Spec>;
#[doc = "SPROT\\_SIDG0"]
pub mod sprot010;
#[doc = "SPROT014 (rw) register accessor: SPROT\\_SIDG1\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot014::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot014::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sprot014`] module"]
#[doc(alias = "SPROT014")]
pub type Sprot014 = crate::Reg<sprot014::Sprot014Spec>;
#[doc = "SPROT\\_SIDG1"]
pub mod sprot014;
#[doc = "SPROT080 (rw) register accessor: SPROT\\_CTL00\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot080::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot080::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sprot080`] module"]
#[doc(alias = "SPROT080")]
pub type Sprot080 = crate::Reg<sprot080::Sprot080Spec>;
#[doc = "SPROT\\_CTL00"]
pub mod sprot080;
#[doc = "SPROT084 (rw) register accessor: SPROT\\_CTL01\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot084::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot084::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sprot084`] module"]
#[doc(alias = "SPROT084")]
pub type Sprot084 = crate::Reg<sprot084::Sprot084Spec>;
#[doc = "SPROT\\_CTL01"]
pub mod sprot084;
#[doc = "SPROT088 (rw) register accessor: SPROT\\_CTL02\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot088::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot088::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sprot088`] module"]
#[doc(alias = "SPROT088")]
pub type Sprot088 = crate::Reg<sprot088::Sprot088Spec>;
#[doc = "SPROT\\_CTL02"]
pub mod sprot088;
#[doc = "SPROT08C (rw) register accessor: SPROT\\_CTL03\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot08c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot08c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sprot08c`] module"]
#[doc(alias = "SPROT08C")]
pub type Sprot08c = crate::Reg<sprot08c::Sprot08cSpec>;
#[doc = "SPROT\\_CTL03"]
pub mod sprot08c;
#[doc = "SPROT090 (rw) register accessor: SPROT\\_CTL04\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot090::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot090::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sprot090`] module"]
#[doc(alias = "SPROT090")]
pub type Sprot090 = crate::Reg<sprot090::Sprot090Spec>;
#[doc = "SPROT\\_CTL04"]
pub mod sprot090;
#[doc = "SPROT094 (rw) register accessor: SPROT\\_CTL05\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot094::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot094::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sprot094`] module"]
#[doc(alias = "SPROT094")]
pub type Sprot094 = crate::Reg<sprot094::Sprot094Spec>;
#[doc = "SPROT\\_CTL05"]
pub mod sprot094;
#[doc = "SPROT098 (rw) register accessor: SPROT\\_CTL06\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot098::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot098::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sprot098`] module"]
#[doc(alias = "SPROT098")]
pub type Sprot098 = crate::Reg<sprot098::Sprot098Spec>;
#[doc = "SPROT\\_CTL06"]
pub mod sprot098;
#[doc = "SPROT09C (rw) register accessor: SPROT\\_CTL07\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot09c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot09c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sprot09c`] module"]
#[doc(alias = "SPROT09C")]
pub type Sprot09c = crate::Reg<sprot09c::Sprot09cSpec>;
#[doc = "SPROT\\_CTL07"]
pub mod sprot09c;
#[doc = "SPROT0A0 (rw) register accessor: SPROT\\_CTL08\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot0a0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot0a0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sprot0a0`] module"]
#[doc(alias = "SPROT0A0")]
pub type Sprot0a0 = crate::Reg<sprot0a0::Sprot0a0Spec>;
#[doc = "SPROT\\_CTL08"]
pub mod sprot0a0;
#[doc = "SPROT0A4 (rw) register accessor: SPROT\\_CTL09\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot0a4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot0a4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sprot0a4`] module"]
#[doc(alias = "SPROT0A4")]
pub type Sprot0a4 = crate::Reg<sprot0a4::Sprot0a4Spec>;
#[doc = "SPROT\\_CTL09"]
pub mod sprot0a4;
#[doc = "SPROT0A8 (rw) register accessor: SPROT\\_CTL10\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot0a8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot0a8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sprot0a8`] module"]
#[doc(alias = "SPROT0A8")]
pub type Sprot0a8 = crate::Reg<sprot0a8::Sprot0a8Spec>;
#[doc = "SPROT\\_CTL10"]
pub mod sprot0a8;
#[doc = "SPROT0AC (rw) register accessor: SPROT\\_CTL11\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot0ac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot0ac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sprot0ac`] module"]
#[doc(alias = "SPROT0AC")]
pub type Sprot0ac = crate::Reg<sprot0ac::Sprot0acSpec>;
#[doc = "SPROT\\_CTL11"]
pub mod sprot0ac;
#[doc = "SPROT0B0 (rw) register accessor: SPROT\\_CTL12\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot0b0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot0b0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sprot0b0`] module"]
#[doc(alias = "SPROT0B0")]
pub type Sprot0b0 = crate::Reg<sprot0b0::Sprot0b0Spec>;
#[doc = "SPROT\\_CTL12"]
pub mod sprot0b0;
#[doc = "SPROT0B4 (rw) register accessor: SPROT\\_CTL13\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot0b4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot0b4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sprot0b4`] module"]
#[doc(alias = "SPROT0B4")]
pub type Sprot0b4 = crate::Reg<sprot0b4::Sprot0b4Spec>;
#[doc = "SPROT\\_CTL13"]
pub mod sprot0b4;
#[doc = "SPROT0B8 (rw) register accessor: SPROT\\_CTL14\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot0b8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot0b8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sprot0b8`] module"]
#[doc(alias = "SPROT0B8")]
pub type Sprot0b8 = crate::Reg<sprot0b8::Sprot0b8Spec>;
#[doc = "SPROT\\_CTL14"]
pub mod sprot0b8;
#[doc = "SPROT0BC (rw) register accessor: SPROT\\_CTL15\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot0bc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot0bc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sprot0bc`] module"]
#[doc(alias = "SPROT0BC")]
pub type Sprot0bc = crate::Reg<sprot0bc::Sprot0bcSpec>;
#[doc = "SPROT\\_CTL15"]
pub mod sprot0bc;
#[doc = "SPROT0C0 (rw) register accessor: SPROT\\_ADR00\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot0c0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot0c0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sprot0c0`] module"]
#[doc(alias = "SPROT0C0")]
pub type Sprot0c0 = crate::Reg<sprot0c0::Sprot0c0Spec>;
#[doc = "SPROT\\_ADR00"]
pub mod sprot0c0;
#[doc = "SPROT0C4 (rw) register accessor: SPROT\\_ADR01\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot0c4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot0c4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sprot0c4`] module"]
#[doc(alias = "SPROT0C4")]
pub type Sprot0c4 = crate::Reg<sprot0c4::Sprot0c4Spec>;
#[doc = "SPROT\\_ADR01"]
pub mod sprot0c4;
#[doc = "SPROT0C8 (rw) register accessor: SPROT\\_ADR02\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot0c8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot0c8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sprot0c8`] module"]
#[doc(alias = "SPROT0C8")]
pub type Sprot0c8 = crate::Reg<sprot0c8::Sprot0c8Spec>;
#[doc = "SPROT\\_ADR02"]
pub mod sprot0c8;
#[doc = "SPROT0CC (rw) register accessor: SPROT\\_ADR03\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot0cc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot0cc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sprot0cc`] module"]
#[doc(alias = "SPROT0CC")]
pub type Sprot0cc = crate::Reg<sprot0cc::Sprot0ccSpec>;
#[doc = "SPROT\\_ADR03"]
pub mod sprot0cc;
#[doc = "SPROT0D0 (rw) register accessor: SPROT\\_ADR04\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot0d0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot0d0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sprot0d0`] module"]
#[doc(alias = "SPROT0D0")]
pub type Sprot0d0 = crate::Reg<sprot0d0::Sprot0d0Spec>;
#[doc = "SPROT\\_ADR04"]
pub mod sprot0d0;
#[doc = "SPROT0D4 (rw) register accessor: SPROT\\_ADR05\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot0d4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot0d4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sprot0d4`] module"]
#[doc(alias = "SPROT0D4")]
pub type Sprot0d4 = crate::Reg<sprot0d4::Sprot0d4Spec>;
#[doc = "SPROT\\_ADR05"]
pub mod sprot0d4;
#[doc = "SPROT0D8 (rw) register accessor: SPROT\\_ADR06\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot0d8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot0d8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sprot0d8`] module"]
#[doc(alias = "SPROT0D8")]
pub type Sprot0d8 = crate::Reg<sprot0d8::Sprot0d8Spec>;
#[doc = "SPROT\\_ADR06"]
pub mod sprot0d8;
#[doc = "SPROT0DC (rw) register accessor: SPROT\\_ADR07\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot0dc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot0dc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sprot0dc`] module"]
#[doc(alias = "SPROT0DC")]
pub type Sprot0dc = crate::Reg<sprot0dc::Sprot0dcSpec>;
#[doc = "SPROT\\_ADR07"]
pub mod sprot0dc;
#[doc = "SPROT0E0 (rw) register accessor: SPROT\\_ADR08\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot0e0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot0e0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sprot0e0`] module"]
#[doc(alias = "SPROT0E0")]
pub type Sprot0e0 = crate::Reg<sprot0e0::Sprot0e0Spec>;
#[doc = "SPROT\\_ADR08"]
pub mod sprot0e0;
#[doc = "SPROT0E4 (rw) register accessor: SPROT\\_ADR09\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot0e4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot0e4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sprot0e4`] module"]
#[doc(alias = "SPROT0E4")]
pub type Sprot0e4 = crate::Reg<sprot0e4::Sprot0e4Spec>;
#[doc = "SPROT\\_ADR09"]
pub mod sprot0e4;
#[doc = "SPROT0E8 (rw) register accessor: SPROT\\_ADR10\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot0e8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot0e8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sprot0e8`] module"]
#[doc(alias = "SPROT0E8")]
pub type Sprot0e8 = crate::Reg<sprot0e8::Sprot0e8Spec>;
#[doc = "SPROT\\_ADR10"]
pub mod sprot0e8;
#[doc = "SPROT0EC (rw) register accessor: SPROT\\_ADR11\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot0ec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot0ec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sprot0ec`] module"]
#[doc(alias = "SPROT0EC")]
pub type Sprot0ec = crate::Reg<sprot0ec::Sprot0ecSpec>;
#[doc = "SPROT\\_ADR11"]
pub mod sprot0ec;
#[doc = "SPROT0F0 (rw) register accessor: SPROT\\_ADR12\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot0f0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot0f0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sprot0f0`] module"]
#[doc(alias = "SPROT0F0")]
pub type Sprot0f0 = crate::Reg<sprot0f0::Sprot0f0Spec>;
#[doc = "SPROT\\_ADR12"]
pub mod sprot0f0;
#[doc = "SPROT0F4 (rw) register accessor: SPROT\\_ADR13\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot0f4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot0f4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sprot0f4`] module"]
#[doc(alias = "SPROT0F4")]
pub type Sprot0f4 = crate::Reg<sprot0f4::Sprot0f4Spec>;
#[doc = "SPROT\\_ADR13"]
pub mod sprot0f4;
#[doc = "SPROT0F8 (rw) register accessor: SPROT\\_ADR14\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot0f8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot0f8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sprot0f8`] module"]
#[doc(alias = "SPROT0F8")]
pub type Sprot0f8 = crate::Reg<sprot0f8::Sprot0f8Spec>;
#[doc = "SPROT\\_ADR14"]
pub mod sprot0f8;
#[doc = "SPROT0FC (rw) register accessor: SPROT\\_ADR15\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot0fc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot0fc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sprot0fc`] module"]
#[doc(alias = "SPROT0FC")]
pub type Sprot0fc = crate::Reg<sprot0fc::Sprot0fcSpec>;
#[doc = "SPROT\\_ADR15"]
pub mod sprot0fc;
