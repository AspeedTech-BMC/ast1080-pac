#[doc = "Register `WDT028` reader"]
pub type R = crate::R<Wdt028Spec>;
#[doc = "Register `WDT028` writer"]
pub type W = crate::W<Wdt028Spec>;
#[doc = "Field `EnblRstARMRelatedCtrls` reader - Enable reset ARM related controllers"]
pub type EnblRstArmrelatedCtrlsR = crate::BitReader;
#[doc = "Field `EnblRstARMRelatedCtrls` writer - Enable reset ARM related controllers"]
pub type EnblRstArmrelatedCtrlsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblRstSOCCtrl` reader - Enable reset SOC controller"]
pub type EnblRstSocctrlR = crate::BitReader;
#[doc = "Field `EnblRstSOCCtrl` writer - Enable reset SOC controller"]
pub type EnblRstSocctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblRstAHBBridges` reader - Enable reset AHB bridges"]
pub type EnblRstAhbbridgesR = crate::BitReader;
#[doc = "Field `EnblRstAHBBridges` writer - Enable reset AHB bridges"]
pub type EnblRstAhbbridgesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved7` reader - Reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `Reserved7` writer - Reserved"]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblRstUART0Ctrl` reader - Enable reset UART #0 controller"]
pub type EnblRstUart0ctrlR = crate::BitReader;
#[doc = "Field `EnblRstUART0Ctrl` writer - Enable reset UART #0 controller"]
pub type EnblRstUart0ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblRstURAT1Ctrl` reader - Enable reset URAT #1 controller"]
pub type EnblRstUrat1ctrlR = crate::BitReader;
#[doc = "Field `EnblRstURAT1Ctrl` writer - Enable reset URAT #1 controller"]
pub type EnblRstUrat1ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblRstUART2Ctrl` reader - Enable reset UART #2 controller"]
pub type EnblRstUart2ctrlR = crate::BitReader;
#[doc = "Field `EnblRstUART2Ctrl` writer - Enable reset UART #2 controller"]
pub type EnblRstUart2ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblRstUART3Ctrl` reader - Enable reset UART #3 controller"]
pub type EnblRstUart3ctrlR = crate::BitReader;
#[doc = "Field `EnblRstUART3Ctrl` writer - Enable reset UART #3 controller"]
pub type EnblRstUart3ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved5` writer - Reserved"]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblRstSMBusFilterCtrl` reader - Enable reset SMBus Filter controller"]
pub type EnblRstSmbusFilterCtrlR = crate::BitReader;
#[doc = "Field `EnblRstSMBusFilterCtrl` writer - Enable reset SMBus Filter controller"]
pub type EnblRstSmbusFilterCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblRstSAFSCtrl` reader - Enable reset SAFS controller"]
pub type EnblRstSafsctrlR = crate::BitReader;
#[doc = "Field `EnblRstSAFSCtrl` writer - Enable reset SAFS controller"]
pub type EnblRstSafsctrlW<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `EnblRstGPIOCtrl` reader - Enable reset GPIO controller"]
pub type EnblRstGpioctrlR = crate::BitReader;
#[doc = "Field `EnblRstGPIOCtrl` writer - Enable reset GPIO controller"]
pub type EnblRstGpioctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblRstRTCCtrl` reader - Enable reset RTC controller"]
pub type EnblRstRtcctrlR = crate::BitReader;
#[doc = "Field `EnblRstRTCCtrl` writer - Enable reset RTC controller"]
pub type EnblRstRtcctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblRstTimerCtrl` reader - Enable reset Timer controller"]
pub type EnblRstTimerCtrlR = crate::BitReader;
#[doc = "Field `EnblRstTimerCtrl` writer - Enable reset Timer controller"]
pub type EnblRstTimerCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblRstWDTCtrl` reader - Enable reset WDT controller"]
pub type EnblRstWdtctrlR = crate::BitReader;
#[doc = "Field `EnblRstWDTCtrl` writer - Enable reset WDT controller"]
pub type EnblRstWdtctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblRstUART4567891011Ctrl` reader - Enable reset UART #4/#5/#6/#7/#8/#9/#10/#11 controller"]
pub type EnblRstUart4567891011ctrlR = crate::BitReader;
#[doc = "Field `EnblRstUART4567891011Ctrl` writer - Enable reset UART #4/#5/#6/#7/#8/#9/#10/#11 controller"]
pub type EnblRstUart4567891011ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EnblRstBootMCUCtrl` reader - Enable reset BootMCU controller"]
pub type EnblRstBootMcuctrlR = crate::BitReader;
#[doc = "Field `EnblRstBootMCUCtrl` writer - Enable reset BootMCU controller"]
pub type EnblRstBootMcuctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EnblRstCaliptra` reader - Enable reset Caliptra"]
pub type EnblRstCaliptraR = crate::BitReader;
#[doc = "Field `EnblRstCaliptra` writer - Enable reset Caliptra"]
pub type EnblRstCaliptraW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblRstI3CGlobalCtrl` reader - Enable reset I3C global controller"]
pub type EnblRstI3cglobalCtrlR = crate::BitReader;
#[doc = "Field `EnblRstI3CGlobalCtrl` writer - Enable reset I3C global controller"]
pub type EnblRstI3cglobalCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable reset ARM related controllers"]
    #[inline(always)]
    pub fn enbl_rst_armrelated_ctrls(&self) -> EnblRstArmrelatedCtrlsR {
        EnblRstArmrelatedCtrlsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable reset SOC controller"]
    #[inline(always)]
    pub fn enbl_rst_socctrl(&self) -> EnblRstSocctrlR {
        EnblRstSocctrlR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable reset AHB bridges"]
    #[inline(always)]
    pub fn enbl_rst_ahbbridges(&self) -> EnblRstAhbbridgesR {
        EnblRstAhbbridgesR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable reset UART #0 controller"]
    #[inline(always)]
    pub fn enbl_rst_uart0ctrl(&self) -> EnblRstUart0ctrlR {
        EnblRstUart0ctrlR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable reset URAT #1 controller"]
    #[inline(always)]
    pub fn enbl_rst_urat1ctrl(&self) -> EnblRstUrat1ctrlR {
        EnblRstUrat1ctrlR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable reset UART #2 controller"]
    #[inline(always)]
    pub fn enbl_rst_uart2ctrl(&self) -> EnblRstUart2ctrlR {
        EnblRstUart2ctrlR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable reset UART #3 controller"]
    #[inline(always)]
    pub fn enbl_rst_uart3ctrl(&self) -> EnblRstUart3ctrlR {
        EnblRstUart3ctrlR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable reset SMBus Filter controller"]
    #[inline(always)]
    pub fn enbl_rst_smbus_filter_ctrl(&self) -> EnblRstSmbusFilterCtrlR {
        EnblRstSmbusFilterCtrlR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable reset SAFS controller"]
    #[inline(always)]
    pub fn enbl_rst_safsctrl(&self) -> EnblRstSafsctrlR {
        EnblRstSafsctrlR::new(((self.bits >> 9) & 1) != 0)
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
    #[doc = "Bit 11 - Enable reset GPIO controller"]
    #[inline(always)]
    pub fn enbl_rst_gpioctrl(&self) -> EnblRstGpioctrlR {
        EnblRstGpioctrlR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable reset RTC controller"]
    #[inline(always)]
    pub fn enbl_rst_rtcctrl(&self) -> EnblRstRtcctrlR {
        EnblRstRtcctrlR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable reset Timer controller"]
    #[inline(always)]
    pub fn enbl_rst_timer_ctrl(&self) -> EnblRstTimerCtrlR {
        EnblRstTimerCtrlR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable reset WDT controller"]
    #[inline(always)]
    pub fn enbl_rst_wdtctrl(&self) -> EnblRstWdtctrlR {
        EnblRstWdtctrlR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable reset UART #4/#5/#6/#7/#8/#9/#10/#11 controller"]
    #[inline(always)]
    pub fn enbl_rst_uart4567891011ctrl(&self) -> EnblRstUart4567891011ctrlR {
        EnblRstUart4567891011ctrlR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Enable reset BootMCU controller"]
    #[inline(always)]
    pub fn enbl_rst_boot_mcuctrl(&self) -> EnblRstBootMcuctrlR {
        EnblRstBootMcuctrlR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 21) & 0xff) as u8)
    }
    #[doc = "Bit 29 - Enable reset Caliptra"]
    #[inline(always)]
    pub fn enbl_rst_caliptra(&self) -> EnblRstCaliptraR {
        EnblRstCaliptraR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable reset I3C global controller"]
    #[inline(always)]
    pub fn enbl_rst_i3cglobal_ctrl(&self) -> EnblRstI3cglobalCtrlR {
        EnblRstI3cglobalCtrlR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable reset ARM related controllers"]
    #[inline(always)]
    pub fn enbl_rst_armrelated_ctrls(&mut self) -> EnblRstArmrelatedCtrlsW<Wdt028Spec> {
        EnblRstArmrelatedCtrlsW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable reset SOC controller"]
    #[inline(always)]
    pub fn enbl_rst_socctrl(&mut self) -> EnblRstSocctrlW<Wdt028Spec> {
        EnblRstSocctrlW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable reset AHB bridges"]
    #[inline(always)]
    pub fn enbl_rst_ahbbridges(&mut self) -> EnblRstAhbbridgesW<Wdt028Spec> {
        EnblRstAhbbridgesW::new(self, 2)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<Wdt028Spec> {
        Reserved7W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable reset UART #0 controller"]
    #[inline(always)]
    pub fn enbl_rst_uart0ctrl(&mut self) -> EnblRstUart0ctrlW<Wdt028Spec> {
        EnblRstUart0ctrlW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable reset URAT #1 controller"]
    #[inline(always)]
    pub fn enbl_rst_urat1ctrl(&mut self) -> EnblRstUrat1ctrlW<Wdt028Spec> {
        EnblRstUrat1ctrlW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable reset UART #2 controller"]
    #[inline(always)]
    pub fn enbl_rst_uart2ctrl(&mut self) -> EnblRstUart2ctrlW<Wdt028Spec> {
        EnblRstUart2ctrlW::new(self, 6)
    }
    #[doc = "Bit 7 - Enable reset UART #3 controller"]
    #[inline(always)]
    pub fn enbl_rst_uart3ctrl(&mut self) -> EnblRstUart3ctrlW<Wdt028Spec> {
        EnblRstUart3ctrlW::new(self, 7)
    }
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<Wdt028Spec> {
        Reserved5W::new(self, 8)
    }
    #[doc = "Bit 8 - Enable reset SMBus Filter controller"]
    #[inline(always)]
    pub fn enbl_rst_smbus_filter_ctrl(&mut self) -> EnblRstSmbusFilterCtrlW<Wdt028Spec> {
        EnblRstSmbusFilterCtrlW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable reset SAFS controller"]
    #[inline(always)]
    pub fn enbl_rst_safsctrl(&mut self) -> EnblRstSafsctrlW<Wdt028Spec> {
        EnblRstSafsctrlW::new(self, 9)
    }
    #[doc = "Bits 9:10 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<Wdt028Spec> {
        Reserved6W::new(self, 9)
    }
    #[doc = "Bit 10 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Wdt028Spec> {
        Reserved3W::new(self, 10)
    }
    #[doc = "Bit 10 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<Wdt028Spec> {
        Reserved4W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable reset GPIO controller"]
    #[inline(always)]
    pub fn enbl_rst_gpioctrl(&mut self) -> EnblRstGpioctrlW<Wdt028Spec> {
        EnblRstGpioctrlW::new(self, 11)
    }
    #[doc = "Bit 12 - Enable reset RTC controller"]
    #[inline(always)]
    pub fn enbl_rst_rtcctrl(&mut self) -> EnblRstRtcctrlW<Wdt028Spec> {
        EnblRstRtcctrlW::new(self, 12)
    }
    #[doc = "Bit 13 - Enable reset Timer controller"]
    #[inline(always)]
    pub fn enbl_rst_timer_ctrl(&mut self) -> EnblRstTimerCtrlW<Wdt028Spec> {
        EnblRstTimerCtrlW::new(self, 13)
    }
    #[doc = "Bit 14 - Enable reset WDT controller"]
    #[inline(always)]
    pub fn enbl_rst_wdtctrl(&mut self) -> EnblRstWdtctrlW<Wdt028Spec> {
        EnblRstWdtctrlW::new(self, 14)
    }
    #[doc = "Bit 15 - Enable reset UART #4/#5/#6/#7/#8/#9/#10/#11 controller"]
    #[inline(always)]
    pub fn enbl_rst_uart4567891011ctrl(&mut self) -> EnblRstUart4567891011ctrlW<Wdt028Spec> {
        EnblRstUart4567891011ctrlW::new(self, 15)
    }
    #[doc = "Bits 16:19 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Wdt028Spec> {
        Reserved2W::new(self, 16)
    }
    #[doc = "Bit 20 - Enable reset BootMCU controller"]
    #[inline(always)]
    pub fn enbl_rst_boot_mcuctrl(&mut self) -> EnblRstBootMcuctrlW<Wdt028Spec> {
        EnblRstBootMcuctrlW::new(self, 20)
    }
    #[doc = "Bits 21:28 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Wdt028Spec> {
        Reserved1W::new(self, 21)
    }
    #[doc = "Bit 29 - Enable reset Caliptra"]
    #[inline(always)]
    pub fn enbl_rst_caliptra(&mut self) -> EnblRstCaliptraW<Wdt028Spec> {
        EnblRstCaliptraW::new(self, 29)
    }
    #[doc = "Bit 31 - Enable reset I3C global controller"]
    #[inline(always)]
    pub fn enbl_rst_i3cglobal_ctrl(&mut self) -> EnblRstI3cglobalCtrlW<Wdt028Spec> {
        EnblRstI3cglobalCtrlW::new(self, 31)
    }
}
#[doc = "WDTn Reset Mask Register \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`wdt028::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt028::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wdt028Spec;
impl crate::RegisterSpec for Wdt028Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdt028::R`](R) reader structure"]
impl crate::Readable for Wdt028Spec {}
#[doc = "`write(|w| ..)` method takes [`wdt028::W`](W) writer structure"]
impl crate::Writable for Wdt028Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WDT028 to value 0"]
impl crate::Resettable for Wdt028Spec {}
