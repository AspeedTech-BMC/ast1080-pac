#[doc = "Register `HUB0C` reader"]
pub type R = crate::R<Hub0cSpec>;
#[doc = "Register `HUB0C` writer"]
pub type W = crate::W<Hub0cSpec>;
#[doc = "Field `EP0SetupDataArrives` reader - EP0 Setup Data Arrives"]
pub type Ep0setupDataArrivesR = crate::BitReader;
#[doc = "Field `EP0SetupDataArrives` writer - EP0 Setup Data Arrives"]
pub type Ep0setupDataArrivesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP0OUTDataPktACKSTALLReturned` reader - EP0 OUT Data Packet ACK/STALL Returned"]
pub type Ep0outdataPktAckstallreturnedR = crate::BitReader;
#[doc = "Field `EP0OUTDataPktACKSTALLReturned` writer - EP0 OUT Data Packet ACK/STALL Returned"]
pub type Ep0outdataPktAckstallreturnedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP0OUTDataPktNAKReturned` reader - EP0 OUT Data Packet NAK Returned"]
pub type Ep0outdataPktNakreturnedR = crate::BitReader;
#[doc = "Field `EP0OUTDataPktNAKReturned` writer - EP0 OUT Data Packet NAK Returned"]
pub type Ep0outdataPktNakreturnedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP0INDataPktACKSTALLReturned` reader - EP0 IN Data Packet ACK/STALL Returned"]
pub type Ep0indataPktAckstallreturnedR = crate::BitReader;
#[doc = "Field `EP0INDataPktACKSTALLReturned` writer - EP0 IN Data Packet ACK/STALL Returned"]
pub type Ep0indataPktAckstallreturnedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP0INDataPktNAKReturned` reader - EP0 IN Data Packet NAK Returned"]
pub type Ep0indataPktNakreturnedR = crate::BitReader;
#[doc = "Field `EP0INDataPktNAKReturned` writer - EP0 IN Data Packet NAK Returned"]
pub type Ep0indataPktNakreturnedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP1INDataPktACKSTALLReturned` reader - EP1 IN Data Packet ACK/STALL Returned"]
pub type Ep1indataPktAckstallreturnedR = crate::BitReader;
#[doc = "Field `EP1INDataPktACKSTALLReturned` writer - EP1 IN Data Packet ACK/STALL Returned"]
pub type Ep1indataPktAckstallreturnedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBBusRstHasOccurred` reader - USB Bus Reset has Occurred"]
pub type UsbbusRstHasOccurredR = crate::BitReader;
#[doc = "Field `USBBusRstHasOccurred` writer - USB Bus Reset has Occurred"]
pub type UsbbusRstHasOccurredW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBSuspendEntryEventHasOccurred` reader - USB Suspend Entry event has occurred"]
pub type UsbsuspendEntryEventHasOccurredR = crate::BitReader;
#[doc = "Field `USBSuspendEntryEventHasOccurred` writer - USB Suspend Entry event has occurred"]
pub type UsbsuspendEntryEventHasOccurredW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBSuspendResumeEventHasOccurred` reader - USB Suspend Resume event has occurred"]
pub type UsbsuspendResumeEventHasOccurredR = crate::BitReader;
#[doc = "Field `USBSuspendResumeEventHasOccurred` writer - USB Suspend Resume event has occurred"]
pub type UsbsuspendResumeEventHasOccurredW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Dev1CtrlINTOccurs` reader - Device #1 Controller Interrupt Occurs"]
pub type Dev1ctrlIntoccursR = crate::BitReader;
#[doc = "Field `Dev2CtrlINTOccurs` reader - Device #2 Controller Interrupt Occurs"]
pub type Dev2ctrlIntoccursR = crate::BitReader;
#[doc = "Field `Dev3CtrlINTOccurs` reader - Device #3 Controller Interrupt Occurs"]
pub type Dev3ctrlIntoccursR = crate::BitReader;
#[doc = "Field `Dev4CtrlINTOccurs` reader - Device #4 Controller Interrupt Occurs"]
pub type Dev4ctrlIntoccursR = crate::BitReader;
#[doc = "Field `Dev5CtrlINTOccurs` reader - Device #5 Controller Interrupt Occurs"]
pub type Dev5ctrlIntoccursR = crate::BitReader;
#[doc = "Field `Dev6CtrlINTOccurs` reader - Device #6 Controller Interrupt Occurs"]
pub type Dev6ctrlIntoccursR = crate::BitReader;
#[doc = "Field `Dev7CtrlINTOccurs` reader - Device #7 Controller Interrupt Occurs"]
pub type Dev7ctrlIntoccursR = crate::BitReader;
#[doc = "Field `ProgrammableEndpointPoolACKSTALLINTOccurs` reader - Programmable Endpoint Pool ACK/STALL Interrupt Occurs"]
pub type ProgrammableEndpointPoolAckstallintoccursR = crate::BitReader;
#[doc = "Field `ProgrammableEndpointPoolNAKINTOccurs` reader - Programmable Endpoint Pool NAK Interrupt Occurs"]
pub type ProgrammableEndpointPoolNakintoccursR = crate::BitReader;
#[doc = "Field `DefeatureUSBCmdBusIsDeadLocke` reader - defeatureUSB command bus is dead locke"]
pub type DefeatureUsbcmdBusIsDeadLockeR = crate::BitReader;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u16>;
#[doc = "Field `DMAToMemoryFlushIdleSts` reader - DMA to Memory flush idle status"]
pub type DmatoMemoryFlushIdleStsR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - EP0 Setup Data Arrives"]
    #[inline(always)]
    pub fn ep0setup_data_arrives(&self) -> Ep0setupDataArrivesR {
        Ep0setupDataArrivesR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EP0 OUT Data Packet ACK/STALL Returned"]
    #[inline(always)]
    pub fn ep0outdata_pkt_ackstallreturned(&self) -> Ep0outdataPktAckstallreturnedR {
        Ep0outdataPktAckstallreturnedR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EP0 OUT Data Packet NAK Returned"]
    #[inline(always)]
    pub fn ep0outdata_pkt_nakreturned(&self) -> Ep0outdataPktNakreturnedR {
        Ep0outdataPktNakreturnedR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EP0 IN Data Packet ACK/STALL Returned"]
    #[inline(always)]
    pub fn ep0indata_pkt_ackstallreturned(&self) -> Ep0indataPktAckstallreturnedR {
        Ep0indataPktAckstallreturnedR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - EP0 IN Data Packet NAK Returned"]
    #[inline(always)]
    pub fn ep0indata_pkt_nakreturned(&self) -> Ep0indataPktNakreturnedR {
        Ep0indataPktNakreturnedR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - EP1 IN Data Packet ACK/STALL Returned"]
    #[inline(always)]
    pub fn ep1indata_pkt_ackstallreturned(&self) -> Ep1indataPktAckstallreturnedR {
        Ep1indataPktAckstallreturnedR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - USB Bus Reset has Occurred"]
    #[inline(always)]
    pub fn usbbus_rst_has_occurred(&self) -> UsbbusRstHasOccurredR {
        UsbbusRstHasOccurredR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USB Suspend Entry event has occurred"]
    #[inline(always)]
    pub fn usbsuspend_entry_event_has_occurred(&self) -> UsbsuspendEntryEventHasOccurredR {
        UsbsuspendEntryEventHasOccurredR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - USB Suspend Resume event has occurred"]
    #[inline(always)]
    pub fn usbsuspend_resume_event_has_occurred(&self) -> UsbsuspendResumeEventHasOccurredR {
        UsbsuspendResumeEventHasOccurredR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Device #1 Controller Interrupt Occurs"]
    #[inline(always)]
    pub fn dev1ctrl_intoccurs(&self) -> Dev1ctrlIntoccursR {
        Dev1ctrlIntoccursR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Device #2 Controller Interrupt Occurs"]
    #[inline(always)]
    pub fn dev2ctrl_intoccurs(&self) -> Dev2ctrlIntoccursR {
        Dev2ctrlIntoccursR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Device #3 Controller Interrupt Occurs"]
    #[inline(always)]
    pub fn dev3ctrl_intoccurs(&self) -> Dev3ctrlIntoccursR {
        Dev3ctrlIntoccursR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Device #4 Controller Interrupt Occurs"]
    #[inline(always)]
    pub fn dev4ctrl_intoccurs(&self) -> Dev4ctrlIntoccursR {
        Dev4ctrlIntoccursR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Device #5 Controller Interrupt Occurs"]
    #[inline(always)]
    pub fn dev5ctrl_intoccurs(&self) -> Dev5ctrlIntoccursR {
        Dev5ctrlIntoccursR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Device #6 Controller Interrupt Occurs"]
    #[inline(always)]
    pub fn dev6ctrl_intoccurs(&self) -> Dev6ctrlIntoccursR {
        Dev6ctrlIntoccursR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Device #7 Controller Interrupt Occurs"]
    #[inline(always)]
    pub fn dev7ctrl_intoccurs(&self) -> Dev7ctrlIntoccursR {
        Dev7ctrlIntoccursR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Programmable Endpoint Pool ACK/STALL Interrupt Occurs"]
    #[inline(always)]
    pub fn programmable_endpoint_pool_ackstallintoccurs(
        &self,
    ) -> ProgrammableEndpointPoolAckstallintoccursR {
        ProgrammableEndpointPoolAckstallintoccursR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Programmable Endpoint Pool NAK Interrupt Occurs"]
    #[inline(always)]
    pub fn programmable_endpoint_pool_nakintoccurs(&self) -> ProgrammableEndpointPoolNakintoccursR {
        ProgrammableEndpointPoolNakintoccursR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - defeatureUSB command bus is dead locke"]
    #[inline(always)]
    pub fn defeature_usbcmd_bus_is_dead_locke(&self) -> DefeatureUsbcmdBusIsDeadLockeR {
        DefeatureUsbcmdBusIsDeadLockeR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:30 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 19) & 0x0fff) as u16)
    }
    #[doc = "Bit 31 - DMA to Memory flush idle status"]
    #[inline(always)]
    pub fn dmato_memory_flush_idle_sts(&self) -> DmatoMemoryFlushIdleStsR {
        DmatoMemoryFlushIdleStsR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EP0 Setup Data Arrives"]
    #[inline(always)]
    pub fn ep0setup_data_arrives(&mut self) -> Ep0setupDataArrivesW<Hub0cSpec> {
        Ep0setupDataArrivesW::new(self, 0)
    }
    #[doc = "Bit 1 - EP0 OUT Data Packet ACK/STALL Returned"]
    #[inline(always)]
    pub fn ep0outdata_pkt_ackstallreturned(&mut self) -> Ep0outdataPktAckstallreturnedW<Hub0cSpec> {
        Ep0outdataPktAckstallreturnedW::new(self, 1)
    }
    #[doc = "Bit 2 - EP0 OUT Data Packet NAK Returned"]
    #[inline(always)]
    pub fn ep0outdata_pkt_nakreturned(&mut self) -> Ep0outdataPktNakreturnedW<Hub0cSpec> {
        Ep0outdataPktNakreturnedW::new(self, 2)
    }
    #[doc = "Bit 3 - EP0 IN Data Packet ACK/STALL Returned"]
    #[inline(always)]
    pub fn ep0indata_pkt_ackstallreturned(&mut self) -> Ep0indataPktAckstallreturnedW<Hub0cSpec> {
        Ep0indataPktAckstallreturnedW::new(self, 3)
    }
    #[doc = "Bit 4 - EP0 IN Data Packet NAK Returned"]
    #[inline(always)]
    pub fn ep0indata_pkt_nakreturned(&mut self) -> Ep0indataPktNakreturnedW<Hub0cSpec> {
        Ep0indataPktNakreturnedW::new(self, 4)
    }
    #[doc = "Bit 5 - EP1 IN Data Packet ACK/STALL Returned"]
    #[inline(always)]
    pub fn ep1indata_pkt_ackstallreturned(&mut self) -> Ep1indataPktAckstallreturnedW<Hub0cSpec> {
        Ep1indataPktAckstallreturnedW::new(self, 5)
    }
    #[doc = "Bit 6 - USB Bus Reset has Occurred"]
    #[inline(always)]
    pub fn usbbus_rst_has_occurred(&mut self) -> UsbbusRstHasOccurredW<Hub0cSpec> {
        UsbbusRstHasOccurredW::new(self, 6)
    }
    #[doc = "Bit 7 - USB Suspend Entry event has occurred"]
    #[inline(always)]
    pub fn usbsuspend_entry_event_has_occurred(
        &mut self,
    ) -> UsbsuspendEntryEventHasOccurredW<Hub0cSpec> {
        UsbsuspendEntryEventHasOccurredW::new(self, 7)
    }
    #[doc = "Bit 8 - USB Suspend Resume event has occurred"]
    #[inline(always)]
    pub fn usbsuspend_resume_event_has_occurred(
        &mut self,
    ) -> UsbsuspendResumeEventHasOccurredW<Hub0cSpec> {
        UsbsuspendResumeEventHasOccurredW::new(self, 8)
    }
}
#[doc = "Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hub0c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hub0c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hub0cSpec;
impl crate::RegisterSpec for Hub0cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hub0c::R`](R) reader structure"]
impl crate::Readable for Hub0cSpec {}
#[doc = "`write(|w| ..)` method takes [`hub0c::W`](W) writer structure"]
impl crate::Writable for Hub0cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HUB0C to value 0"]
impl crate::Resettable for Hub0cSpec {}
