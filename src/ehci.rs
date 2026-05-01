#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ehci000: Ehci000,
    ehci004: Ehci004,
    ehci008: Ehci008,
    ehcic: [Ehcic; 5],
    ehci020: Ehci020,
    ehci024: Ehci024,
    ehci028: Ehci028,
    ehci02c: Ehci02c,
    ehci030: Ehci030,
    ehci034: Ehci034,
    ehci038: Ehci038,
    _reserved11: [u8; 0x24],
    ehci060: Ehci060,
    ehci064: Ehci064,
    _reserved13: [u8; 0x18],
    ehci080: Ehci080,
    ehci084: Ehci084,
    ehci088: Ehci088,
    ehci08c: Ehci08c,
}
impl RegisterBlock {
    #[doc = "0x00 - Capability Registers Length (CAPLENGTH)"]
    #[inline(always)]
    pub const fn ehci000(&self) -> &Ehci000 {
        &self.ehci000
    }
    #[doc = "0x04 - Structural Parameters (HCSPARAMS)"]
    #[inline(always)]
    pub const fn ehci004(&self) -> &Ehci004 {
        &self.ehci004
    }
    #[doc = "0x08 - Capability Parameters (HCCPARAMS)"]
    #[inline(always)]
    pub const fn ehci008(&self) -> &Ehci008 {
        &self.ehci008
    }
    #[doc = "0x0c..0x20 - Companion Port Route Description (HCSP-PORTROUTE)"]
    #[inline(always)]
    pub const fn ehcic(&self, n: usize) -> &Ehcic {
        &self.ehcic[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x0c..0x20 - Companion Port Route Description (HCSP-PORTROUTE)"]
    #[inline(always)]
    pub fn ehcic_iter(&self) -> impl Iterator<Item = &Ehcic> {
        self.ehcic.iter()
    }
    #[doc = "0x20 - USB Command Register (USBCMD)"]
    #[inline(always)]
    pub const fn ehci020(&self) -> &Ehci020 {
        &self.ehci020
    }
    #[doc = "0x24 - USB Status Register (USBSTS)"]
    #[inline(always)]
    pub const fn ehci024(&self) -> &Ehci024 {
        &self.ehci024
    }
    #[doc = "0x28 - USB Interrupt Enable Register (USBINTR)"]
    #[inline(always)]
    pub const fn ehci028(&self) -> &Ehci028 {
        &self.ehci028
    }
    #[doc = "0x2c - Frame Index Register (FRINDEX)"]
    #[inline(always)]
    pub const fn ehci02c(&self) -> &Ehci02c {
        &self.ehci02c
    }
    #[doc = "0x30 - Control Data Structure Segment Register (CTRLDSSEGMENT)"]
    #[inline(always)]
    pub const fn ehci030(&self) -> &Ehci030 {
        &self.ehci030
    }
    #[doc = "0x34 - Periodic Frame List Base Address Register (PERIODICLISTBASE)"]
    #[inline(always)]
    pub const fn ehci034(&self) -> &Ehci034 {
        &self.ehci034
    }
    #[doc = "0x38 - Current Asynchronous List Address Register (ASYNCLISTADDR)"]
    #[inline(always)]
    pub const fn ehci038(&self) -> &Ehci038 {
        &self.ehci038
    }
    #[doc = "0x60 - Configure Flag Register (CONFIGFLAG)"]
    #[inline(always)]
    pub const fn ehci060(&self) -> &Ehci060 {
        &self.ehci060
    }
    #[doc = "0x64 - Port1 Status/Control Register (PORTSC1)"]
    #[inline(always)]
    pub const fn ehci064(&self) -> &Ehci064 {
        &self.ehci064
    }
    #[doc = "0x80 - Frame Length Adjustment Register (FLADJ)"]
    #[inline(always)]
    pub const fn ehci080(&self) -> &Ehci080 {
        &self.ehci080
    }
    #[doc = "0x84 - Controller Fine-tune Register"]
    #[inline(always)]
    pub const fn ehci084(&self) -> &Ehci084 {
        &self.ehci084
    }
    #[doc = "0x88 - Frame Timing Adjustment"]
    #[inline(always)]
    pub const fn ehci088(&self) -> &Ehci088 {
        &self.ehci088
    }
    #[doc = "0x8c - Hardware Revision Number Register"]
    #[inline(always)]
    pub const fn ehci08c(&self) -> &Ehci08c {
        &self.ehci08c
    }
}
#[doc = "EHCI000 (rw) register accessor: Capability Registers Length (CAPLENGTH)\n\nYou can [`read`](crate::Reg::read) this register and get [`ehci000::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ehci000::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ehci000`] module"]
#[doc(alias = "EHCI000")]
pub type Ehci000 = crate::Reg<ehci000::Ehci000Spec>;
#[doc = "Capability Registers Length (CAPLENGTH)"]
pub mod ehci000;
#[doc = "EHCI004 (rw) register accessor: Structural Parameters (HCSPARAMS)\n\nYou can [`read`](crate::Reg::read) this register and get [`ehci004::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ehci004::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ehci004`] module"]
#[doc(alias = "EHCI004")]
pub type Ehci004 = crate::Reg<ehci004::Ehci004Spec>;
#[doc = "Structural Parameters (HCSPARAMS)"]
pub mod ehci004;
#[doc = "EHCI008 (rw) register accessor: Capability Parameters (HCCPARAMS)\n\nYou can [`read`](crate::Reg::read) this register and get [`ehci008::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ehci008::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ehci008`] module"]
#[doc(alias = "EHCI008")]
pub type Ehci008 = crate::Reg<ehci008::Ehci008Spec>;
#[doc = "Capability Parameters (HCCPARAMS)"]
pub mod ehci008;
#[doc = "EHCIC (rw) register accessor: Companion Port Route Description (HCSP-PORTROUTE)\n\nYou can [`read`](crate::Reg::read) this register and get [`ehcic::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ehcic::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ehcic`] module"]
#[doc(alias = "EHCIC")]
pub type Ehcic = crate::Reg<ehcic::EhcicSpec>;
#[doc = "Companion Port Route Description (HCSP-PORTROUTE)"]
pub mod ehcic;
#[doc = "EHCI020 (rw) register accessor: USB Command Register (USBCMD)\n\nYou can [`read`](crate::Reg::read) this register and get [`ehci020::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ehci020::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ehci020`] module"]
#[doc(alias = "EHCI020")]
pub type Ehci020 = crate::Reg<ehci020::Ehci020Spec>;
#[doc = "USB Command Register (USBCMD)"]
pub mod ehci020;
#[doc = "EHCI024 (rw) register accessor: USB Status Register (USBSTS)\n\nYou can [`read`](crate::Reg::read) this register and get [`ehci024::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ehci024::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ehci024`] module"]
#[doc(alias = "EHCI024")]
pub type Ehci024 = crate::Reg<ehci024::Ehci024Spec>;
#[doc = "USB Status Register (USBSTS)"]
pub mod ehci024;
#[doc = "EHCI028 (rw) register accessor: USB Interrupt Enable Register (USBINTR)\n\nYou can [`read`](crate::Reg::read) this register and get [`ehci028::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ehci028::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ehci028`] module"]
#[doc(alias = "EHCI028")]
pub type Ehci028 = crate::Reg<ehci028::Ehci028Spec>;
#[doc = "USB Interrupt Enable Register (USBINTR)"]
pub mod ehci028;
#[doc = "EHCI02C (rw) register accessor: Frame Index Register (FRINDEX)\n\nYou can [`read`](crate::Reg::read) this register and get [`ehci02c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ehci02c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ehci02c`] module"]
#[doc(alias = "EHCI02C")]
pub type Ehci02c = crate::Reg<ehci02c::Ehci02cSpec>;
#[doc = "Frame Index Register (FRINDEX)"]
pub mod ehci02c;
#[doc = "EHCI030 (rw) register accessor: Control Data Structure Segment Register (CTRLDSSEGMENT)\n\nYou can [`read`](crate::Reg::read) this register and get [`ehci030::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ehci030::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ehci030`] module"]
#[doc(alias = "EHCI030")]
pub type Ehci030 = crate::Reg<ehci030::Ehci030Spec>;
#[doc = "Control Data Structure Segment Register (CTRLDSSEGMENT)"]
pub mod ehci030;
#[doc = "EHCI034 (rw) register accessor: Periodic Frame List Base Address Register (PERIODICLISTBASE)\n\nYou can [`read`](crate::Reg::read) this register and get [`ehci034::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ehci034::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ehci034`] module"]
#[doc(alias = "EHCI034")]
pub type Ehci034 = crate::Reg<ehci034::Ehci034Spec>;
#[doc = "Periodic Frame List Base Address Register (PERIODICLISTBASE)"]
pub mod ehci034;
#[doc = "EHCI038 (rw) register accessor: Current Asynchronous List Address Register (ASYNCLISTADDR)\n\nYou can [`read`](crate::Reg::read) this register and get [`ehci038::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ehci038::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ehci038`] module"]
#[doc(alias = "EHCI038")]
pub type Ehci038 = crate::Reg<ehci038::Ehci038Spec>;
#[doc = "Current Asynchronous List Address Register (ASYNCLISTADDR)"]
pub mod ehci038;
#[doc = "EHCI060 (rw) register accessor: Configure Flag Register (CONFIGFLAG)\n\nYou can [`read`](crate::Reg::read) this register and get [`ehci060::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ehci060::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ehci060`] module"]
#[doc(alias = "EHCI060")]
pub type Ehci060 = crate::Reg<ehci060::Ehci060Spec>;
#[doc = "Configure Flag Register (CONFIGFLAG)"]
pub mod ehci060;
#[doc = "EHCI064 (rw) register accessor: Port1 Status/Control Register (PORTSC1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ehci064::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ehci064::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ehci064`] module"]
#[doc(alias = "EHCI064")]
pub type Ehci064 = crate::Reg<ehci064::Ehci064Spec>;
#[doc = "Port1 Status/Control Register (PORTSC1)"]
pub mod ehci064;
#[doc = "EHCI080 (rw) register accessor: Frame Length Adjustment Register (FLADJ)\n\nYou can [`read`](crate::Reg::read) this register and get [`ehci080::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ehci080::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ehci080`] module"]
#[doc(alias = "EHCI080")]
pub type Ehci080 = crate::Reg<ehci080::Ehci080Spec>;
#[doc = "Frame Length Adjustment Register (FLADJ)"]
pub mod ehci080;
#[doc = "EHCI084 (rw) register accessor: Controller Fine-tune Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ehci084::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ehci084::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ehci084`] module"]
#[doc(alias = "EHCI084")]
pub type Ehci084 = crate::Reg<ehci084::Ehci084Spec>;
#[doc = "Controller Fine-tune Register"]
pub mod ehci084;
#[doc = "EHCI088 (rw) register accessor: Frame Timing Adjustment\n\nYou can [`read`](crate::Reg::read) this register and get [`ehci088::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ehci088::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ehci088`] module"]
#[doc(alias = "EHCI088")]
pub type Ehci088 = crate::Reg<ehci088::Ehci088Spec>;
#[doc = "Frame Timing Adjustment"]
pub mod ehci088;
#[doc = "EHCI08C (rw) register accessor: Hardware Revision Number Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ehci08c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ehci08c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ehci08c`] module"]
#[doc(alias = "EHCI08C")]
pub type Ehci08c = crate::Reg<ehci08c::Ehci08cSpec>;
#[doc = "Hardware Revision Number Register"]
pub mod ehci08c;
