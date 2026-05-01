#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    gsram000: Gsram000,
    gsram004: Gsram004,
    gsram008: Gsram008,
    gsram00c: Gsram00c,
    gsram010: Gsram010,
    gsram014: Gsram014,
    gsram018: Gsram018,
    _reserved7: [u8; 0x64],
    gsram080: Gsram080,
    gsram084: Gsram084,
    gsram088: Gsram088,
    gsram08c: Gsram08c,
    gsram090: Gsram090,
    gsram094: Gsram094,
    gsram098: Gsram098,
    gsram09c: Gsram09c,
    gsram0a0: Gsram0a0,
    gsram0a4: Gsram0a4,
    gsram0a8: Gsram0a8,
    gsram0ac: Gsram0ac,
    gsram0b0: Gsram0b0,
    gsram0b4: Gsram0b4,
    gsram0b8: Gsram0b8,
    gsram0bc: Gsram0bc,
    gsram0c0: Gsram0c0,
    gsram0c4: Gsram0c4,
    gsram0c8: Gsram0c8,
    gsram0cc: Gsram0cc,
    gsram0d0: Gsram0d0,
    gsram0d4: Gsram0d4,
    gsram0d8: Gsram0d8,
    gsram0dc: Gsram0dc,
    gsram0e0: Gsram0e0,
    gsram0e4: Gsram0e4,
    gsram0e8: Gsram0e8,
    gsram0ec: Gsram0ec,
    gsram0f0: Gsram0f0,
    gsram0f4: Gsram0f4,
    gsram0f8: Gsram0f8,
    gsram0fc: Gsram0fc,
    gsram100: Gsram100,
    gsram104: Gsram104,
    gsram108: Gsram108,
    gsram10c: Gsram10c,
    gsram110: Gsram110,
    gsram114: Gsram114,
    gsram118: Gsram118,
    gsram11c: Gsram11c,
    gsram120: Gsram120,
    gsram124: Gsram124,
    gsram128: Gsram128,
    gsram12c: Gsram12c,
    gsram130: Gsram130,
    gsram134: Gsram134,
    gsram138: Gsram138,
    gsram13c: Gsram13c,
    gsram140: Gsram140,
    gsram144: Gsram144,
    gsram148: Gsram148,
    gsram14c: Gsram14c,
    gsram150: Gsram150,
    gsram154: Gsram154,
    gsram158: Gsram158,
    gsram15c: Gsram15c,
    gsram160: Gsram160,
    gsram164: Gsram164,
    gsram168: Gsram168,
    gsram16c: Gsram16c,
    gsram170: Gsram170,
    gsram174: Gsram174,
    gsram178: Gsram178,
    gsram17c: Gsram17c,
}
impl RegisterBlock {
    #[doc = "0x00 - GSRAM\\_CTL"]
    #[inline(always)]
    pub const fn gsram000(&self) -> &Gsram000 {
        &self.gsram000
    }
    #[doc = "0x04 - GSRAM\\_INFO"]
    #[inline(always)]
    pub const fn gsram004(&self) -> &Gsram004 {
        &self.gsram004
    }
    #[doc = "0x08 - GSRAM\\_WP"]
    #[inline(always)]
    pub const fn gsram008(&self) -> &Gsram008 {
        &self.gsram008
    }
    #[doc = "0x0c - GSRAM\\_GWLOCK"]
    #[inline(always)]
    pub const fn gsram00c(&self) -> &Gsram00c {
        &self.gsram00c
    }
    #[doc = "0x10 - GSRAM\\_OADDR\\_BASE"]
    #[inline(always)]
    pub const fn gsram010(&self) -> &Gsram010 {
        &self.gsram010
    }
    #[doc = "0x14 - GSRAM\\_MAP\\_ADDR0"]
    #[inline(always)]
    pub const fn gsram014(&self) -> &Gsram014 {
        &self.gsram014
    }
    #[doc = "0x18 - GSRAM\\_MAP\\_ADDR1"]
    #[inline(always)]
    pub const fn gsram018(&self) -> &Gsram018 {
        &self.gsram018
    }
    #[doc = "0x80 - GSRAM\\_WLOCK00"]
    #[inline(always)]
    pub const fn gsram080(&self) -> &Gsram080 {
        &self.gsram080
    }
    #[doc = "0x84 - GSRAM\\_WLOCK01"]
    #[inline(always)]
    pub const fn gsram084(&self) -> &Gsram084 {
        &self.gsram084
    }
    #[doc = "0x88 - GSRAM\\_WLOCK02"]
    #[inline(always)]
    pub const fn gsram088(&self) -> &Gsram088 {
        &self.gsram088
    }
    #[doc = "0x8c - GSRAM\\_WLOCK03"]
    #[inline(always)]
    pub const fn gsram08c(&self) -> &Gsram08c {
        &self.gsram08c
    }
    #[doc = "0x90 - GSRAM\\_WLOCK04"]
    #[inline(always)]
    pub const fn gsram090(&self) -> &Gsram090 {
        &self.gsram090
    }
    #[doc = "0x94 - GSRAM\\_WLOCK05"]
    #[inline(always)]
    pub const fn gsram094(&self) -> &Gsram094 {
        &self.gsram094
    }
    #[doc = "0x98 - GSRAM\\_WLOCK06"]
    #[inline(always)]
    pub const fn gsram098(&self) -> &Gsram098 {
        &self.gsram098
    }
    #[doc = "0x9c - GSRAM\\_WLOCK07"]
    #[inline(always)]
    pub const fn gsram09c(&self) -> &Gsram09c {
        &self.gsram09c
    }
    #[doc = "0xa0 - GSRAM\\_WLOCK08"]
    #[inline(always)]
    pub const fn gsram0a0(&self) -> &Gsram0a0 {
        &self.gsram0a0
    }
    #[doc = "0xa4 - GSRAM\\_WLOCK09"]
    #[inline(always)]
    pub const fn gsram0a4(&self) -> &Gsram0a4 {
        &self.gsram0a4
    }
    #[doc = "0xa8 - GSRAM\\_WLOCK10"]
    #[inline(always)]
    pub const fn gsram0a8(&self) -> &Gsram0a8 {
        &self.gsram0a8
    }
    #[doc = "0xac - GSRAM\\_WLOCK11"]
    #[inline(always)]
    pub const fn gsram0ac(&self) -> &Gsram0ac {
        &self.gsram0ac
    }
    #[doc = "0xb0 - GSRAM\\_WLOCK12"]
    #[inline(always)]
    pub const fn gsram0b0(&self) -> &Gsram0b0 {
        &self.gsram0b0
    }
    #[doc = "0xb4 - GSRAM\\_WLOCK13"]
    #[inline(always)]
    pub const fn gsram0b4(&self) -> &Gsram0b4 {
        &self.gsram0b4
    }
    #[doc = "0xb8 - GSRAM\\_WLOCK14"]
    #[inline(always)]
    pub const fn gsram0b8(&self) -> &Gsram0b8 {
        &self.gsram0b8
    }
    #[doc = "0xbc - GSRAM\\_WLOCK15"]
    #[inline(always)]
    pub const fn gsram0bc(&self) -> &Gsram0bc {
        &self.gsram0bc
    }
    #[doc = "0xc0 - GSRAM\\_WLOCK16"]
    #[inline(always)]
    pub const fn gsram0c0(&self) -> &Gsram0c0 {
        &self.gsram0c0
    }
    #[doc = "0xc4 - GSRAM\\_WLOCK17"]
    #[inline(always)]
    pub const fn gsram0c4(&self) -> &Gsram0c4 {
        &self.gsram0c4
    }
    #[doc = "0xc8 - GSRAM\\_WLOCK18"]
    #[inline(always)]
    pub const fn gsram0c8(&self) -> &Gsram0c8 {
        &self.gsram0c8
    }
    #[doc = "0xcc - GSRAM\\_WLOCK19"]
    #[inline(always)]
    pub const fn gsram0cc(&self) -> &Gsram0cc {
        &self.gsram0cc
    }
    #[doc = "0xd0 - GSRAM\\_WLOCK20"]
    #[inline(always)]
    pub const fn gsram0d0(&self) -> &Gsram0d0 {
        &self.gsram0d0
    }
    #[doc = "0xd4 - GSRAM\\_WLOCK21"]
    #[inline(always)]
    pub const fn gsram0d4(&self) -> &Gsram0d4 {
        &self.gsram0d4
    }
    #[doc = "0xd8 - GSRAM\\_WLOCK22"]
    #[inline(always)]
    pub const fn gsram0d8(&self) -> &Gsram0d8 {
        &self.gsram0d8
    }
    #[doc = "0xdc - GSRAM\\_WLOCK23"]
    #[inline(always)]
    pub const fn gsram0dc(&self) -> &Gsram0dc {
        &self.gsram0dc
    }
    #[doc = "0xe0 - GSRAM\\_WLOCK24"]
    #[inline(always)]
    pub const fn gsram0e0(&self) -> &Gsram0e0 {
        &self.gsram0e0
    }
    #[doc = "0xe4 - GSRAM\\_WLOCK25"]
    #[inline(always)]
    pub const fn gsram0e4(&self) -> &Gsram0e4 {
        &self.gsram0e4
    }
    #[doc = "0xe8 - GSRAM\\_WLOCK26"]
    #[inline(always)]
    pub const fn gsram0e8(&self) -> &Gsram0e8 {
        &self.gsram0e8
    }
    #[doc = "0xec - GSRAM\\_WLOCK27"]
    #[inline(always)]
    pub const fn gsram0ec(&self) -> &Gsram0ec {
        &self.gsram0ec
    }
    #[doc = "0xf0 - GSRAM\\_WLOCK28"]
    #[inline(always)]
    pub const fn gsram0f0(&self) -> &Gsram0f0 {
        &self.gsram0f0
    }
    #[doc = "0xf4 - GSRAM\\_WLOCK29"]
    #[inline(always)]
    pub const fn gsram0f4(&self) -> &Gsram0f4 {
        &self.gsram0f4
    }
    #[doc = "0xf8 - GSRAM\\_WLOCK30"]
    #[inline(always)]
    pub const fn gsram0f8(&self) -> &Gsram0f8 {
        &self.gsram0f8
    }
    #[doc = "0xfc - GSRAM\\_WLOCK31"]
    #[inline(always)]
    pub const fn gsram0fc(&self) -> &Gsram0fc {
        &self.gsram0fc
    }
    #[doc = "0x100 - GSRAM\\_WLOCK32"]
    #[inline(always)]
    pub const fn gsram100(&self) -> &Gsram100 {
        &self.gsram100
    }
    #[doc = "0x104 - GSRAM\\_WLOCK33"]
    #[inline(always)]
    pub const fn gsram104(&self) -> &Gsram104 {
        &self.gsram104
    }
    #[doc = "0x108 - GSRAM\\_WLOCK34"]
    #[inline(always)]
    pub const fn gsram108(&self) -> &Gsram108 {
        &self.gsram108
    }
    #[doc = "0x10c - GSRAM\\_WLOCK35"]
    #[inline(always)]
    pub const fn gsram10c(&self) -> &Gsram10c {
        &self.gsram10c
    }
    #[doc = "0x110 - GSRAM\\_WLOCK36"]
    #[inline(always)]
    pub const fn gsram110(&self) -> &Gsram110 {
        &self.gsram110
    }
    #[doc = "0x114 - GSRAM\\_WLOCK37"]
    #[inline(always)]
    pub const fn gsram114(&self) -> &Gsram114 {
        &self.gsram114
    }
    #[doc = "0x118 - GSRAM\\_WLOCK38"]
    #[inline(always)]
    pub const fn gsram118(&self) -> &Gsram118 {
        &self.gsram118
    }
    #[doc = "0x11c - GSRAM\\_WLOCK39"]
    #[inline(always)]
    pub const fn gsram11c(&self) -> &Gsram11c {
        &self.gsram11c
    }
    #[doc = "0x120 - GSRAM\\_WLOCK40"]
    #[inline(always)]
    pub const fn gsram120(&self) -> &Gsram120 {
        &self.gsram120
    }
    #[doc = "0x124 - GSRAM\\_WLOCK41"]
    #[inline(always)]
    pub const fn gsram124(&self) -> &Gsram124 {
        &self.gsram124
    }
    #[doc = "0x128 - GSRAM\\_WLOCK42"]
    #[inline(always)]
    pub const fn gsram128(&self) -> &Gsram128 {
        &self.gsram128
    }
    #[doc = "0x12c - GSRAM\\_WLOCK43"]
    #[inline(always)]
    pub const fn gsram12c(&self) -> &Gsram12c {
        &self.gsram12c
    }
    #[doc = "0x130 - GSRAM\\_WLOCK44"]
    #[inline(always)]
    pub const fn gsram130(&self) -> &Gsram130 {
        &self.gsram130
    }
    #[doc = "0x134 - GSRAM\\_WLOCK45"]
    #[inline(always)]
    pub const fn gsram134(&self) -> &Gsram134 {
        &self.gsram134
    }
    #[doc = "0x138 - GSRAM\\_WLOCK46"]
    #[inline(always)]
    pub const fn gsram138(&self) -> &Gsram138 {
        &self.gsram138
    }
    #[doc = "0x13c - GSRAM\\_WLOCK47"]
    #[inline(always)]
    pub const fn gsram13c(&self) -> &Gsram13c {
        &self.gsram13c
    }
    #[doc = "0x140 - GSRAM\\_WLOCK48"]
    #[inline(always)]
    pub const fn gsram140(&self) -> &Gsram140 {
        &self.gsram140
    }
    #[doc = "0x144 - GSRAM\\_WLOCK49"]
    #[inline(always)]
    pub const fn gsram144(&self) -> &Gsram144 {
        &self.gsram144
    }
    #[doc = "0x148 - GSRAM\\_WLOCK50"]
    #[inline(always)]
    pub const fn gsram148(&self) -> &Gsram148 {
        &self.gsram148
    }
    #[doc = "0x14c - GSRAM\\_WLOCK51"]
    #[inline(always)]
    pub const fn gsram14c(&self) -> &Gsram14c {
        &self.gsram14c
    }
    #[doc = "0x150 - GSRAM\\_WLOCK52"]
    #[inline(always)]
    pub const fn gsram150(&self) -> &Gsram150 {
        &self.gsram150
    }
    #[doc = "0x154 - GSRAM\\_WLOCK53"]
    #[inline(always)]
    pub const fn gsram154(&self) -> &Gsram154 {
        &self.gsram154
    }
    #[doc = "0x158 - GSRAM\\_WLOCK54"]
    #[inline(always)]
    pub const fn gsram158(&self) -> &Gsram158 {
        &self.gsram158
    }
    #[doc = "0x15c - GSRAM\\_WLOCK55"]
    #[inline(always)]
    pub const fn gsram15c(&self) -> &Gsram15c {
        &self.gsram15c
    }
    #[doc = "0x160 - GSRAM\\_WLOCK56"]
    #[inline(always)]
    pub const fn gsram160(&self) -> &Gsram160 {
        &self.gsram160
    }
    #[doc = "0x164 - GSRAM\\_WLOCK57"]
    #[inline(always)]
    pub const fn gsram164(&self) -> &Gsram164 {
        &self.gsram164
    }
    #[doc = "0x168 - GSRAM\\_WLOCK58"]
    #[inline(always)]
    pub const fn gsram168(&self) -> &Gsram168 {
        &self.gsram168
    }
    #[doc = "0x16c - GSRAM\\_WLOCK59"]
    #[inline(always)]
    pub const fn gsram16c(&self) -> &Gsram16c {
        &self.gsram16c
    }
    #[doc = "0x170 - GSRAM\\_WLOCK60"]
    #[inline(always)]
    pub const fn gsram170(&self) -> &Gsram170 {
        &self.gsram170
    }
    #[doc = "0x174 - GSRAM\\_WLOCK61"]
    #[inline(always)]
    pub const fn gsram174(&self) -> &Gsram174 {
        &self.gsram174
    }
    #[doc = "0x178 - GSRAM\\_WLOCK62"]
    #[inline(always)]
    pub const fn gsram178(&self) -> &Gsram178 {
        &self.gsram178
    }
    #[doc = "0x17c - GSRAM\\_WLOCK63"]
    #[inline(always)]
    pub const fn gsram17c(&self) -> &Gsram17c {
        &self.gsram17c
    }
}
#[doc = "GSRAM000 (rw) register accessor: GSRAM\\_CTL\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram000::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram000::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram000`] module"]
#[doc(alias = "GSRAM000")]
pub type Gsram000 = crate::Reg<gsram000::Gsram000Spec>;
#[doc = "GSRAM\\_CTL"]
pub mod gsram000;
#[doc = "GSRAM004 (rw) register accessor: GSRAM\\_INFO\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram004::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram004::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram004`] module"]
#[doc(alias = "GSRAM004")]
pub type Gsram004 = crate::Reg<gsram004::Gsram004Spec>;
#[doc = "GSRAM\\_INFO"]
pub mod gsram004;
#[doc = "GSRAM008 (rw) register accessor: GSRAM\\_WP\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram008::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram008::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram008`] module"]
#[doc(alias = "GSRAM008")]
pub type Gsram008 = crate::Reg<gsram008::Gsram008Spec>;
#[doc = "GSRAM\\_WP"]
pub mod gsram008;
#[doc = "GSRAM00C (rw) register accessor: GSRAM\\_GWLOCK\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram00c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram00c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram00c`] module"]
#[doc(alias = "GSRAM00C")]
pub type Gsram00c = crate::Reg<gsram00c::Gsram00cSpec>;
#[doc = "GSRAM\\_GWLOCK"]
pub mod gsram00c;
#[doc = "GSRAM010 (rw) register accessor: GSRAM\\_OADDR\\_BASE\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram010::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram010::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram010`] module"]
#[doc(alias = "GSRAM010")]
pub type Gsram010 = crate::Reg<gsram010::Gsram010Spec>;
#[doc = "GSRAM\\_OADDR\\_BASE"]
pub mod gsram010;
#[doc = "GSRAM014 (rw) register accessor: GSRAM\\_MAP\\_ADDR0\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram014::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram014::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram014`] module"]
#[doc(alias = "GSRAM014")]
pub type Gsram014 = crate::Reg<gsram014::Gsram014Spec>;
#[doc = "GSRAM\\_MAP\\_ADDR0"]
pub mod gsram014;
#[doc = "GSRAM018 (rw) register accessor: GSRAM\\_MAP\\_ADDR1\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram018::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram018::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram018`] module"]
#[doc(alias = "GSRAM018")]
pub type Gsram018 = crate::Reg<gsram018::Gsram018Spec>;
#[doc = "GSRAM\\_MAP\\_ADDR1"]
pub mod gsram018;
#[doc = "GSRAM080 (rw) register accessor: GSRAM\\_WLOCK00\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram080::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram080::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram080`] module"]
#[doc(alias = "GSRAM080")]
pub type Gsram080 = crate::Reg<gsram080::Gsram080Spec>;
#[doc = "GSRAM\\_WLOCK00"]
pub mod gsram080;
#[doc = "GSRAM084 (rw) register accessor: GSRAM\\_WLOCK01\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram084::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram084::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram084`] module"]
#[doc(alias = "GSRAM084")]
pub type Gsram084 = crate::Reg<gsram084::Gsram084Spec>;
#[doc = "GSRAM\\_WLOCK01"]
pub mod gsram084;
#[doc = "GSRAM088 (rw) register accessor: GSRAM\\_WLOCK02\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram088::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram088::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram088`] module"]
#[doc(alias = "GSRAM088")]
pub type Gsram088 = crate::Reg<gsram088::Gsram088Spec>;
#[doc = "GSRAM\\_WLOCK02"]
pub mod gsram088;
#[doc = "GSRAM08C (rw) register accessor: GSRAM\\_WLOCK03\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram08c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram08c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram08c`] module"]
#[doc(alias = "GSRAM08C")]
pub type Gsram08c = crate::Reg<gsram08c::Gsram08cSpec>;
#[doc = "GSRAM\\_WLOCK03"]
pub mod gsram08c;
#[doc = "GSRAM090 (rw) register accessor: GSRAM\\_WLOCK04\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram090::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram090::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram090`] module"]
#[doc(alias = "GSRAM090")]
pub type Gsram090 = crate::Reg<gsram090::Gsram090Spec>;
#[doc = "GSRAM\\_WLOCK04"]
pub mod gsram090;
#[doc = "GSRAM094 (rw) register accessor: GSRAM\\_WLOCK05\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram094::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram094::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram094`] module"]
#[doc(alias = "GSRAM094")]
pub type Gsram094 = crate::Reg<gsram094::Gsram094Spec>;
#[doc = "GSRAM\\_WLOCK05"]
pub mod gsram094;
#[doc = "GSRAM098 (rw) register accessor: GSRAM\\_WLOCK06\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram098::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram098::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram098`] module"]
#[doc(alias = "GSRAM098")]
pub type Gsram098 = crate::Reg<gsram098::Gsram098Spec>;
#[doc = "GSRAM\\_WLOCK06"]
pub mod gsram098;
#[doc = "GSRAM09C (rw) register accessor: GSRAM\\_WLOCK07\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram09c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram09c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram09c`] module"]
#[doc(alias = "GSRAM09C")]
pub type Gsram09c = crate::Reg<gsram09c::Gsram09cSpec>;
#[doc = "GSRAM\\_WLOCK07"]
pub mod gsram09c;
#[doc = "GSRAM0A0 (rw) register accessor: GSRAM\\_WLOCK08\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram0a0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram0a0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram0a0`] module"]
#[doc(alias = "GSRAM0A0")]
pub type Gsram0a0 = crate::Reg<gsram0a0::Gsram0a0Spec>;
#[doc = "GSRAM\\_WLOCK08"]
pub mod gsram0a0;
#[doc = "GSRAM0A4 (rw) register accessor: GSRAM\\_WLOCK09\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram0a4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram0a4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram0a4`] module"]
#[doc(alias = "GSRAM0A4")]
pub type Gsram0a4 = crate::Reg<gsram0a4::Gsram0a4Spec>;
#[doc = "GSRAM\\_WLOCK09"]
pub mod gsram0a4;
#[doc = "GSRAM0A8 (rw) register accessor: GSRAM\\_WLOCK10\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram0a8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram0a8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram0a8`] module"]
#[doc(alias = "GSRAM0A8")]
pub type Gsram0a8 = crate::Reg<gsram0a8::Gsram0a8Spec>;
#[doc = "GSRAM\\_WLOCK10"]
pub mod gsram0a8;
#[doc = "GSRAM0AC (rw) register accessor: GSRAM\\_WLOCK11\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram0ac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram0ac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram0ac`] module"]
#[doc(alias = "GSRAM0AC")]
pub type Gsram0ac = crate::Reg<gsram0ac::Gsram0acSpec>;
#[doc = "GSRAM\\_WLOCK11"]
pub mod gsram0ac;
#[doc = "GSRAM0B0 (rw) register accessor: GSRAM\\_WLOCK12\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram0b0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram0b0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram0b0`] module"]
#[doc(alias = "GSRAM0B0")]
pub type Gsram0b0 = crate::Reg<gsram0b0::Gsram0b0Spec>;
#[doc = "GSRAM\\_WLOCK12"]
pub mod gsram0b0;
#[doc = "GSRAM0B4 (rw) register accessor: GSRAM\\_WLOCK13\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram0b4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram0b4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram0b4`] module"]
#[doc(alias = "GSRAM0B4")]
pub type Gsram0b4 = crate::Reg<gsram0b4::Gsram0b4Spec>;
#[doc = "GSRAM\\_WLOCK13"]
pub mod gsram0b4;
#[doc = "GSRAM0B8 (rw) register accessor: GSRAM\\_WLOCK14\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram0b8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram0b8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram0b8`] module"]
#[doc(alias = "GSRAM0B8")]
pub type Gsram0b8 = crate::Reg<gsram0b8::Gsram0b8Spec>;
#[doc = "GSRAM\\_WLOCK14"]
pub mod gsram0b8;
#[doc = "GSRAM0BC (rw) register accessor: GSRAM\\_WLOCK15\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram0bc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram0bc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram0bc`] module"]
#[doc(alias = "GSRAM0BC")]
pub type Gsram0bc = crate::Reg<gsram0bc::Gsram0bcSpec>;
#[doc = "GSRAM\\_WLOCK15"]
pub mod gsram0bc;
#[doc = "GSRAM0C0 (rw) register accessor: GSRAM\\_WLOCK16\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram0c0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram0c0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram0c0`] module"]
#[doc(alias = "GSRAM0C0")]
pub type Gsram0c0 = crate::Reg<gsram0c0::Gsram0c0Spec>;
#[doc = "GSRAM\\_WLOCK16"]
pub mod gsram0c0;
#[doc = "GSRAM0C4 (rw) register accessor: GSRAM\\_WLOCK17\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram0c4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram0c4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram0c4`] module"]
#[doc(alias = "GSRAM0C4")]
pub type Gsram0c4 = crate::Reg<gsram0c4::Gsram0c4Spec>;
#[doc = "GSRAM\\_WLOCK17"]
pub mod gsram0c4;
#[doc = "GSRAM0C8 (rw) register accessor: GSRAM\\_WLOCK18\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram0c8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram0c8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram0c8`] module"]
#[doc(alias = "GSRAM0C8")]
pub type Gsram0c8 = crate::Reg<gsram0c8::Gsram0c8Spec>;
#[doc = "GSRAM\\_WLOCK18"]
pub mod gsram0c8;
#[doc = "GSRAM0CC (rw) register accessor: GSRAM\\_WLOCK19\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram0cc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram0cc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram0cc`] module"]
#[doc(alias = "GSRAM0CC")]
pub type Gsram0cc = crate::Reg<gsram0cc::Gsram0ccSpec>;
#[doc = "GSRAM\\_WLOCK19"]
pub mod gsram0cc;
#[doc = "GSRAM0D0 (rw) register accessor: GSRAM\\_WLOCK20\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram0d0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram0d0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram0d0`] module"]
#[doc(alias = "GSRAM0D0")]
pub type Gsram0d0 = crate::Reg<gsram0d0::Gsram0d0Spec>;
#[doc = "GSRAM\\_WLOCK20"]
pub mod gsram0d0;
#[doc = "GSRAM0D4 (rw) register accessor: GSRAM\\_WLOCK21\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram0d4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram0d4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram0d4`] module"]
#[doc(alias = "GSRAM0D4")]
pub type Gsram0d4 = crate::Reg<gsram0d4::Gsram0d4Spec>;
#[doc = "GSRAM\\_WLOCK21"]
pub mod gsram0d4;
#[doc = "GSRAM0D8 (rw) register accessor: GSRAM\\_WLOCK22\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram0d8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram0d8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram0d8`] module"]
#[doc(alias = "GSRAM0D8")]
pub type Gsram0d8 = crate::Reg<gsram0d8::Gsram0d8Spec>;
#[doc = "GSRAM\\_WLOCK22"]
pub mod gsram0d8;
#[doc = "GSRAM0DC (rw) register accessor: GSRAM\\_WLOCK23\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram0dc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram0dc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram0dc`] module"]
#[doc(alias = "GSRAM0DC")]
pub type Gsram0dc = crate::Reg<gsram0dc::Gsram0dcSpec>;
#[doc = "GSRAM\\_WLOCK23"]
pub mod gsram0dc;
#[doc = "GSRAM0E0 (rw) register accessor: GSRAM\\_WLOCK24\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram0e0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram0e0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram0e0`] module"]
#[doc(alias = "GSRAM0E0")]
pub type Gsram0e0 = crate::Reg<gsram0e0::Gsram0e0Spec>;
#[doc = "GSRAM\\_WLOCK24"]
pub mod gsram0e0;
#[doc = "GSRAM0E4 (rw) register accessor: GSRAM\\_WLOCK25\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram0e4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram0e4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram0e4`] module"]
#[doc(alias = "GSRAM0E4")]
pub type Gsram0e4 = crate::Reg<gsram0e4::Gsram0e4Spec>;
#[doc = "GSRAM\\_WLOCK25"]
pub mod gsram0e4;
#[doc = "GSRAM0E8 (rw) register accessor: GSRAM\\_WLOCK26\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram0e8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram0e8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram0e8`] module"]
#[doc(alias = "GSRAM0E8")]
pub type Gsram0e8 = crate::Reg<gsram0e8::Gsram0e8Spec>;
#[doc = "GSRAM\\_WLOCK26"]
pub mod gsram0e8;
#[doc = "GSRAM0EC (rw) register accessor: GSRAM\\_WLOCK27\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram0ec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram0ec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram0ec`] module"]
#[doc(alias = "GSRAM0EC")]
pub type Gsram0ec = crate::Reg<gsram0ec::Gsram0ecSpec>;
#[doc = "GSRAM\\_WLOCK27"]
pub mod gsram0ec;
#[doc = "GSRAM0F0 (rw) register accessor: GSRAM\\_WLOCK28\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram0f0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram0f0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram0f0`] module"]
#[doc(alias = "GSRAM0F0")]
pub type Gsram0f0 = crate::Reg<gsram0f0::Gsram0f0Spec>;
#[doc = "GSRAM\\_WLOCK28"]
pub mod gsram0f0;
#[doc = "GSRAM0F4 (rw) register accessor: GSRAM\\_WLOCK29\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram0f4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram0f4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram0f4`] module"]
#[doc(alias = "GSRAM0F4")]
pub type Gsram0f4 = crate::Reg<gsram0f4::Gsram0f4Spec>;
#[doc = "GSRAM\\_WLOCK29"]
pub mod gsram0f4;
#[doc = "GSRAM0F8 (rw) register accessor: GSRAM\\_WLOCK30\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram0f8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram0f8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram0f8`] module"]
#[doc(alias = "GSRAM0F8")]
pub type Gsram0f8 = crate::Reg<gsram0f8::Gsram0f8Spec>;
#[doc = "GSRAM\\_WLOCK30"]
pub mod gsram0f8;
#[doc = "GSRAM0FC (rw) register accessor: GSRAM\\_WLOCK31\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram0fc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram0fc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram0fc`] module"]
#[doc(alias = "GSRAM0FC")]
pub type Gsram0fc = crate::Reg<gsram0fc::Gsram0fcSpec>;
#[doc = "GSRAM\\_WLOCK31"]
pub mod gsram0fc;
#[doc = "GSRAM100 (rw) register accessor: GSRAM\\_WLOCK32\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram100::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram100::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram100`] module"]
#[doc(alias = "GSRAM100")]
pub type Gsram100 = crate::Reg<gsram100::Gsram100Spec>;
#[doc = "GSRAM\\_WLOCK32"]
pub mod gsram100;
#[doc = "GSRAM104 (rw) register accessor: GSRAM\\_WLOCK33\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram104::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram104::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram104`] module"]
#[doc(alias = "GSRAM104")]
pub type Gsram104 = crate::Reg<gsram104::Gsram104Spec>;
#[doc = "GSRAM\\_WLOCK33"]
pub mod gsram104;
#[doc = "GSRAM108 (rw) register accessor: GSRAM\\_WLOCK34\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram108::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram108::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram108`] module"]
#[doc(alias = "GSRAM108")]
pub type Gsram108 = crate::Reg<gsram108::Gsram108Spec>;
#[doc = "GSRAM\\_WLOCK34"]
pub mod gsram108;
#[doc = "GSRAM10C (rw) register accessor: GSRAM\\_WLOCK35\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram10c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram10c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram10c`] module"]
#[doc(alias = "GSRAM10C")]
pub type Gsram10c = crate::Reg<gsram10c::Gsram10cSpec>;
#[doc = "GSRAM\\_WLOCK35"]
pub mod gsram10c;
#[doc = "GSRAM110 (rw) register accessor: GSRAM\\_WLOCK36\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram110::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram110::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram110`] module"]
#[doc(alias = "GSRAM110")]
pub type Gsram110 = crate::Reg<gsram110::Gsram110Spec>;
#[doc = "GSRAM\\_WLOCK36"]
pub mod gsram110;
#[doc = "GSRAM114 (rw) register accessor: GSRAM\\_WLOCK37\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram114::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram114::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram114`] module"]
#[doc(alias = "GSRAM114")]
pub type Gsram114 = crate::Reg<gsram114::Gsram114Spec>;
#[doc = "GSRAM\\_WLOCK37"]
pub mod gsram114;
#[doc = "GSRAM118 (rw) register accessor: GSRAM\\_WLOCK38\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram118::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram118::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram118`] module"]
#[doc(alias = "GSRAM118")]
pub type Gsram118 = crate::Reg<gsram118::Gsram118Spec>;
#[doc = "GSRAM\\_WLOCK38"]
pub mod gsram118;
#[doc = "GSRAM11C (rw) register accessor: GSRAM\\_WLOCK39\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram11c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram11c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram11c`] module"]
#[doc(alias = "GSRAM11C")]
pub type Gsram11c = crate::Reg<gsram11c::Gsram11cSpec>;
#[doc = "GSRAM\\_WLOCK39"]
pub mod gsram11c;
#[doc = "GSRAM120 (rw) register accessor: GSRAM\\_WLOCK40\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram120::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram120::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram120`] module"]
#[doc(alias = "GSRAM120")]
pub type Gsram120 = crate::Reg<gsram120::Gsram120Spec>;
#[doc = "GSRAM\\_WLOCK40"]
pub mod gsram120;
#[doc = "GSRAM124 (rw) register accessor: GSRAM\\_WLOCK41\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram124::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram124::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram124`] module"]
#[doc(alias = "GSRAM124")]
pub type Gsram124 = crate::Reg<gsram124::Gsram124Spec>;
#[doc = "GSRAM\\_WLOCK41"]
pub mod gsram124;
#[doc = "GSRAM128 (rw) register accessor: GSRAM\\_WLOCK42\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram128::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram128::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram128`] module"]
#[doc(alias = "GSRAM128")]
pub type Gsram128 = crate::Reg<gsram128::Gsram128Spec>;
#[doc = "GSRAM\\_WLOCK42"]
pub mod gsram128;
#[doc = "GSRAM12C (rw) register accessor: GSRAM\\_WLOCK43\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram12c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram12c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram12c`] module"]
#[doc(alias = "GSRAM12C")]
pub type Gsram12c = crate::Reg<gsram12c::Gsram12cSpec>;
#[doc = "GSRAM\\_WLOCK43"]
pub mod gsram12c;
#[doc = "GSRAM130 (rw) register accessor: GSRAM\\_WLOCK44\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram130::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram130::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram130`] module"]
#[doc(alias = "GSRAM130")]
pub type Gsram130 = crate::Reg<gsram130::Gsram130Spec>;
#[doc = "GSRAM\\_WLOCK44"]
pub mod gsram130;
#[doc = "GSRAM134 (rw) register accessor: GSRAM\\_WLOCK45\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram134::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram134::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram134`] module"]
#[doc(alias = "GSRAM134")]
pub type Gsram134 = crate::Reg<gsram134::Gsram134Spec>;
#[doc = "GSRAM\\_WLOCK45"]
pub mod gsram134;
#[doc = "GSRAM138 (rw) register accessor: GSRAM\\_WLOCK46\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram138::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram138::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram138`] module"]
#[doc(alias = "GSRAM138")]
pub type Gsram138 = crate::Reg<gsram138::Gsram138Spec>;
#[doc = "GSRAM\\_WLOCK46"]
pub mod gsram138;
#[doc = "GSRAM13C (rw) register accessor: GSRAM\\_WLOCK47\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram13c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram13c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram13c`] module"]
#[doc(alias = "GSRAM13C")]
pub type Gsram13c = crate::Reg<gsram13c::Gsram13cSpec>;
#[doc = "GSRAM\\_WLOCK47"]
pub mod gsram13c;
#[doc = "GSRAM140 (rw) register accessor: GSRAM\\_WLOCK48\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram140::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram140::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram140`] module"]
#[doc(alias = "GSRAM140")]
pub type Gsram140 = crate::Reg<gsram140::Gsram140Spec>;
#[doc = "GSRAM\\_WLOCK48"]
pub mod gsram140;
#[doc = "GSRAM144 (rw) register accessor: GSRAM\\_WLOCK49\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram144::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram144::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram144`] module"]
#[doc(alias = "GSRAM144")]
pub type Gsram144 = crate::Reg<gsram144::Gsram144Spec>;
#[doc = "GSRAM\\_WLOCK49"]
pub mod gsram144;
#[doc = "GSRAM148 (rw) register accessor: GSRAM\\_WLOCK50\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram148::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram148::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram148`] module"]
#[doc(alias = "GSRAM148")]
pub type Gsram148 = crate::Reg<gsram148::Gsram148Spec>;
#[doc = "GSRAM\\_WLOCK50"]
pub mod gsram148;
#[doc = "GSRAM14C (rw) register accessor: GSRAM\\_WLOCK51\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram14c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram14c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram14c`] module"]
#[doc(alias = "GSRAM14C")]
pub type Gsram14c = crate::Reg<gsram14c::Gsram14cSpec>;
#[doc = "GSRAM\\_WLOCK51"]
pub mod gsram14c;
#[doc = "GSRAM150 (rw) register accessor: GSRAM\\_WLOCK52\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram150::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram150::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram150`] module"]
#[doc(alias = "GSRAM150")]
pub type Gsram150 = crate::Reg<gsram150::Gsram150Spec>;
#[doc = "GSRAM\\_WLOCK52"]
pub mod gsram150;
#[doc = "GSRAM154 (rw) register accessor: GSRAM\\_WLOCK53\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram154::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram154::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram154`] module"]
#[doc(alias = "GSRAM154")]
pub type Gsram154 = crate::Reg<gsram154::Gsram154Spec>;
#[doc = "GSRAM\\_WLOCK53"]
pub mod gsram154;
#[doc = "GSRAM158 (rw) register accessor: GSRAM\\_WLOCK54\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram158::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram158::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram158`] module"]
#[doc(alias = "GSRAM158")]
pub type Gsram158 = crate::Reg<gsram158::Gsram158Spec>;
#[doc = "GSRAM\\_WLOCK54"]
pub mod gsram158;
#[doc = "GSRAM15C (rw) register accessor: GSRAM\\_WLOCK55\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram15c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram15c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram15c`] module"]
#[doc(alias = "GSRAM15C")]
pub type Gsram15c = crate::Reg<gsram15c::Gsram15cSpec>;
#[doc = "GSRAM\\_WLOCK55"]
pub mod gsram15c;
#[doc = "GSRAM160 (rw) register accessor: GSRAM\\_WLOCK56\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram160::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram160::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram160`] module"]
#[doc(alias = "GSRAM160")]
pub type Gsram160 = crate::Reg<gsram160::Gsram160Spec>;
#[doc = "GSRAM\\_WLOCK56"]
pub mod gsram160;
#[doc = "GSRAM164 (rw) register accessor: GSRAM\\_WLOCK57\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram164::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram164::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram164`] module"]
#[doc(alias = "GSRAM164")]
pub type Gsram164 = crate::Reg<gsram164::Gsram164Spec>;
#[doc = "GSRAM\\_WLOCK57"]
pub mod gsram164;
#[doc = "GSRAM168 (rw) register accessor: GSRAM\\_WLOCK58\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram168::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram168::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram168`] module"]
#[doc(alias = "GSRAM168")]
pub type Gsram168 = crate::Reg<gsram168::Gsram168Spec>;
#[doc = "GSRAM\\_WLOCK58"]
pub mod gsram168;
#[doc = "GSRAM16C (rw) register accessor: GSRAM\\_WLOCK59\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram16c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram16c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram16c`] module"]
#[doc(alias = "GSRAM16C")]
pub type Gsram16c = crate::Reg<gsram16c::Gsram16cSpec>;
#[doc = "GSRAM\\_WLOCK59"]
pub mod gsram16c;
#[doc = "GSRAM170 (rw) register accessor: GSRAM\\_WLOCK60\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram170::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram170::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram170`] module"]
#[doc(alias = "GSRAM170")]
pub type Gsram170 = crate::Reg<gsram170::Gsram170Spec>;
#[doc = "GSRAM\\_WLOCK60"]
pub mod gsram170;
#[doc = "GSRAM174 (rw) register accessor: GSRAM\\_WLOCK61\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram174::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram174::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram174`] module"]
#[doc(alias = "GSRAM174")]
pub type Gsram174 = crate::Reg<gsram174::Gsram174Spec>;
#[doc = "GSRAM\\_WLOCK61"]
pub mod gsram174;
#[doc = "GSRAM178 (rw) register accessor: GSRAM\\_WLOCK62\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram178::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram178::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram178`] module"]
#[doc(alias = "GSRAM178")]
pub type Gsram178 = crate::Reg<gsram178::Gsram178Spec>;
#[doc = "GSRAM\\_WLOCK62"]
pub mod gsram178;
#[doc = "GSRAM17C (rw) register accessor: GSRAM\\_WLOCK63\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram17c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram17c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsram17c`] module"]
#[doc(alias = "GSRAM17C")]
pub type Gsram17c = crate::Reg<gsram17c::Gsram17cSpec>;
#[doc = "GSRAM\\_WLOCK63"]
pub mod gsram17c;
