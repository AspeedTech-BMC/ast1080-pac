#[doc = "Register `UARTMSR` reader"]
pub type R = crate::R<UartmsrSpec>;
#[doc = "Register `UARTMSR` writer"]
pub type W = crate::W<UartmsrSpec>;
#[doc = "Field `DeltaClearToSendDCTSIndicatorReadClear` reader - Delta Clear To Send (DCTS) indicator (Read clear)"]
pub type DeltaClearToSendDctsindicatorReadClearR = crate::BitReader;
#[doc = "Field `DeltaDataSetReadyDDSRIndicatorReadClear` reader - Delta Data Set Ready (DDSR) indicator (Read clear)"]
pub type DeltaDataSetReadyDdsrindicatorReadClearR = crate::BitReader;
#[doc = "Field `TrailingEdgeOfRingIndicatorTERIDetectorReadClear` reader - Trailing Edge of Ring Indicator (TERI) detector (Read clear)"]
pub type TrailingEdgeOfRingIndicatorTeridetectorReadClearR = crate::BitReader;
#[doc = "Field `DeltaDataCarrierDetectDDCDIndicatorReadClear` reader - Delta Data Carrier Detect (DDCD) indicator (Read clear)"]
pub type DeltaDataCarrierDetectDdcdindicatorReadClearR = crate::BitReader;
#[doc = "Field `ComplementOfTheNCTSInputOrEqualsToNRTSMCR1InLoopbackMode` reader - Complement of the nCTS input or equals to nRTS(MCR\\[1\\]) in loopback mode."]
pub type ComplementOfTheNctsinputOrEqualsToNrtsmcr1inLoopbackModeR = crate::BitReader;
#[doc = "Field `ComplementOfTheNDSRInputOrEqualsToNDTRMCR0InLoopbackMode` reader - Complement of the nDSR input or equals to nDTR(MCR\\[0\\]) in loopback mode."]
pub type ComplementOfTheNdsrinputOrEqualsToNdtrmcr0inLoopbackModeR = crate::BitReader;
#[doc = "Field `ComplementOfTheNRIInputOrEqualsToOut1MCR2InLoopbackMode` reader - Complement of the nRI input or equals to Out1(MCR\\[2\\]) in loopback mode."]
pub type ComplementOfTheNriinputOrEqualsToOut1mcr2inLoopbackModeR = crate::BitReader;
#[doc = "Field `ComplementOfTheNDCDInputOrEqualsToOut2MCR3InLoopbackMode` reader - Complement of the nDCD input or equals to Out2(MCR\\[3\\]) in loopback mode."]
pub type ComplementOfTheNdcdinputOrEqualsToOut2mcr3inLoopbackModeR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Delta Clear To Send (DCTS) indicator (Read clear)"]
    #[inline(always)]
    pub fn delta_clear_to_send_dctsindicator_read_clear(
        &self,
    ) -> DeltaClearToSendDctsindicatorReadClearR {
        DeltaClearToSendDctsindicatorReadClearR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Delta Data Set Ready (DDSR) indicator (Read clear)"]
    #[inline(always)]
    pub fn delta_data_set_ready_ddsrindicator_read_clear(
        &self,
    ) -> DeltaDataSetReadyDdsrindicatorReadClearR {
        DeltaDataSetReadyDdsrindicatorReadClearR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Trailing Edge of Ring Indicator (TERI) detector (Read clear)"]
    #[inline(always)]
    pub fn trailing_edge_of_ring_indicator_teridetector_read_clear(
        &self,
    ) -> TrailingEdgeOfRingIndicatorTeridetectorReadClearR {
        TrailingEdgeOfRingIndicatorTeridetectorReadClearR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Delta Data Carrier Detect (DDCD) indicator (Read clear)"]
    #[inline(always)]
    pub fn delta_data_carrier_detect_ddcdindicator_read_clear(
        &self,
    ) -> DeltaDataCarrierDetectDdcdindicatorReadClearR {
        DeltaDataCarrierDetectDdcdindicatorReadClearR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Complement of the nCTS input or equals to nRTS(MCR\\[1\\]) in loopback mode."]
    #[inline(always)]
    pub fn complement_of_the_nctsinput_or_equals_to_nrtsmcr1in_loopback_mode(
        &self,
    ) -> ComplementOfTheNctsinputOrEqualsToNrtsmcr1inLoopbackModeR {
        ComplementOfTheNctsinputOrEqualsToNrtsmcr1inLoopbackModeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Complement of the nDSR input or equals to nDTR(MCR\\[0\\]) in loopback mode."]
    #[inline(always)]
    pub fn complement_of_the_ndsrinput_or_equals_to_ndtrmcr0in_loopback_mode(
        &self,
    ) -> ComplementOfTheNdsrinputOrEqualsToNdtrmcr0inLoopbackModeR {
        ComplementOfTheNdsrinputOrEqualsToNdtrmcr0inLoopbackModeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Complement of the nRI input or equals to Out1(MCR\\[2\\]) in loopback mode."]
    #[inline(always)]
    pub fn complement_of_the_nriinput_or_equals_to_out1mcr2in_loopback_mode(
        &self,
    ) -> ComplementOfTheNriinputOrEqualsToOut1mcr2inLoopbackModeR {
        ComplementOfTheNriinputOrEqualsToOut1mcr2inLoopbackModeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Complement of the nDCD input or equals to Out2(MCR\\[3\\]) in loopback mode."]
    #[inline(always)]
    pub fn complement_of_the_ndcdinput_or_equals_to_out2mcr3in_loopback_mode(
        &self,
    ) -> ComplementOfTheNdcdinputOrEqualsToOut2mcr3inLoopbackModeR {
        ComplementOfTheNdcdinputOrEqualsToOut2mcr3inLoopbackModeR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {}
#[doc = "Modem Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartmsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartmsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartmsrSpec;
impl crate::RegisterSpec for UartmsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartmsr::R`](R) reader structure"]
impl crate::Readable for UartmsrSpec {}
#[doc = "`write(|w| ..)` method takes [`uartmsr::W`](W) writer structure"]
impl crate::Writable for UartmsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTMSR to value 0"]
impl crate::Resettable for UartmsrSpec {}
