#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    spi000: Spi000,
    spi004: Spi004,
    spi008: Spi008,
    spi00c: Spi00c,
    spi010: Spi010,
    spi014: Spi014,
    spi018: Spi018,
    spi01c: Spi01c,
    _reserved8: [u8; 0x10],
    spi030: Spi030,
    spi034: Spi034,
    spi038: Spi038,
    spi03c: Spi03c,
    _reserved12: [u8; 0x14],
    spi054: Spi054,
    _reserved13: [u8; 0x24],
    spi07c: Spi07c,
    spi080: Spi080,
    spi084: Spi084,
    spi088: Spi088,
    spi08c: Spi08c,
    spi090: Spi090,
    spi094: Spi094,
    spi098: Spi098,
    spi09c: Spi09c,
    spi0a0: Spi0a0,
    _reserved23: [u8; 0x011c],
    spi1c0: Spi1c0,
    _reserved24: [u8; 0x1c],
    spi1e0: Spi1e0,
    spi1e4: Spi1e4,
    spi1e8: Spi1e8,
    _reserved27: [u8; 0x04],
    spi1f0: Spi1f0,
    spi1f4: Spi1f4,
    spi1f8: Spi1f8,
    spi1fc: Spi1fc,
}
impl RegisterBlock {
    #[doc = "0x00 - SPI Flash Configuration Register"]
    #[inline(always)]
    pub const fn spi000(&self) -> &Spi000 {
        &self.spi000
    }
    #[doc = "0x04 - CE Control Register"]
    #[inline(always)]
    pub const fn spi004(&self) -> &Spi004 {
        &self.spi004
    }
    #[doc = "0x08 - Interrupt Control and Status Register"]
    #[inline(always)]
    pub const fn spi008(&self) -> &Spi008 {
        &self.spi008
    }
    #[doc = "0x0c - Command Control Register"]
    #[inline(always)]
    pub const fn spi00c(&self) -> &Spi00c {
        &self.spi00c
    }
    #[doc = "0x10 - CE0 Control Register"]
    #[inline(always)]
    pub const fn spi010(&self) -> &Spi010 {
        &self.spi010
    }
    #[doc = "0x14 - CE1 Control Register"]
    #[inline(always)]
    pub const fn spi014(&self) -> &Spi014 {
        &self.spi014
    }
    #[doc = "0x18 - CE2 Control Register"]
    #[inline(always)]
    pub const fn spi018(&self) -> &Spi018 {
        &self.spi018
    }
    #[doc = "0x1c - CE3 Control Register"]
    #[inline(always)]
    pub const fn spi01c(&self) -> &Spi01c {
        &self.spi01c
    }
    #[doc = "0x30 - CE0 Address Decoding Range Register"]
    #[inline(always)]
    pub const fn spi030(&self) -> &Spi030 {
        &self.spi030
    }
    #[doc = "0x34 - CE1 Address Decoding Range Register"]
    #[inline(always)]
    pub const fn spi034(&self) -> &Spi034 {
        &self.spi034
    }
    #[doc = "0x38 - CE2 Address Decoding Range Register"]
    #[inline(always)]
    pub const fn spi038(&self) -> &Spi038 {
        &self.spi038
    }
    #[doc = "0x3c - CE3 Address Decoding Range Register"]
    #[inline(always)]
    pub const fn spi03c(&self) -> &Spi03c {
        &self.spi03c
    }
    #[doc = "0x54 - SPI Dummy Cycle Data Register"]
    #[inline(always)]
    pub const fn spi054(&self) -> &Spi054 {
        &self.spi054
    }
    #[doc = "0x7c - DMA DRAM Side Address High Part"]
    #[inline(always)]
    pub const fn spi07c(&self) -> &Spi07c {
        &self.spi07c
    }
    #[doc = "0x80 - DMA Control/Status Register"]
    #[inline(always)]
    pub const fn spi080(&self) -> &Spi080 {
        &self.spi080
    }
    #[doc = "0x84 - DMA Flash Side Address"]
    #[inline(always)]
    pub const fn spi084(&self) -> &Spi084 {
        &self.spi084
    }
    #[doc = "0x88 - DMA DRAM Side Address"]
    #[inline(always)]
    pub const fn spi088(&self) -> &Spi088 {
        &self.spi088
    }
    #[doc = "0x8c - DMA Length Register"]
    #[inline(always)]
    pub const fn spi08c(&self) -> &Spi08c {
        &self.spi08c
    }
    #[doc = "0x90 - CheckSum Calculation Result"]
    #[inline(always)]
    pub const fn spi090(&self) -> &Spi090 {
        &self.spi090
    }
    #[doc = "0x94 - CE0 SPI Flash Read Timing Compensation"]
    #[inline(always)]
    pub const fn spi094(&self) -> &Spi094 {
        &self.spi094
    }
    #[doc = "0x98 - CE1 SPI Flash Read Timing Compensation"]
    #[inline(always)]
    pub const fn spi098(&self) -> &Spi098 {
        &self.spi098
    }
    #[doc = "0x9c - CE2 SPI Flash Read Timing Compensation"]
    #[inline(always)]
    pub const fn spi09c(&self) -> &Spi09c {
        &self.spi09c
    }
    #[doc = "0xa0 - CE3 SPI Flash Read Timing Compensation"]
    #[inline(always)]
    pub const fn spi0a0(&self) -> &Spi0a0 {
        &self.spi0a0
    }
    #[doc = "0x1c0 - reserved"]
    #[inline(always)]
    pub const fn spi1c0(&self) -> &Spi1c0 {
        &self.spi1c0
    }
    #[doc = "0x1e0 - SPI Engine Status report"]
    #[inline(always)]
    pub const fn spi1e0(&self) -> &Spi1e0 {
        &self.spi1e0
    }
    #[doc = "0x1e4 - SPI Data in monitor"]
    #[inline(always)]
    pub const fn spi1e4(&self) -> &Spi1e4 {
        &self.spi1e4
    }
    #[doc = "0x1e8 - SPI SOC RESET information"]
    #[inline(always)]
    pub const fn spi1e8(&self) -> &Spi1e8 {
        &self.spi1e8
    }
    #[doc = "0x1f0 - SPI Write lock Status for Configure Write until SRST\\#"]
    #[inline(always)]
    pub const fn spi1f0(&self) -> &Spi1f0 {
        &self.spi1f0
    }
    #[doc = "0x1f4 - SPI Write lock Status when SOC RESET is coming"]
    #[inline(always)]
    pub const fn spi1f4(&self) -> &Spi1f4 {
        &self.spi1f4
    }
    #[doc = "0x1f8 - SPI Write lock Status for Configure Write until soc reset"]
    #[inline(always)]
    pub const fn spi1f8(&self) -> &Spi1f8 {
        &self.spi1f8
    }
    #[doc = "0x1fc - SPI Write lock status for Configure Write\n"]
    #[inline(always)]
    pub const fn spi1fc(&self) -> &Spi1fc {
        &self.spi1fc
    }
}
#[doc = "SPI000 (rw) register accessor: SPI Flash Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi000::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi000::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi000`] module"]
#[doc(alias = "SPI000")]
pub type Spi000 = crate::Reg<spi000::Spi000Spec>;
#[doc = "SPI Flash Configuration Register"]
pub mod spi000;
#[doc = "SPI004 (rw) register accessor: CE Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi004::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi004::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi004`] module"]
#[doc(alias = "SPI004")]
pub type Spi004 = crate::Reg<spi004::Spi004Spec>;
#[doc = "CE Control Register"]
pub mod spi004;
#[doc = "SPI008 (rw) register accessor: Interrupt Control and Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi008::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi008::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi008`] module"]
#[doc(alias = "SPI008")]
pub type Spi008 = crate::Reg<spi008::Spi008Spec>;
#[doc = "Interrupt Control and Status Register"]
pub mod spi008;
#[doc = "SPI00C (rw) register accessor: Command Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi00c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi00c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi00c`] module"]
#[doc(alias = "SPI00C")]
pub type Spi00c = crate::Reg<spi00c::Spi00cSpec>;
#[doc = "Command Control Register"]
pub mod spi00c;
#[doc = "SPI010 (rw) register accessor: CE0 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi010::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi010::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi010`] module"]
#[doc(alias = "SPI010")]
pub type Spi010 = crate::Reg<spi010::Spi010Spec>;
#[doc = "CE0 Control Register"]
pub mod spi010;
#[doc = "SPI014 (rw) register accessor: CE1 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi014::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi014::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi014`] module"]
#[doc(alias = "SPI014")]
pub type Spi014 = crate::Reg<spi014::Spi014Spec>;
#[doc = "CE1 Control Register"]
pub mod spi014;
#[doc = "SPI018 (rw) register accessor: CE2 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi018::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi018::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi018`] module"]
#[doc(alias = "SPI018")]
pub type Spi018 = crate::Reg<spi018::Spi018Spec>;
#[doc = "CE2 Control Register"]
pub mod spi018;
#[doc = "SPI01C (rw) register accessor: CE3 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi01c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi01c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi01c`] module"]
#[doc(alias = "SPI01C")]
pub type Spi01c = crate::Reg<spi01c::Spi01cSpec>;
#[doc = "CE3 Control Register"]
pub mod spi01c;
#[doc = "SPI030 (rw) register accessor: CE0 Address Decoding Range Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi030::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi030::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi030`] module"]
#[doc(alias = "SPI030")]
pub type Spi030 = crate::Reg<spi030::Spi030Spec>;
#[doc = "CE0 Address Decoding Range Register"]
pub mod spi030;
#[doc = "SPI034 (rw) register accessor: CE1 Address Decoding Range Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi034::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi034::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi034`] module"]
#[doc(alias = "SPI034")]
pub type Spi034 = crate::Reg<spi034::Spi034Spec>;
#[doc = "CE1 Address Decoding Range Register"]
pub mod spi034;
#[doc = "SPI038 (rw) register accessor: CE2 Address Decoding Range Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi038::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi038::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi038`] module"]
#[doc(alias = "SPI038")]
pub type Spi038 = crate::Reg<spi038::Spi038Spec>;
#[doc = "CE2 Address Decoding Range Register"]
pub mod spi038;
#[doc = "SPI03C (rw) register accessor: CE3 Address Decoding Range Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi03c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi03c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi03c`] module"]
#[doc(alias = "SPI03C")]
pub type Spi03c = crate::Reg<spi03c::Spi03cSpec>;
#[doc = "CE3 Address Decoding Range Register"]
pub mod spi03c;
#[doc = "SPI054 (rw) register accessor: SPI Dummy Cycle Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi054::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi054::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi054`] module"]
#[doc(alias = "SPI054")]
pub type Spi054 = crate::Reg<spi054::Spi054Spec>;
#[doc = "SPI Dummy Cycle Data Register"]
pub mod spi054;
#[doc = "SPI07C (rw) register accessor: DMA DRAM Side Address High Part\n\nYou can [`read`](crate::Reg::read) this register and get [`spi07c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi07c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi07c`] module"]
#[doc(alias = "SPI07C")]
pub type Spi07c = crate::Reg<spi07c::Spi07cSpec>;
#[doc = "DMA DRAM Side Address High Part"]
pub mod spi07c;
#[doc = "SPI080 (rw) register accessor: DMA Control/Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi080::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi080::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi080`] module"]
#[doc(alias = "SPI080")]
pub type Spi080 = crate::Reg<spi080::Spi080Spec>;
#[doc = "DMA Control/Status Register"]
pub mod spi080;
#[doc = "SPI084 (rw) register accessor: DMA Flash Side Address\n\nYou can [`read`](crate::Reg::read) this register and get [`spi084::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi084::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi084`] module"]
#[doc(alias = "SPI084")]
pub type Spi084 = crate::Reg<spi084::Spi084Spec>;
#[doc = "DMA Flash Side Address"]
pub mod spi084;
#[doc = "SPI088 (rw) register accessor: DMA DRAM Side Address\n\nYou can [`read`](crate::Reg::read) this register and get [`spi088::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi088::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi088`] module"]
#[doc(alias = "SPI088")]
pub type Spi088 = crate::Reg<spi088::Spi088Spec>;
#[doc = "DMA DRAM Side Address"]
pub mod spi088;
#[doc = "SPI08C (rw) register accessor: DMA Length Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi08c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi08c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi08c`] module"]
#[doc(alias = "SPI08C")]
pub type Spi08c = crate::Reg<spi08c::Spi08cSpec>;
#[doc = "DMA Length Register"]
pub mod spi08c;
#[doc = "SPI090 (rw) register accessor: CheckSum Calculation Result\n\nYou can [`read`](crate::Reg::read) this register and get [`spi090::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi090::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi090`] module"]
#[doc(alias = "SPI090")]
pub type Spi090 = crate::Reg<spi090::Spi090Spec>;
#[doc = "CheckSum Calculation Result"]
pub mod spi090;
#[doc = "SPI094 (rw) register accessor: CE0 SPI Flash Read Timing Compensation\n\nYou can [`read`](crate::Reg::read) this register and get [`spi094::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi094::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi094`] module"]
#[doc(alias = "SPI094")]
pub type Spi094 = crate::Reg<spi094::Spi094Spec>;
#[doc = "CE0 SPI Flash Read Timing Compensation"]
pub mod spi094;
#[doc = "SPI098 (rw) register accessor: CE1 SPI Flash Read Timing Compensation\n\nYou can [`read`](crate::Reg::read) this register and get [`spi098::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi098::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi098`] module"]
#[doc(alias = "SPI098")]
pub type Spi098 = crate::Reg<spi098::Spi098Spec>;
#[doc = "CE1 SPI Flash Read Timing Compensation"]
pub mod spi098;
#[doc = "SPI09C (rw) register accessor: CE2 SPI Flash Read Timing Compensation\n\nYou can [`read`](crate::Reg::read) this register and get [`spi09c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi09c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi09c`] module"]
#[doc(alias = "SPI09C")]
pub type Spi09c = crate::Reg<spi09c::Spi09cSpec>;
#[doc = "CE2 SPI Flash Read Timing Compensation"]
pub mod spi09c;
#[doc = "SPI0A0 (rw) register accessor: CE3 SPI Flash Read Timing Compensation\n\nYou can [`read`](crate::Reg::read) this register and get [`spi0a0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi0a0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi0a0`] module"]
#[doc(alias = "SPI0A0")]
pub type Spi0a0 = crate::Reg<spi0a0::Spi0a0Spec>;
#[doc = "CE3 SPI Flash Read Timing Compensation"]
pub mod spi0a0;
#[doc = "SPI1C0 (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`spi1c0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi1c0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi1c0`] module"]
#[doc(alias = "SPI1C0")]
pub type Spi1c0 = crate::Reg<spi1c0::Spi1c0Spec>;
#[doc = "reserved"]
pub mod spi1c0;
#[doc = "SPI1E0 (rw) register accessor: SPI Engine Status report\n\nYou can [`read`](crate::Reg::read) this register and get [`spi1e0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi1e0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi1e0`] module"]
#[doc(alias = "SPI1E0")]
pub type Spi1e0 = crate::Reg<spi1e0::Spi1e0Spec>;
#[doc = "SPI Engine Status report"]
pub mod spi1e0;
#[doc = "SPI1E4 (rw) register accessor: SPI Data in monitor\n\nYou can [`read`](crate::Reg::read) this register and get [`spi1e4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi1e4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi1e4`] module"]
#[doc(alias = "SPI1E4")]
pub type Spi1e4 = crate::Reg<spi1e4::Spi1e4Spec>;
#[doc = "SPI Data in monitor"]
pub mod spi1e4;
#[doc = "SPI1E8 (rw) register accessor: SPI SOC RESET information\n\nYou can [`read`](crate::Reg::read) this register and get [`spi1e8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi1e8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi1e8`] module"]
#[doc(alias = "SPI1E8")]
pub type Spi1e8 = crate::Reg<spi1e8::Spi1e8Spec>;
#[doc = "SPI SOC RESET information"]
pub mod spi1e8;
#[doc = "SPI1F0 (rw) register accessor: SPI Write lock Status for Configure Write until SRST\\#\n\nYou can [`read`](crate::Reg::read) this register and get [`spi1f0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi1f0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi1f0`] module"]
#[doc(alias = "SPI1F0")]
pub type Spi1f0 = crate::Reg<spi1f0::Spi1f0Spec>;
#[doc = "SPI Write lock Status for Configure Write until SRST\\#"]
pub mod spi1f0;
#[doc = "SPI1F4 (rw) register accessor: SPI Write lock Status when SOC RESET is coming\n\nYou can [`read`](crate::Reg::read) this register and get [`spi1f4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi1f4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi1f4`] module"]
#[doc(alias = "SPI1F4")]
pub type Spi1f4 = crate::Reg<spi1f4::Spi1f4Spec>;
#[doc = "SPI Write lock Status when SOC RESET is coming"]
pub mod spi1f4;
#[doc = "SPI1F8 (rw) register accessor: SPI Write lock Status for Configure Write until soc reset\n\nYou can [`read`](crate::Reg::read) this register and get [`spi1f8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi1f8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi1f8`] module"]
#[doc(alias = "SPI1F8")]
pub type Spi1f8 = crate::Reg<spi1f8::Spi1f8Spec>;
#[doc = "SPI Write lock Status for Configure Write until soc reset"]
pub mod spi1f8;
#[doc = "SPI1FC (rw) register accessor: SPI Write lock status for Configure Write\n\n\nYou can [`read`](crate::Reg::read) this register and get [`spi1fc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi1fc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi1fc`] module"]
#[doc(alias = "SPI1FC")]
pub type Spi1fc = crate::Reg<spi1fc::Spi1fcSpec>;
#[doc = "SPI Write lock status for Configure Write\n"]
pub mod spi1fc;
