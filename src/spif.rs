#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    spif000: Spif000,
    spif004: Spif004,
    spif008: Spif008,
    spif00c: Spif00c,
    spif010: Spif010,
    spif014: Spif014,
    spif018: Spif018,
    spif01c: Spif01c,
    spif020: Spif020,
    _reserved9: [u8; 0x1c],
    spif040: Spif040,
    spif044: Spif044,
    spif048: Spif048,
    spif04c: Spif04c,
    spif050: Spif050,
    spif054: Spif054,
    spif058: Spif058,
    spif05c: Spif05c,
    _reserved17: [u8; 0x1c],
    spif07c: Spif07c,
    spif080: Spif080,
    spif084: Spif084,
    spif088: Spif088,
    spif08c: Spif08c,
    spif090: Spif090,
    spif094: Spif094,
    spif098: Spif098,
    spif09c: Spif09c,
    spif0a0: Spif0a0,
    spif0a4: Spif0a4,
    spif0a8: Spif0a8,
    spif0ac: Spif0ac,
    spif0b0: Spif0b0,
    spif0b4: Spif0b4,
    spif0b8: Spif0b8,
    spif0bc: Spif0bc,
    spif0c0: Spif0c0,
    spif0c4: Spif0c4,
    spif0c8: Spif0c8,
    spif0cc: Spif0cc,
    spif0d0: Spif0d0,
    spif0d4: Spif0d4,
    spif0d8: Spif0d8,
    spif0dc: Spif0dc,
    spif0e0: Spif0e0,
    spif0e4: Spif0e4,
    spif0e8: Spif0e8,
    spif0ec: Spif0ec,
    spif0f0: Spif0f0,
    spif0f4: Spif0f4,
    spif0f8: Spif0f8,
    spif0fc: Spif0fc,
    spif100: Spif100,
    spif104: Spif104,
    spif108: Spif108,
    spif10c: Spif10c,
    spif110: Spif110,
    spif114: Spif114,
    spif118: Spif118,
    spif11c: Spif11c,
    spif120: Spif120,
    spif124: Spif124,
    spif128: Spif128,
    spif12c: Spif12c,
    spif130: Spif130,
    spif134: Spif134,
    spif138: Spif138,
    spif13c: Spif13c,
    _reserved66: [u8; 0xc0],
    spif200: Spif200,
    spif204: Spif204,
    spif208: Spif208,
    spif20c: Spif20c,
    spif210: Spif210,
    spif214: Spif214,
    spif218: Spif218,
    spif21c: Spif21c,
    spif220: Spif220,
    spif224: Spif224,
    spif228: Spif228,
    spif22c: Spif22c,
    spif230: Spif230,
    spif234: Spif234,
    spif238: Spif238,
    spif23c: Spif23c,
    _reserved82: [u8; 0xc0],
    spif300: Spif300,
    _reserved83: [u8; 0x0c],
    spif310: Spif310,
}
impl RegisterBlock {
    #[doc = "0x00 - SPIF\\_CFG"]
    #[inline(always)]
    pub const fn spif000(&self) -> &Spif000 {
        &self.spif000
    }
    #[doc = "0x04 - SPIF\\_IRQ"]
    #[inline(always)]
    pub const fn spif004(&self) -> &Spif004 {
        &self.spif004
    }
    #[doc = "0x08 - SPIF\\_OS"]
    #[inline(always)]
    pub const fn spif008(&self) -> &Spif008 {
        &self.spif008
    }
    #[doc = "0x0c - SPIF\\_ELOG\\_CNT"]
    #[inline(always)]
    pub const fn spif00c(&self) -> &Spif00c {
        &self.spif00c
    }
    #[doc = "0x10 - SPIF\\_CSBASE0"]
    #[inline(always)]
    pub const fn spif010(&self) -> &Spif010 {
        &self.spif010
    }
    #[doc = "0x14 - SPIF\\_CSBASE1"]
    #[inline(always)]
    pub const fn spif014(&self) -> &Spif014 {
        &self.spif014
    }
    #[doc = "0x18 - SPIF\\_CSBASE2"]
    #[inline(always)]
    pub const fn spif018(&self) -> &Spif018 {
        &self.spif018
    }
    #[doc = "0x1c - SPIF\\_CSBASE3"]
    #[inline(always)]
    pub const fn spif01c(&self) -> &Spif01c {
        &self.spif01c
    }
    #[doc = "0x20 - SPIF\\_WLOCKTB"]
    #[inline(always)]
    pub const fn spif020(&self) -> &Spif020 {
        &self.spif020
    }
    #[doc = "0x40 - SPIF\\_ELOG00"]
    #[inline(always)]
    pub const fn spif040(&self) -> &Spif040 {
        &self.spif040
    }
    #[doc = "0x44 - SPIF\\_ELOG01"]
    #[inline(always)]
    pub const fn spif044(&self) -> &Spif044 {
        &self.spif044
    }
    #[doc = "0x48 - SPIF\\_ELOG02"]
    #[inline(always)]
    pub const fn spif048(&self) -> &Spif048 {
        &self.spif048
    }
    #[doc = "0x4c - SPIF\\_ELOG03"]
    #[inline(always)]
    pub const fn spif04c(&self) -> &Spif04c {
        &self.spif04c
    }
    #[doc = "0x50 - SPIF\\_ELOG04"]
    #[inline(always)]
    pub const fn spif050(&self) -> &Spif050 {
        &self.spif050
    }
    #[doc = "0x54 - SPIF\\_ELOG05"]
    #[inline(always)]
    pub const fn spif054(&self) -> &Spif054 {
        &self.spif054
    }
    #[doc = "0x58 - SPIF\\_ELOG06"]
    #[inline(always)]
    pub const fn spif058(&self) -> &Spif058 {
        &self.spif058
    }
    #[doc = "0x5c - SPIF\\_ELOG07"]
    #[inline(always)]
    pub const fn spif05c(&self) -> &Spif05c {
        &self.spif05c
    }
    #[doc = "0x7c - SPIF\\_WLOCK"]
    #[inline(always)]
    pub const fn spif07c(&self) -> &Spif07c {
        &self.spif07c
    }
    #[doc = "0x80 - SPIF\\_WTABLE00"]
    #[inline(always)]
    pub const fn spif080(&self) -> &Spif080 {
        &self.spif080
    }
    #[doc = "0x84 - SPIF\\_WTABLE01"]
    #[inline(always)]
    pub const fn spif084(&self) -> &Spif084 {
        &self.spif084
    }
    #[doc = "0x88 - SPIF\\_WTABLE02"]
    #[inline(always)]
    pub const fn spif088(&self) -> &Spif088 {
        &self.spif088
    }
    #[doc = "0x8c - SPIF\\_WTABLE03"]
    #[inline(always)]
    pub const fn spif08c(&self) -> &Spif08c {
        &self.spif08c
    }
    #[doc = "0x90 - SPIF\\_WTABLE04"]
    #[inline(always)]
    pub const fn spif090(&self) -> &Spif090 {
        &self.spif090
    }
    #[doc = "0x94 - SPIF\\_WTABLE05"]
    #[inline(always)]
    pub const fn spif094(&self) -> &Spif094 {
        &self.spif094
    }
    #[doc = "0x98 - SPIF\\_WTABLE06"]
    #[inline(always)]
    pub const fn spif098(&self) -> &Spif098 {
        &self.spif098
    }
    #[doc = "0x9c - SPIF\\_WTABLE07"]
    #[inline(always)]
    pub const fn spif09c(&self) -> &Spif09c {
        &self.spif09c
    }
    #[doc = "0xa0 - SPIF\\_WTABLE08"]
    #[inline(always)]
    pub const fn spif0a0(&self) -> &Spif0a0 {
        &self.spif0a0
    }
    #[doc = "0xa4 - SPIF\\_WTABLE09"]
    #[inline(always)]
    pub const fn spif0a4(&self) -> &Spif0a4 {
        &self.spif0a4
    }
    #[doc = "0xa8 - SPIF\\_WTABLE10"]
    #[inline(always)]
    pub const fn spif0a8(&self) -> &Spif0a8 {
        &self.spif0a8
    }
    #[doc = "0xac - SPIF\\_WTABLE11"]
    #[inline(always)]
    pub const fn spif0ac(&self) -> &Spif0ac {
        &self.spif0ac
    }
    #[doc = "0xb0 - SPIF\\_WTABLE12"]
    #[inline(always)]
    pub const fn spif0b0(&self) -> &Spif0b0 {
        &self.spif0b0
    }
    #[doc = "0xb4 - SPIF\\_WTABLE13"]
    #[inline(always)]
    pub const fn spif0b4(&self) -> &Spif0b4 {
        &self.spif0b4
    }
    #[doc = "0xb8 - SPIF\\_WTABLE14"]
    #[inline(always)]
    pub const fn spif0b8(&self) -> &Spif0b8 {
        &self.spif0b8
    }
    #[doc = "0xbc - SPIF\\_WTABLE15"]
    #[inline(always)]
    pub const fn spif0bc(&self) -> &Spif0bc {
        &self.spif0bc
    }
    #[doc = "0xc0 - SPIF\\_WTABLE16"]
    #[inline(always)]
    pub const fn spif0c0(&self) -> &Spif0c0 {
        &self.spif0c0
    }
    #[doc = "0xc4 - SPIF\\_WTABLE17"]
    #[inline(always)]
    pub const fn spif0c4(&self) -> &Spif0c4 {
        &self.spif0c4
    }
    #[doc = "0xc8 - SPIF\\_WTABLE18"]
    #[inline(always)]
    pub const fn spif0c8(&self) -> &Spif0c8 {
        &self.spif0c8
    }
    #[doc = "0xcc - SPIF\\_WTABLE19"]
    #[inline(always)]
    pub const fn spif0cc(&self) -> &Spif0cc {
        &self.spif0cc
    }
    #[doc = "0xd0 - SPIF\\_WTABLE20"]
    #[inline(always)]
    pub const fn spif0d0(&self) -> &Spif0d0 {
        &self.spif0d0
    }
    #[doc = "0xd4 - SPIF\\_WTABLE21"]
    #[inline(always)]
    pub const fn spif0d4(&self) -> &Spif0d4 {
        &self.spif0d4
    }
    #[doc = "0xd8 - SPIF\\_WTABLE22"]
    #[inline(always)]
    pub const fn spif0d8(&self) -> &Spif0d8 {
        &self.spif0d8
    }
    #[doc = "0xdc - SPIF\\_WTABLE23"]
    #[inline(always)]
    pub const fn spif0dc(&self) -> &Spif0dc {
        &self.spif0dc
    }
    #[doc = "0xe0 - SPIF\\_WTABLE24"]
    #[inline(always)]
    pub const fn spif0e0(&self) -> &Spif0e0 {
        &self.spif0e0
    }
    #[doc = "0xe4 - SPIF\\_WTABLE25"]
    #[inline(always)]
    pub const fn spif0e4(&self) -> &Spif0e4 {
        &self.spif0e4
    }
    #[doc = "0xe8 - SPIF\\_WTABLE26"]
    #[inline(always)]
    pub const fn spif0e8(&self) -> &Spif0e8 {
        &self.spif0e8
    }
    #[doc = "0xec - SPIF\\_WTABLE27"]
    #[inline(always)]
    pub const fn spif0ec(&self) -> &Spif0ec {
        &self.spif0ec
    }
    #[doc = "0xf0 - SPIF\\_WTABLE28"]
    #[inline(always)]
    pub const fn spif0f0(&self) -> &Spif0f0 {
        &self.spif0f0
    }
    #[doc = "0xf4 - SPIF\\_WTABLE29"]
    #[inline(always)]
    pub const fn spif0f4(&self) -> &Spif0f4 {
        &self.spif0f4
    }
    #[doc = "0xf8 - SPIF\\_WTABLE30"]
    #[inline(always)]
    pub const fn spif0f8(&self) -> &Spif0f8 {
        &self.spif0f8
    }
    #[doc = "0xfc - SPIF\\_WTABLE31"]
    #[inline(always)]
    pub const fn spif0fc(&self) -> &Spif0fc {
        &self.spif0fc
    }
    #[doc = "0x100 - SPIF\\_ADDRCTL00"]
    #[inline(always)]
    pub const fn spif100(&self) -> &Spif100 {
        &self.spif100
    }
    #[doc = "0x104 - SPIF\\_ADDRCTL01"]
    #[inline(always)]
    pub const fn spif104(&self) -> &Spif104 {
        &self.spif104
    }
    #[doc = "0x108 - SPIF\\_ADDRCTL02"]
    #[inline(always)]
    pub const fn spif108(&self) -> &Spif108 {
        &self.spif108
    }
    #[doc = "0x10c - SPIF\\_ADDRCTL03"]
    #[inline(always)]
    pub const fn spif10c(&self) -> &Spif10c {
        &self.spif10c
    }
    #[doc = "0x110 - SPIF\\_ADDRCTL04"]
    #[inline(always)]
    pub const fn spif110(&self) -> &Spif110 {
        &self.spif110
    }
    #[doc = "0x114 - SPIF\\_ADDRCTL05"]
    #[inline(always)]
    pub const fn spif114(&self) -> &Spif114 {
        &self.spif114
    }
    #[doc = "0x118 - SPIF\\_ADDRCTL06"]
    #[inline(always)]
    pub const fn spif118(&self) -> &Spif118 {
        &self.spif118
    }
    #[doc = "0x11c - SPIF\\_ADDRCTL07"]
    #[inline(always)]
    pub const fn spif11c(&self) -> &Spif11c {
        &self.spif11c
    }
    #[doc = "0x120 - SPIF\\_ADDRCTL08"]
    #[inline(always)]
    pub const fn spif120(&self) -> &Spif120 {
        &self.spif120
    }
    #[doc = "0x124 - SPIF\\_ADDRCTL09"]
    #[inline(always)]
    pub const fn spif124(&self) -> &Spif124 {
        &self.spif124
    }
    #[doc = "0x128 - SPIF\\_ADDRCTL10"]
    #[inline(always)]
    pub const fn spif128(&self) -> &Spif128 {
        &self.spif128
    }
    #[doc = "0x12c - SPIF\\_ADDRCTL11"]
    #[inline(always)]
    pub const fn spif12c(&self) -> &Spif12c {
        &self.spif12c
    }
    #[doc = "0x130 - SPIF\\_ADDRCTL12"]
    #[inline(always)]
    pub const fn spif130(&self) -> &Spif130 {
        &self.spif130
    }
    #[doc = "0x134 - SPIF\\_ADDRCTL13"]
    #[inline(always)]
    pub const fn spif134(&self) -> &Spif134 {
        &self.spif134
    }
    #[doc = "0x138 - SPIF\\_ADDRCTL14"]
    #[inline(always)]
    pub const fn spif138(&self) -> &Spif138 {
        &self.spif138
    }
    #[doc = "0x13c - SPIF\\_ADDRCTL15"]
    #[inline(always)]
    pub const fn spif13c(&self) -> &Spif13c {
        &self.spif13c
    }
    #[doc = "0x200 - SPIF\\_ADDRBND00"]
    #[inline(always)]
    pub const fn spif200(&self) -> &Spif200 {
        &self.spif200
    }
    #[doc = "0x204 - SPIF\\_ADDRBND01"]
    #[inline(always)]
    pub const fn spif204(&self) -> &Spif204 {
        &self.spif204
    }
    #[doc = "0x208 - SPIF\\_ADDRBND02"]
    #[inline(always)]
    pub const fn spif208(&self) -> &Spif208 {
        &self.spif208
    }
    #[doc = "0x20c - SPIF\\_ADDRBND03"]
    #[inline(always)]
    pub const fn spif20c(&self) -> &Spif20c {
        &self.spif20c
    }
    #[doc = "0x210 - SPIF\\_ADDRBND04"]
    #[inline(always)]
    pub const fn spif210(&self) -> &Spif210 {
        &self.spif210
    }
    #[doc = "0x214 - SPIF\\_ADDRBND05"]
    #[inline(always)]
    pub const fn spif214(&self) -> &Spif214 {
        &self.spif214
    }
    #[doc = "0x218 - SPIF\\_ADDRBND06"]
    #[inline(always)]
    pub const fn spif218(&self) -> &Spif218 {
        &self.spif218
    }
    #[doc = "0x21c - SPIF\\_ADDRBND07"]
    #[inline(always)]
    pub const fn spif21c(&self) -> &Spif21c {
        &self.spif21c
    }
    #[doc = "0x220 - SPIF\\_ADDRBND08"]
    #[inline(always)]
    pub const fn spif220(&self) -> &Spif220 {
        &self.spif220
    }
    #[doc = "0x224 - SPIF\\_ADDRBND09"]
    #[inline(always)]
    pub const fn spif224(&self) -> &Spif224 {
        &self.spif224
    }
    #[doc = "0x228 - SPIF\\_ADDRBND10"]
    #[inline(always)]
    pub const fn spif228(&self) -> &Spif228 {
        &self.spif228
    }
    #[doc = "0x22c - SPIF\\_ADDRBND11"]
    #[inline(always)]
    pub const fn spif22c(&self) -> &Spif22c {
        &self.spif22c
    }
    #[doc = "0x230 - SPIF\\_ADDRBND12"]
    #[inline(always)]
    pub const fn spif230(&self) -> &Spif230 {
        &self.spif230
    }
    #[doc = "0x234 - SPIF\\_ADDRBND13"]
    #[inline(always)]
    pub const fn spif234(&self) -> &Spif234 {
        &self.spif234
    }
    #[doc = "0x238 - SPIF\\_ADDRBND14"]
    #[inline(always)]
    pub const fn spif238(&self) -> &Spif238 {
        &self.spif238
    }
    #[doc = "0x23c - SPIF\\_ADDRBND15"]
    #[inline(always)]
    pub const fn spif23c(&self) -> &Spif23c {
        &self.spif23c
    }
    #[doc = "0x300 - SPIF\\_WPWTABLE\\_EN"]
    #[inline(always)]
    pub const fn spif300(&self) -> &Spif300 {
        &self.spif300
    }
    #[doc = "0x310 - SPIF\\_WPADDRCTL"]
    #[inline(always)]
    pub const fn spif310(&self) -> &Spif310 {
        &self.spif310
    }
}
#[doc = "SPIF000 (rw) register accessor: SPIF\\_CFG\n\nYou can [`read`](crate::Reg::read) this register and get [`spif000::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif000::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif000`] module"]
#[doc(alias = "SPIF000")]
pub type Spif000 = crate::Reg<spif000::Spif000Spec>;
#[doc = "SPIF\\_CFG"]
pub mod spif000;
#[doc = "SPIF004 (rw) register accessor: SPIF\\_IRQ\n\nYou can [`read`](crate::Reg::read) this register and get [`spif004::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif004::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif004`] module"]
#[doc(alias = "SPIF004")]
pub type Spif004 = crate::Reg<spif004::Spif004Spec>;
#[doc = "SPIF\\_IRQ"]
pub mod spif004;
#[doc = "SPIF008 (rw) register accessor: SPIF\\_OS\n\nYou can [`read`](crate::Reg::read) this register and get [`spif008::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif008::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif008`] module"]
#[doc(alias = "SPIF008")]
pub type Spif008 = crate::Reg<spif008::Spif008Spec>;
#[doc = "SPIF\\_OS"]
pub mod spif008;
#[doc = "SPIF00C (rw) register accessor: SPIF\\_ELOG\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`spif00c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif00c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif00c`] module"]
#[doc(alias = "SPIF00C")]
pub type Spif00c = crate::Reg<spif00c::Spif00cSpec>;
#[doc = "SPIF\\_ELOG\\_CNT"]
pub mod spif00c;
#[doc = "SPIF010 (rw) register accessor: SPIF\\_CSBASE0\n\nYou can [`read`](crate::Reg::read) this register and get [`spif010::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif010::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif010`] module"]
#[doc(alias = "SPIF010")]
pub type Spif010 = crate::Reg<spif010::Spif010Spec>;
#[doc = "SPIF\\_CSBASE0"]
pub mod spif010;
#[doc = "SPIF014 (rw) register accessor: SPIF\\_CSBASE1\n\nYou can [`read`](crate::Reg::read) this register and get [`spif014::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif014::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif014`] module"]
#[doc(alias = "SPIF014")]
pub type Spif014 = crate::Reg<spif014::Spif014Spec>;
#[doc = "SPIF\\_CSBASE1"]
pub mod spif014;
#[doc = "SPIF018 (rw) register accessor: SPIF\\_CSBASE2\n\nYou can [`read`](crate::Reg::read) this register and get [`spif018::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif018::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif018`] module"]
#[doc(alias = "SPIF018")]
pub type Spif018 = crate::Reg<spif018::Spif018Spec>;
#[doc = "SPIF\\_CSBASE2"]
pub mod spif018;
#[doc = "SPIF01C (rw) register accessor: SPIF\\_CSBASE3\n\nYou can [`read`](crate::Reg::read) this register and get [`spif01c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif01c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif01c`] module"]
#[doc(alias = "SPIF01C")]
pub type Spif01c = crate::Reg<spif01c::Spif01cSpec>;
#[doc = "SPIF\\_CSBASE3"]
pub mod spif01c;
#[doc = "SPIF020 (rw) register accessor: SPIF\\_WLOCKTB\n\nYou can [`read`](crate::Reg::read) this register and get [`spif020::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif020::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif020`] module"]
#[doc(alias = "SPIF020")]
pub type Spif020 = crate::Reg<spif020::Spif020Spec>;
#[doc = "SPIF\\_WLOCKTB"]
pub mod spif020;
#[doc = "SPIF040 (rw) register accessor: SPIF\\_ELOG00\n\nYou can [`read`](crate::Reg::read) this register and get [`spif040::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif040::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif040`] module"]
#[doc(alias = "SPIF040")]
pub type Spif040 = crate::Reg<spif040::Spif040Spec>;
#[doc = "SPIF\\_ELOG00"]
pub mod spif040;
#[doc = "SPIF044 (rw) register accessor: SPIF\\_ELOG01\n\nYou can [`read`](crate::Reg::read) this register and get [`spif044::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif044::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif044`] module"]
#[doc(alias = "SPIF044")]
pub type Spif044 = crate::Reg<spif044::Spif044Spec>;
#[doc = "SPIF\\_ELOG01"]
pub mod spif044;
#[doc = "SPIF048 (rw) register accessor: SPIF\\_ELOG02\n\nYou can [`read`](crate::Reg::read) this register and get [`spif048::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif048::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif048`] module"]
#[doc(alias = "SPIF048")]
pub type Spif048 = crate::Reg<spif048::Spif048Spec>;
#[doc = "SPIF\\_ELOG02"]
pub mod spif048;
#[doc = "SPIF04C (rw) register accessor: SPIF\\_ELOG03\n\nYou can [`read`](crate::Reg::read) this register and get [`spif04c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif04c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif04c`] module"]
#[doc(alias = "SPIF04C")]
pub type Spif04c = crate::Reg<spif04c::Spif04cSpec>;
#[doc = "SPIF\\_ELOG03"]
pub mod spif04c;
#[doc = "SPIF050 (rw) register accessor: SPIF\\_ELOG04\n\nYou can [`read`](crate::Reg::read) this register and get [`spif050::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif050::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif050`] module"]
#[doc(alias = "SPIF050")]
pub type Spif050 = crate::Reg<spif050::Spif050Spec>;
#[doc = "SPIF\\_ELOG04"]
pub mod spif050;
#[doc = "SPIF054 (rw) register accessor: SPIF\\_ELOG05\n\nYou can [`read`](crate::Reg::read) this register and get [`spif054::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif054::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif054`] module"]
#[doc(alias = "SPIF054")]
pub type Spif054 = crate::Reg<spif054::Spif054Spec>;
#[doc = "SPIF\\_ELOG05"]
pub mod spif054;
#[doc = "SPIF058 (rw) register accessor: SPIF\\_ELOG06\n\nYou can [`read`](crate::Reg::read) this register and get [`spif058::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif058::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif058`] module"]
#[doc(alias = "SPIF058")]
pub type Spif058 = crate::Reg<spif058::Spif058Spec>;
#[doc = "SPIF\\_ELOG06"]
pub mod spif058;
#[doc = "SPIF05C (rw) register accessor: SPIF\\_ELOG07\n\nYou can [`read`](crate::Reg::read) this register and get [`spif05c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif05c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif05c`] module"]
#[doc(alias = "SPIF05C")]
pub type Spif05c = crate::Reg<spif05c::Spif05cSpec>;
#[doc = "SPIF\\_ELOG07"]
pub mod spif05c;
#[doc = "SPIF07C (rw) register accessor: SPIF\\_WLOCK\n\nYou can [`read`](crate::Reg::read) this register and get [`spif07c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif07c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif07c`] module"]
#[doc(alias = "SPIF07C")]
pub type Spif07c = crate::Reg<spif07c::Spif07cSpec>;
#[doc = "SPIF\\_WLOCK"]
pub mod spif07c;
#[doc = "SPIF080 (rw) register accessor: SPIF\\_WTABLE00\n\nYou can [`read`](crate::Reg::read) this register and get [`spif080::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif080::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif080`] module"]
#[doc(alias = "SPIF080")]
pub type Spif080 = crate::Reg<spif080::Spif080Spec>;
#[doc = "SPIF\\_WTABLE00"]
pub mod spif080;
#[doc = "SPIF084 (rw) register accessor: SPIF\\_WTABLE01\n\nYou can [`read`](crate::Reg::read) this register and get [`spif084::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif084::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif084`] module"]
#[doc(alias = "SPIF084")]
pub type Spif084 = crate::Reg<spif084::Spif084Spec>;
#[doc = "SPIF\\_WTABLE01"]
pub mod spif084;
#[doc = "SPIF088 (rw) register accessor: SPIF\\_WTABLE02\n\nYou can [`read`](crate::Reg::read) this register and get [`spif088::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif088::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif088`] module"]
#[doc(alias = "SPIF088")]
pub type Spif088 = crate::Reg<spif088::Spif088Spec>;
#[doc = "SPIF\\_WTABLE02"]
pub mod spif088;
#[doc = "SPIF08C (rw) register accessor: SPIF\\_WTABLE03\n\nYou can [`read`](crate::Reg::read) this register and get [`spif08c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif08c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif08c`] module"]
#[doc(alias = "SPIF08C")]
pub type Spif08c = crate::Reg<spif08c::Spif08cSpec>;
#[doc = "SPIF\\_WTABLE03"]
pub mod spif08c;
#[doc = "SPIF090 (rw) register accessor: SPIF\\_WTABLE04\n\nYou can [`read`](crate::Reg::read) this register and get [`spif090::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif090::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif090`] module"]
#[doc(alias = "SPIF090")]
pub type Spif090 = crate::Reg<spif090::Spif090Spec>;
#[doc = "SPIF\\_WTABLE04"]
pub mod spif090;
#[doc = "SPIF094 (rw) register accessor: SPIF\\_WTABLE05\n\nYou can [`read`](crate::Reg::read) this register and get [`spif094::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif094::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif094`] module"]
#[doc(alias = "SPIF094")]
pub type Spif094 = crate::Reg<spif094::Spif094Spec>;
#[doc = "SPIF\\_WTABLE05"]
pub mod spif094;
#[doc = "SPIF098 (rw) register accessor: SPIF\\_WTABLE06\n\nYou can [`read`](crate::Reg::read) this register and get [`spif098::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif098::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif098`] module"]
#[doc(alias = "SPIF098")]
pub type Spif098 = crate::Reg<spif098::Spif098Spec>;
#[doc = "SPIF\\_WTABLE06"]
pub mod spif098;
#[doc = "SPIF09C (rw) register accessor: SPIF\\_WTABLE07\n\nYou can [`read`](crate::Reg::read) this register and get [`spif09c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif09c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif09c`] module"]
#[doc(alias = "SPIF09C")]
pub type Spif09c = crate::Reg<spif09c::Spif09cSpec>;
#[doc = "SPIF\\_WTABLE07"]
pub mod spif09c;
#[doc = "SPIF0A0 (rw) register accessor: SPIF\\_WTABLE08\n\nYou can [`read`](crate::Reg::read) this register and get [`spif0a0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif0a0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif0a0`] module"]
#[doc(alias = "SPIF0A0")]
pub type Spif0a0 = crate::Reg<spif0a0::Spif0a0Spec>;
#[doc = "SPIF\\_WTABLE08"]
pub mod spif0a0;
#[doc = "SPIF0A4 (rw) register accessor: SPIF\\_WTABLE09\n\nYou can [`read`](crate::Reg::read) this register and get [`spif0a4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif0a4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif0a4`] module"]
#[doc(alias = "SPIF0A4")]
pub type Spif0a4 = crate::Reg<spif0a4::Spif0a4Spec>;
#[doc = "SPIF\\_WTABLE09"]
pub mod spif0a4;
#[doc = "SPIF0A8 (rw) register accessor: SPIF\\_WTABLE10\n\nYou can [`read`](crate::Reg::read) this register and get [`spif0a8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif0a8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif0a8`] module"]
#[doc(alias = "SPIF0A8")]
pub type Spif0a8 = crate::Reg<spif0a8::Spif0a8Spec>;
#[doc = "SPIF\\_WTABLE10"]
pub mod spif0a8;
#[doc = "SPIF0AC (rw) register accessor: SPIF\\_WTABLE11\n\nYou can [`read`](crate::Reg::read) this register and get [`spif0ac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif0ac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif0ac`] module"]
#[doc(alias = "SPIF0AC")]
pub type Spif0ac = crate::Reg<spif0ac::Spif0acSpec>;
#[doc = "SPIF\\_WTABLE11"]
pub mod spif0ac;
#[doc = "SPIF0B0 (rw) register accessor: SPIF\\_WTABLE12\n\nYou can [`read`](crate::Reg::read) this register and get [`spif0b0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif0b0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif0b0`] module"]
#[doc(alias = "SPIF0B0")]
pub type Spif0b0 = crate::Reg<spif0b0::Spif0b0Spec>;
#[doc = "SPIF\\_WTABLE12"]
pub mod spif0b0;
#[doc = "SPIF0B4 (rw) register accessor: SPIF\\_WTABLE13\n\nYou can [`read`](crate::Reg::read) this register and get [`spif0b4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif0b4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif0b4`] module"]
#[doc(alias = "SPIF0B4")]
pub type Spif0b4 = crate::Reg<spif0b4::Spif0b4Spec>;
#[doc = "SPIF\\_WTABLE13"]
pub mod spif0b4;
#[doc = "SPIF0B8 (rw) register accessor: SPIF\\_WTABLE14\n\nYou can [`read`](crate::Reg::read) this register and get [`spif0b8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif0b8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif0b8`] module"]
#[doc(alias = "SPIF0B8")]
pub type Spif0b8 = crate::Reg<spif0b8::Spif0b8Spec>;
#[doc = "SPIF\\_WTABLE14"]
pub mod spif0b8;
#[doc = "SPIF0BC (rw) register accessor: SPIF\\_WTABLE15\n\nYou can [`read`](crate::Reg::read) this register and get [`spif0bc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif0bc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif0bc`] module"]
#[doc(alias = "SPIF0BC")]
pub type Spif0bc = crate::Reg<spif0bc::Spif0bcSpec>;
#[doc = "SPIF\\_WTABLE15"]
pub mod spif0bc;
#[doc = "SPIF0C0 (rw) register accessor: SPIF\\_WTABLE16\n\nYou can [`read`](crate::Reg::read) this register and get [`spif0c0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif0c0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif0c0`] module"]
#[doc(alias = "SPIF0C0")]
pub type Spif0c0 = crate::Reg<spif0c0::Spif0c0Spec>;
#[doc = "SPIF\\_WTABLE16"]
pub mod spif0c0;
#[doc = "SPIF0C4 (rw) register accessor: SPIF\\_WTABLE17\n\nYou can [`read`](crate::Reg::read) this register and get [`spif0c4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif0c4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif0c4`] module"]
#[doc(alias = "SPIF0C4")]
pub type Spif0c4 = crate::Reg<spif0c4::Spif0c4Spec>;
#[doc = "SPIF\\_WTABLE17"]
pub mod spif0c4;
#[doc = "SPIF0C8 (rw) register accessor: SPIF\\_WTABLE18\n\nYou can [`read`](crate::Reg::read) this register and get [`spif0c8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif0c8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif0c8`] module"]
#[doc(alias = "SPIF0C8")]
pub type Spif0c8 = crate::Reg<spif0c8::Spif0c8Spec>;
#[doc = "SPIF\\_WTABLE18"]
pub mod spif0c8;
#[doc = "SPIF0CC (rw) register accessor: SPIF\\_WTABLE19\n\nYou can [`read`](crate::Reg::read) this register and get [`spif0cc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif0cc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif0cc`] module"]
#[doc(alias = "SPIF0CC")]
pub type Spif0cc = crate::Reg<spif0cc::Spif0ccSpec>;
#[doc = "SPIF\\_WTABLE19"]
pub mod spif0cc;
#[doc = "SPIF0D0 (rw) register accessor: SPIF\\_WTABLE20\n\nYou can [`read`](crate::Reg::read) this register and get [`spif0d0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif0d0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif0d0`] module"]
#[doc(alias = "SPIF0D0")]
pub type Spif0d0 = crate::Reg<spif0d0::Spif0d0Spec>;
#[doc = "SPIF\\_WTABLE20"]
pub mod spif0d0;
#[doc = "SPIF0D4 (rw) register accessor: SPIF\\_WTABLE21\n\nYou can [`read`](crate::Reg::read) this register and get [`spif0d4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif0d4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif0d4`] module"]
#[doc(alias = "SPIF0D4")]
pub type Spif0d4 = crate::Reg<spif0d4::Spif0d4Spec>;
#[doc = "SPIF\\_WTABLE21"]
pub mod spif0d4;
#[doc = "SPIF0D8 (rw) register accessor: SPIF\\_WTABLE22\n\nYou can [`read`](crate::Reg::read) this register and get [`spif0d8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif0d8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif0d8`] module"]
#[doc(alias = "SPIF0D8")]
pub type Spif0d8 = crate::Reg<spif0d8::Spif0d8Spec>;
#[doc = "SPIF\\_WTABLE22"]
pub mod spif0d8;
#[doc = "SPIF0DC (rw) register accessor: SPIF\\_WTABLE23\n\nYou can [`read`](crate::Reg::read) this register and get [`spif0dc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif0dc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif0dc`] module"]
#[doc(alias = "SPIF0DC")]
pub type Spif0dc = crate::Reg<spif0dc::Spif0dcSpec>;
#[doc = "SPIF\\_WTABLE23"]
pub mod spif0dc;
#[doc = "SPIF0E0 (rw) register accessor: SPIF\\_WTABLE24\n\nYou can [`read`](crate::Reg::read) this register and get [`spif0e0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif0e0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif0e0`] module"]
#[doc(alias = "SPIF0E0")]
pub type Spif0e0 = crate::Reg<spif0e0::Spif0e0Spec>;
#[doc = "SPIF\\_WTABLE24"]
pub mod spif0e0;
#[doc = "SPIF0E4 (rw) register accessor: SPIF\\_WTABLE25\n\nYou can [`read`](crate::Reg::read) this register and get [`spif0e4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif0e4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif0e4`] module"]
#[doc(alias = "SPIF0E4")]
pub type Spif0e4 = crate::Reg<spif0e4::Spif0e4Spec>;
#[doc = "SPIF\\_WTABLE25"]
pub mod spif0e4;
#[doc = "SPIF0E8 (rw) register accessor: SPIF\\_WTABLE26\n\nYou can [`read`](crate::Reg::read) this register and get [`spif0e8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif0e8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif0e8`] module"]
#[doc(alias = "SPIF0E8")]
pub type Spif0e8 = crate::Reg<spif0e8::Spif0e8Spec>;
#[doc = "SPIF\\_WTABLE26"]
pub mod spif0e8;
#[doc = "SPIF0EC (rw) register accessor: SPIF\\_WTABLE27\n\nYou can [`read`](crate::Reg::read) this register and get [`spif0ec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif0ec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif0ec`] module"]
#[doc(alias = "SPIF0EC")]
pub type Spif0ec = crate::Reg<spif0ec::Spif0ecSpec>;
#[doc = "SPIF\\_WTABLE27"]
pub mod spif0ec;
#[doc = "SPIF0F0 (rw) register accessor: SPIF\\_WTABLE28\n\nYou can [`read`](crate::Reg::read) this register and get [`spif0f0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif0f0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif0f0`] module"]
#[doc(alias = "SPIF0F0")]
pub type Spif0f0 = crate::Reg<spif0f0::Spif0f0Spec>;
#[doc = "SPIF\\_WTABLE28"]
pub mod spif0f0;
#[doc = "SPIF0F4 (rw) register accessor: SPIF\\_WTABLE29\n\nYou can [`read`](crate::Reg::read) this register and get [`spif0f4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif0f4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif0f4`] module"]
#[doc(alias = "SPIF0F4")]
pub type Spif0f4 = crate::Reg<spif0f4::Spif0f4Spec>;
#[doc = "SPIF\\_WTABLE29"]
pub mod spif0f4;
#[doc = "SPIF0F8 (rw) register accessor: SPIF\\_WTABLE30\n\nYou can [`read`](crate::Reg::read) this register and get [`spif0f8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif0f8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif0f8`] module"]
#[doc(alias = "SPIF0F8")]
pub type Spif0f8 = crate::Reg<spif0f8::Spif0f8Spec>;
#[doc = "SPIF\\_WTABLE30"]
pub mod spif0f8;
#[doc = "SPIF0FC (rw) register accessor: SPIF\\_WTABLE31\n\nYou can [`read`](crate::Reg::read) this register and get [`spif0fc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif0fc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif0fc`] module"]
#[doc(alias = "SPIF0FC")]
pub type Spif0fc = crate::Reg<spif0fc::Spif0fcSpec>;
#[doc = "SPIF\\_WTABLE31"]
pub mod spif0fc;
#[doc = "SPIF100 (rw) register accessor: SPIF\\_ADDRCTL00\n\nYou can [`read`](crate::Reg::read) this register and get [`spif100::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif100::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif100`] module"]
#[doc(alias = "SPIF100")]
pub type Spif100 = crate::Reg<spif100::Spif100Spec>;
#[doc = "SPIF\\_ADDRCTL00"]
pub mod spif100;
#[doc = "SPIF104 (rw) register accessor: SPIF\\_ADDRCTL01\n\nYou can [`read`](crate::Reg::read) this register and get [`spif104::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif104::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif104`] module"]
#[doc(alias = "SPIF104")]
pub type Spif104 = crate::Reg<spif104::Spif104Spec>;
#[doc = "SPIF\\_ADDRCTL01"]
pub mod spif104;
#[doc = "SPIF108 (rw) register accessor: SPIF\\_ADDRCTL02\n\nYou can [`read`](crate::Reg::read) this register and get [`spif108::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif108::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif108`] module"]
#[doc(alias = "SPIF108")]
pub type Spif108 = crate::Reg<spif108::Spif108Spec>;
#[doc = "SPIF\\_ADDRCTL02"]
pub mod spif108;
#[doc = "SPIF10C (rw) register accessor: SPIF\\_ADDRCTL03\n\nYou can [`read`](crate::Reg::read) this register and get [`spif10c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif10c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif10c`] module"]
#[doc(alias = "SPIF10C")]
pub type Spif10c = crate::Reg<spif10c::Spif10cSpec>;
#[doc = "SPIF\\_ADDRCTL03"]
pub mod spif10c;
#[doc = "SPIF110 (rw) register accessor: SPIF\\_ADDRCTL04\n\nYou can [`read`](crate::Reg::read) this register and get [`spif110::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif110::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif110`] module"]
#[doc(alias = "SPIF110")]
pub type Spif110 = crate::Reg<spif110::Spif110Spec>;
#[doc = "SPIF\\_ADDRCTL04"]
pub mod spif110;
#[doc = "SPIF114 (rw) register accessor: SPIF\\_ADDRCTL05\n\nYou can [`read`](crate::Reg::read) this register and get [`spif114::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif114::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif114`] module"]
#[doc(alias = "SPIF114")]
pub type Spif114 = crate::Reg<spif114::Spif114Spec>;
#[doc = "SPIF\\_ADDRCTL05"]
pub mod spif114;
#[doc = "SPIF118 (rw) register accessor: SPIF\\_ADDRCTL06\n\nYou can [`read`](crate::Reg::read) this register and get [`spif118::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif118::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif118`] module"]
#[doc(alias = "SPIF118")]
pub type Spif118 = crate::Reg<spif118::Spif118Spec>;
#[doc = "SPIF\\_ADDRCTL06"]
pub mod spif118;
#[doc = "SPIF11C (rw) register accessor: SPIF\\_ADDRCTL07\n\nYou can [`read`](crate::Reg::read) this register and get [`spif11c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif11c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif11c`] module"]
#[doc(alias = "SPIF11C")]
pub type Spif11c = crate::Reg<spif11c::Spif11cSpec>;
#[doc = "SPIF\\_ADDRCTL07"]
pub mod spif11c;
#[doc = "SPIF120 (rw) register accessor: SPIF\\_ADDRCTL08\n\nYou can [`read`](crate::Reg::read) this register and get [`spif120::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif120::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif120`] module"]
#[doc(alias = "SPIF120")]
pub type Spif120 = crate::Reg<spif120::Spif120Spec>;
#[doc = "SPIF\\_ADDRCTL08"]
pub mod spif120;
#[doc = "SPIF124 (rw) register accessor: SPIF\\_ADDRCTL09\n\nYou can [`read`](crate::Reg::read) this register and get [`spif124::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif124::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif124`] module"]
#[doc(alias = "SPIF124")]
pub type Spif124 = crate::Reg<spif124::Spif124Spec>;
#[doc = "SPIF\\_ADDRCTL09"]
pub mod spif124;
#[doc = "SPIF128 (rw) register accessor: SPIF\\_ADDRCTL10\n\nYou can [`read`](crate::Reg::read) this register and get [`spif128::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif128::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif128`] module"]
#[doc(alias = "SPIF128")]
pub type Spif128 = crate::Reg<spif128::Spif128Spec>;
#[doc = "SPIF\\_ADDRCTL10"]
pub mod spif128;
#[doc = "SPIF12C (rw) register accessor: SPIF\\_ADDRCTL11\n\nYou can [`read`](crate::Reg::read) this register and get [`spif12c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif12c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif12c`] module"]
#[doc(alias = "SPIF12C")]
pub type Spif12c = crate::Reg<spif12c::Spif12cSpec>;
#[doc = "SPIF\\_ADDRCTL11"]
pub mod spif12c;
#[doc = "SPIF130 (rw) register accessor: SPIF\\_ADDRCTL12\n\nYou can [`read`](crate::Reg::read) this register and get [`spif130::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif130::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif130`] module"]
#[doc(alias = "SPIF130")]
pub type Spif130 = crate::Reg<spif130::Spif130Spec>;
#[doc = "SPIF\\_ADDRCTL12"]
pub mod spif130;
#[doc = "SPIF134 (rw) register accessor: SPIF\\_ADDRCTL13\n\nYou can [`read`](crate::Reg::read) this register and get [`spif134::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif134::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif134`] module"]
#[doc(alias = "SPIF134")]
pub type Spif134 = crate::Reg<spif134::Spif134Spec>;
#[doc = "SPIF\\_ADDRCTL13"]
pub mod spif134;
#[doc = "SPIF138 (rw) register accessor: SPIF\\_ADDRCTL14\n\nYou can [`read`](crate::Reg::read) this register and get [`spif138::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif138::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif138`] module"]
#[doc(alias = "SPIF138")]
pub type Spif138 = crate::Reg<spif138::Spif138Spec>;
#[doc = "SPIF\\_ADDRCTL14"]
pub mod spif138;
#[doc = "SPIF13C (rw) register accessor: SPIF\\_ADDRCTL15\n\nYou can [`read`](crate::Reg::read) this register and get [`spif13c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif13c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif13c`] module"]
#[doc(alias = "SPIF13C")]
pub type Spif13c = crate::Reg<spif13c::Spif13cSpec>;
#[doc = "SPIF\\_ADDRCTL15"]
pub mod spif13c;
#[doc = "SPIF200 (rw) register accessor: SPIF\\_ADDRBND00\n\nYou can [`read`](crate::Reg::read) this register and get [`spif200::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif200::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif200`] module"]
#[doc(alias = "SPIF200")]
pub type Spif200 = crate::Reg<spif200::Spif200Spec>;
#[doc = "SPIF\\_ADDRBND00"]
pub mod spif200;
#[doc = "SPIF204 (rw) register accessor: SPIF\\_ADDRBND01\n\nYou can [`read`](crate::Reg::read) this register and get [`spif204::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif204::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif204`] module"]
#[doc(alias = "SPIF204")]
pub type Spif204 = crate::Reg<spif204::Spif204Spec>;
#[doc = "SPIF\\_ADDRBND01"]
pub mod spif204;
#[doc = "SPIF208 (rw) register accessor: SPIF\\_ADDRBND02\n\nYou can [`read`](crate::Reg::read) this register and get [`spif208::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif208::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif208`] module"]
#[doc(alias = "SPIF208")]
pub type Spif208 = crate::Reg<spif208::Spif208Spec>;
#[doc = "SPIF\\_ADDRBND02"]
pub mod spif208;
#[doc = "SPIF20C (rw) register accessor: SPIF\\_ADDRBND03\n\nYou can [`read`](crate::Reg::read) this register and get [`spif20c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif20c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif20c`] module"]
#[doc(alias = "SPIF20C")]
pub type Spif20c = crate::Reg<spif20c::Spif20cSpec>;
#[doc = "SPIF\\_ADDRBND03"]
pub mod spif20c;
#[doc = "SPIF210 (rw) register accessor: SPIF\\_ADDRBND04\n\nYou can [`read`](crate::Reg::read) this register and get [`spif210::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif210::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif210`] module"]
#[doc(alias = "SPIF210")]
pub type Spif210 = crate::Reg<spif210::Spif210Spec>;
#[doc = "SPIF\\_ADDRBND04"]
pub mod spif210;
#[doc = "SPIF214 (rw) register accessor: SPIF\\_ADDRBND05\n\nYou can [`read`](crate::Reg::read) this register and get [`spif214::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif214::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif214`] module"]
#[doc(alias = "SPIF214")]
pub type Spif214 = crate::Reg<spif214::Spif214Spec>;
#[doc = "SPIF\\_ADDRBND05"]
pub mod spif214;
#[doc = "SPIF218 (rw) register accessor: SPIF\\_ADDRBND06\n\nYou can [`read`](crate::Reg::read) this register and get [`spif218::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif218::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif218`] module"]
#[doc(alias = "SPIF218")]
pub type Spif218 = crate::Reg<spif218::Spif218Spec>;
#[doc = "SPIF\\_ADDRBND06"]
pub mod spif218;
#[doc = "SPIF21C (rw) register accessor: SPIF\\_ADDRBND07\n\nYou can [`read`](crate::Reg::read) this register and get [`spif21c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif21c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif21c`] module"]
#[doc(alias = "SPIF21C")]
pub type Spif21c = crate::Reg<spif21c::Spif21cSpec>;
#[doc = "SPIF\\_ADDRBND07"]
pub mod spif21c;
#[doc = "SPIF220 (rw) register accessor: SPIF\\_ADDRBND08\n\nYou can [`read`](crate::Reg::read) this register and get [`spif220::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif220::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif220`] module"]
#[doc(alias = "SPIF220")]
pub type Spif220 = crate::Reg<spif220::Spif220Spec>;
#[doc = "SPIF\\_ADDRBND08"]
pub mod spif220;
#[doc = "SPIF224 (rw) register accessor: SPIF\\_ADDRBND09\n\nYou can [`read`](crate::Reg::read) this register and get [`spif224::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif224::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif224`] module"]
#[doc(alias = "SPIF224")]
pub type Spif224 = crate::Reg<spif224::Spif224Spec>;
#[doc = "SPIF\\_ADDRBND09"]
pub mod spif224;
#[doc = "SPIF228 (rw) register accessor: SPIF\\_ADDRBND10\n\nYou can [`read`](crate::Reg::read) this register and get [`spif228::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif228::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif228`] module"]
#[doc(alias = "SPIF228")]
pub type Spif228 = crate::Reg<spif228::Spif228Spec>;
#[doc = "SPIF\\_ADDRBND10"]
pub mod spif228;
#[doc = "SPIF22C (rw) register accessor: SPIF\\_ADDRBND11\n\nYou can [`read`](crate::Reg::read) this register and get [`spif22c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif22c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif22c`] module"]
#[doc(alias = "SPIF22C")]
pub type Spif22c = crate::Reg<spif22c::Spif22cSpec>;
#[doc = "SPIF\\_ADDRBND11"]
pub mod spif22c;
#[doc = "SPIF230 (rw) register accessor: SPIF\\_ADDRBND12\n\nYou can [`read`](crate::Reg::read) this register and get [`spif230::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif230::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif230`] module"]
#[doc(alias = "SPIF230")]
pub type Spif230 = crate::Reg<spif230::Spif230Spec>;
#[doc = "SPIF\\_ADDRBND12"]
pub mod spif230;
#[doc = "SPIF234 (rw) register accessor: SPIF\\_ADDRBND13\n\nYou can [`read`](crate::Reg::read) this register and get [`spif234::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif234::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif234`] module"]
#[doc(alias = "SPIF234")]
pub type Spif234 = crate::Reg<spif234::Spif234Spec>;
#[doc = "SPIF\\_ADDRBND13"]
pub mod spif234;
#[doc = "SPIF238 (rw) register accessor: SPIF\\_ADDRBND14\n\nYou can [`read`](crate::Reg::read) this register and get [`spif238::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif238::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif238`] module"]
#[doc(alias = "SPIF238")]
pub type Spif238 = crate::Reg<spif238::Spif238Spec>;
#[doc = "SPIF\\_ADDRBND14"]
pub mod spif238;
#[doc = "SPIF23C (rw) register accessor: SPIF\\_ADDRBND15\n\nYou can [`read`](crate::Reg::read) this register and get [`spif23c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif23c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif23c`] module"]
#[doc(alias = "SPIF23C")]
pub type Spif23c = crate::Reg<spif23c::Spif23cSpec>;
#[doc = "SPIF\\_ADDRBND15"]
pub mod spif23c;
#[doc = "SPIF300 (rw) register accessor: SPIF\\_WPWTABLE\\_EN\n\nYou can [`read`](crate::Reg::read) this register and get [`spif300::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif300::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif300`] module"]
#[doc(alias = "SPIF300")]
pub type Spif300 = crate::Reg<spif300::Spif300Spec>;
#[doc = "SPIF\\_WPWTABLE\\_EN"]
pub mod spif300;
#[doc = "SPIF310 (rw) register accessor: SPIF\\_WPADDRCTL\n\nYou can [`read`](crate::Reg::read) this register and get [`spif310::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif310::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spif310`] module"]
#[doc(alias = "SPIF310")]
pub type Spif310 = crate::Reg<spif310::Spif310Spec>;
#[doc = "SPIF\\_WPADDRCTL"]
pub mod spif310;
