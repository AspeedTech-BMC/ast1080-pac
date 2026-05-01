#[doc = "Register `UARTMCR` reader"]
pub type R = crate::R<UartmcrSpec>;
#[doc = "Register `UARTMCR` writer"]
pub type W = crate::W<UartmcrSpec>;
#[doc = "Field `DataTerminalReadyNDTRSigCtrl` reader - Data Terminal Ready (nDTR) signal control."]
pub type DataTerminalReadyNdtrsigCtrlR = crate::BitReader;
#[doc = "Field `DataTerminalReadyNDTRSigCtrl` writer - Data Terminal Ready (nDTR) signal control."]
pub type DataTerminalReadyNdtrsigCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ReqToSendNRTSSigCtrl` reader - Request To Send (nRTS) signal control."]
pub type ReqToSendNrtssigCtrlR = crate::BitReader;
#[doc = "Field `ReqToSendNRTSSigCtrl` writer - Request To Send (nRTS) signal control."]
pub type ReqToSendNrtssigCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Out1` reader - Out1."]
pub type Out1R = crate::BitReader;
#[doc = "Field `Out1` writer - Out1."]
pub type Out1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Out2` reader - Out2."]
pub type Out2R = crate::BitReader;
#[doc = "Field `Out2` writer - Out2."]
pub type Out2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LoopbackMode` reader - Loopback mode."]
pub type LoopbackModeR = crate::BitReader;
#[doc = "Field `LoopbackMode` writer - Loopback mode."]
pub type LoopbackModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Data Terminal Ready (nDTR) signal control."]
    #[inline(always)]
    pub fn data_terminal_ready_ndtrsig_ctrl(&self) -> DataTerminalReadyNdtrsigCtrlR {
        DataTerminalReadyNdtrsigCtrlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Request To Send (nRTS) signal control."]
    #[inline(always)]
    pub fn req_to_send_nrtssig_ctrl(&self) -> ReqToSendNrtssigCtrlR {
        ReqToSendNrtssigCtrlR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Out1."]
    #[inline(always)]
    pub fn out1(&self) -> Out1R {
        Out1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Out2."]
    #[inline(always)]
    pub fn out2(&self) -> Out2R {
        Out2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Loopback mode."]
    #[inline(always)]
    pub fn loopback_mode(&self) -> LoopbackModeR {
        LoopbackModeR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data Terminal Ready (nDTR) signal control."]
    #[inline(always)]
    pub fn data_terminal_ready_ndtrsig_ctrl(
        &mut self,
    ) -> DataTerminalReadyNdtrsigCtrlW<UartmcrSpec> {
        DataTerminalReadyNdtrsigCtrlW::new(self, 0)
    }
    #[doc = "Bit 1 - Request To Send (nRTS) signal control."]
    #[inline(always)]
    pub fn req_to_send_nrtssig_ctrl(&mut self) -> ReqToSendNrtssigCtrlW<UartmcrSpec> {
        ReqToSendNrtssigCtrlW::new(self, 1)
    }
    #[doc = "Bit 2 - Out1."]
    #[inline(always)]
    pub fn out1(&mut self) -> Out1W<UartmcrSpec> {
        Out1W::new(self, 2)
    }
    #[doc = "Bit 3 - Out2."]
    #[inline(always)]
    pub fn out2(&mut self) -> Out2W<UartmcrSpec> {
        Out2W::new(self, 3)
    }
    #[doc = "Bit 4 - Loopback mode."]
    #[inline(always)]
    pub fn loopback_mode(&mut self) -> LoopbackModeW<UartmcrSpec> {
        LoopbackModeW::new(self, 4)
    }
}
#[doc = "Modem Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartmcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartmcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartmcrSpec;
impl crate::RegisterSpec for UartmcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartmcr::R`](R) reader structure"]
impl crate::Readable for UartmcrSpec {}
#[doc = "`write(|w| ..)` method takes [`uartmcr::W`](W) writer structure"]
impl crate::Writable for UartmcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTMCR to value 0"]
impl crate::Resettable for UartmcrSpec {}
