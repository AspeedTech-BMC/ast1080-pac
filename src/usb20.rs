#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    hub00: Hub00,
    hub04: Hub04,
    hub08: Hub08,
    hub0c: Hub0c,
    hub10: Hub10,
    hub14: Hub14,
    hub18: Hub18,
    hub1c: Hub1c,
    hub20: Hub20,
    hub24: Hub24,
    hub28: Hub28,
    hub2c: Hub2c,
    hub30: Hub30,
    hub34: Hub34,
    hub38: Hub38,
    hub3c: Hub3c,
    hub40: Hub40,
    hub44: Hub44,
    _reserved18: [u8; 0x38],
    setup: [Setup; 8],
    _reserved19: [u8; 0x40],
    dev: [Dev; 7],
    _reserved20: [u8; 0x90],
    epp: [Epp; 21],
}
impl RegisterBlock {
    #[doc = "0x00 - Root Function Control And Status Register"]
    #[inline(always)]
    pub const fn hub00(&self) -> &Hub00 {
        &self.hub00
    }
    #[doc = "0x04 - Root Configuration Setting Register"]
    #[inline(always)]
    pub const fn hub04(&self) -> &Hub04 {
        &self.hub04
    }
    #[doc = "0x08 - Interrupt Control Register"]
    #[inline(always)]
    pub const fn hub08(&self) -> &Hub08 {
        &self.hub08
    }
    #[doc = "0x0c - Interrupt Status Register"]
    #[inline(always)]
    pub const fn hub0c(&self) -> &Hub0c {
        &self.hub0c
    }
    #[doc = "0x10 - Programmable Endpoint Pool ACK Interrupt Enable Register"]
    #[inline(always)]
    pub const fn hub10(&self) -> &Hub10 {
        &self.hub10
    }
    #[doc = "0x14 - Programmable Endpoint Pool NAK Interrupt Enable Register"]
    #[inline(always)]
    pub const fn hub14(&self) -> &Hub14 {
        &self.hub14
    }
    #[doc = "0x18 - Programmable Endpoint Pool ACK Interrupt Status Register"]
    #[inline(always)]
    pub const fn hub18(&self) -> &Hub18 {
        &self.hub18
    }
    #[doc = "0x1c - Programmable Endpoint Pool NAK Interrupt Status Register"]
    #[inline(always)]
    pub const fn hub1c(&self) -> &Hub1c {
        &self.hub1c
    }
    #[doc = "0x20 - Device Controller Soft Reset Enable Register"]
    #[inline(always)]
    pub const fn hub20(&self) -> &Hub20 {
        &self.hub20
    }
    #[doc = "0x24 - USB Status Register \\regdebugh"]
    #[inline(always)]
    pub const fn hub24(&self) -> &Hub24 {
        &self.hub24
    }
    #[doc = "0x28 - Programmable Endpoint Pool Data Toggle Value Set"]
    #[inline(always)]
    pub const fn hub28(&self) -> &Hub28 {
        &self.hub28
    }
    #[doc = "0x2c - Isochronous Transaction Fail Accumulator \\regdebugh"]
    #[inline(always)]
    pub const fn hub2c(&self) -> &Hub2c {
        &self.hub2c
    }
    #[doc = "0x30 - Endpoint 0 Control/Status Register"]
    #[inline(always)]
    pub const fn hub30(&self) -> &Hub30 {
        &self.hub30
    }
    #[doc = "0x34 - Base Address of Endpoint 0 IN/OUT Data Buffer Register"]
    #[inline(always)]
    pub const fn hub34(&self) -> &Hub34 {
        &self.hub34
    }
    #[doc = "0x38 - Endpoint 1 Control/Status Register"]
    #[inline(always)]
    pub const fn hub38(&self) -> &Hub38 {
        &self.hub38
    }
    #[doc = "0x3c - Endpoint 1 Status Change Bitmap Data"]
    #[inline(always)]
    pub const fn hub3c(&self) -> &Hub3c {
        &self.hub3c
    }
    #[doc = "0x40 - SOF Counter"]
    #[inline(always)]
    pub const fn hub40(&self) -> &Hub40 {
        &self.hub40
    }
    #[doc = "0x44 - DMA to Memory Synchronization Status"]
    #[inline(always)]
    pub const fn hub44(&self) -> &Hub44 {
        &self.hub44
    }
    #[doc = "0x80..0xc0 - SETUP\\[%s\\]"]
    #[inline(always)]
    pub const fn setup(&self, n: usize) -> &Setup {
        &self.setup[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0xc0 - SETUP\\[%s\\]"]
    #[inline(always)]
    pub fn setup_iter(&self) -> impl Iterator<Item = &Setup> {
        self.setup.iter()
    }
    #[doc = "0x100..0x170 - DEV\\[%s\\]"]
    #[inline(always)]
    pub const fn dev(&self, n: usize) -> &Dev {
        &self.dev[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x170 - DEV\\[%s\\]"]
    #[inline(always)]
    pub fn dev_iter(&self) -> impl Iterator<Item = &Dev> {
        self.dev.iter()
    }
    #[doc = "0x200..0x350 - EPP\\[%s\\]"]
    #[inline(always)]
    pub const fn epp(&self, n: usize) -> &Epp {
        &self.epp[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x200..0x350 - EPP\\[%s\\]"]
    #[inline(always)]
    pub fn epp_iter(&self) -> impl Iterator<Item = &Epp> {
        self.epp.iter()
    }
}
#[doc = "SETUP\\[%s\\]"]
pub use self::setup::Setup;
#[doc = r"Cluster"]
#[doc = "SETUP\\[%s\\]"]
pub mod setup;
#[doc = "HUB00 (rw) register accessor: Root Function Control And Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hub00::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hub00::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hub00`] module"]
#[doc(alias = "HUB00")]
pub type Hub00 = crate::Reg<hub00::Hub00Spec>;
#[doc = "Root Function Control And Status Register"]
pub mod hub00;
#[doc = "HUB04 (rw) register accessor: Root Configuration Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hub04::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hub04::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hub04`] module"]
#[doc(alias = "HUB04")]
pub type Hub04 = crate::Reg<hub04::Hub04Spec>;
#[doc = "Root Configuration Setting Register"]
pub mod hub04;
#[doc = "HUB08 (rw) register accessor: Interrupt Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hub08::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hub08::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hub08`] module"]
#[doc(alias = "HUB08")]
pub type Hub08 = crate::Reg<hub08::Hub08Spec>;
#[doc = "Interrupt Control Register"]
pub mod hub08;
#[doc = "HUB0C (rw) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hub0c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hub0c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hub0c`] module"]
#[doc(alias = "HUB0C")]
pub type Hub0c = crate::Reg<hub0c::Hub0cSpec>;
#[doc = "Interrupt Status Register"]
pub mod hub0c;
#[doc = "HUB10 (rw) register accessor: Programmable Endpoint Pool ACK Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hub10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hub10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hub10`] module"]
#[doc(alias = "HUB10")]
pub type Hub10 = crate::Reg<hub10::Hub10Spec>;
#[doc = "Programmable Endpoint Pool ACK Interrupt Enable Register"]
pub mod hub10;
#[doc = "HUB14 (rw) register accessor: Programmable Endpoint Pool NAK Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hub14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hub14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hub14`] module"]
#[doc(alias = "HUB14")]
pub type Hub14 = crate::Reg<hub14::Hub14Spec>;
#[doc = "Programmable Endpoint Pool NAK Interrupt Enable Register"]
pub mod hub14;
#[doc = "HUB18 (rw) register accessor: Programmable Endpoint Pool ACK Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hub18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hub18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hub18`] module"]
#[doc(alias = "HUB18")]
pub type Hub18 = crate::Reg<hub18::Hub18Spec>;
#[doc = "Programmable Endpoint Pool ACK Interrupt Status Register"]
pub mod hub18;
#[doc = "HUB1C (rw) register accessor: Programmable Endpoint Pool NAK Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hub1c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hub1c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hub1c`] module"]
#[doc(alias = "HUB1C")]
pub type Hub1c = crate::Reg<hub1c::Hub1cSpec>;
#[doc = "Programmable Endpoint Pool NAK Interrupt Status Register"]
pub mod hub1c;
#[doc = "HUB20 (rw) register accessor: Device Controller Soft Reset Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hub20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hub20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hub20`] module"]
#[doc(alias = "HUB20")]
pub type Hub20 = crate::Reg<hub20::Hub20Spec>;
#[doc = "Device Controller Soft Reset Enable Register"]
pub mod hub20;
#[doc = "HUB24 (rw) register accessor: USB Status Register \\regdebugh\n\nYou can [`read`](crate::Reg::read) this register and get [`hub24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hub24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hub24`] module"]
#[doc(alias = "HUB24")]
pub type Hub24 = crate::Reg<hub24::Hub24Spec>;
#[doc = "USB Status Register \\regdebugh"]
pub mod hub24;
#[doc = "HUB28 (rw) register accessor: Programmable Endpoint Pool Data Toggle Value Set\n\nYou can [`read`](crate::Reg::read) this register and get [`hub28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hub28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hub28`] module"]
#[doc(alias = "HUB28")]
pub type Hub28 = crate::Reg<hub28::Hub28Spec>;
#[doc = "Programmable Endpoint Pool Data Toggle Value Set"]
pub mod hub28;
#[doc = "HUB2C (rw) register accessor: Isochronous Transaction Fail Accumulator \\regdebugh\n\nYou can [`read`](crate::Reg::read) this register and get [`hub2c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hub2c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hub2c`] module"]
#[doc(alias = "HUB2C")]
pub type Hub2c = crate::Reg<hub2c::Hub2cSpec>;
#[doc = "Isochronous Transaction Fail Accumulator \\regdebugh"]
pub mod hub2c;
#[doc = "HUB30 (rw) register accessor: Endpoint 0 Control/Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hub30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hub30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hub30`] module"]
#[doc(alias = "HUB30")]
pub type Hub30 = crate::Reg<hub30::Hub30Spec>;
#[doc = "Endpoint 0 Control/Status Register"]
pub mod hub30;
#[doc = "HUB34 (rw) register accessor: Base Address of Endpoint 0 IN/OUT Data Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hub34::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hub34::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hub34`] module"]
#[doc(alias = "HUB34")]
pub type Hub34 = crate::Reg<hub34::Hub34Spec>;
#[doc = "Base Address of Endpoint 0 IN/OUT Data Buffer Register"]
pub mod hub34;
#[doc = "HUB38 (rw) register accessor: Endpoint 1 Control/Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hub38::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hub38::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hub38`] module"]
#[doc(alias = "HUB38")]
pub type Hub38 = crate::Reg<hub38::Hub38Spec>;
#[doc = "Endpoint 1 Control/Status Register"]
pub mod hub38;
#[doc = "HUB3C (rw) register accessor: Endpoint 1 Status Change Bitmap Data\n\nYou can [`read`](crate::Reg::read) this register and get [`hub3c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hub3c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hub3c`] module"]
#[doc(alias = "HUB3C")]
pub type Hub3c = crate::Reg<hub3c::Hub3cSpec>;
#[doc = "Endpoint 1 Status Change Bitmap Data"]
pub mod hub3c;
#[doc = "HUB40 (rw) register accessor: SOF Counter\n\nYou can [`read`](crate::Reg::read) this register and get [`hub40::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hub40::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hub40`] module"]
#[doc(alias = "HUB40")]
pub type Hub40 = crate::Reg<hub40::Hub40Spec>;
#[doc = "SOF Counter"]
pub mod hub40;
#[doc = "HUB44 (rw) register accessor: DMA to Memory Synchronization Status\n\nYou can [`read`](crate::Reg::read) this register and get [`hub44::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hub44::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hub44`] module"]
#[doc(alias = "HUB44")]
pub type Hub44 = crate::Reg<hub44::Hub44Spec>;
#[doc = "DMA to Memory Synchronization Status"]
pub mod hub44;
#[doc = "EPP\\[%s\\]"]
pub use self::epp::Epp;
#[doc = r"Cluster"]
#[doc = "EPP\\[%s\\]"]
pub mod epp;
#[doc = "DEV\\[%s\\]"]
pub use self::dev::Dev;
#[doc = r"Cluster"]
#[doc = "DEV\\[%s\\]"]
pub mod dev;
