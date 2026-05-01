#[doc = "Register `HUB00` reader"]
pub type R = crate::R<Hub00Spec>;
#[doc = "Register `HUB00` writer"]
pub type W = crate::W<Hub00Spec>;
#[doc = "Field `EnblUpstreamPortConnection` reader - Enable upstream port connection"]
pub type EnblUpstreamPortConnectionR = crate::BitReader;
#[doc = "Field `EnblUpstreamPortConnection` writer - Enable upstream port connection"]
pub type EnblUpstreamPortConnectionW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UpstreamPortConnectionSpeedSel` reader - Upstream port connection speed selection"]
pub type UpstreamPortConnectionSpeedSelR = crate::BitReader;
#[doc = "Field `UpstreamPortConnectionSpeedSel` writer - Upstream port connection speed selection"]
pub type UpstreamPortConnectionSpeedSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblClkStoppingInSuspendState` reader - Enable clock stopping in suspend state"]
pub type EnblClkStoppingInSuspendStateR = crate::BitReader;
#[doc = "Field `EnblClkStoppingInSuspendState` writer - Enable clock stopping in suspend state"]
pub type EnblClkStoppingInSuspendStateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblAutomaticRemoteWakeup` reader - Enable automatic Remote Wakeup"]
pub type EnblAutomaticRemoteWakeupR = crate::BitReader;
#[doc = "Field `EnblAutomaticRemoteWakeup` writer - Enable automatic Remote Wakeup"]
pub type EnblAutomaticRemoteWakeupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblManualRemoteWakeup` reader - Enable manual Remote Wakeup"]
pub type EnblManualRemoteWakeupR = crate::BitReader;
#[doc = "Field `EnblManualRemoteWakeup` writer - Enable manual Remote Wakeup"]
pub type EnblManualRemoteWakeupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBRemoteWakeupSigingPulseWidthSel` reader - USB Remote Wakeup signaling pulse width selection"]
pub type UsbremoteWakeupSigingPulseWidthSelR = crate::BitReader;
#[doc = "Field `USBRemoteWakeupSigingPulseWidthSel` writer - USB Remote Wakeup signaling pulse width selection"]
pub type UsbremoteWakeupSigingPulseWidthSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBForceToHighSpeedStateModeRegdebug` reader - USB Force to High Speed State Mode regdebug"]
pub type UsbforceToHighSpeedStateModeRegdebugR = crate::BitReader;
#[doc = "Field `USBForceToHighSpeedStateModeRegdebug` writer - USB Force to High Speed State Mode regdebug"]
pub type UsbforceToHighSpeedStateModeRegdebugW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ForceUSBBusStateTimerToWorkAtTestModeRegdebug` reader - Force USB bus state timer to work at test mode regdebug"]
pub type ForceUsbbusStateTimerToWorkAtTestModeRegdebugR = crate::BitReader;
#[doc = "Field `ForceUSBBusStateTimerToWorkAtTestModeRegdebug` writer - Force USB bus state timer to work at test mode regdebug"]
pub type ForceUsbbusStateTimerToWorkAtTestModeRegdebugW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBTestModeSel` reader - USB Test Mode selection"]
pub type UsbtestModeSelR = crate::FieldReader;
#[doc = "Field `USBTestModeSel` writer - USB Test Mode selection"]
pub type UsbtestModeSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DisUSBPHYRst` reader - Disable USB PHY reset"]
pub type DisUsbphyrstR = crate::BitReader;
#[doc = "Field `DisUSBPHYRst` writer - Disable USB PHY reset"]
pub type DisUsbphyrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBPHYBISTCtrl` reader - USB PHY BIST control"]
pub type UsbphybistctrlR = crate::BitReader;
#[doc = "Field `USBPHYBISTCtrl` writer - USB PHY BIST control"]
pub type UsbphybistctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBPHYBISTResult` reader - USB PHY BIST result"]
pub type UsbphybistresultR = crate::BitReader;
#[doc = "Field `Reserved01` reader - Reserved (0)"]
pub type Reserved01R = crate::FieldReader;
#[doc = "Field `CompleteASPLITINTransactionAfterSOFHasBeenRxd` reader - Complete a \"SPLIT IN Transaction\" after SOF has been received"]
pub type CompleteAsplitintransactionAfterSofhasBeenRxdR = crate::BitReader;
#[doc = "Field `CompleteASPLITINTransactionAfterSOFHasBeenRxd` writer - Complete a \"SPLIT IN Transaction\" after SOF has been received"]
pub type CompleteAsplitintransactionAfterSofhasBeenRxdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IsochronousINNullDataRespCtrl` reader - Isochronous IN null data response control"]
pub type IsochronousInnullDataRespCtrlR = crate::BitReader;
#[doc = "Field `IsochronousINNullDataRespCtrl` writer - Isochronous IN null data response control"]
pub type IsochronousInnullDataRespCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ProgrammableEndpointLongDescriptorListMode` reader - Programmable endpoint long descriptor list mode"]
pub type ProgrammableEndpointLongDescriptorListModeR = crate::BitReader;
#[doc = "Field `ProgrammableEndpointLongDescriptorListMode` writer - Programmable endpoint long descriptor list mode"]
pub type ProgrammableEndpointLongDescriptorListModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblFIFODynamicPowerDown` reader - Enable FIFO dynamic power down"]
pub type EnblFifodynamicPowerDownR = crate::BitReader;
#[doc = "Field `EnblFIFODynamicPowerDown` writer - Enable FIFO dynamic power down"]
pub type EnblFifodynamicPowerDownW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblIsochronousEndpointDescriptorPrefetchDrop` reader - Enable isochronous endpoint descriptor prefetch drop"]
pub type EnblIsochronousEndpointDescriptorPrefetchDropR = crate::BitReader;
#[doc = "Field `EnblIsochronousEndpointDescriptorPrefetchDrop` writer - Enable isochronous endpoint descriptor prefetch drop"]
pub type EnblIsochronousEndpointDescriptorPrefetchDropW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `0` reader - disable"]
pub type _0R = crate::BitReader;
#[doc = "Field `0` writer - disable"]
pub type _0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ClearSuspendStateForUSBDevs` reader - Clear suspend state for USB devices"]
pub type ClearSuspendStateForUsbdevsR = crate::BitReader;
#[doc = "Field `ClearSuspendStateForUSBDevs` writer - Clear suspend state for USB devices"]
pub type ClearSuspendStateForUsbdevsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader;
#[doc = "Field `Reserved1` reader - Reserved (1)"]
pub type Reserved1R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Enable upstream port connection"]
    #[inline(always)]
    pub fn enbl_upstream_port_connection(&self) -> EnblUpstreamPortConnectionR {
        EnblUpstreamPortConnectionR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Upstream port connection speed selection"]
    #[inline(always)]
    pub fn upstream_port_connection_speed_sel(&self) -> UpstreamPortConnectionSpeedSelR {
        UpstreamPortConnectionSpeedSelR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable clock stopping in suspend state"]
    #[inline(always)]
    pub fn enbl_clk_stopping_in_suspend_state(&self) -> EnblClkStoppingInSuspendStateR {
        EnblClkStoppingInSuspendStateR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable automatic Remote Wakeup"]
    #[inline(always)]
    pub fn enbl_automatic_remote_wakeup(&self) -> EnblAutomaticRemoteWakeupR {
        EnblAutomaticRemoteWakeupR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable manual Remote Wakeup"]
    #[inline(always)]
    pub fn enbl_manual_remote_wakeup(&self) -> EnblManualRemoteWakeupR {
        EnblManualRemoteWakeupR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USB Remote Wakeup signaling pulse width selection"]
    #[inline(always)]
    pub fn usbremote_wakeup_siging_pulse_width_sel(&self) -> UsbremoteWakeupSigingPulseWidthSelR {
        UsbremoteWakeupSigingPulseWidthSelR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - USB Force to High Speed State Mode regdebug"]
    #[inline(always)]
    pub fn usbforce_to_high_speed_state_mode_regdebug(
        &self,
    ) -> UsbforceToHighSpeedStateModeRegdebugR {
        UsbforceToHighSpeedStateModeRegdebugR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Force USB bus state timer to work at test mode regdebug"]
    #[inline(always)]
    pub fn force_usbbus_state_timer_to_work_at_test_mode_regdebug(
        &self,
    ) -> ForceUsbbusStateTimerToWorkAtTestModeRegdebugR {
        ForceUsbbusStateTimerToWorkAtTestModeRegdebugR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - USB Test Mode selection"]
    #[inline(always)]
    pub fn usbtest_mode_sel(&self) -> UsbtestModeSelR {
        UsbtestModeSelR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Disable USB PHY reset"]
    #[inline(always)]
    pub fn dis_usbphyrst(&self) -> DisUsbphyrstR {
        DisUsbphyrstR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - USB PHY BIST control"]
    #[inline(always)]
    pub fn usbphybistctrl(&self) -> UsbphybistctrlR {
        UsbphybistctrlR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - USB PHY BIST result"]
    #[inline(always)]
    pub fn usbphybistresult(&self) -> UsbphybistresultR {
        UsbphybistresultR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved01(&self) -> Reserved01R {
        Reserved01R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Complete a \"SPLIT IN Transaction\" after SOF has been received"]
    #[inline(always)]
    pub fn complete_asplitintransaction_after_sofhas_been_rxd(
        &self,
    ) -> CompleteAsplitintransactionAfterSofhasBeenRxdR {
        CompleteAsplitintransactionAfterSofhasBeenRxdR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Isochronous IN null data response control"]
    #[inline(always)]
    pub fn isochronous_innull_data_resp_ctrl(&self) -> IsochronousInnullDataRespCtrlR {
        IsochronousInnullDataRespCtrlR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Programmable endpoint long descriptor list mode"]
    #[inline(always)]
    pub fn programmable_endpoint_long_descriptor_list_mode(
        &self,
    ) -> ProgrammableEndpointLongDescriptorListModeR {
        ProgrammableEndpointLongDescriptorListModeR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable FIFO dynamic power down"]
    #[inline(always)]
    pub fn enbl_fifodynamic_power_down(&self) -> EnblFifodynamicPowerDownR {
        EnblFifodynamicPowerDownR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable isochronous endpoint descriptor prefetch drop"]
    #[inline(always)]
    pub fn enbl_isochronous_endpoint_descriptor_prefetch_drop(
        &self,
    ) -> EnblIsochronousEndpointDescriptorPrefetchDropR {
        EnblIsochronousEndpointDescriptorPrefetchDropR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - disable"]
    #[inline(always)]
    pub fn _0(&self) -> _0R {
        _0R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Clear suspend state for USB devices"]
    #[inline(always)]
    pub fn clear_suspend_state_for_usbdevs(&self) -> ClearSuspendStateForUsbdevsR {
        ClearSuspendStateForUsbdevsR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:30 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 23) & 0xff) as u8)
    }
    #[doc = "Bit 31 - Reserved (1)"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable upstream port connection"]
    #[inline(always)]
    pub fn enbl_upstream_port_connection(&mut self) -> EnblUpstreamPortConnectionW<Hub00Spec> {
        EnblUpstreamPortConnectionW::new(self, 0)
    }
    #[doc = "Bit 1 - Upstream port connection speed selection"]
    #[inline(always)]
    pub fn upstream_port_connection_speed_sel(
        &mut self,
    ) -> UpstreamPortConnectionSpeedSelW<Hub00Spec> {
        UpstreamPortConnectionSpeedSelW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable clock stopping in suspend state"]
    #[inline(always)]
    pub fn enbl_clk_stopping_in_suspend_state(
        &mut self,
    ) -> EnblClkStoppingInSuspendStateW<Hub00Spec> {
        EnblClkStoppingInSuspendStateW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable automatic Remote Wakeup"]
    #[inline(always)]
    pub fn enbl_automatic_remote_wakeup(&mut self) -> EnblAutomaticRemoteWakeupW<Hub00Spec> {
        EnblAutomaticRemoteWakeupW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable manual Remote Wakeup"]
    #[inline(always)]
    pub fn enbl_manual_remote_wakeup(&mut self) -> EnblManualRemoteWakeupW<Hub00Spec> {
        EnblManualRemoteWakeupW::new(self, 4)
    }
    #[doc = "Bit 5 - USB Remote Wakeup signaling pulse width selection"]
    #[inline(always)]
    pub fn usbremote_wakeup_siging_pulse_width_sel(
        &mut self,
    ) -> UsbremoteWakeupSigingPulseWidthSelW<Hub00Spec> {
        UsbremoteWakeupSigingPulseWidthSelW::new(self, 5)
    }
    #[doc = "Bit 6 - USB Force to High Speed State Mode regdebug"]
    #[inline(always)]
    pub fn usbforce_to_high_speed_state_mode_regdebug(
        &mut self,
    ) -> UsbforceToHighSpeedStateModeRegdebugW<Hub00Spec> {
        UsbforceToHighSpeedStateModeRegdebugW::new(self, 6)
    }
    #[doc = "Bit 7 - Force USB bus state timer to work at test mode regdebug"]
    #[inline(always)]
    pub fn force_usbbus_state_timer_to_work_at_test_mode_regdebug(
        &mut self,
    ) -> ForceUsbbusStateTimerToWorkAtTestModeRegdebugW<Hub00Spec> {
        ForceUsbbusStateTimerToWorkAtTestModeRegdebugW::new(self, 7)
    }
    #[doc = "Bits 8:10 - USB Test Mode selection"]
    #[inline(always)]
    pub fn usbtest_mode_sel(&mut self) -> UsbtestModeSelW<Hub00Spec> {
        UsbtestModeSelW::new(self, 8)
    }
    #[doc = "Bit 11 - Disable USB PHY reset"]
    #[inline(always)]
    pub fn dis_usbphyrst(&mut self) -> DisUsbphyrstW<Hub00Spec> {
        DisUsbphyrstW::new(self, 11)
    }
    #[doc = "Bit 12 - USB PHY BIST control"]
    #[inline(always)]
    pub fn usbphybistctrl(&mut self) -> UsbphybistctrlW<Hub00Spec> {
        UsbphybistctrlW::new(self, 12)
    }
    #[doc = "Bit 16 - Complete a \"SPLIT IN Transaction\" after SOF has been received"]
    #[inline(always)]
    pub fn complete_asplitintransaction_after_sofhas_been_rxd(
        &mut self,
    ) -> CompleteAsplitintransactionAfterSofhasBeenRxdW<Hub00Spec> {
        CompleteAsplitintransactionAfterSofhasBeenRxdW::new(self, 16)
    }
    #[doc = "Bit 17 - Isochronous IN null data response control"]
    #[inline(always)]
    pub fn isochronous_innull_data_resp_ctrl(
        &mut self,
    ) -> IsochronousInnullDataRespCtrlW<Hub00Spec> {
        IsochronousInnullDataRespCtrlW::new(self, 17)
    }
    #[doc = "Bit 18 - Programmable endpoint long descriptor list mode"]
    #[inline(always)]
    pub fn programmable_endpoint_long_descriptor_list_mode(
        &mut self,
    ) -> ProgrammableEndpointLongDescriptorListModeW<Hub00Spec> {
        ProgrammableEndpointLongDescriptorListModeW::new(self, 18)
    }
    #[doc = "Bit 19 - Enable FIFO dynamic power down"]
    #[inline(always)]
    pub fn enbl_fifodynamic_power_down(&mut self) -> EnblFifodynamicPowerDownW<Hub00Spec> {
        EnblFifodynamicPowerDownW::new(self, 19)
    }
    #[doc = "Bit 20 - Enable isochronous endpoint descriptor prefetch drop"]
    #[inline(always)]
    pub fn enbl_isochronous_endpoint_descriptor_prefetch_drop(
        &mut self,
    ) -> EnblIsochronousEndpointDescriptorPrefetchDropW<Hub00Spec> {
        EnblIsochronousEndpointDescriptorPrefetchDropW::new(self, 20)
    }
    #[doc = "Bit 21 - disable"]
    #[inline(always)]
    pub fn _0(&mut self) -> _0W<Hub00Spec> {
        _0W::new(self, 21)
    }
    #[doc = "Bit 22 - Clear suspend state for USB devices"]
    #[inline(always)]
    pub fn clear_suspend_state_for_usbdevs(&mut self) -> ClearSuspendStateForUsbdevsW<Hub00Spec> {
        ClearSuspendStateForUsbdevsW::new(self, 22)
    }
}
#[doc = "Root Function Control And Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hub00::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hub00::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hub00Spec;
impl crate::RegisterSpec for Hub00Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hub00::R`](R) reader structure"]
impl crate::Readable for Hub00Spec {}
#[doc = "`write(|w| ..)` method takes [`hub00::W`](W) writer structure"]
impl crate::Writable for Hub00Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HUB00 to value 0x8000_0000"]
impl crate::Resettable for Hub00Spec {
    const RESET_VALUE: u32 = 0x8000_0000;
}
