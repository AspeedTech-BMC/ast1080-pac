#[doc = "Register `WDT024` reader"]
pub type R = crate::R<Wdt024Spec>;
#[doc = "Register `WDT024` writer"]
pub type W = crate::W<Wdt024Spec>;
#[doc = "Field `EnblRstLSCRelatedCtrls` reader - Enable reset LSC related controllers"]
pub type EnblRstLscrelatedCtrlsR = crate::BitReader;
#[doc = "Field `EnblRstLSCRelatedCtrls` writer - Enable reset LSC related controllers"]
pub type EnblRstLscrelatedCtrlsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::FieldReader;
#[doc = "Field `Reserved5` writer - Reserved"]
pub type Reserved5W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::FieldReader;
#[doc = "Field `Reserved6` writer - Reserved"]
pub type Reserved6W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EnblRstPECIBridge` reader - Enable reset PECI bridge"]
pub type EnblRstPecibridgeR = crate::BitReader;
#[doc = "Field `EnblRstPECIBridge` writer - Enable reset PECI bridge"]
pub type EnblRstPecibridgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblRstPWMCtrl` reader - Enable reset PWM controller"]
pub type EnblRstPwmctrlR = crate::BitReader;
#[doc = "Field `EnblRstPWMCtrl` writer - Enable reset PWM controller"]
pub type EnblRstPwmctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::FieldReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `EnblRstADCCtrl` reader - Enable reset ADC controller"]
pub type EnblRstAdcctrlR = crate::BitReader;
#[doc = "Field `EnblRstADCCtrl` writer - Enable reset ADC controller"]
pub type EnblRstAdcctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblRstESPICtrl` reader - Enable reset eSPI controller"]
pub type EnblRstEspictrlR = crate::BitReader;
#[doc = "Field `EnblRstESPICtrl` writer - Enable reset eSPI controller"]
pub type EnblRstEspictrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblRstESPICtrl1` reader - Enable reset eSPI controller"]
pub type EnblRstEspictrl1R = crate::BitReader;
#[doc = "Field `EnblRstESPICtrl1` writer - Enable reset eSPI controller"]
pub type EnblRstEspictrl1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblRstJTAGMaster0Ctrl` reader - Enable reset JTAG Master #0 controller"]
pub type EnblRstJtagmaster0ctrlR = crate::BitReader;
#[doc = "Field `EnblRstJTAGMaster0Ctrl` writer - Enable reset JTAG Master #0 controller"]
pub type EnblRstJtagmaster0ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblRstJTAGMasterCtrl` reader - Enable reset JTAG Master controller"]
pub type EnblRstJtagmasterCtrlR = crate::BitReader;
#[doc = "Field `EnblRstJTAGMasterCtrl` writer - Enable reset JTAG Master controller"]
pub type EnblRstJtagmasterCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblRstJTAGMaster1Ctrl` reader - Enable reset JTAG Master #1 controller"]
pub type EnblRstJtagmaster1ctrlR = crate::BitReader;
#[doc = "Field `EnblRstJTAGMaster1Ctrl` writer - Enable reset JTAG Master #1 controller"]
pub type EnblRstJtagmaster1ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblRstSPI0Ctrl` reader - Enable reset SPI0 controller"]
pub type EnblRstSpi0ctrlR = crate::BitReader;
#[doc = "Field `EnblRstSPI0Ctrl` writer - Enable reset SPI0 controller"]
pub type EnblRstSpi0ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblRstSPI1Ctrl` reader - Enable reset SPI1 controller"]
pub type EnblRstSpi1ctrlR = crate::BitReader;
#[doc = "Field `EnblRstSPI1Ctrl` writer - Enable reset SPI1 controller"]
pub type EnblRstSpi1ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblRstI3CBus0Ctrl` reader - Enable reset I3C bus0 controller"]
pub type EnblRstI3cbus0ctrlR = crate::BitReader;
#[doc = "Field `EnblRstI3CBus0Ctrl` writer - Enable reset I3C bus0 controller"]
pub type EnblRstI3cbus0ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblRstI3CBus1Ctrl` reader - Enable reset I3C bus1 controller"]
pub type EnblRstI3cbus1ctrlR = crate::BitReader;
#[doc = "Field `EnblRstI3CBus1Ctrl` writer - Enable reset I3C bus1 controller"]
pub type EnblRstI3cbus1ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblRstI3CBus2Ctrl` reader - Enable reset I3C bus2 controller"]
pub type EnblRstI3cbus2ctrlR = crate::BitReader;
#[doc = "Field `EnblRstI3CBus2Ctrl` writer - Enable reset I3C bus2 controller"]
pub type EnblRstI3cbus2ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblRstI3CBus3Ctrl` reader - Enable reset I3C bus3 controller"]
pub type EnblRstI3cbus3ctrlR = crate::BitReader;
#[doc = "Field `EnblRstI3CBus3Ctrl` writer - Enable reset I3C bus3 controller"]
pub type EnblRstI3cbus3ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblRstI3CBus4Ctrl` reader - Enable reset I3C bus4 controller"]
pub type EnblRstI3cbus4ctrlR = crate::BitReader;
#[doc = "Field `EnblRstI3CBus4Ctrl` writer - Enable reset I3C bus4 controller"]
pub type EnblRstI3cbus4ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblRstI3CBus5Ctrl` reader - Enable reset I3C bus5 controller"]
pub type EnblRstI3cbus5ctrlR = crate::BitReader;
#[doc = "Field `EnblRstI3CBus5Ctrl` writer - Enable reset I3C bus5 controller"]
pub type EnblRstI3cbus5ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblRstI3CBus6Ctrl` reader - Enable reset I3C bus6 controller"]
pub type EnblRstI3cbus6ctrlR = crate::BitReader;
#[doc = "Field `EnblRstI3CBus6Ctrl` writer - Enable reset I3C bus6 controller"]
pub type EnblRstI3cbus6ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblRstI3CBus7Ctrl` reader - Enable reset I3C bus7 controller"]
pub type EnblRstI3cbus7ctrlR = crate::BitReader;
#[doc = "Field `EnblRstI3CBus7Ctrl` writer - Enable reset I3C bus7 controller"]
pub type EnblRstI3cbus7ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable reset LSC related controllers"]
    #[inline(always)]
    pub fn enbl_rst_lscrelated_ctrls(&self) -> EnblRstLscrelatedCtrlsR {
        EnblRstLscrelatedCtrlsR::new((self.bits & 1) != 0)
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
    #[doc = "Bit 3 - Enable reset PECI bridge"]
    #[inline(always)]
    pub fn enbl_rst_pecibridge(&self) -> EnblRstPecibridgeR {
        EnblRstPecibridgeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable reset PWM controller"]
    #[inline(always)]
    pub fn enbl_rst_pwmctrl(&self) -> EnblRstPwmctrlR {
        EnblRstPwmctrlR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - Enable reset ADC controller"]
    #[inline(always)]
    pub fn enbl_rst_adcctrl(&self) -> EnblRstAdcctrlR {
        EnblRstAdcctrlR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable reset eSPI controller"]
    #[inline(always)]
    pub fn enbl_rst_espictrl(&self) -> EnblRstEspictrlR {
        EnblRstEspictrlR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable reset eSPI controller"]
    #[inline(always)]
    pub fn enbl_rst_espictrl1(&self) -> EnblRstEspictrl1R {
        EnblRstEspictrl1R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable reset JTAG Master #0 controller"]
    #[inline(always)]
    pub fn enbl_rst_jtagmaster0ctrl(&self) -> EnblRstJtagmaster0ctrlR {
        EnblRstJtagmaster0ctrlR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable reset JTAG Master controller"]
    #[inline(always)]
    pub fn enbl_rst_jtagmaster_ctrl(&self) -> EnblRstJtagmasterCtrlR {
        EnblRstJtagmasterCtrlR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable reset JTAG Master #1 controller"]
    #[inline(always)]
    pub fn enbl_rst_jtagmaster1ctrl(&self) -> EnblRstJtagmaster1ctrlR {
        EnblRstJtagmaster1ctrlR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable reset SPI0 controller"]
    #[inline(always)]
    pub fn enbl_rst_spi0ctrl(&self) -> EnblRstSpi0ctrlR {
        EnblRstSpi0ctrlR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable reset SPI1 controller"]
    #[inline(always)]
    pub fn enbl_rst_spi1ctrl(&self) -> EnblRstSpi1ctrlR {
        EnblRstSpi1ctrlR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable reset I3C bus0 controller"]
    #[inline(always)]
    pub fn enbl_rst_i3cbus0ctrl(&self) -> EnblRstI3cbus0ctrlR {
        EnblRstI3cbus0ctrlR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable reset I3C bus1 controller"]
    #[inline(always)]
    pub fn enbl_rst_i3cbus1ctrl(&self) -> EnblRstI3cbus1ctrlR {
        EnblRstI3cbus1ctrlR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable reset I3C bus2 controller"]
    #[inline(always)]
    pub fn enbl_rst_i3cbus2ctrl(&self) -> EnblRstI3cbus2ctrlR {
        EnblRstI3cbus2ctrlR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable reset I3C bus3 controller"]
    #[inline(always)]
    pub fn enbl_rst_i3cbus3ctrl(&self) -> EnblRstI3cbus3ctrlR {
        EnblRstI3cbus3ctrlR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable reset I3C bus4 controller"]
    #[inline(always)]
    pub fn enbl_rst_i3cbus4ctrl(&self) -> EnblRstI3cbus4ctrlR {
        EnblRstI3cbus4ctrlR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable reset I3C bus5 controller"]
    #[inline(always)]
    pub fn enbl_rst_i3cbus5ctrl(&self) -> EnblRstI3cbus5ctrlR {
        EnblRstI3cbus5ctrlR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable reset I3C bus6 controller"]
    #[inline(always)]
    pub fn enbl_rst_i3cbus6ctrl(&self) -> EnblRstI3cbus6ctrlR {
        EnblRstI3cbus6ctrlR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable reset I3C bus7 controller"]
    #[inline(always)]
    pub fn enbl_rst_i3cbus7ctrl(&self) -> EnblRstI3cbus7ctrlR {
        EnblRstI3cbus7ctrlR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable reset LSC related controllers"]
    #[inline(always)]
    pub fn enbl_rst_lscrelated_ctrls(&mut self) -> EnblRstLscrelatedCtrlsW<Wdt024Spec> {
        EnblRstLscrelatedCtrlsW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<Wdt024Spec> {
        Reserved5W::new(self, 1)
    }
    #[doc = "Bits 1:4 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<Wdt024Spec> {
        Reserved6W::new(self, 1)
    }
    #[doc = "Bit 3 - Enable reset PECI bridge"]
    #[inline(always)]
    pub fn enbl_rst_pecibridge(&mut self) -> EnblRstPecibridgeW<Wdt024Spec> {
        EnblRstPecibridgeW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable reset PWM controller"]
    #[inline(always)]
    pub fn enbl_rst_pwmctrl(&mut self) -> EnblRstPwmctrlW<Wdt024Spec> {
        EnblRstPwmctrlW::new(self, 4)
    }
    #[doc = "Bits 5:7 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<Wdt024Spec> {
        Reserved4W::new(self, 5)
    }
    #[doc = "Bit 8 - Enable reset ADC controller"]
    #[inline(always)]
    pub fn enbl_rst_adcctrl(&mut self) -> EnblRstAdcctrlW<Wdt024Spec> {
        EnblRstAdcctrlW::new(self, 8)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Wdt024Spec> {
        Reserved3W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable reset eSPI controller"]
    #[inline(always)]
    pub fn enbl_rst_espictrl(&mut self) -> EnblRstEspictrlW<Wdt024Spec> {
        EnblRstEspictrlW::new(self, 10)
    }
    #[doc = "Bit 10 - Enable reset eSPI controller"]
    #[inline(always)]
    pub fn enbl_rst_espictrl1(&mut self) -> EnblRstEspictrl1W<Wdt024Spec> {
        EnblRstEspictrl1W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable reset JTAG Master #0 controller"]
    #[inline(always)]
    pub fn enbl_rst_jtagmaster0ctrl(&mut self) -> EnblRstJtagmaster0ctrlW<Wdt024Spec> {
        EnblRstJtagmaster0ctrlW::new(self, 11)
    }
    #[doc = "Bit 11 - Enable reset JTAG Master controller"]
    #[inline(always)]
    pub fn enbl_rst_jtagmaster_ctrl(&mut self) -> EnblRstJtagmasterCtrlW<Wdt024Spec> {
        EnblRstJtagmasterCtrlW::new(self, 11)
    }
    #[doc = "Bit 12 - Enable reset JTAG Master #1 controller"]
    #[inline(always)]
    pub fn enbl_rst_jtagmaster1ctrl(&mut self) -> EnblRstJtagmaster1ctrlW<Wdt024Spec> {
        EnblRstJtagmaster1ctrlW::new(self, 12)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Wdt024Spec> {
        Reserved2W::new(self, 12)
    }
    #[doc = "Bit 13 - Enable reset SPI0 controller"]
    #[inline(always)]
    pub fn enbl_rst_spi0ctrl(&mut self) -> EnblRstSpi0ctrlW<Wdt024Spec> {
        EnblRstSpi0ctrlW::new(self, 13)
    }
    #[doc = "Bit 14 - Enable reset SPI1 controller"]
    #[inline(always)]
    pub fn enbl_rst_spi1ctrl(&mut self) -> EnblRstSpi1ctrlW<Wdt024Spec> {
        EnblRstSpi1ctrlW::new(self, 14)
    }
    #[doc = "Bit 15 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Wdt024Spec> {
        Reserved1W::new(self, 15)
    }
    #[doc = "Bit 16 - Enable reset I3C bus0 controller"]
    #[inline(always)]
    pub fn enbl_rst_i3cbus0ctrl(&mut self) -> EnblRstI3cbus0ctrlW<Wdt024Spec> {
        EnblRstI3cbus0ctrlW::new(self, 16)
    }
    #[doc = "Bit 17 - Enable reset I3C bus1 controller"]
    #[inline(always)]
    pub fn enbl_rst_i3cbus1ctrl(&mut self) -> EnblRstI3cbus1ctrlW<Wdt024Spec> {
        EnblRstI3cbus1ctrlW::new(self, 17)
    }
    #[doc = "Bit 18 - Enable reset I3C bus2 controller"]
    #[inline(always)]
    pub fn enbl_rst_i3cbus2ctrl(&mut self) -> EnblRstI3cbus2ctrlW<Wdt024Spec> {
        EnblRstI3cbus2ctrlW::new(self, 18)
    }
    #[doc = "Bit 19 - Enable reset I3C bus3 controller"]
    #[inline(always)]
    pub fn enbl_rst_i3cbus3ctrl(&mut self) -> EnblRstI3cbus3ctrlW<Wdt024Spec> {
        EnblRstI3cbus3ctrlW::new(self, 19)
    }
    #[doc = "Bit 20 - Enable reset I3C bus4 controller"]
    #[inline(always)]
    pub fn enbl_rst_i3cbus4ctrl(&mut self) -> EnblRstI3cbus4ctrlW<Wdt024Spec> {
        EnblRstI3cbus4ctrlW::new(self, 20)
    }
    #[doc = "Bit 21 - Enable reset I3C bus5 controller"]
    #[inline(always)]
    pub fn enbl_rst_i3cbus5ctrl(&mut self) -> EnblRstI3cbus5ctrlW<Wdt024Spec> {
        EnblRstI3cbus5ctrlW::new(self, 21)
    }
    #[doc = "Bit 22 - Enable reset I3C bus6 controller"]
    #[inline(always)]
    pub fn enbl_rst_i3cbus6ctrl(&mut self) -> EnblRstI3cbus6ctrlW<Wdt024Spec> {
        EnblRstI3cbus6ctrlW::new(self, 22)
    }
    #[doc = "Bit 23 - Enable reset I3C bus7 controller"]
    #[inline(always)]
    pub fn enbl_rst_i3cbus7ctrl(&mut self) -> EnblRstI3cbus7ctrlW<Wdt024Spec> {
        EnblRstI3cbus7ctrlW::new(self, 23)
    }
}
#[doc = "WDTn Reset Mask Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`wdt024::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt024::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wdt024Spec;
impl crate::RegisterSpec for Wdt024Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdt024::R`](R) reader structure"]
impl crate::Readable for Wdt024Spec {}
#[doc = "`write(|w| ..)` method takes [`wdt024::W`](W) writer structure"]
impl crate::Writable for Wdt024Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WDT024 to value 0x03ff_fff1"]
impl crate::Resettable for Wdt024Spec {
    const RESET_VALUE: u32 = 0x03ff_fff1;
}
