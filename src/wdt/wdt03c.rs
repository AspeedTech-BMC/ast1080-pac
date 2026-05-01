#[doc = "Register `WDT03C` reader"]
pub type R = crate::R<Wdt03cSpec>;
#[doc = "Register `WDT03C` writer"]
pub type W = crate::W<Wdt03cSpec>;
#[doc = "Field `EnblSwModeRstLSCRelatedCtrls` reader - Enable Software Mode reset LSC related controllers"]
pub type EnblSwModeRstLscrelatedCtrlsR = crate::BitReader;
#[doc = "Field `EnblSwModeRstLSCRelatedCtrls` writer - Enable Software Mode reset LSC related controllers"]
pub type EnblSwModeRstLscrelatedCtrlsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::FieldReader;
#[doc = "Field `Reserved5` writer - Reserved"]
pub type Reserved5W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::FieldReader;
#[doc = "Field `Reserved6` writer - Reserved"]
pub type Reserved6W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EnblSwModeRstPECIBridge` reader - Enable Software Mode reset PECI bridge"]
pub type EnblSwModeRstPecibridgeR = crate::BitReader;
#[doc = "Field `EnblSwModeRstPECIBridge` writer - Enable Software Mode reset PECI bridge"]
pub type EnblSwModeRstPecibridgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSwModeRstPWMCtrl` reader - Enable Software Mode reset PWM controller"]
pub type EnblSwModeRstPwmctrlR = crate::BitReader;
#[doc = "Field `EnblSwModeRstPWMCtrl` writer - Enable Software Mode reset PWM controller"]
pub type EnblSwModeRstPwmctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::FieldReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `EnblSwModeRstADCCtrl` reader - Enable Software Mode reset ADC controller"]
pub type EnblSwModeRstAdcctrlR = crate::BitReader;
#[doc = "Field `EnblSwModeRstADCCtrl` writer - Enable Software Mode reset ADC controller"]
pub type EnblSwModeRstAdcctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSwModeRstESPICtrl` reader - Enable Software Mode reset eSPI controller"]
pub type EnblSwModeRstEspictrlR = crate::BitReader;
#[doc = "Field `EnblSwModeRstESPICtrl` writer - Enable Software Mode reset eSPI controller"]
pub type EnblSwModeRstEspictrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSwModeRstESPICtrl1` reader - Enable Software Mode reset eSPI controller"]
pub type EnblSwModeRstEspictrl1R = crate::BitReader;
#[doc = "Field `EnblSwModeRstESPICtrl1` writer - Enable Software Mode reset eSPI controller"]
pub type EnblSwModeRstEspictrl1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSwModeRstJTAGMaster0Ctrl` reader - Enable Software Mode reset JTAG Master #0 controller"]
pub type EnblSwModeRstJtagmaster0ctrlR = crate::BitReader;
#[doc = "Field `EnblSwModeRstJTAGMaster0Ctrl` writer - Enable Software Mode reset JTAG Master #0 controller"]
pub type EnblSwModeRstJtagmaster0ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSwModeRstJTAGMasterCtrl` reader - Enable Software Mode reset JTAG Master controller"]
pub type EnblSwModeRstJtagmasterCtrlR = crate::BitReader;
#[doc = "Field `EnblSwModeRstJTAGMasterCtrl` writer - Enable Software Mode reset JTAG Master controller"]
pub type EnblSwModeRstJtagmasterCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSwModeRstJTAGMaster1Ctrl` reader - Enable Software Mode reset JTAG Master #1 controller"]
pub type EnblSwModeRstJtagmaster1ctrlR = crate::BitReader;
#[doc = "Field `EnblSwModeRstJTAGMaster1Ctrl` writer - Enable Software Mode reset JTAG Master #1 controller"]
pub type EnblSwModeRstJtagmaster1ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSwModeRstSPI0Ctrl` reader - Enable Software Mode reset SPI0 controller"]
pub type EnblSwModeRstSpi0ctrlR = crate::BitReader;
#[doc = "Field `EnblSwModeRstSPI0Ctrl` writer - Enable Software Mode reset SPI0 controller"]
pub type EnblSwModeRstSpi0ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSwModeRstSPI1Ctrl` reader - Enable Software Mode reset SPI1 controller"]
pub type EnblSwModeRstSpi1ctrlR = crate::BitReader;
#[doc = "Field `EnblSwModeRstSPI1Ctrl` writer - Enable Software Mode reset SPI1 controller"]
pub type EnblSwModeRstSpi1ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSwModeRstI3CBus0Ctrl` reader - Enable Software Mode reset I3C bus0 controller"]
pub type EnblSwModeRstI3cbus0ctrlR = crate::BitReader;
#[doc = "Field `EnblSwModeRstI3CBus0Ctrl` writer - Enable Software Mode reset I3C bus0 controller"]
pub type EnblSwModeRstI3cbus0ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSwModeRstI3CBus1Ctrl` reader - Enable Software Mode reset I3C bus1 controller"]
pub type EnblSwModeRstI3cbus1ctrlR = crate::BitReader;
#[doc = "Field `EnblSwModeRstI3CBus1Ctrl` writer - Enable Software Mode reset I3C bus1 controller"]
pub type EnblSwModeRstI3cbus1ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSwModeRstI3CBus2Ctrl` reader - Enable Software Mode reset I3C bus2 controller"]
pub type EnblSwModeRstI3cbus2ctrlR = crate::BitReader;
#[doc = "Field `EnblSwModeRstI3CBus2Ctrl` writer - Enable Software Mode reset I3C bus2 controller"]
pub type EnblSwModeRstI3cbus2ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSwModeRstI3CBus3Ctrl` reader - Enable Software Mode reset I3C bus3 controller"]
pub type EnblSwModeRstI3cbus3ctrlR = crate::BitReader;
#[doc = "Field `EnblSwModeRstI3CBus3Ctrl` writer - Enable Software Mode reset I3C bus3 controller"]
pub type EnblSwModeRstI3cbus3ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSwModeRstI3CBus4Ctrl` reader - Enable Software Mode reset I3C bus4 controller"]
pub type EnblSwModeRstI3cbus4ctrlR = crate::BitReader;
#[doc = "Field `EnblSwModeRstI3CBus4Ctrl` writer - Enable Software Mode reset I3C bus4 controller"]
pub type EnblSwModeRstI3cbus4ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSwModeRstI3CBus5Ctrl` reader - Enable Software Mode reset I3C bus5 controller"]
pub type EnblSwModeRstI3cbus5ctrlR = crate::BitReader;
#[doc = "Field `EnblSwModeRstI3CBus5Ctrl` writer - Enable Software Mode reset I3C bus5 controller"]
pub type EnblSwModeRstI3cbus5ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSwModeRstI3CBus6Ctrl` reader - Enable Software Mode reset I3C bus6 controller"]
pub type EnblSwModeRstI3cbus6ctrlR = crate::BitReader;
#[doc = "Field `EnblSwModeRstI3CBus6Ctrl` writer - Enable Software Mode reset I3C bus6 controller"]
pub type EnblSwModeRstI3cbus6ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSwModeRstI3CBus7Ctrl` reader - Enable Software Mode reset I3C bus7 controller"]
pub type EnblSwModeRstI3cbus7ctrlR = crate::BitReader;
#[doc = "Field `EnblSwModeRstI3CBus7Ctrl` writer - Enable Software Mode reset I3C bus7 controller"]
pub type EnblSwModeRstI3cbus7ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Software Mode reset LSC related controllers"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_lscrelated_ctrls(&self) -> EnblSwModeRstLscrelatedCtrlsR {
        EnblSwModeRstLscrelatedCtrlsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 1:4 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 3 - Enable Software Mode reset PECI bridge"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_pecibridge(&self) -> EnblSwModeRstPecibridgeR {
        EnblSwModeRstPecibridgeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Software Mode reset PWM controller"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_pwmctrl(&self) -> EnblSwModeRstPwmctrlR {
        EnblSwModeRstPwmctrlR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - Enable Software Mode reset ADC controller"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_adcctrl(&self) -> EnblSwModeRstAdcctrlR {
        EnblSwModeRstAdcctrlR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Software Mode reset eSPI controller"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_espictrl(&self) -> EnblSwModeRstEspictrlR {
        EnblSwModeRstEspictrlR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Software Mode reset eSPI controller"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_espictrl1(&self) -> EnblSwModeRstEspictrl1R {
        EnblSwModeRstEspictrl1R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Software Mode reset JTAG Master #0 controller"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_jtagmaster0ctrl(&self) -> EnblSwModeRstJtagmaster0ctrlR {
        EnblSwModeRstJtagmaster0ctrlR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Software Mode reset JTAG Master controller"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_jtagmaster_ctrl(&self) -> EnblSwModeRstJtagmasterCtrlR {
        EnblSwModeRstJtagmasterCtrlR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Software Mode reset JTAG Master #1 controller"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_jtagmaster1ctrl(&self) -> EnblSwModeRstJtagmaster1ctrlR {
        EnblSwModeRstJtagmaster1ctrlR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Software Mode reset SPI0 controller"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_spi0ctrl(&self) -> EnblSwModeRstSpi0ctrlR {
        EnblSwModeRstSpi0ctrlR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Software Mode reset SPI1 controller"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_spi1ctrl(&self) -> EnblSwModeRstSpi1ctrlR {
        EnblSwModeRstSpi1ctrlR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Software Mode reset I3C bus0 controller"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_i3cbus0ctrl(&self) -> EnblSwModeRstI3cbus0ctrlR {
        EnblSwModeRstI3cbus0ctrlR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Software Mode reset I3C bus1 controller"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_i3cbus1ctrl(&self) -> EnblSwModeRstI3cbus1ctrlR {
        EnblSwModeRstI3cbus1ctrlR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Software Mode reset I3C bus2 controller"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_i3cbus2ctrl(&self) -> EnblSwModeRstI3cbus2ctrlR {
        EnblSwModeRstI3cbus2ctrlR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Software Mode reset I3C bus3 controller"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_i3cbus3ctrl(&self) -> EnblSwModeRstI3cbus3ctrlR {
        EnblSwModeRstI3cbus3ctrlR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Software Mode reset I3C bus4 controller"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_i3cbus4ctrl(&self) -> EnblSwModeRstI3cbus4ctrlR {
        EnblSwModeRstI3cbus4ctrlR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable Software Mode reset I3C bus5 controller"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_i3cbus5ctrl(&self) -> EnblSwModeRstI3cbus5ctrlR {
        EnblSwModeRstI3cbus5ctrlR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Software Mode reset I3C bus6 controller"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_i3cbus6ctrl(&self) -> EnblSwModeRstI3cbus6ctrlR {
        EnblSwModeRstI3cbus6ctrlR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Software Mode reset I3C bus7 controller"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_i3cbus7ctrl(&self) -> EnblSwModeRstI3cbus7ctrlR {
        EnblSwModeRstI3cbus7ctrlR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Software Mode reset LSC related controllers"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_lscrelated_ctrls(
        &mut self,
    ) -> EnblSwModeRstLscrelatedCtrlsW<Wdt03cSpec> {
        EnblSwModeRstLscrelatedCtrlsW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<Wdt03cSpec> {
        Reserved5W::new(self, 1)
    }
    #[doc = "Bits 1:4 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<Wdt03cSpec> {
        Reserved6W::new(self, 1)
    }
    #[doc = "Bit 3 - Enable Software Mode reset PECI bridge"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_pecibridge(&mut self) -> EnblSwModeRstPecibridgeW<Wdt03cSpec> {
        EnblSwModeRstPecibridgeW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Software Mode reset PWM controller"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_pwmctrl(&mut self) -> EnblSwModeRstPwmctrlW<Wdt03cSpec> {
        EnblSwModeRstPwmctrlW::new(self, 4)
    }
    #[doc = "Bits 5:7 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<Wdt03cSpec> {
        Reserved4W::new(self, 5)
    }
    #[doc = "Bit 8 - Enable Software Mode reset ADC controller"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_adcctrl(&mut self) -> EnblSwModeRstAdcctrlW<Wdt03cSpec> {
        EnblSwModeRstAdcctrlW::new(self, 8)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Wdt03cSpec> {
        Reserved3W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Software Mode reset eSPI controller"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_espictrl(&mut self) -> EnblSwModeRstEspictrlW<Wdt03cSpec> {
        EnblSwModeRstEspictrlW::new(self, 10)
    }
    #[doc = "Bit 10 - Enable Software Mode reset eSPI controller"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_espictrl1(&mut self) -> EnblSwModeRstEspictrl1W<Wdt03cSpec> {
        EnblSwModeRstEspictrl1W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Software Mode reset JTAG Master #0 controller"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_jtagmaster0ctrl(
        &mut self,
    ) -> EnblSwModeRstJtagmaster0ctrlW<Wdt03cSpec> {
        EnblSwModeRstJtagmaster0ctrlW::new(self, 11)
    }
    #[doc = "Bit 11 - Enable Software Mode reset JTAG Master controller"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_jtagmaster_ctrl(&mut self) -> EnblSwModeRstJtagmasterCtrlW<Wdt03cSpec> {
        EnblSwModeRstJtagmasterCtrlW::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Software Mode reset JTAG Master #1 controller"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_jtagmaster1ctrl(
        &mut self,
    ) -> EnblSwModeRstJtagmaster1ctrlW<Wdt03cSpec> {
        EnblSwModeRstJtagmaster1ctrlW::new(self, 12)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Wdt03cSpec> {
        Reserved2W::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Software Mode reset SPI0 controller"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_spi0ctrl(&mut self) -> EnblSwModeRstSpi0ctrlW<Wdt03cSpec> {
        EnblSwModeRstSpi0ctrlW::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Software Mode reset SPI1 controller"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_spi1ctrl(&mut self) -> EnblSwModeRstSpi1ctrlW<Wdt03cSpec> {
        EnblSwModeRstSpi1ctrlW::new(self, 14)
    }
    #[doc = "Bit 15 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Wdt03cSpec> {
        Reserved1W::new(self, 15)
    }
    #[doc = "Bit 16 - Enable Software Mode reset I3C bus0 controller"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_i3cbus0ctrl(&mut self) -> EnblSwModeRstI3cbus0ctrlW<Wdt03cSpec> {
        EnblSwModeRstI3cbus0ctrlW::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Software Mode reset I3C bus1 controller"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_i3cbus1ctrl(&mut self) -> EnblSwModeRstI3cbus1ctrlW<Wdt03cSpec> {
        EnblSwModeRstI3cbus1ctrlW::new(self, 17)
    }
    #[doc = "Bit 18 - Enable Software Mode reset I3C bus2 controller"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_i3cbus2ctrl(&mut self) -> EnblSwModeRstI3cbus2ctrlW<Wdt03cSpec> {
        EnblSwModeRstI3cbus2ctrlW::new(self, 18)
    }
    #[doc = "Bit 19 - Enable Software Mode reset I3C bus3 controller"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_i3cbus3ctrl(&mut self) -> EnblSwModeRstI3cbus3ctrlW<Wdt03cSpec> {
        EnblSwModeRstI3cbus3ctrlW::new(self, 19)
    }
    #[doc = "Bit 20 - Enable Software Mode reset I3C bus4 controller"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_i3cbus4ctrl(&mut self) -> EnblSwModeRstI3cbus4ctrlW<Wdt03cSpec> {
        EnblSwModeRstI3cbus4ctrlW::new(self, 20)
    }
    #[doc = "Bit 21 - Enable Software Mode reset I3C bus5 controller"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_i3cbus5ctrl(&mut self) -> EnblSwModeRstI3cbus5ctrlW<Wdt03cSpec> {
        EnblSwModeRstI3cbus5ctrlW::new(self, 21)
    }
    #[doc = "Bit 22 - Enable Software Mode reset I3C bus6 controller"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_i3cbus6ctrl(&mut self) -> EnblSwModeRstI3cbus6ctrlW<Wdt03cSpec> {
        EnblSwModeRstI3cbus6ctrlW::new(self, 22)
    }
    #[doc = "Bit 23 - Enable Software Mode reset I3C bus7 controller"]
    #[inline(always)]
    pub fn enbl_sw_mode_rst_i3cbus7ctrl(&mut self) -> EnblSwModeRstI3cbus7ctrlW<Wdt03cSpec> {
        EnblSwModeRstI3cbus7ctrlW::new(self, 23)
    }
}
#[doc = "WDTn Software Mode Reset Mask Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`wdt03c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt03c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wdt03cSpec;
impl crate::RegisterSpec for Wdt03cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdt03c::R`](R) reader structure"]
impl crate::Readable for Wdt03cSpec {}
#[doc = "`write(|w| ..)` method takes [`wdt03c::W`](W) writer structure"]
impl crate::Writable for Wdt03cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WDT03C to value 0x03ff_fff1"]
impl crate::Resettable for Wdt03cSpec {
    const RESET_VALUE: u32 = 0x03ff_fff1;
}
