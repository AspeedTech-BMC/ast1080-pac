#[doc = "Register `WDT070` reader"]
pub type R = crate::R<Wdt070Spec>;
#[doc = "Register `WDT070` writer"]
pub type W = crate::W<Wdt070Spec>;
#[doc = "Field `WrProtOfEnblSwModeRstARMRelatedCtrls` reader - Write Protection of Enable Software Mode reset ARM related controllers"]
pub type WrProtOfEnblSwModeRstArmrelatedCtrlsR = crate::BitReader;
#[doc = "Field `WrProtOfEnblSwModeRstARMRelatedCtrls` writer - Write Protection of Enable Software Mode reset ARM related controllers"]
pub type WrProtOfEnblSwModeRstArmrelatedCtrlsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrProtOfEnblSwModeRstSOCCtrl` reader - Write Protection of Enable Software Mode reset SOC controller"]
pub type WrProtOfEnblSwModeRstSocctrlR = crate::BitReader;
#[doc = "Field `WrProtOfEnblSwModeRstSOCCtrl` writer - Write Protection of Enable Software Mode reset SOC controller"]
pub type WrProtOfEnblSwModeRstSocctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrProtOfEnblSwModeRstAHBBridges` reader - Write Protection of Enable Software Mode reset AHB bridges"]
pub type WrProtOfEnblSwModeRstAhbbridgesR = crate::BitReader;
#[doc = "Field `WrProtOfEnblSwModeRstAHBBridges` writer - Write Protection of Enable Software Mode reset AHB bridges"]
pub type WrProtOfEnblSwModeRstAhbbridgesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved7` reader - Reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `Reserved7` writer - Reserved"]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrProtOfEnblSwModeRstUART0Ctrl` reader - Write Protection of Enable Software Mode reset UART #0 controller"]
pub type WrProtOfEnblSwModeRstUart0ctrlR = crate::BitReader;
#[doc = "Field `WrProtOfEnblSwModeRstUART0Ctrl` writer - Write Protection of Enable Software Mode reset UART #0 controller"]
pub type WrProtOfEnblSwModeRstUart0ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrProtOfEnblSwModeRstURAT1Ctrl` reader - Write Protection of Enable Software Mode reset URAT #1 controller"]
pub type WrProtOfEnblSwModeRstUrat1ctrlR = crate::BitReader;
#[doc = "Field `WrProtOfEnblSwModeRstURAT1Ctrl` writer - Write Protection of Enable Software Mode reset URAT #1 controller"]
pub type WrProtOfEnblSwModeRstUrat1ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrProtOfEnblSwModeRstUART2Ctrl` reader - Write Protection of Enable Software Mode reset UART #2 controller"]
pub type WrProtOfEnblSwModeRstUart2ctrlR = crate::BitReader;
#[doc = "Field `WrProtOfEnblSwModeRstUART2Ctrl` writer - Write Protection of Enable Software Mode reset UART #2 controller"]
pub type WrProtOfEnblSwModeRstUart2ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrProtOfEnblSwModeRstUART3Ctrl` reader - Write Protection of Enable Software Mode reset UART #3 controller"]
pub type WrProtOfEnblSwModeRstUart3ctrlR = crate::BitReader;
#[doc = "Field `WrProtOfEnblSwModeRstUART3Ctrl` writer - Write Protection of Enable Software Mode reset UART #3 controller"]
pub type WrProtOfEnblSwModeRstUart3ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved5` writer - Reserved"]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrProtOfEnblSwModeRstSMBusFilterCtrl` reader - Write Protection of Enable Software Mode reset SMBus Filter controller"]
pub type WrProtOfEnblSwModeRstSmbusFilterCtrlR = crate::BitReader;
#[doc = "Field `WrProtOfEnblSwModeRstSMBusFilterCtrl` writer - Write Protection of Enable Software Mode reset SMBus Filter controller"]
pub type WrProtOfEnblSwModeRstSmbusFilterCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrProtOfEnblSwModeRstSAFSCtrl` reader - Write Protection of Enable Software Mode reset SAFS controller"]
pub type WrProtOfEnblSwModeRstSafsctrlR = crate::BitReader;
#[doc = "Field `WrProtOfEnblSwModeRstSAFSCtrl` writer - Write Protection of Enable Software Mode reset SAFS controller"]
pub type WrProtOfEnblSwModeRstSafsctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::FieldReader;
#[doc = "Field `Reserved6` writer - Reserved"]
pub type Reserved6W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrProtOfEnblSwModeRstGPIOCtrl` reader - Write Protection of Enable Software Mode reset GPIO controller"]
pub type WrProtOfEnblSwModeRstGpioctrlR = crate::BitReader;
#[doc = "Field `WrProtOfEnblSwModeRstGPIOCtrl` writer - Write Protection of Enable Software Mode reset GPIO controller"]
pub type WrProtOfEnblSwModeRstGpioctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrProtOfEnblSwModeRstRTCCtrl` reader - Write Protection of Enable Software Mode reset RTC controller"]
pub type WrProtOfEnblSwModeRstRtcctrlR = crate::BitReader;
#[doc = "Field `WrProtOfEnblSwModeRstRTCCtrl` writer - Write Protection of Enable Software Mode reset RTC controller"]
pub type WrProtOfEnblSwModeRstRtcctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrProtOfEnblSwModeRstTimerCtrl` reader - Write Protection of Enable Software Mode reset Timer controller"]
pub type WrProtOfEnblSwModeRstTimerCtrlR = crate::BitReader;
#[doc = "Field `WrProtOfEnblSwModeRstTimerCtrl` writer - Write Protection of Enable Software Mode reset Timer controller"]
pub type WrProtOfEnblSwModeRstTimerCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrProtOfEnblSwModeRstWDTCtrl` reader - Write Protection of Enable Software Mode reset WDT controller"]
pub type WrProtOfEnblSwModeRstWdtctrlR = crate::BitReader;
#[doc = "Field `WrProtOfEnblSwModeRstWDTCtrl` writer - Write Protection of Enable Software Mode reset WDT controller"]
pub type WrProtOfEnblSwModeRstWdtctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrProtOfEnblSwModeRstUART4567891011Ctrl` reader - Write Protection of Enable Software Mode reset UART #4/#5/#6/#7/#8/#9/#10/#11 controller"]
pub type WrProtOfEnblSwModeRstUart4567891011ctrlR = crate::BitReader;
#[doc = "Field `WrProtOfEnblSwModeRstUART4567891011Ctrl` writer - Write Protection of Enable Software Mode reset UART #4/#5/#6/#7/#8/#9/#10/#11 controller"]
pub type WrProtOfEnblSwModeRstUart4567891011ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WrProtOfEnblSwModeRstBootMCUCtrl` reader - Write Protection of Enable Software Mode reset BootMCU controller"]
pub type WrProtOfEnblSwModeRstBootMcuctrlR = crate::BitReader;
#[doc = "Field `WrProtOfEnblSwModeRstBootMCUCtrl` writer - Write Protection of Enable Software Mode reset BootMCU controller"]
pub type WrProtOfEnblSwModeRstBootMcuctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WrProtOfEnblSwModeRstCaliptra` reader - Write Protection of Enable Software Mode reset Caliptra"]
pub type WrProtOfEnblSwModeRstCaliptraR = crate::BitReader;
#[doc = "Field `WrProtOfEnblSwModeRstCaliptra` writer - Write Protection of Enable Software Mode reset Caliptra"]
pub type WrProtOfEnblSwModeRstCaliptraW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrProtOfEnblSwModeRstI3CGlobalCtrl` reader - Write Protection of Enable Software Mode reset I3C global controller"]
pub type WrProtOfEnblSwModeRstI3cglobalCtrlR = crate::BitReader;
#[doc = "Field `WrProtOfEnblSwModeRstI3CGlobalCtrl` writer - Write Protection of Enable Software Mode reset I3C global controller"]
pub type WrProtOfEnblSwModeRstI3cglobalCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write Protection of Enable Software Mode reset ARM related controllers"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_armrelated_ctrls(
        &self,
    ) -> WrProtOfEnblSwModeRstArmrelatedCtrlsR {
        WrProtOfEnblSwModeRstArmrelatedCtrlsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write Protection of Enable Software Mode reset SOC controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_socctrl(&self) -> WrProtOfEnblSwModeRstSocctrlR {
        WrProtOfEnblSwModeRstSocctrlR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write Protection of Enable Software Mode reset AHB bridges"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_ahbbridges(&self) -> WrProtOfEnblSwModeRstAhbbridgesR {
        WrProtOfEnblSwModeRstAhbbridgesR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write Protection of Enable Software Mode reset UART #0 controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_uart0ctrl(&self) -> WrProtOfEnblSwModeRstUart0ctrlR {
        WrProtOfEnblSwModeRstUart0ctrlR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write Protection of Enable Software Mode reset URAT #1 controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_urat1ctrl(&self) -> WrProtOfEnblSwModeRstUrat1ctrlR {
        WrProtOfEnblSwModeRstUrat1ctrlR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write Protection of Enable Software Mode reset UART #2 controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_uart2ctrl(&self) -> WrProtOfEnblSwModeRstUart2ctrlR {
        WrProtOfEnblSwModeRstUart2ctrlR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write Protection of Enable Software Mode reset UART #3 controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_uart3ctrl(&self) -> WrProtOfEnblSwModeRstUart3ctrlR {
        WrProtOfEnblSwModeRstUart3ctrlR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 8 - Write Protection of Enable Software Mode reset SMBus Filter controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_smbus_filter_ctrl(
        &self,
    ) -> WrProtOfEnblSwModeRstSmbusFilterCtrlR {
        WrProtOfEnblSwModeRstSmbusFilterCtrlR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Write Protection of Enable Software Mode reset SAFS controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_safsctrl(&self) -> WrProtOfEnblSwModeRstSafsctrlR {
        WrProtOfEnblSwModeRstSafsctrlR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 9:10 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 10 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 10 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Write Protection of Enable Software Mode reset GPIO controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_gpioctrl(&self) -> WrProtOfEnblSwModeRstGpioctrlR {
        WrProtOfEnblSwModeRstGpioctrlR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Write Protection of Enable Software Mode reset RTC controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_rtcctrl(&self) -> WrProtOfEnblSwModeRstRtcctrlR {
        WrProtOfEnblSwModeRstRtcctrlR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Write Protection of Enable Software Mode reset Timer controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_timer_ctrl(&self) -> WrProtOfEnblSwModeRstTimerCtrlR {
        WrProtOfEnblSwModeRstTimerCtrlR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Write Protection of Enable Software Mode reset WDT controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_wdtctrl(&self) -> WrProtOfEnblSwModeRstWdtctrlR {
        WrProtOfEnblSwModeRstWdtctrlR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Write Protection of Enable Software Mode reset UART #4/#5/#6/#7/#8/#9/#10/#11 controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_uart4567891011ctrl(
        &self,
    ) -> WrProtOfEnblSwModeRstUart4567891011ctrlR {
        WrProtOfEnblSwModeRstUart4567891011ctrlR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Write Protection of Enable Software Mode reset BootMCU controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_boot_mcuctrl(&self) -> WrProtOfEnblSwModeRstBootMcuctrlR {
        WrProtOfEnblSwModeRstBootMcuctrlR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 21) & 0xff) as u8)
    }
    #[doc = "Bit 29 - Write Protection of Enable Software Mode reset Caliptra"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_caliptra(&self) -> WrProtOfEnblSwModeRstCaliptraR {
        WrProtOfEnblSwModeRstCaliptraR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Write Protection of Enable Software Mode reset I3C global controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_i3cglobal_ctrl(
        &self,
    ) -> WrProtOfEnblSwModeRstI3cglobalCtrlR {
        WrProtOfEnblSwModeRstI3cglobalCtrlR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write Protection of Enable Software Mode reset ARM related controllers"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_armrelated_ctrls(
        &mut self,
    ) -> WrProtOfEnblSwModeRstArmrelatedCtrlsW<Wdt070Spec> {
        WrProtOfEnblSwModeRstArmrelatedCtrlsW::new(self, 0)
    }
    #[doc = "Bit 1 - Write Protection of Enable Software Mode reset SOC controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_socctrl(
        &mut self,
    ) -> WrProtOfEnblSwModeRstSocctrlW<Wdt070Spec> {
        WrProtOfEnblSwModeRstSocctrlW::new(self, 1)
    }
    #[doc = "Bit 2 - Write Protection of Enable Software Mode reset AHB bridges"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_ahbbridges(
        &mut self,
    ) -> WrProtOfEnblSwModeRstAhbbridgesW<Wdt070Spec> {
        WrProtOfEnblSwModeRstAhbbridgesW::new(self, 2)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<Wdt070Spec> {
        Reserved7W::new(self, 3)
    }
    #[doc = "Bit 4 - Write Protection of Enable Software Mode reset UART #0 controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_uart0ctrl(
        &mut self,
    ) -> WrProtOfEnblSwModeRstUart0ctrlW<Wdt070Spec> {
        WrProtOfEnblSwModeRstUart0ctrlW::new(self, 4)
    }
    #[doc = "Bit 5 - Write Protection of Enable Software Mode reset URAT #1 controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_urat1ctrl(
        &mut self,
    ) -> WrProtOfEnblSwModeRstUrat1ctrlW<Wdt070Spec> {
        WrProtOfEnblSwModeRstUrat1ctrlW::new(self, 5)
    }
    #[doc = "Bit 6 - Write Protection of Enable Software Mode reset UART #2 controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_uart2ctrl(
        &mut self,
    ) -> WrProtOfEnblSwModeRstUart2ctrlW<Wdt070Spec> {
        WrProtOfEnblSwModeRstUart2ctrlW::new(self, 6)
    }
    #[doc = "Bit 7 - Write Protection of Enable Software Mode reset UART #3 controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_uart3ctrl(
        &mut self,
    ) -> WrProtOfEnblSwModeRstUart3ctrlW<Wdt070Spec> {
        WrProtOfEnblSwModeRstUart3ctrlW::new(self, 7)
    }
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<Wdt070Spec> {
        Reserved5W::new(self, 8)
    }
    #[doc = "Bit 8 - Write Protection of Enable Software Mode reset SMBus Filter controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_smbus_filter_ctrl(
        &mut self,
    ) -> WrProtOfEnblSwModeRstSmbusFilterCtrlW<Wdt070Spec> {
        WrProtOfEnblSwModeRstSmbusFilterCtrlW::new(self, 8)
    }
    #[doc = "Bit 9 - Write Protection of Enable Software Mode reset SAFS controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_safsctrl(
        &mut self,
    ) -> WrProtOfEnblSwModeRstSafsctrlW<Wdt070Spec> {
        WrProtOfEnblSwModeRstSafsctrlW::new(self, 9)
    }
    #[doc = "Bits 9:10 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<Wdt070Spec> {
        Reserved6W::new(self, 9)
    }
    #[doc = "Bit 10 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Wdt070Spec> {
        Reserved3W::new(self, 10)
    }
    #[doc = "Bit 10 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<Wdt070Spec> {
        Reserved4W::new(self, 10)
    }
    #[doc = "Bit 11 - Write Protection of Enable Software Mode reset GPIO controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_gpioctrl(
        &mut self,
    ) -> WrProtOfEnblSwModeRstGpioctrlW<Wdt070Spec> {
        WrProtOfEnblSwModeRstGpioctrlW::new(self, 11)
    }
    #[doc = "Bit 12 - Write Protection of Enable Software Mode reset RTC controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_rtcctrl(
        &mut self,
    ) -> WrProtOfEnblSwModeRstRtcctrlW<Wdt070Spec> {
        WrProtOfEnblSwModeRstRtcctrlW::new(self, 12)
    }
    #[doc = "Bit 13 - Write Protection of Enable Software Mode reset Timer controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_timer_ctrl(
        &mut self,
    ) -> WrProtOfEnblSwModeRstTimerCtrlW<Wdt070Spec> {
        WrProtOfEnblSwModeRstTimerCtrlW::new(self, 13)
    }
    #[doc = "Bit 14 - Write Protection of Enable Software Mode reset WDT controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_wdtctrl(
        &mut self,
    ) -> WrProtOfEnblSwModeRstWdtctrlW<Wdt070Spec> {
        WrProtOfEnblSwModeRstWdtctrlW::new(self, 14)
    }
    #[doc = "Bit 15 - Write Protection of Enable Software Mode reset UART #4/#5/#6/#7/#8/#9/#10/#11 controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_uart4567891011ctrl(
        &mut self,
    ) -> WrProtOfEnblSwModeRstUart4567891011ctrlW<Wdt070Spec> {
        WrProtOfEnblSwModeRstUart4567891011ctrlW::new(self, 15)
    }
    #[doc = "Bits 16:19 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Wdt070Spec> {
        Reserved2W::new(self, 16)
    }
    #[doc = "Bit 20 - Write Protection of Enable Software Mode reset BootMCU controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_boot_mcuctrl(
        &mut self,
    ) -> WrProtOfEnblSwModeRstBootMcuctrlW<Wdt070Spec> {
        WrProtOfEnblSwModeRstBootMcuctrlW::new(self, 20)
    }
    #[doc = "Bits 21:28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Wdt070Spec> {
        Reserved1W::new(self, 21)
    }
    #[doc = "Bit 29 - Write Protection of Enable Software Mode reset Caliptra"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_caliptra(
        &mut self,
    ) -> WrProtOfEnblSwModeRstCaliptraW<Wdt070Spec> {
        WrProtOfEnblSwModeRstCaliptraW::new(self, 29)
    }
    #[doc = "Bit 31 - Write Protection of Enable Software Mode reset I3C global controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_i3cglobal_ctrl(
        &mut self,
    ) -> WrProtOfEnblSwModeRstI3cglobalCtrlW<Wdt070Spec> {
        WrProtOfEnblSwModeRstI3cglobalCtrlW::new(self, 31)
    }
}
#[doc = "WDTn Reset Mask Write Protection Register \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`wdt070::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt070::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wdt070Spec;
impl crate::RegisterSpec for Wdt070Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdt070::R`](R) reader structure"]
impl crate::Readable for Wdt070Spec {}
#[doc = "`write(|w| ..)` method takes [`wdt070::W`](W) writer structure"]
impl crate::Writable for Wdt070Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WDT070 to value 0"]
impl crate::Resettable for Wdt070Spec {}
