#[doc = "Register `WDT040` reader"]
pub type R = crate::R<Wdt040Spec>;
#[doc = "Register `WDT040` writer"]
pub type W = crate::W<Wdt040Spec>;
#[doc = "Field `EnblSwModeRstARMRelatedCtrls` reader - Enable Software Mode reset ARM related controllers"]
pub type EnblSwModeRstArmrelatedCtrlsR = crate::BitReader;
#[doc = "Field `EnblSwModeRstARMRelatedCtrls` writer - Enable Software Mode reset ARM related controllers"]
pub type EnblSwModeRstArmrelatedCtrlsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSwModeRstSOCCtrl` reader - Enable Software Mode reset SOC controller"]
pub type EnblSwModeRstSocctrlR = crate::BitReader;
#[doc = "Field `EnblSwModeRstSOCCtrl` writer - Enable Software Mode reset SOC controller"]
pub type EnblSwModeRstSocctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSwModeRstAHBBridges` reader - Enable Software Mode reset AHB bridges"]
pub type EnblSwModeRstAhbbridgesR = crate::BitReader;
#[doc = "Field `EnblSwModeRstAHBBridges` writer - Enable Software Mode reset AHB bridges"]
pub type EnblSwModeRstAhbbridgesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved7` reader - Reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `Reserved7` writer - Reserved"]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSwModeRstUART0Ctrl` reader - Enable Software Mode reset UART #0 controller"]
pub type EnblSwModeRstUart0ctrlR = crate::BitReader;
#[doc = "Field `EnblSwModeRstUART0Ctrl` writer - Enable Software Mode reset UART #0 controller"]
pub type EnblSwModeRstUart0ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSwModeRstURAT1Ctrl` reader - Enable Software Mode reset URAT #1 controller"]
pub type EnblSwModeRstUrat1ctrlR = crate::BitReader;
#[doc = "Field `EnblSwModeRstURAT1Ctrl` writer - Enable Software Mode reset URAT #1 controller"]
pub type EnblSwModeRstUrat1ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSwModeRstUART2Ctrl` reader - Enable Software Mode reset UART #2 controller"]
pub type EnblSwModeRstUart2ctrlR = crate::BitReader;
#[doc = "Field `EnblSwModeRstUART2Ctrl` writer - Enable Software Mode reset UART #2 controller"]
pub type EnblSwModeRstUart2ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSwModeRstUART3Ctrl` reader - Enable Software Mode reset UART #3 controller"]
pub type EnblSwModeRstUart3ctrlR = crate::BitReader;
#[doc = "Field `EnblSwModeRstUART3Ctrl` writer - Enable Software Mode reset UART #3 controller"]
pub type EnblSwModeRstUart3ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved5` writer - Reserved"]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSwModeRstSMBusFilterCtrl` reader - Enable Software Mode reset SMBus Filter controller"]
pub type EnblSwModeRstSmbusFilterCtrlR = crate::BitReader;
#[doc = "Field `EnblSwModeRstSMBusFilterCtrl` writer - Enable Software Mode reset SMBus Filter controller"]
pub type EnblSwModeRstSmbusFilterCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSwModeRstSAFSCtrl` reader - Enable Software Mode reset SAFS controller"]
pub type EnblSwModeRstSafsctrlR = crate::BitReader;
#[doc = "Field `EnblSwModeRstSAFSCtrl` writer - Enable Software Mode reset SAFS controller"]
pub type EnblSwModeRstSafsctrlW<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `EnblSwModeRstGPIOCtrl` reader - Enable Software Mode reset GPIO controller"]
pub type EnblSwModeRstGpioctrlR = crate::BitReader;
#[doc = "Field `EnblSwModeRstGPIOCtrl` writer - Enable Software Mode reset GPIO controller"]
pub type EnblSwModeRstGpioctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSwModeRstRTCCtrl` reader - Enable Software Mode reset RTC controller"]
pub type EnblSwModeRstRtcctrlR = crate::BitReader;
#[doc = "Field `EnblSwModeRstRTCCtrl` writer - Enable Software Mode reset RTC controller"]
pub type EnblSwModeRstRtcctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSwModeRstTimerCtrl` reader - Enable Software Mode reset Timer controller"]
pub type EnblSwModeRstTimerCtrlR = crate::BitReader;
#[doc = "Field `EnblSwModeRstTimerCtrl` writer - Enable Software Mode reset Timer controller"]
pub type EnblSwModeRstTimerCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSwModeRstWDTCtrl` reader - Enable Software Mode reset WDT controller"]
pub type EnblSwModeRstWdtctrlR = crate::BitReader;
#[doc = "Field `EnblSwModeRstWDTCtrl` writer - Enable Software Mode reset WDT controller"]
pub type EnblSwModeRstWdtctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSwModeRstUART4567891011Ctrl` reader - Enable Software Mode reset UART #4/#5/#6/#7/#8/#9/#10/#11 controller"]
pub type EnblSwModeRstUart4567891011ctrlR = crate::BitReader;
#[doc = "Field `EnblSwModeRstUART4567891011Ctrl` writer - Enable Software Mode reset UART #4/#5/#6/#7/#8/#9/#10/#11 controller"]
pub type EnblSwModeRstUart4567891011ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EnblSwModeRstBootMCUCtrl` reader - Enable Software Mode reset BootMCU controller"]
pub type EnblSwModeRstBootMcuctrlR = crate::BitReader;
#[doc = "Field `EnblSwModeRstBootMCUCtrl` writer - Enable Software Mode reset BootMCU controller"]
pub type EnblSwModeRstBootMcuctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EnblSwModeRstCaliptra` reader - Enable Software Mode reset Caliptra"]
pub type EnblSwModeRstCaliptraR = crate::BitReader;
#[doc = "Field `EnblSwModeRstCaliptra` writer - Enable Software Mode reset Caliptra"]
pub type EnblSwModeRstCaliptraW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSwModeRstI3CGlobalCtrl` reader - Enable Software Mode reset I3C global controller"]
pub type EnblSwModeRstI3cglobalCtrlR = crate::BitReader;
#[doc = "Field `EnblSwModeRstI3CGlobalCtrl` writer - Enable Software Mode reset I3C global controller"]
pub type EnblSwModeRstI3cglobalCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Software Mode reset ARM related controllers"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_armrelated_ctrls(&self) -> EnblSwModeRstArmrelatedCtrlsR {
        EnblSwModeRstArmrelatedCtrlsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Software Mode reset SOC controller"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_socctrl(&self) -> EnblSwModeRstSocctrlR {
        EnblSwModeRstSocctrlR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Software Mode reset AHB bridges"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_ahbbridges(&self) -> EnblSwModeRstAhbbridgesR {
        EnblSwModeRstAhbbridgesR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Software Mode reset UART #0 controller"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_uart0ctrl(&self) -> EnblSwModeRstUart0ctrlR {
        EnblSwModeRstUart0ctrlR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Software Mode reset URAT #1 controller"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_urat1ctrl(&self) -> EnblSwModeRstUrat1ctrlR {
        EnblSwModeRstUrat1ctrlR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Software Mode reset UART #2 controller"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_uart2ctrl(&self) -> EnblSwModeRstUart2ctrlR {
        EnblSwModeRstUart2ctrlR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Software Mode reset UART #3 controller"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_uart3ctrl(&self) -> EnblSwModeRstUart3ctrlR {
        EnblSwModeRstUart3ctrlR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Software Mode reset SMBus Filter controller"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_smbus_filter_ctrl(&self) -> EnblSwModeRstSmbusFilterCtrlR {
        EnblSwModeRstSmbusFilterCtrlR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Software Mode reset SAFS controller"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_safsctrl(&self) -> EnblSwModeRstSafsctrlR {
        EnblSwModeRstSafsctrlR::new(((self.bits >> 9) & 1) != 0)
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
    #[doc = "Bit 11 - Enable Software Mode reset GPIO controller"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_gpioctrl(&self) -> EnblSwModeRstGpioctrlR {
        EnblSwModeRstGpioctrlR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Software Mode reset RTC controller"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_rtcctrl(&self) -> EnblSwModeRstRtcctrlR {
        EnblSwModeRstRtcctrlR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Software Mode reset Timer controller"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_timer_ctrl(&self) -> EnblSwModeRstTimerCtrlR {
        EnblSwModeRstTimerCtrlR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Software Mode reset WDT controller"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_wdtctrl(&self) -> EnblSwModeRstWdtctrlR {
        EnblSwModeRstWdtctrlR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Software Mode reset UART #4/#5/#6/#7/#8/#9/#10/#11 controller"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_uart4567891011ctrl(&self) -> EnblSwModeRstUart4567891011ctrlR {
        EnblSwModeRstUart4567891011ctrlR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Enable Software Mode reset BootMCU controller"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_boot_mcuctrl(&self) -> EnblSwModeRstBootMcuctrlR {
        EnblSwModeRstBootMcuctrlR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 21) & 0xff) as u8)
    }
    #[doc = "Bit 29 - Enable Software Mode reset Caliptra"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_caliptra(&self) -> EnblSwModeRstCaliptraR {
        EnblSwModeRstCaliptraR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Software Mode reset I3C global controller"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_i3cglobal_ctrl(&self) -> EnblSwModeRstI3cglobalCtrlR {
        EnblSwModeRstI3cglobalCtrlR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Software Mode reset ARM related controllers"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_armrelated_ctrls(
        &mut self,
    ) -> EnblSwModeRstArmrelatedCtrlsW<Wdt040Spec> {
        EnblSwModeRstArmrelatedCtrlsW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Software Mode reset SOC controller"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_socctrl(&mut self) -> EnblSwModeRstSocctrlW<Wdt040Spec> {
        EnblSwModeRstSocctrlW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Software Mode reset AHB bridges"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_ahbbridges(&mut self) -> EnblSwModeRstAhbbridgesW<Wdt040Spec> {
        EnblSwModeRstAhbbridgesW::new(self, 2)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<Wdt040Spec> {
        Reserved7W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Software Mode reset UART #0 controller"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_uart0ctrl(&mut self) -> EnblSwModeRstUart0ctrlW<Wdt040Spec> {
        EnblSwModeRstUart0ctrlW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Software Mode reset URAT #1 controller"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_urat1ctrl(&mut self) -> EnblSwModeRstUrat1ctrlW<Wdt040Spec> {
        EnblSwModeRstUrat1ctrlW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Software Mode reset UART #2 controller"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_uart2ctrl(&mut self) -> EnblSwModeRstUart2ctrlW<Wdt040Spec> {
        EnblSwModeRstUart2ctrlW::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Software Mode reset UART #3 controller"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_uart3ctrl(&mut self) -> EnblSwModeRstUart3ctrlW<Wdt040Spec> {
        EnblSwModeRstUart3ctrlW::new(self, 7)
    }
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<Wdt040Spec> {
        Reserved5W::new(self, 8)
    }
    #[doc = "Bit 8 - Enable Software Mode reset SMBus Filter controller"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_smbus_filter_ctrl(
        &mut self,
    ) -> EnblSwModeRstSmbusFilterCtrlW<Wdt040Spec> {
        EnblSwModeRstSmbusFilterCtrlW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Software Mode reset SAFS controller"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_safsctrl(&mut self) -> EnblSwModeRstSafsctrlW<Wdt040Spec> {
        EnblSwModeRstSafsctrlW::new(self, 9)
    }
    #[doc = "Bits 9:10 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<Wdt040Spec> {
        Reserved6W::new(self, 9)
    }
    #[doc = "Bit 10 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Wdt040Spec> {
        Reserved3W::new(self, 10)
    }
    #[doc = "Bit 10 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<Wdt040Spec> {
        Reserved4W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Software Mode reset GPIO controller"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_gpioctrl(&mut self) -> EnblSwModeRstGpioctrlW<Wdt040Spec> {
        EnblSwModeRstGpioctrlW::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Software Mode reset RTC controller"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_rtcctrl(&mut self) -> EnblSwModeRstRtcctrlW<Wdt040Spec> {
        EnblSwModeRstRtcctrlW::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Software Mode reset Timer controller"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_timer_ctrl(&mut self) -> EnblSwModeRstTimerCtrlW<Wdt040Spec> {
        EnblSwModeRstTimerCtrlW::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Software Mode reset WDT controller"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_wdtctrl(&mut self) -> EnblSwModeRstWdtctrlW<Wdt040Spec> {
        EnblSwModeRstWdtctrlW::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Software Mode reset UART #4/#5/#6/#7/#8/#9/#10/#11 controller"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_uart4567891011ctrl(
        &mut self,
    ) -> EnblSwModeRstUart4567891011ctrlW<Wdt040Spec> {
        EnblSwModeRstUart4567891011ctrlW::new(self, 15)
    }
    #[doc = "Bits 16:19 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Wdt040Spec> {
        Reserved2W::new(self, 16)
    }
    #[doc = "Bit 20 - Enable Software Mode reset BootMCU controller"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_boot_mcuctrl(&mut self) -> EnblSwModeRstBootMcuctrlW<Wdt040Spec> {
        EnblSwModeRstBootMcuctrlW::new(self, 20)
    }
    #[doc = "Bits 21:28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Wdt040Spec> {
        Reserved1W::new(self, 21)
    }
    #[doc = "Bit 29 - Enable Software Mode reset Caliptra"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_caliptra(&mut self) -> EnblSwModeRstCaliptraW<Wdt040Spec> {
        EnblSwModeRstCaliptraW::new(self, 29)
    }
    #[doc = "Bit 31 - Enable Software Mode reset I3C global controller"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_i3cglobal_ctrl(&mut self) -> EnblSwModeRstI3cglobalCtrlW<Wdt040Spec> {
        EnblSwModeRstI3cglobalCtrlW::new(self, 31)
    }
}
#[doc = "WDTn Software Mode Reset Mask Register \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`wdt040::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt040::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wdt040Spec;
impl crate::RegisterSpec for Wdt040Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdt040::R`](R) reader structure"]
impl crate::Readable for Wdt040Spec {}
#[doc = "`write(|w| ..)` method takes [`wdt040::W`](W) writer structure"]
impl crate::Writable for Wdt040Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WDT040 to value 0"]
impl crate::Resettable for Wdt040Spec {}
