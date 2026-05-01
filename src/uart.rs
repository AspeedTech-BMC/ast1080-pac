#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved_0_uartdll: [u8; 0x04],
    _reserved_1_uartdlh: [u8; 0x04],
    _reserved_2_uartfcr: [u8; 0x04],
    uartlcr: Uartlcr,
    uartmcr: Uartmcr,
    uartlsr: Uartlsr,
    uartmsr: Uartmsr,
    uartscr: Uartscr,
}
impl RegisterBlock {
    #[doc = "0x00 - Divisor Latch Low Register : (DLAB = 1)"]
    #[inline(always)]
    pub const fn uartdll(&self) -> &Uartdll {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x00 - Transmit Holding Register (DLAB = 0)"]
    #[inline(always)]
    pub const fn uartthr(&self) -> &Uartthr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x00 - Receiving Buffer Register (DLAB = 0)"]
    #[inline(always)]
    pub const fn uartrbr(&self) -> &Uartrbr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x04 - Divisor Latch High Register : (DLAB = 1)"]
    #[inline(always)]
    pub const fn uartdlh(&self) -> &Uartdlh {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x04 - Interrupt Enable Register (DLAB = 0)"]
    #[inline(always)]
    pub const fn uartier(&self) -> &Uartier {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x08 - FIFO Control Register"]
    #[inline(always)]
    pub const fn uartfcr(&self) -> &Uartfcr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x08 - Interrupt Identity Register"]
    #[inline(always)]
    pub const fn uartiir(&self) -> &Uartiir {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x0c - Line Control Register"]
    #[inline(always)]
    pub const fn uartlcr(&self) -> &Uartlcr {
        &self.uartlcr
    }
    #[doc = "0x10 - Modem Control Register"]
    #[inline(always)]
    pub const fn uartmcr(&self) -> &Uartmcr {
        &self.uartmcr
    }
    #[doc = "0x14 - Line Status Register"]
    #[inline(always)]
    pub const fn uartlsr(&self) -> &Uartlsr {
        &self.uartlsr
    }
    #[doc = "0x18 - Modem Status Register"]
    #[inline(always)]
    pub const fn uartmsr(&self) -> &Uartmsr {
        &self.uartmsr
    }
    #[doc = "0x1c - Scratch Register"]
    #[inline(always)]
    pub const fn uartscr(&self) -> &Uartscr {
        &self.uartscr
    }
}
#[doc = "UARTRBR (rw) register accessor: Receiving Buffer Register (DLAB = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`uartrbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartrbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartrbr`] module"]
#[doc(alias = "UARTRBR")]
pub type Uartrbr = crate::Reg<uartrbr::UartrbrSpec>;
#[doc = "Receiving Buffer Register (DLAB = 0)"]
pub mod uartrbr;
#[doc = "UARTTHR (rw) register accessor: Transmit Holding Register (DLAB = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`uartthr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartthr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartthr`] module"]
#[doc(alias = "UARTTHR")]
pub type Uartthr = crate::Reg<uartthr::UartthrSpec>;
#[doc = "Transmit Holding Register (DLAB = 0)"]
pub mod uartthr;
#[doc = "UARTIER (rw) register accessor: Interrupt Enable Register (DLAB = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`uartier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartier`] module"]
#[doc(alias = "UARTIER")]
pub type Uartier = crate::Reg<uartier::UartierSpec>;
#[doc = "Interrupt Enable Register (DLAB = 0)"]
pub mod uartier;
#[doc = "UARTIIR (rw) register accessor: Interrupt Identity Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartiir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartiir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartiir`] module"]
#[doc(alias = "UARTIIR")]
pub type Uartiir = crate::Reg<uartiir::UartiirSpec>;
#[doc = "Interrupt Identity Register"]
pub mod uartiir;
#[doc = "UARTFCR (rw) register accessor: FIFO Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartfcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartfcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartfcr`] module"]
#[doc(alias = "UARTFCR")]
pub type Uartfcr = crate::Reg<uartfcr::UartfcrSpec>;
#[doc = "FIFO Control Register"]
pub mod uartfcr;
#[doc = "UARTLCR (rw) register accessor: Line Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartlcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartlcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartlcr`] module"]
#[doc(alias = "UARTLCR")]
pub type Uartlcr = crate::Reg<uartlcr::UartlcrSpec>;
#[doc = "Line Control Register"]
pub mod uartlcr;
#[doc = "UARTMCR (rw) register accessor: Modem Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartmcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartmcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartmcr`] module"]
#[doc(alias = "UARTMCR")]
pub type Uartmcr = crate::Reg<uartmcr::UartmcrSpec>;
#[doc = "Modem Control Register"]
pub mod uartmcr;
#[doc = "UARTLSR (rw) register accessor: Line Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartlsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartlsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartlsr`] module"]
#[doc(alias = "UARTLSR")]
pub type Uartlsr = crate::Reg<uartlsr::UartlsrSpec>;
#[doc = "Line Status Register"]
pub mod uartlsr;
#[doc = "UARTMSR (rw) register accessor: Modem Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartmsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartmsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartmsr`] module"]
#[doc(alias = "UARTMSR")]
pub type Uartmsr = crate::Reg<uartmsr::UartmsrSpec>;
#[doc = "Modem Status Register"]
pub mod uartmsr;
#[doc = "UARTSCR (rw) register accessor: Scratch Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartscr`] module"]
#[doc(alias = "UARTSCR")]
pub type Uartscr = crate::Reg<uartscr::UartscrSpec>;
#[doc = "Scratch Register"]
pub mod uartscr;
#[doc = "UARTDLL (rw) register accessor: Divisor Latch Low Register : (DLAB = 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdll::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdll::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdll`] module"]
#[doc(alias = "UARTDLL")]
pub type Uartdll = crate::Reg<uartdll::UartdllSpec>;
#[doc = "Divisor Latch Low Register : (DLAB = 1)"]
pub mod uartdll;
#[doc = "UARTDLH (rw) register accessor: Divisor Latch High Register : (DLAB = 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdlh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdlh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartdlh`] module"]
#[doc(alias = "UARTDLH")]
pub type Uartdlh = crate::Reg<uartdlh::UartdlhSpec>;
#[doc = "Divisor Latch High Register : (DLAB = 1)"]
pub mod uartdlh;
