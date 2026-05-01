#[doc = "Register `WDT05C` reader"]
pub type R = crate::R<Wdt05cSpec>;
#[doc = "Register `WDT05C` writer"]
pub type W = crate::W<Wdt05cSpec>;
#[doc = "Field `WrProtOfEnblRstARMRelatedCtrls` reader - Write Protection of Enable reset ARM related controllers"]
pub type WrProtOfEnblRstArmrelatedCtrlsR = crate::BitReader;
#[doc = "Field `WrProtOfEnblRstARMRelatedCtrls` writer - Write Protection of Enable reset ARM related controllers"]
pub type WrProtOfEnblRstArmrelatedCtrlsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrProtOfEnblRstSOCCtrl` reader - Write Protection of Enable reset SOC controller"]
pub type WrProtOfEnblRstSocctrlR = crate::BitReader;
#[doc = "Field `WrProtOfEnblRstSOCCtrl` writer - Write Protection of Enable reset SOC controller"]
pub type WrProtOfEnblRstSocctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrProtOfEnblRstAHBBridges` reader - Write Protection of Enable reset AHB bridges"]
pub type WrProtOfEnblRstAhbbridgesR = crate::BitReader;
#[doc = "Field `WrProtOfEnblRstAHBBridges` writer - Write Protection of Enable reset AHB bridges"]
pub type WrProtOfEnblRstAhbbridgesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved7` reader - Reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `Reserved7` writer - Reserved"]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrProtOfEnblRstUART0Ctrl` reader - Write Protection of Enable reset UART #0 controller"]
pub type WrProtOfEnblRstUart0ctrlR = crate::BitReader;
#[doc = "Field `WrProtOfEnblRstUART0Ctrl` writer - Write Protection of Enable reset UART #0 controller"]
pub type WrProtOfEnblRstUart0ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrProtOfEnblRstURAT1Ctrl` reader - Write Protection of Enable reset URAT #1 controller"]
pub type WrProtOfEnblRstUrat1ctrlR = crate::BitReader;
#[doc = "Field `WrProtOfEnblRstURAT1Ctrl` writer - Write Protection of Enable reset URAT #1 controller"]
pub type WrProtOfEnblRstUrat1ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrProtOfEnblRstUART2Ctrl` reader - Write Protection of Enable reset UART #2 controller"]
pub type WrProtOfEnblRstUart2ctrlR = crate::BitReader;
#[doc = "Field `WrProtOfEnblRstUART2Ctrl` writer - Write Protection of Enable reset UART #2 controller"]
pub type WrProtOfEnblRstUart2ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrProtOfEnblRstUART3Ctrl` reader - Write Protection of Enable reset UART #3 controller"]
pub type WrProtOfEnblRstUart3ctrlR = crate::BitReader;
#[doc = "Field `WrProtOfEnblRstUART3Ctrl` writer - Write Protection of Enable reset UART #3 controller"]
pub type WrProtOfEnblRstUart3ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved5` writer - Reserved"]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrProtOfEnblRstSMBusFilterCtrl` reader - Write Protection of Enable reset SMBus Filter controller"]
pub type WrProtOfEnblRstSmbusFilterCtrlR = crate::BitReader;
#[doc = "Field `WrProtOfEnblRstSMBusFilterCtrl` writer - Write Protection of Enable reset SMBus Filter controller"]
pub type WrProtOfEnblRstSmbusFilterCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrProtOfEnblRstSAFSCtrl` reader - Write Protection of Enable reset SAFS controller"]
pub type WrProtOfEnblRstSafsctrlR = crate::BitReader;
#[doc = "Field `WrProtOfEnblRstSAFSCtrl` writer - Write Protection of Enable reset SAFS controller"]
pub type WrProtOfEnblRstSafsctrlW<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `WrProtOfEnblRstGPIOCtrl` reader - Write Protection of Enable reset GPIO controller"]
pub type WrProtOfEnblRstGpioctrlR = crate::BitReader;
#[doc = "Field `WrProtOfEnblRstGPIOCtrl` writer - Write Protection of Enable reset GPIO controller"]
pub type WrProtOfEnblRstGpioctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrProtOfEnblRstRTCCtrl` reader - Write Protection of Enable reset RTC controller"]
pub type WrProtOfEnblRstRtcctrlR = crate::BitReader;
#[doc = "Field `WrProtOfEnblRstRTCCtrl` writer - Write Protection of Enable reset RTC controller"]
pub type WrProtOfEnblRstRtcctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrProtOfEnblRstTimerCtrl` reader - Write Protection of Enable reset Timer controller"]
pub type WrProtOfEnblRstTimerCtrlR = crate::BitReader;
#[doc = "Field `WrProtOfEnblRstTimerCtrl` writer - Write Protection of Enable reset Timer controller"]
pub type WrProtOfEnblRstTimerCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrProtOfEnblRstWDTCtrl` reader - Write Protection of Enable reset WDT controller"]
pub type WrProtOfEnblRstWdtctrlR = crate::BitReader;
#[doc = "Field `WrProtOfEnblRstWDTCtrl` writer - Write Protection of Enable reset WDT controller"]
pub type WrProtOfEnblRstWdtctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrProtOfEnblRstUART4567891011Ctrl` reader - Write Protection of Enable reset UART #4/#5/#6/#7/#8/#9/#10/#11 controller"]
pub type WrProtOfEnblRstUart4567891011ctrlR = crate::BitReader;
#[doc = "Field `WrProtOfEnblRstUART4567891011Ctrl` writer - Write Protection of Enable reset UART #4/#5/#6/#7/#8/#9/#10/#11 controller"]
pub type WrProtOfEnblRstUart4567891011ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WrProtOfEnblRstBootMCUCtrl` reader - Write Protection of Enable reset BootMCU controller"]
pub type WrProtOfEnblRstBootMcuctrlR = crate::BitReader;
#[doc = "Field `WrProtOfEnblRstBootMCUCtrl` writer - Write Protection of Enable reset BootMCU controller"]
pub type WrProtOfEnblRstBootMcuctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WrProtOfEnblRstCaliptra` reader - Write Protection of Enable reset Caliptra"]
pub type WrProtOfEnblRstCaliptraR = crate::BitReader;
#[doc = "Field `WrProtOfEnblRstCaliptra` writer - Write Protection of Enable reset Caliptra"]
pub type WrProtOfEnblRstCaliptraW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrProtOfEnblRstI3CGlobalCtrl` reader - Write Protection of Enable reset I3C global controller"]
pub type WrProtOfEnblRstI3cglobalCtrlR = crate::BitReader;
#[doc = "Field `WrProtOfEnblRstI3CGlobalCtrl` writer - Write Protection of Enable reset I3C global controller"]
pub type WrProtOfEnblRstI3cglobalCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write Protection of Enable reset ARM related controllers"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_rst_armrelated_ctrls(&self) -> WrProtOfEnblRstArmrelatedCtrlsR {
        WrProtOfEnblRstArmrelatedCtrlsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write Protection of Enable reset SOC controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_rst_socctrl(&self) -> WrProtOfEnblRstSocctrlR {
        WrProtOfEnblRstSocctrlR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write Protection of Enable reset AHB bridges"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_rst_ahbbridges(&self) -> WrProtOfEnblRstAhbbridgesR {
        WrProtOfEnblRstAhbbridgesR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write Protection of Enable reset UART #0 controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_rst_uart0ctrl(&self) -> WrProtOfEnblRstUart0ctrlR {
        WrProtOfEnblRstUart0ctrlR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write Protection of Enable reset URAT #1 controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_rst_urat1ctrl(&self) -> WrProtOfEnblRstUrat1ctrlR {
        WrProtOfEnblRstUrat1ctrlR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write Protection of Enable reset UART #2 controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_rst_uart2ctrl(&self) -> WrProtOfEnblRstUart2ctrlR {
        WrProtOfEnblRstUart2ctrlR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write Protection of Enable reset UART #3 controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_rst_uart3ctrl(&self) -> WrProtOfEnblRstUart3ctrlR {
        WrProtOfEnblRstUart3ctrlR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 8 - Write Protection of Enable reset SMBus Filter controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_rst_smbus_filter_ctrl(&self) -> WrProtOfEnblRstSmbusFilterCtrlR {
        WrProtOfEnblRstSmbusFilterCtrlR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Write Protection of Enable reset SAFS controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_rst_safsctrl(&self) -> WrProtOfEnblRstSafsctrlR {
        WrProtOfEnblRstSafsctrlR::new(((self.bits >> 9) & 1) != 0)
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
    #[doc = "Bit 11 - Write Protection of Enable reset GPIO controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_rst_gpioctrl(&self) -> WrProtOfEnblRstGpioctrlR {
        WrProtOfEnblRstGpioctrlR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Write Protection of Enable reset RTC controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_rst_rtcctrl(&self) -> WrProtOfEnblRstRtcctrlR {
        WrProtOfEnblRstRtcctrlR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Write Protection of Enable reset Timer controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_rst_timer_ctrl(&self) -> WrProtOfEnblRstTimerCtrlR {
        WrProtOfEnblRstTimerCtrlR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Write Protection of Enable reset WDT controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_rst_wdtctrl(&self) -> WrProtOfEnblRstWdtctrlR {
        WrProtOfEnblRstWdtctrlR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Write Protection of Enable reset UART #4/#5/#6/#7/#8/#9/#10/#11 controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_rst_uart4567891011ctrl(&self) -> WrProtOfEnblRstUart4567891011ctrlR {
        WrProtOfEnblRstUart4567891011ctrlR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Write Protection of Enable reset BootMCU controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_rst_boot_mcuctrl(&self) -> WrProtOfEnblRstBootMcuctrlR {
        WrProtOfEnblRstBootMcuctrlR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 21) & 0xff) as u8)
    }
    #[doc = "Bit 29 - Write Protection of Enable reset Caliptra"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_rst_caliptra(&self) -> WrProtOfEnblRstCaliptraR {
        WrProtOfEnblRstCaliptraR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Write Protection of Enable reset I3C global controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_rst_i3cglobal_ctrl(&self) -> WrProtOfEnblRstI3cglobalCtrlR {
        WrProtOfEnblRstI3cglobalCtrlR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write Protection of Enable reset ARM related controllers"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_rst_armrelated_ctrls(
        &mut self,
    ) -> WrProtOfEnblRstArmrelatedCtrlsW<Wdt05cSpec> {
        WrProtOfEnblRstArmrelatedCtrlsW::new(self, 0)
    }
    #[doc = "Bit 1 - Write Protection of Enable reset SOC controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_rst_socctrl(&mut self) -> WrProtOfEnblRstSocctrlW<Wdt05cSpec> {
        WrProtOfEnblRstSocctrlW::new(self, 1)
    }
    #[doc = "Bit 2 - Write Protection of Enable reset AHB bridges"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_rst_ahbbridges(&mut self) -> WrProtOfEnblRstAhbbridgesW<Wdt05cSpec> {
        WrProtOfEnblRstAhbbridgesW::new(self, 2)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<Wdt05cSpec> {
        Reserved7W::new(self, 3)
    }
    #[doc = "Bit 4 - Write Protection of Enable reset UART #0 controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_rst_uart0ctrl(&mut self) -> WrProtOfEnblRstUart0ctrlW<Wdt05cSpec> {
        WrProtOfEnblRstUart0ctrlW::new(self, 4)
    }
    #[doc = "Bit 5 - Write Protection of Enable reset URAT #1 controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_rst_urat1ctrl(&mut self) -> WrProtOfEnblRstUrat1ctrlW<Wdt05cSpec> {
        WrProtOfEnblRstUrat1ctrlW::new(self, 5)
    }
    #[doc = "Bit 6 - Write Protection of Enable reset UART #2 controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_rst_uart2ctrl(&mut self) -> WrProtOfEnblRstUart2ctrlW<Wdt05cSpec> {
        WrProtOfEnblRstUart2ctrlW::new(self, 6)
    }
    #[doc = "Bit 7 - Write Protection of Enable reset UART #3 controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_rst_uart3ctrl(&mut self) -> WrProtOfEnblRstUart3ctrlW<Wdt05cSpec> {
        WrProtOfEnblRstUart3ctrlW::new(self, 7)
    }
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<Wdt05cSpec> {
        Reserved5W::new(self, 8)
    }
    #[doc = "Bit 8 - Write Protection of Enable reset SMBus Filter controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_rst_smbus_filter_ctrl(
        &mut self,
    ) -> WrProtOfEnblRstSmbusFilterCtrlW<Wdt05cSpec> {
        WrProtOfEnblRstSmbusFilterCtrlW::new(self, 8)
    }
    #[doc = "Bit 9 - Write Protection of Enable reset SAFS controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_rst_safsctrl(&mut self) -> WrProtOfEnblRstSafsctrlW<Wdt05cSpec> {
        WrProtOfEnblRstSafsctrlW::new(self, 9)
    }
    #[doc = "Bits 9:10 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<Wdt05cSpec> {
        Reserved6W::new(self, 9)
    }
    #[doc = "Bit 10 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Wdt05cSpec> {
        Reserved3W::new(self, 10)
    }
    #[doc = "Bit 10 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<Wdt05cSpec> {
        Reserved4W::new(self, 10)
    }
    #[doc = "Bit 11 - Write Protection of Enable reset GPIO controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_rst_gpioctrl(&mut self) -> WrProtOfEnblRstGpioctrlW<Wdt05cSpec> {
        WrProtOfEnblRstGpioctrlW::new(self, 11)
    }
    #[doc = "Bit 12 - Write Protection of Enable reset RTC controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_rst_rtcctrl(&mut self) -> WrProtOfEnblRstRtcctrlW<Wdt05cSpec> {
        WrProtOfEnblRstRtcctrlW::new(self, 12)
    }
    #[doc = "Bit 13 - Write Protection of Enable reset Timer controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_rst_timer_ctrl(&mut self) -> WrProtOfEnblRstTimerCtrlW<Wdt05cSpec> {
        WrProtOfEnblRstTimerCtrlW::new(self, 13)
    }
    #[doc = "Bit 14 - Write Protection of Enable reset WDT controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_rst_wdtctrl(&mut self) -> WrProtOfEnblRstWdtctrlW<Wdt05cSpec> {
        WrProtOfEnblRstWdtctrlW::new(self, 14)
    }
    #[doc = "Bit 15 - Write Protection of Enable reset UART #4/#5/#6/#7/#8/#9/#10/#11 controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_rst_uart4567891011ctrl(
        &mut self,
    ) -> WrProtOfEnblRstUart4567891011ctrlW<Wdt05cSpec> {
        WrProtOfEnblRstUart4567891011ctrlW::new(self, 15)
    }
    #[doc = "Bits 16:19 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Wdt05cSpec> {
        Reserved2W::new(self, 16)
    }
    #[doc = "Bit 20 - Write Protection of Enable reset BootMCU controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_rst_boot_mcuctrl(&mut self) -> WrProtOfEnblRstBootMcuctrlW<Wdt05cSpec> {
        WrProtOfEnblRstBootMcuctrlW::new(self, 20)
    }
    #[doc = "Bits 21:28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Wdt05cSpec> {
        Reserved1W::new(self, 21)
    }
    #[doc = "Bit 29 - Write Protection of Enable reset Caliptra"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_rst_caliptra(&mut self) -> WrProtOfEnblRstCaliptraW<Wdt05cSpec> {
        WrProtOfEnblRstCaliptraW::new(self, 29)
    }
    #[doc = "Bit 31 - Write Protection of Enable reset I3C global controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_rst_i3cglobal_ctrl(
        &mut self,
    ) -> WrProtOfEnblRstI3cglobalCtrlW<Wdt05cSpec> {
        WrProtOfEnblRstI3cglobalCtrlW::new(self, 31)
    }
}
#[doc = "WDTn Reset Mask Write Protection Register \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`wdt05c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt05c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wdt05cSpec;
impl crate::RegisterSpec for Wdt05cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdt05c::R`](R) reader structure"]
impl crate::Readable for Wdt05cSpec {}
#[doc = "`write(|w| ..)` method takes [`wdt05c::W`](W) writer structure"]
impl crate::Writable for Wdt05cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WDT05C to value 0"]
impl crate::Resettable for Wdt05cSpec {}
