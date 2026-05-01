#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    uhci000: Uhci000,
    uhci004: Uhci004,
    uhci008: Uhci008,
    uhci00c: Uhci00c,
    _reserved4: [u8; 0x30],
    uhci040: Uhci040,
    _reserved5: [u8; 0x3c],
    uhci080: Uhci080,
    uhci084: Uhci084,
    uhci088: Uhci088,
    uhci08c: Uhci08c,
}
impl RegisterBlock {
    #[doc = "0x00 - USB Command Register (USBCMD)"]
    #[inline(always)]
    pub const fn uhci000(&self) -> &Uhci000 {
        &self.uhci000
    }
    #[doc = "0x04 - USB Status Register (USBSTS)"]
    #[inline(always)]
    pub const fn uhci004(&self) -> &Uhci004 {
        &self.uhci004
    }
    #[doc = "0x08 - USB Interrupt Enable Register (USBINT)"]
    #[inline(always)]
    pub const fn uhci008(&self) -> &Uhci008 {
        &self.uhci008
    }
    #[doc = "0x0c - Frame List Based Address Register (FRBASEADD)"]
    #[inline(always)]
    pub const fn uhci00c(&self) -> &Uhci00c {
        &self.uhci00c
    }
    #[doc = "0x40 - Test Control Register"]
    #[inline(always)]
    pub const fn uhci040(&self) -> &Uhci040 {
        &self.uhci040
    }
    #[doc = "0x80 - Frame Number Register (FRNUM)"]
    #[inline(always)]
    pub const fn uhci080(&self) -> &Uhci080 {
        &self.uhci080
    }
    #[doc = "0x84 - Start of Frame Modify Register (SOFMOD)"]
    #[inline(always)]
    pub const fn uhci084(&self) -> &Uhci084 {
        &self.uhci084
    }
    #[doc = "0x88 - Port1 Status/Control Register (PORTSC1)"]
    #[inline(always)]
    pub const fn uhci088(&self) -> &Uhci088 {
        &self.uhci088
    }
    #[doc = "0x8c - Port2 Status/Control Register (PORTSC2)"]
    #[inline(always)]
    pub const fn uhci08c(&self) -> &Uhci08c {
        &self.uhci08c
    }
}
#[doc = "UHCI000 (rw) register accessor: USB Command Register (USBCMD)\n\nYou can [`read`](crate::Reg::read) this register and get [`uhci000::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhci000::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uhci000`] module"]
#[doc(alias = "UHCI000")]
pub type Uhci000 = crate::Reg<uhci000::Uhci000Spec>;
#[doc = "USB Command Register (USBCMD)"]
pub mod uhci000;
#[doc = "UHCI004 (rw) register accessor: USB Status Register (USBSTS)\n\nYou can [`read`](crate::Reg::read) this register and get [`uhci004::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhci004::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uhci004`] module"]
#[doc(alias = "UHCI004")]
pub type Uhci004 = crate::Reg<uhci004::Uhci004Spec>;
#[doc = "USB Status Register (USBSTS)"]
pub mod uhci004;
#[doc = "UHCI008 (rw) register accessor: USB Interrupt Enable Register (USBINT)\n\nYou can [`read`](crate::Reg::read) this register and get [`uhci008::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhci008::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uhci008`] module"]
#[doc(alias = "UHCI008")]
pub type Uhci008 = crate::Reg<uhci008::Uhci008Spec>;
#[doc = "USB Interrupt Enable Register (USBINT)"]
pub mod uhci008;
#[doc = "UHCI00C (rw) register accessor: Frame List Based Address Register (FRBASEADD)\n\nYou can [`read`](crate::Reg::read) this register and get [`uhci00c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhci00c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uhci00c`] module"]
#[doc(alias = "UHCI00C")]
pub type Uhci00c = crate::Reg<uhci00c::Uhci00cSpec>;
#[doc = "Frame List Based Address Register (FRBASEADD)"]
pub mod uhci00c;
#[doc = "UHCI040 (rw) register accessor: Test Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uhci040::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhci040::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uhci040`] module"]
#[doc(alias = "UHCI040")]
pub type Uhci040 = crate::Reg<uhci040::Uhci040Spec>;
#[doc = "Test Control Register"]
pub mod uhci040;
#[doc = "UHCI080 (rw) register accessor: Frame Number Register (FRNUM)\n\nYou can [`read`](crate::Reg::read) this register and get [`uhci080::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhci080::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uhci080`] module"]
#[doc(alias = "UHCI080")]
pub type Uhci080 = crate::Reg<uhci080::Uhci080Spec>;
#[doc = "Frame Number Register (FRNUM)"]
pub mod uhci080;
#[doc = "UHCI084 (rw) register accessor: Start of Frame Modify Register (SOFMOD)\n\nYou can [`read`](crate::Reg::read) this register and get [`uhci084::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhci084::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uhci084`] module"]
#[doc(alias = "UHCI084")]
pub type Uhci084 = crate::Reg<uhci084::Uhci084Spec>;
#[doc = "Start of Frame Modify Register (SOFMOD)"]
pub mod uhci084;
#[doc = "UHCI088 (rw) register accessor: Port1 Status/Control Register (PORTSC1)\n\nYou can [`read`](crate::Reg::read) this register and get [`uhci088::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhci088::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uhci088`] module"]
#[doc(alias = "UHCI088")]
pub type Uhci088 = crate::Reg<uhci088::Uhci088Spec>;
#[doc = "Port1 Status/Control Register (PORTSC1)"]
pub mod uhci088;
#[doc = "UHCI08C (rw) register accessor: Port2 Status/Control Register (PORTSC2)\n\nYou can [`read`](crate::Reg::read) this register and get [`uhci08c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhci08c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uhci08c`] module"]
#[doc(alias = "UHCI08C")]
pub type Uhci08c = crate::Reg<uhci08c::Uhci08cSpec>;
#[doc = "Port2 Status/Control Register (PORTSC2)"]
pub mod uhci08c;
