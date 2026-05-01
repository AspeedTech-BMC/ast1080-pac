#[doc = "Register `HUB24` reader"]
pub type R = crate::R<Hub24Spec>;
#[doc = "Register `HUB24` writer"]
pub type W = crate::W<Hub24Spec>;
#[doc = "Field `DevAddrOfTheLastUSBTransaction` reader - Device Address of the Last USB Transaction"]
pub type DevAddrOfTheLastUsbtransactionR = crate::FieldReader;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::BitReader;
#[doc = "Field `EndpointNumberOfTheLastUSBTransaction` reader - Endpoint Number of the Last USB Transaction"]
pub type EndpointNumberOfTheLastUsbtransactionR = crate::FieldReader;
#[doc = "Field `UTMIStateOPMode` reader - UTMI State OPMode"]
pub type UtmistateOpmodeR = crate::FieldReader;
#[doc = "Field `UTMIStateTermSelect` reader - UTMI State TermSelect"]
pub type UtmistateTermSelectR = crate::BitReader;
#[doc = "Field `UTMIStateXcvrSelect` reader - UTMI State XcvrSelect"]
pub type UtmistateXcvrSelectR = crate::BitReader;
#[doc = "Field `USBLastFrameNumberRecord` reader - USB Last Frame Number record"]
pub type UsblastFrameNumberRecordR = crate::FieldReader<u16>;
#[doc = "Field `USBBusSpeed` reader - USB Bus Speed"]
pub type UsbbusSpeedR = crate::BitReader;
#[doc = "Field `USBBusLineStateDP` reader - USB Bus Line State DP"]
pub type UsbbusLineStateDpR = crate::BitReader;
#[doc = "Field `USBBusLineStateDN` reader - USB Bus Line State DN"]
pub type UsbbusLineStateDnR = crate::BitReader;
#[doc = "Field `USBBusRstState` reader - USB Bus Reset State"]
pub type UsbbusRstStateR = crate::BitReader;
#[doc = "Field `USBSuspendState` reader - USB Suspend State"]
pub type UsbsuspendStateR = crate::BitReader;
impl R {
    #[doc = "Bits 0:6 - Device Address of the Last USB Transaction"]
    #[inline(always)]
    pub fn dev_addr_of_the_last_usbtransaction(&self) -> DevAddrOfTheLastUsbtransactionR {
        DevAddrOfTheLastUsbtransactionR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Endpoint Number of the Last USB Transaction"]
    #[inline(always)]
    pub fn endpoint_number_of_the_last_usbtransaction(
        &self,
    ) -> EndpointNumberOfTheLastUsbtransactionR {
        EndpointNumberOfTheLastUsbtransactionR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - UTMI State OPMode"]
    #[inline(always)]
    pub fn utmistate_opmode(&self) -> UtmistateOpmodeR {
        UtmistateOpmodeR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - UTMI State TermSelect"]
    #[inline(always)]
    pub fn utmistate_term_select(&self) -> UtmistateTermSelectR {
        UtmistateTermSelectR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - UTMI State XcvrSelect"]
    #[inline(always)]
    pub fn utmistate_xcvr_select(&self) -> UtmistateXcvrSelectR {
        UtmistateXcvrSelectR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:26 - USB Last Frame Number record"]
    #[inline(always)]
    pub fn usblast_frame_number_record(&self) -> UsblastFrameNumberRecordR {
        UsblastFrameNumberRecordR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bit 27 - USB Bus Speed"]
    #[inline(always)]
    pub fn usbbus_speed(&self) -> UsbbusSpeedR {
        UsbbusSpeedR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - USB Bus Line State DP"]
    #[inline(always)]
    pub fn usbbus_line_state_dp(&self) -> UsbbusLineStateDpR {
        UsbbusLineStateDpR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - USB Bus Line State DN"]
    #[inline(always)]
    pub fn usbbus_line_state_dn(&self) -> UsbbusLineStateDnR {
        UsbbusLineStateDnR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - USB Bus Reset State"]
    #[inline(always)]
    pub fn usbbus_rst_state(&self) -> UsbbusRstStateR {
        UsbbusRstStateR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - USB Suspend State"]
    #[inline(always)]
    pub fn usbsuspend_state(&self) -> UsbsuspendStateR {
        UsbsuspendStateR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {}
#[doc = "USB Status Register \\regdebugh\n\nYou can [`read`](crate::Reg::read) this register and get [`hub24::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hub24::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hub24Spec;
impl crate::RegisterSpec for Hub24Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hub24::R`](R) reader structure"]
impl crate::Readable for Hub24Spec {}
#[doc = "`write(|w| ..)` method takes [`hub24::W`](W) writer structure"]
impl crate::Writable for Hub24Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HUB24 to value 0"]
impl crate::Resettable for Hub24Spec {}
