#[doc = "Register `UARTMCR` reader"]
pub type R = crate::R<UartmcrSpec>;
#[doc = "Register `UARTMCR` writer"]
pub type W = crate::W<UartmcrSpec>;
#[doc = "Data Terminal Ready (nDTR) signal control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DataTerminalReadyNdtrsigCtrl {
    #[doc = "0: nDTR is '1'"]
    NdtrIs1 = 0,
    #[doc = "1: nDTR is '0'"]
    NdtrIs0 = 1,
}
impl From<DataTerminalReadyNdtrsigCtrl> for bool {
    #[inline(always)]
    fn from(variant: DataTerminalReadyNdtrsigCtrl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DataTerminalReadyNDTRSigCtrl` reader - Data Terminal Ready (nDTR) signal control."]
pub type DataTerminalReadyNdtrsigCtrlR = crate::BitReader<DataTerminalReadyNdtrsigCtrl>;
impl DataTerminalReadyNdtrsigCtrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DataTerminalReadyNdtrsigCtrl {
        match self.bits {
            false => DataTerminalReadyNdtrsigCtrl::NdtrIs1,
            true => DataTerminalReadyNdtrsigCtrl::NdtrIs0,
        }
    }
    #[doc = "nDTR is '1'"]
    #[inline(always)]
    pub fn is_ndtr_is_1(&self) -> bool {
        *self == DataTerminalReadyNdtrsigCtrl::NdtrIs1
    }
    #[doc = "nDTR is '0'"]
    #[inline(always)]
    pub fn is_ndtr_is_0(&self) -> bool {
        *self == DataTerminalReadyNdtrsigCtrl::NdtrIs0
    }
}
#[doc = "Field `DataTerminalReadyNDTRSigCtrl` writer - Data Terminal Ready (nDTR) signal control."]
pub type DataTerminalReadyNdtrsigCtrlW<'a, REG> =
    crate::BitWriter<'a, REG, DataTerminalReadyNdtrsigCtrl>;
impl<'a, REG> DataTerminalReadyNdtrsigCtrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "nDTR is '1'"]
    #[inline(always)]
    pub fn ndtr_is_1(self) -> &'a mut crate::W<REG> {
        self.variant(DataTerminalReadyNdtrsigCtrl::NdtrIs1)
    }
    #[doc = "nDTR is '0'"]
    #[inline(always)]
    pub fn ndtr_is_0(self) -> &'a mut crate::W<REG> {
        self.variant(DataTerminalReadyNdtrsigCtrl::NdtrIs0)
    }
}
#[doc = "Request To Send (nRTS) signal control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReqToSendNrtssigCtrl {
    #[doc = "0: nRTS is '1'"]
    NrtsIs1 = 0,
    #[doc = "1: nRTS is '0'"]
    NrtsIs0 = 1,
}
impl From<ReqToSendNrtssigCtrl> for bool {
    #[inline(always)]
    fn from(variant: ReqToSendNrtssigCtrl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ReqToSendNRTSSigCtrl` reader - Request To Send (nRTS) signal control."]
pub type ReqToSendNrtssigCtrlR = crate::BitReader<ReqToSendNrtssigCtrl>;
impl ReqToSendNrtssigCtrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ReqToSendNrtssigCtrl {
        match self.bits {
            false => ReqToSendNrtssigCtrl::NrtsIs1,
            true => ReqToSendNrtssigCtrl::NrtsIs0,
        }
    }
    #[doc = "nRTS is '1'"]
    #[inline(always)]
    pub fn is_nrts_is_1(&self) -> bool {
        *self == ReqToSendNrtssigCtrl::NrtsIs1
    }
    #[doc = "nRTS is '0'"]
    #[inline(always)]
    pub fn is_nrts_is_0(&self) -> bool {
        *self == ReqToSendNrtssigCtrl::NrtsIs0
    }
}
#[doc = "Field `ReqToSendNRTSSigCtrl` writer - Request To Send (nRTS) signal control."]
pub type ReqToSendNrtssigCtrlW<'a, REG> = crate::BitWriter<'a, REG, ReqToSendNrtssigCtrl>;
impl<'a, REG> ReqToSendNrtssigCtrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "nRTS is '1'"]
    #[inline(always)]
    pub fn nrts_is_1(self) -> &'a mut crate::W<REG> {
        self.variant(ReqToSendNrtssigCtrl::NrtsIs1)
    }
    #[doc = "nRTS is '0'"]
    #[inline(always)]
    pub fn nrts_is_0(self) -> &'a mut crate::W<REG> {
        self.variant(ReqToSendNrtssigCtrl::NrtsIs0)
    }
}
#[doc = "Field `Out1` reader - Out1."]
pub type Out1R = crate::BitReader;
#[doc = "Field `Out1` writer - Out1."]
pub type Out1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Out2` reader - Out2."]
pub type Out2R = crate::BitReader;
#[doc = "Field `Out2` writer - Out2."]
pub type Out2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Loopback mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LoopbackMode {
    #[doc = "0: normal operation."]
    NormalOperation = 0,
    #[doc = "1: loopback mode. When in loopback mode, the Serial Output Signal (TXD) is set to logic '1'."]
    LoopbackModeWhenInLoopbackModeTheSerialOutputSignalTxdIsSetToLogic1 = 1,
}
impl From<LoopbackMode> for bool {
    #[inline(always)]
    fn from(variant: LoopbackMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LoopbackMode` reader - Loopback mode."]
pub type LoopbackModeR = crate::BitReader<LoopbackMode>;
impl LoopbackModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LoopbackMode {
        match self.bits {
            false => LoopbackMode::NormalOperation,
            true => {
                LoopbackMode::LoopbackModeWhenInLoopbackModeTheSerialOutputSignalTxdIsSetToLogic1
            }
        }
    }
    #[doc = "normal operation."]
    #[inline(always)]
    pub fn is_normal_operation(&self) -> bool {
        *self == LoopbackMode::NormalOperation
    }
    #[doc = "loopback mode. When in loopback mode, the Serial Output Signal (TXD) is set to logic '1'."]
    #[inline(always)]
    pub fn is_loopback_mode_when_in_loopback_mode_the_serial_output_signal_txd_is_set_to_logic_1(
        &self,
    ) -> bool {
        *self == LoopbackMode::LoopbackModeWhenInLoopbackModeTheSerialOutputSignalTxdIsSetToLogic1
    }
}
#[doc = "Field `LoopbackMode` writer - Loopback mode."]
pub type LoopbackModeW<'a, REG> = crate::BitWriter<'a, REG, LoopbackMode>;
impl<'a, REG> LoopbackModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal operation."]
    #[inline(always)]
    pub fn normal_operation(self) -> &'a mut crate::W<REG> {
        self.variant(LoopbackMode::NormalOperation)
    }
    #[doc = "loopback mode. When in loopback mode, the Serial Output Signal (TXD) is set to logic '1'."]
    #[inline(always)]
    pub fn loopback_mode_when_in_loopback_mode_the_serial_output_signal_txd_is_set_to_logic_1(
        self,
    ) -> &'a mut crate::W<REG> {
        self.variant(
            LoopbackMode::LoopbackModeWhenInLoopbackModeTheSerialOutputSignalTxdIsSetToLogic1,
        )
    }
}
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u32>;
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
    #[doc = "Bits 5:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 5) & 0x07ff_ffff)
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
