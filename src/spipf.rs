#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    spipf000: Spipf000,
    spipf004: Spipf004,
    spipf008: Spipf008,
    spipf00c: Spipf00c,
    spipf010: Spipf010,
    spipf014: Spipf014,
    spipf018: Spipf018,
    _reserved7: [u8; 0x04],
    spipf020: Spipf020,
    _reserved8: [u8; 0x58],
    spipf07c: Spipf07c,
    spipfwt: Spipfwt,
    _reserved10: [u8; 0x7c],
    spipf100: Spipf100,
    spipf108: Spipf108,
    spipf110: Spipf110,
    spipf118: Spipf118,
    spipf120: Spipf120,
    spipf128: Spipf128,
    spipf130: Spipf130,
    spipf138: Spipf138,
    spipf140: Spipf140,
    spipf148: Spipf148,
    spipf150: Spipf150,
    spipf158: Spipf158,
    spipf160: Spipf160,
    spipf168: Spipf168,
    spipf170: Spipf170,
    spipf178: Spipf178,
    spipf180: Spipf180,
    spipf188: Spipf188,
    spipf190: Spipf190,
    spipf198: Spipf198,
    spipf1a0: Spipf1a0,
    spipf1a8: Spipf1a8,
    spipf1b0: Spipf1b0,
    spipf1b8: Spipf1b8,
    spipf1c0: Spipf1c0,
    spipf1c8: Spipf1c8,
    spipf1d0: Spipf1d0,
    spipf1d8: Spipf1d8,
    spipf1e0: Spipf1e0,
    spipf1e8: Spipf1e8,
    spipf1f0: Spipf1f0,
    spipf1f8: Spipf1f8,
    spipf200: Spipf200,
    spipf208: Spipf208,
    spipf210: Spipf210,
    spipf218: Spipf218,
    spipf220: Spipf220,
    spipf228: Spipf228,
    spipf230: Spipf230,
    spipf238: Spipf238,
    spipf240: Spipf240,
    spipf248: Spipf248,
    spipf250: Spipf250,
    spipf258: Spipf258,
    spipf260: Spipf260,
    spipf268: Spipf268,
    spipf270: Spipf270,
    spipf278: Spipf278,
    spipf280: Spipf280,
    spipf288: Spipf288,
    spipf290: Spipf290,
    spipf298: Spipf298,
    spipf2a0: Spipf2a0,
    spipf2a8: Spipf2a8,
    spipf2b0: Spipf2b0,
    spipf2b8: Spipf2b8,
    spipf2c0: Spipf2c0,
    spipf2c8: Spipf2c8,
    spipf2d0: Spipf2d0,
    spipf2d8: Spipf2d8,
    spipf2e0: Spipf2e0,
    spipf2e8: Spipf2e8,
    spipf2f0: Spipf2f0,
    spipf2f8: Spipf2f8,
}
impl RegisterBlock {
    #[doc = "0x00 - Engine Control Register"]
    #[inline(always)]
    pub const fn spipf000(&self) -> &Spipf000 {
        &self.spipf000
    }
    #[doc = "0x04 - Interrupt Enable and Status Register"]
    #[inline(always)]
    pub const fn spipf004(&self) -> &Spipf004 {
        &self.spipf004
    }
    #[doc = "0x08 - EAR and Over Speed Register"]
    #[inline(always)]
    pub const fn spipf008(&self) -> &Spipf008 {
        &self.spipf008
    }
    #[doc = "0x0c - Block FIFO Data Register"]
    #[inline(always)]
    pub const fn spipf00c(&self) -> &Spipf00c {
        &self.spipf00c
    }
    #[doc = "0x10 - Block Log DMA Base Address"]
    #[inline(always)]
    pub const fn spipf010(&self) -> &Spipf010 {
        &self.spipf010
    }
    #[doc = "0x14 - Block Log DMA Size"]
    #[inline(always)]
    pub const fn spipf014(&self) -> &Spipf014 {
        &self.spipf014
    }
    #[doc = "0x18 - Block Log DMA Write Pointer"]
    #[inline(always)]
    pub const fn spipf018(&self) -> &Spipf018 {
        &self.spipf018
    }
    #[doc = "0x20 - CS0 range"]
    #[inline(always)]
    pub const fn spipf020(&self) -> &Spipf020 {
        &self.spipf020
    }
    #[doc = "0x7c - Write Disable Register"]
    #[inline(always)]
    pub const fn spipf07c(&self) -> &Spipf07c {
        &self.spipf07c
    }
    #[doc = "0x80 - Allow Command Table \\#1 $\\sim$ \\#32"]
    #[inline(always)]
    pub const fn spipfwt(&self) -> &Spipfwt {
        &self.spipfwt
    }
    #[doc = "0x100..0x108 - Region 00 setting"]
    #[inline(always)]
    pub const fn spipf100(&self) -> &Spipf100 {
        &self.spipf100
    }
    #[doc = "0x108..0x110 - Region 01 setting"]
    #[inline(always)]
    pub const fn spipf108(&self) -> &Spipf108 {
        &self.spipf108
    }
    #[doc = "0x110..0x118 - Region 02 setting"]
    #[inline(always)]
    pub const fn spipf110(&self) -> &Spipf110 {
        &self.spipf110
    }
    #[doc = "0x118..0x120 - Region 03 setting"]
    #[inline(always)]
    pub const fn spipf118(&self) -> &Spipf118 {
        &self.spipf118
    }
    #[doc = "0x120..0x128 - Region 04 setting"]
    #[inline(always)]
    pub const fn spipf120(&self) -> &Spipf120 {
        &self.spipf120
    }
    #[doc = "0x128..0x130 - Region 05 setting"]
    #[inline(always)]
    pub const fn spipf128(&self) -> &Spipf128 {
        &self.spipf128
    }
    #[doc = "0x130..0x138 - Region 06 setting"]
    #[inline(always)]
    pub const fn spipf130(&self) -> &Spipf130 {
        &self.spipf130
    }
    #[doc = "0x138..0x140 - Region 07 setting"]
    #[inline(always)]
    pub const fn spipf138(&self) -> &Spipf138 {
        &self.spipf138
    }
    #[doc = "0x140..0x148 - Region 08 setting"]
    #[inline(always)]
    pub const fn spipf140(&self) -> &Spipf140 {
        &self.spipf140
    }
    #[doc = "0x148..0x150 - Region 09 setting"]
    #[inline(always)]
    pub const fn spipf148(&self) -> &Spipf148 {
        &self.spipf148
    }
    #[doc = "0x150..0x158 - Region 10 setting"]
    #[inline(always)]
    pub const fn spipf150(&self) -> &Spipf150 {
        &self.spipf150
    }
    #[doc = "0x158..0x160 - Region 11 setting"]
    #[inline(always)]
    pub const fn spipf158(&self) -> &Spipf158 {
        &self.spipf158
    }
    #[doc = "0x160..0x168 - Region 12 setting"]
    #[inline(always)]
    pub const fn spipf160(&self) -> &Spipf160 {
        &self.spipf160
    }
    #[doc = "0x168..0x170 - Region 13 setting"]
    #[inline(always)]
    pub const fn spipf168(&self) -> &Spipf168 {
        &self.spipf168
    }
    #[doc = "0x170..0x178 - Region 14 setting"]
    #[inline(always)]
    pub const fn spipf170(&self) -> &Spipf170 {
        &self.spipf170
    }
    #[doc = "0x178..0x180 - Region 15 setting"]
    #[inline(always)]
    pub const fn spipf178(&self) -> &Spipf178 {
        &self.spipf178
    }
    #[doc = "0x180..0x188 - Region 16 setting"]
    #[inline(always)]
    pub const fn spipf180(&self) -> &Spipf180 {
        &self.spipf180
    }
    #[doc = "0x188..0x190 - Region 17 setting"]
    #[inline(always)]
    pub const fn spipf188(&self) -> &Spipf188 {
        &self.spipf188
    }
    #[doc = "0x190..0x198 - Region 18 setting"]
    #[inline(always)]
    pub const fn spipf190(&self) -> &Spipf190 {
        &self.spipf190
    }
    #[doc = "0x198..0x1a0 - Region 19 setting"]
    #[inline(always)]
    pub const fn spipf198(&self) -> &Spipf198 {
        &self.spipf198
    }
    #[doc = "0x1a0..0x1a8 - Region 20 setting"]
    #[inline(always)]
    pub const fn spipf1a0(&self) -> &Spipf1a0 {
        &self.spipf1a0
    }
    #[doc = "0x1a8..0x1b0 - Region 21 setting"]
    #[inline(always)]
    pub const fn spipf1a8(&self) -> &Spipf1a8 {
        &self.spipf1a8
    }
    #[doc = "0x1b0..0x1b8 - Region 22 setting"]
    #[inline(always)]
    pub const fn spipf1b0(&self) -> &Spipf1b0 {
        &self.spipf1b0
    }
    #[doc = "0x1b8..0x1c0 - Region 23 setting"]
    #[inline(always)]
    pub const fn spipf1b8(&self) -> &Spipf1b8 {
        &self.spipf1b8
    }
    #[doc = "0x1c0..0x1c8 - Region 24 setting"]
    #[inline(always)]
    pub const fn spipf1c0(&self) -> &Spipf1c0 {
        &self.spipf1c0
    }
    #[doc = "0x1c8..0x1d0 - Region 25 setting"]
    #[inline(always)]
    pub const fn spipf1c8(&self) -> &Spipf1c8 {
        &self.spipf1c8
    }
    #[doc = "0x1d0..0x1d8 - Region 26 setting"]
    #[inline(always)]
    pub const fn spipf1d0(&self) -> &Spipf1d0 {
        &self.spipf1d0
    }
    #[doc = "0x1d8..0x1e0 - Region 27 setting"]
    #[inline(always)]
    pub const fn spipf1d8(&self) -> &Spipf1d8 {
        &self.spipf1d8
    }
    #[doc = "0x1e0..0x1e8 - Region 28 setting"]
    #[inline(always)]
    pub const fn spipf1e0(&self) -> &Spipf1e0 {
        &self.spipf1e0
    }
    #[doc = "0x1e8..0x1f0 - Region 29 setting"]
    #[inline(always)]
    pub const fn spipf1e8(&self) -> &Spipf1e8 {
        &self.spipf1e8
    }
    #[doc = "0x1f0..0x1f8 - Region 30 setting"]
    #[inline(always)]
    pub const fn spipf1f0(&self) -> &Spipf1f0 {
        &self.spipf1f0
    }
    #[doc = "0x1f8..0x200 - Region 31 setting"]
    #[inline(always)]
    pub const fn spipf1f8(&self) -> &Spipf1f8 {
        &self.spipf1f8
    }
    #[doc = "0x200..0x208 - Region 32 setting"]
    #[inline(always)]
    pub const fn spipf200(&self) -> &Spipf200 {
        &self.spipf200
    }
    #[doc = "0x208..0x210 - Region 33 setting"]
    #[inline(always)]
    pub const fn spipf208(&self) -> &Spipf208 {
        &self.spipf208
    }
    #[doc = "0x210..0x218 - Region 34 setting"]
    #[inline(always)]
    pub const fn spipf210(&self) -> &Spipf210 {
        &self.spipf210
    }
    #[doc = "0x218..0x220 - Region 35 setting"]
    #[inline(always)]
    pub const fn spipf218(&self) -> &Spipf218 {
        &self.spipf218
    }
    #[doc = "0x220..0x228 - Region 36 setting"]
    #[inline(always)]
    pub const fn spipf220(&self) -> &Spipf220 {
        &self.spipf220
    }
    #[doc = "0x228..0x230 - Region 37 setting"]
    #[inline(always)]
    pub const fn spipf228(&self) -> &Spipf228 {
        &self.spipf228
    }
    #[doc = "0x230..0x238 - Region 38 setting"]
    #[inline(always)]
    pub const fn spipf230(&self) -> &Spipf230 {
        &self.spipf230
    }
    #[doc = "0x238..0x240 - Region 39 setting"]
    #[inline(always)]
    pub const fn spipf238(&self) -> &Spipf238 {
        &self.spipf238
    }
    #[doc = "0x240..0x248 - Region 40 setting"]
    #[inline(always)]
    pub const fn spipf240(&self) -> &Spipf240 {
        &self.spipf240
    }
    #[doc = "0x248..0x250 - Region 41 setting"]
    #[inline(always)]
    pub const fn spipf248(&self) -> &Spipf248 {
        &self.spipf248
    }
    #[doc = "0x250..0x258 - Region 42 setting"]
    #[inline(always)]
    pub const fn spipf250(&self) -> &Spipf250 {
        &self.spipf250
    }
    #[doc = "0x258..0x260 - Region 43 setting"]
    #[inline(always)]
    pub const fn spipf258(&self) -> &Spipf258 {
        &self.spipf258
    }
    #[doc = "0x260..0x268 - Region 44 setting"]
    #[inline(always)]
    pub const fn spipf260(&self) -> &Spipf260 {
        &self.spipf260
    }
    #[doc = "0x268..0x270 - Region 45 setting"]
    #[inline(always)]
    pub const fn spipf268(&self) -> &Spipf268 {
        &self.spipf268
    }
    #[doc = "0x270..0x278 - Region 46 setting"]
    #[inline(always)]
    pub const fn spipf270(&self) -> &Spipf270 {
        &self.spipf270
    }
    #[doc = "0x278..0x280 - Region 47 setting"]
    #[inline(always)]
    pub const fn spipf278(&self) -> &Spipf278 {
        &self.spipf278
    }
    #[doc = "0x280..0x288 - Region 48 setting"]
    #[inline(always)]
    pub const fn spipf280(&self) -> &Spipf280 {
        &self.spipf280
    }
    #[doc = "0x288..0x290 - Region 49 setting"]
    #[inline(always)]
    pub const fn spipf288(&self) -> &Spipf288 {
        &self.spipf288
    }
    #[doc = "0x290..0x298 - Region 50 setting"]
    #[inline(always)]
    pub const fn spipf290(&self) -> &Spipf290 {
        &self.spipf290
    }
    #[doc = "0x298..0x2a0 - Region 51 setting"]
    #[inline(always)]
    pub const fn spipf298(&self) -> &Spipf298 {
        &self.spipf298
    }
    #[doc = "0x2a0..0x2a8 - Region 52 setting"]
    #[inline(always)]
    pub const fn spipf2a0(&self) -> &Spipf2a0 {
        &self.spipf2a0
    }
    #[doc = "0x2a8..0x2b0 - Region 53 setting"]
    #[inline(always)]
    pub const fn spipf2a8(&self) -> &Spipf2a8 {
        &self.spipf2a8
    }
    #[doc = "0x2b0..0x2b8 - Region 54 setting"]
    #[inline(always)]
    pub const fn spipf2b0(&self) -> &Spipf2b0 {
        &self.spipf2b0
    }
    #[doc = "0x2b8..0x2c0 - Region 55 setting"]
    #[inline(always)]
    pub const fn spipf2b8(&self) -> &Spipf2b8 {
        &self.spipf2b8
    }
    #[doc = "0x2c0..0x2c8 - Region 56 setting"]
    #[inline(always)]
    pub const fn spipf2c0(&self) -> &Spipf2c0 {
        &self.spipf2c0
    }
    #[doc = "0x2c8..0x2d0 - Region 57 setting"]
    #[inline(always)]
    pub const fn spipf2c8(&self) -> &Spipf2c8 {
        &self.spipf2c8
    }
    #[doc = "0x2d0..0x2d8 - Region 58 setting"]
    #[inline(always)]
    pub const fn spipf2d0(&self) -> &Spipf2d0 {
        &self.spipf2d0
    }
    #[doc = "0x2d8..0x2e0 - Region 59 setting"]
    #[inline(always)]
    pub const fn spipf2d8(&self) -> &Spipf2d8 {
        &self.spipf2d8
    }
    #[doc = "0x2e0..0x2e8 - Region 60 setting"]
    #[inline(always)]
    pub const fn spipf2e0(&self) -> &Spipf2e0 {
        &self.spipf2e0
    }
    #[doc = "0x2e8..0x2f0 - Region 61 setting"]
    #[inline(always)]
    pub const fn spipf2e8(&self) -> &Spipf2e8 {
        &self.spipf2e8
    }
    #[doc = "0x2f0..0x2f8 - Region 62 setting"]
    #[inline(always)]
    pub const fn spipf2f0(&self) -> &Spipf2f0 {
        &self.spipf2f0
    }
    #[doc = "0x2f8..0x300 - Region 63 setting"]
    #[inline(always)]
    pub const fn spipf2f8(&self) -> &Spipf2f8 {
        &self.spipf2f8
    }
}
#[doc = "SPIPF000 (rw) register accessor: Engine Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf000::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf000::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf000`] module"]
#[doc(alias = "SPIPF000")]
pub type Spipf000 = crate::Reg<spipf000::Spipf000Spec>;
#[doc = "Engine Control Register"]
pub mod spipf000;
#[doc = "SPIPF004 (rw) register accessor: Interrupt Enable and Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf004::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf004::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf004`] module"]
#[doc(alias = "SPIPF004")]
pub type Spipf004 = crate::Reg<spipf004::Spipf004Spec>;
#[doc = "Interrupt Enable and Status Register"]
pub mod spipf004;
#[doc = "SPIPF008 (rw) register accessor: EAR and Over Speed Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf008::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf008::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf008`] module"]
#[doc(alias = "SPIPF008")]
pub type Spipf008 = crate::Reg<spipf008::Spipf008Spec>;
#[doc = "EAR and Over Speed Register"]
pub mod spipf008;
#[doc = "SPIPF00C (rw) register accessor: Block FIFO Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf00c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf00c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf00c`] module"]
#[doc(alias = "SPIPF00C")]
pub type Spipf00c = crate::Reg<spipf00c::Spipf00cSpec>;
#[doc = "Block FIFO Data Register"]
pub mod spipf00c;
#[doc = "SPIPF010 (rw) register accessor: Block Log DMA Base Address\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf010::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf010::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf010`] module"]
#[doc(alias = "SPIPF010")]
pub type Spipf010 = crate::Reg<spipf010::Spipf010Spec>;
#[doc = "Block Log DMA Base Address"]
pub mod spipf010;
#[doc = "SPIPF014 (rw) register accessor: Block Log DMA Size\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf014::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf014::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf014`] module"]
#[doc(alias = "SPIPF014")]
pub type Spipf014 = crate::Reg<spipf014::Spipf014Spec>;
#[doc = "Block Log DMA Size"]
pub mod spipf014;
#[doc = "SPIPF018 (rw) register accessor: Block Log DMA Write Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf018::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf018::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf018`] module"]
#[doc(alias = "SPIPF018")]
pub type Spipf018 = crate::Reg<spipf018::Spipf018Spec>;
#[doc = "Block Log DMA Write Pointer"]
pub mod spipf018;
#[doc = "SPIPF020 (rw) register accessor: CS0 range\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf020::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf020::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf020`] module"]
#[doc(alias = "SPIPF020")]
pub type Spipf020 = crate::Reg<spipf020::Spipf020Spec>;
#[doc = "CS0 range"]
pub mod spipf020;
#[doc = "SPIPF07C (rw) register accessor: Write Disable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf07c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf07c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf07c`] module"]
#[doc(alias = "SPIPF07C")]
pub type Spipf07c = crate::Reg<spipf07c::Spipf07cSpec>;
#[doc = "Write Disable Register"]
pub mod spipf07c;
#[doc = "SPIPFWT (rw) register accessor: Allow Command Table \\#1 $\\sim$ \\#32\n\nYou can [`read`](crate::Reg::read) this register and get [`spipfwt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipfwt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipfwt`] module"]
#[doc(alias = "SPIPFWT")]
pub type Spipfwt = crate::Reg<spipfwt::SpipfwtSpec>;
#[doc = "Allow Command Table \\#1 $\\sim$ \\#32"]
pub mod spipfwt;
#[doc = "SPIPF100 (rw) register accessor: Region 00 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf100::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf100::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf100`] module"]
#[doc(alias = "SPIPF100")]
pub type Spipf100 = crate::Reg<spipf100::Spipf100Spec>;
#[doc = "Region 00 setting"]
pub mod spipf100;
#[doc = "SPIPF108 (rw) register accessor: Region 01 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf108::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf108::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf108`] module"]
#[doc(alias = "SPIPF108")]
pub type Spipf108 = crate::Reg<spipf108::Spipf108Spec>;
#[doc = "Region 01 setting"]
pub mod spipf108;
#[doc = "SPIPF110 (rw) register accessor: Region 02 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf110::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf110::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf110`] module"]
#[doc(alias = "SPIPF110")]
pub type Spipf110 = crate::Reg<spipf110::Spipf110Spec>;
#[doc = "Region 02 setting"]
pub mod spipf110;
#[doc = "SPIPF118 (rw) register accessor: Region 03 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf118::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf118::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf118`] module"]
#[doc(alias = "SPIPF118")]
pub type Spipf118 = crate::Reg<spipf118::Spipf118Spec>;
#[doc = "Region 03 setting"]
pub mod spipf118;
#[doc = "SPIPF120 (rw) register accessor: Region 04 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf120::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf120::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf120`] module"]
#[doc(alias = "SPIPF120")]
pub type Spipf120 = crate::Reg<spipf120::Spipf120Spec>;
#[doc = "Region 04 setting"]
pub mod spipf120;
#[doc = "SPIPF128 (rw) register accessor: Region 05 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf128::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf128::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf128`] module"]
#[doc(alias = "SPIPF128")]
pub type Spipf128 = crate::Reg<spipf128::Spipf128Spec>;
#[doc = "Region 05 setting"]
pub mod spipf128;
#[doc = "SPIPF130 (rw) register accessor: Region 06 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf130::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf130::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf130`] module"]
#[doc(alias = "SPIPF130")]
pub type Spipf130 = crate::Reg<spipf130::Spipf130Spec>;
#[doc = "Region 06 setting"]
pub mod spipf130;
#[doc = "SPIPF138 (rw) register accessor: Region 07 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf138::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf138::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf138`] module"]
#[doc(alias = "SPIPF138")]
pub type Spipf138 = crate::Reg<spipf138::Spipf138Spec>;
#[doc = "Region 07 setting"]
pub mod spipf138;
#[doc = "SPIPF140 (rw) register accessor: Region 08 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf140::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf140::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf140`] module"]
#[doc(alias = "SPIPF140")]
pub type Spipf140 = crate::Reg<spipf140::Spipf140Spec>;
#[doc = "Region 08 setting"]
pub mod spipf140;
#[doc = "SPIPF148 (rw) register accessor: Region 09 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf148::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf148::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf148`] module"]
#[doc(alias = "SPIPF148")]
pub type Spipf148 = crate::Reg<spipf148::Spipf148Spec>;
#[doc = "Region 09 setting"]
pub mod spipf148;
#[doc = "SPIPF150 (rw) register accessor: Region 10 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf150::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf150::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf150`] module"]
#[doc(alias = "SPIPF150")]
pub type Spipf150 = crate::Reg<spipf150::Spipf150Spec>;
#[doc = "Region 10 setting"]
pub mod spipf150;
#[doc = "SPIPF158 (rw) register accessor: Region 11 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf158::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf158::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf158`] module"]
#[doc(alias = "SPIPF158")]
pub type Spipf158 = crate::Reg<spipf158::Spipf158Spec>;
#[doc = "Region 11 setting"]
pub mod spipf158;
#[doc = "SPIPF160 (rw) register accessor: Region 12 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf160::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf160::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf160`] module"]
#[doc(alias = "SPIPF160")]
pub type Spipf160 = crate::Reg<spipf160::Spipf160Spec>;
#[doc = "Region 12 setting"]
pub mod spipf160;
#[doc = "SPIPF168 (rw) register accessor: Region 13 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf168::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf168::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf168`] module"]
#[doc(alias = "SPIPF168")]
pub type Spipf168 = crate::Reg<spipf168::Spipf168Spec>;
#[doc = "Region 13 setting"]
pub mod spipf168;
#[doc = "SPIPF170 (rw) register accessor: Region 14 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf170::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf170::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf170`] module"]
#[doc(alias = "SPIPF170")]
pub type Spipf170 = crate::Reg<spipf170::Spipf170Spec>;
#[doc = "Region 14 setting"]
pub mod spipf170;
#[doc = "SPIPF178 (rw) register accessor: Region 15 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf178::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf178::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf178`] module"]
#[doc(alias = "SPIPF178")]
pub type Spipf178 = crate::Reg<spipf178::Spipf178Spec>;
#[doc = "Region 15 setting"]
pub mod spipf178;
#[doc = "SPIPF180 (rw) register accessor: Region 16 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf180::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf180::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf180`] module"]
#[doc(alias = "SPIPF180")]
pub type Spipf180 = crate::Reg<spipf180::Spipf180Spec>;
#[doc = "Region 16 setting"]
pub mod spipf180;
#[doc = "SPIPF188 (rw) register accessor: Region 17 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf188::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf188::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf188`] module"]
#[doc(alias = "SPIPF188")]
pub type Spipf188 = crate::Reg<spipf188::Spipf188Spec>;
#[doc = "Region 17 setting"]
pub mod spipf188;
#[doc = "SPIPF190 (rw) register accessor: Region 18 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf190::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf190::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf190`] module"]
#[doc(alias = "SPIPF190")]
pub type Spipf190 = crate::Reg<spipf190::Spipf190Spec>;
#[doc = "Region 18 setting"]
pub mod spipf190;
#[doc = "SPIPF198 (rw) register accessor: Region 19 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf198::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf198::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf198`] module"]
#[doc(alias = "SPIPF198")]
pub type Spipf198 = crate::Reg<spipf198::Spipf198Spec>;
#[doc = "Region 19 setting"]
pub mod spipf198;
#[doc = "SPIPF1A0 (rw) register accessor: Region 20 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf1a0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf1a0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf1a0`] module"]
#[doc(alias = "SPIPF1A0")]
pub type Spipf1a0 = crate::Reg<spipf1a0::Spipf1a0Spec>;
#[doc = "Region 20 setting"]
pub mod spipf1a0;
#[doc = "SPIPF1A8 (rw) register accessor: Region 21 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf1a8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf1a8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf1a8`] module"]
#[doc(alias = "SPIPF1A8")]
pub type Spipf1a8 = crate::Reg<spipf1a8::Spipf1a8Spec>;
#[doc = "Region 21 setting"]
pub mod spipf1a8;
#[doc = "SPIPF1B0 (rw) register accessor: Region 22 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf1b0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf1b0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf1b0`] module"]
#[doc(alias = "SPIPF1B0")]
pub type Spipf1b0 = crate::Reg<spipf1b0::Spipf1b0Spec>;
#[doc = "Region 22 setting"]
pub mod spipf1b0;
#[doc = "SPIPF1B8 (rw) register accessor: Region 23 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf1b8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf1b8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf1b8`] module"]
#[doc(alias = "SPIPF1B8")]
pub type Spipf1b8 = crate::Reg<spipf1b8::Spipf1b8Spec>;
#[doc = "Region 23 setting"]
pub mod spipf1b8;
#[doc = "SPIPF1C0 (rw) register accessor: Region 24 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf1c0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf1c0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf1c0`] module"]
#[doc(alias = "SPIPF1C0")]
pub type Spipf1c0 = crate::Reg<spipf1c0::Spipf1c0Spec>;
#[doc = "Region 24 setting"]
pub mod spipf1c0;
#[doc = "SPIPF1C8 (rw) register accessor: Region 25 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf1c8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf1c8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf1c8`] module"]
#[doc(alias = "SPIPF1C8")]
pub type Spipf1c8 = crate::Reg<spipf1c8::Spipf1c8Spec>;
#[doc = "Region 25 setting"]
pub mod spipf1c8;
#[doc = "SPIPF1D0 (rw) register accessor: Region 26 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf1d0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf1d0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf1d0`] module"]
#[doc(alias = "SPIPF1D0")]
pub type Spipf1d0 = crate::Reg<spipf1d0::Spipf1d0Spec>;
#[doc = "Region 26 setting"]
pub mod spipf1d0;
#[doc = "SPIPF1D8 (rw) register accessor: Region 27 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf1d8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf1d8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf1d8`] module"]
#[doc(alias = "SPIPF1D8")]
pub type Spipf1d8 = crate::Reg<spipf1d8::Spipf1d8Spec>;
#[doc = "Region 27 setting"]
pub mod spipf1d8;
#[doc = "SPIPF1E0 (rw) register accessor: Region 28 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf1e0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf1e0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf1e0`] module"]
#[doc(alias = "SPIPF1E0")]
pub type Spipf1e0 = crate::Reg<spipf1e0::Spipf1e0Spec>;
#[doc = "Region 28 setting"]
pub mod spipf1e0;
#[doc = "SPIPF1E8 (rw) register accessor: Region 29 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf1e8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf1e8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf1e8`] module"]
#[doc(alias = "SPIPF1E8")]
pub type Spipf1e8 = crate::Reg<spipf1e8::Spipf1e8Spec>;
#[doc = "Region 29 setting"]
pub mod spipf1e8;
#[doc = "SPIPF1F0 (rw) register accessor: Region 30 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf1f0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf1f0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf1f0`] module"]
#[doc(alias = "SPIPF1F0")]
pub type Spipf1f0 = crate::Reg<spipf1f0::Spipf1f0Spec>;
#[doc = "Region 30 setting"]
pub mod spipf1f0;
#[doc = "SPIPF1F8 (rw) register accessor: Region 31 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf1f8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf1f8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf1f8`] module"]
#[doc(alias = "SPIPF1F8")]
pub type Spipf1f8 = crate::Reg<spipf1f8::Spipf1f8Spec>;
#[doc = "Region 31 setting"]
pub mod spipf1f8;
#[doc = "SPIPF200 (rw) register accessor: Region 32 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf200::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf200::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf200`] module"]
#[doc(alias = "SPIPF200")]
pub type Spipf200 = crate::Reg<spipf200::Spipf200Spec>;
#[doc = "Region 32 setting"]
pub mod spipf200;
#[doc = "SPIPF208 (rw) register accessor: Region 33 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf208::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf208::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf208`] module"]
#[doc(alias = "SPIPF208")]
pub type Spipf208 = crate::Reg<spipf208::Spipf208Spec>;
#[doc = "Region 33 setting"]
pub mod spipf208;
#[doc = "SPIPF210 (rw) register accessor: Region 34 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf210::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf210::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf210`] module"]
#[doc(alias = "SPIPF210")]
pub type Spipf210 = crate::Reg<spipf210::Spipf210Spec>;
#[doc = "Region 34 setting"]
pub mod spipf210;
#[doc = "SPIPF218 (rw) register accessor: Region 35 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf218::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf218::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf218`] module"]
#[doc(alias = "SPIPF218")]
pub type Spipf218 = crate::Reg<spipf218::Spipf218Spec>;
#[doc = "Region 35 setting"]
pub mod spipf218;
#[doc = "SPIPF220 (rw) register accessor: Region 36 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf220::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf220::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf220`] module"]
#[doc(alias = "SPIPF220")]
pub type Spipf220 = crate::Reg<spipf220::Spipf220Spec>;
#[doc = "Region 36 setting"]
pub mod spipf220;
#[doc = "SPIPF228 (rw) register accessor: Region 37 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf228::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf228::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf228`] module"]
#[doc(alias = "SPIPF228")]
pub type Spipf228 = crate::Reg<spipf228::Spipf228Spec>;
#[doc = "Region 37 setting"]
pub mod spipf228;
#[doc = "SPIPF230 (rw) register accessor: Region 38 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf230::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf230::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf230`] module"]
#[doc(alias = "SPIPF230")]
pub type Spipf230 = crate::Reg<spipf230::Spipf230Spec>;
#[doc = "Region 38 setting"]
pub mod spipf230;
#[doc = "SPIPF238 (rw) register accessor: Region 39 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf238::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf238::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf238`] module"]
#[doc(alias = "SPIPF238")]
pub type Spipf238 = crate::Reg<spipf238::Spipf238Spec>;
#[doc = "Region 39 setting"]
pub mod spipf238;
#[doc = "SPIPF240 (rw) register accessor: Region 40 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf240::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf240::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf240`] module"]
#[doc(alias = "SPIPF240")]
pub type Spipf240 = crate::Reg<spipf240::Spipf240Spec>;
#[doc = "Region 40 setting"]
pub mod spipf240;
#[doc = "SPIPF248 (rw) register accessor: Region 41 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf248::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf248::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf248`] module"]
#[doc(alias = "SPIPF248")]
pub type Spipf248 = crate::Reg<spipf248::Spipf248Spec>;
#[doc = "Region 41 setting"]
pub mod spipf248;
#[doc = "SPIPF250 (rw) register accessor: Region 42 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf250::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf250::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf250`] module"]
#[doc(alias = "SPIPF250")]
pub type Spipf250 = crate::Reg<spipf250::Spipf250Spec>;
#[doc = "Region 42 setting"]
pub mod spipf250;
#[doc = "SPIPF258 (rw) register accessor: Region 43 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf258::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf258::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf258`] module"]
#[doc(alias = "SPIPF258")]
pub type Spipf258 = crate::Reg<spipf258::Spipf258Spec>;
#[doc = "Region 43 setting"]
pub mod spipf258;
#[doc = "SPIPF260 (rw) register accessor: Region 44 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf260::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf260::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf260`] module"]
#[doc(alias = "SPIPF260")]
pub type Spipf260 = crate::Reg<spipf260::Spipf260Spec>;
#[doc = "Region 44 setting"]
pub mod spipf260;
#[doc = "SPIPF268 (rw) register accessor: Region 45 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf268::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf268::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf268`] module"]
#[doc(alias = "SPIPF268")]
pub type Spipf268 = crate::Reg<spipf268::Spipf268Spec>;
#[doc = "Region 45 setting"]
pub mod spipf268;
#[doc = "SPIPF270 (rw) register accessor: Region 46 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf270::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf270::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf270`] module"]
#[doc(alias = "SPIPF270")]
pub type Spipf270 = crate::Reg<spipf270::Spipf270Spec>;
#[doc = "Region 46 setting"]
pub mod spipf270;
#[doc = "SPIPF278 (rw) register accessor: Region 47 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf278::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf278::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf278`] module"]
#[doc(alias = "SPIPF278")]
pub type Spipf278 = crate::Reg<spipf278::Spipf278Spec>;
#[doc = "Region 47 setting"]
pub mod spipf278;
#[doc = "SPIPF280 (rw) register accessor: Region 48 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf280::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf280::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf280`] module"]
#[doc(alias = "SPIPF280")]
pub type Spipf280 = crate::Reg<spipf280::Spipf280Spec>;
#[doc = "Region 48 setting"]
pub mod spipf280;
#[doc = "SPIPF288 (rw) register accessor: Region 49 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf288::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf288::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf288`] module"]
#[doc(alias = "SPIPF288")]
pub type Spipf288 = crate::Reg<spipf288::Spipf288Spec>;
#[doc = "Region 49 setting"]
pub mod spipf288;
#[doc = "SPIPF290 (rw) register accessor: Region 50 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf290::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf290::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf290`] module"]
#[doc(alias = "SPIPF290")]
pub type Spipf290 = crate::Reg<spipf290::Spipf290Spec>;
#[doc = "Region 50 setting"]
pub mod spipf290;
#[doc = "SPIPF298 (rw) register accessor: Region 51 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf298::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf298::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf298`] module"]
#[doc(alias = "SPIPF298")]
pub type Spipf298 = crate::Reg<spipf298::Spipf298Spec>;
#[doc = "Region 51 setting"]
pub mod spipf298;
#[doc = "SPIPF2A0 (rw) register accessor: Region 52 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf2a0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf2a0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf2a0`] module"]
#[doc(alias = "SPIPF2A0")]
pub type Spipf2a0 = crate::Reg<spipf2a0::Spipf2a0Spec>;
#[doc = "Region 52 setting"]
pub mod spipf2a0;
#[doc = "SPIPF2A8 (rw) register accessor: Region 53 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf2a8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf2a8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf2a8`] module"]
#[doc(alias = "SPIPF2A8")]
pub type Spipf2a8 = crate::Reg<spipf2a8::Spipf2a8Spec>;
#[doc = "Region 53 setting"]
pub mod spipf2a8;
#[doc = "SPIPF2B0 (rw) register accessor: Region 54 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf2b0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf2b0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf2b0`] module"]
#[doc(alias = "SPIPF2B0")]
pub type Spipf2b0 = crate::Reg<spipf2b0::Spipf2b0Spec>;
#[doc = "Region 54 setting"]
pub mod spipf2b0;
#[doc = "SPIPF2B8 (rw) register accessor: Region 55 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf2b8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf2b8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf2b8`] module"]
#[doc(alias = "SPIPF2B8")]
pub type Spipf2b8 = crate::Reg<spipf2b8::Spipf2b8Spec>;
#[doc = "Region 55 setting"]
pub mod spipf2b8;
#[doc = "SPIPF2C0 (rw) register accessor: Region 56 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf2c0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf2c0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf2c0`] module"]
#[doc(alias = "SPIPF2C0")]
pub type Spipf2c0 = crate::Reg<spipf2c0::Spipf2c0Spec>;
#[doc = "Region 56 setting"]
pub mod spipf2c0;
#[doc = "SPIPF2C8 (rw) register accessor: Region 57 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf2c8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf2c8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf2c8`] module"]
#[doc(alias = "SPIPF2C8")]
pub type Spipf2c8 = crate::Reg<spipf2c8::Spipf2c8Spec>;
#[doc = "Region 57 setting"]
pub mod spipf2c8;
#[doc = "SPIPF2D0 (rw) register accessor: Region 58 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf2d0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf2d0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf2d0`] module"]
#[doc(alias = "SPIPF2D0")]
pub type Spipf2d0 = crate::Reg<spipf2d0::Spipf2d0Spec>;
#[doc = "Region 58 setting"]
pub mod spipf2d0;
#[doc = "SPIPF2D8 (rw) register accessor: Region 59 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf2d8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf2d8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf2d8`] module"]
#[doc(alias = "SPIPF2D8")]
pub type Spipf2d8 = crate::Reg<spipf2d8::Spipf2d8Spec>;
#[doc = "Region 59 setting"]
pub mod spipf2d8;
#[doc = "SPIPF2E0 (rw) register accessor: Region 60 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf2e0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf2e0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf2e0`] module"]
#[doc(alias = "SPIPF2E0")]
pub type Spipf2e0 = crate::Reg<spipf2e0::Spipf2e0Spec>;
#[doc = "Region 60 setting"]
pub mod spipf2e0;
#[doc = "SPIPF2E8 (rw) register accessor: Region 61 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf2e8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf2e8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf2e8`] module"]
#[doc(alias = "SPIPF2E8")]
pub type Spipf2e8 = crate::Reg<spipf2e8::Spipf2e8Spec>;
#[doc = "Region 61 setting"]
pub mod spipf2e8;
#[doc = "SPIPF2F0 (rw) register accessor: Region 62 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf2f0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf2f0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf2f0`] module"]
#[doc(alias = "SPIPF2F0")]
pub type Spipf2f0 = crate::Reg<spipf2f0::Spipf2f0Spec>;
#[doc = "Region 62 setting"]
pub mod spipf2f0;
#[doc = "SPIPF2F8 (rw) register accessor: Region 63 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf2f8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf2f8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spipf2f8`] module"]
#[doc(alias = "SPIPF2F8")]
pub type Spipf2f8 = crate::Reg<spipf2f8::Spipf2f8Spec>;
#[doc = "Region 63 setting"]
pub mod spipf2f8;
