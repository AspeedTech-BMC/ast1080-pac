#[doc = "Register `WDT06C` reader"]
pub type R = crate::R<Wdt06cSpec>;
#[doc = "Register `WDT06C` writer"]
pub type W = crate::W<Wdt06cSpec>;
#[doc = "Field `WrProtOfEnblSwModeRstLSCRelatedCtrls` reader - Write Protection of Enable Software Mode reset LSC related controllers"]
pub type WrProtOfEnblSwModeRstLscrelatedCtrlsR = crate::BitReader;
#[doc = "Field `WrProtOfEnblSwModeRstLSCRelatedCtrls` writer - Write Protection of Enable Software Mode reset LSC related controllers"]
pub type WrProtOfEnblSwModeRstLscrelatedCtrlsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::FieldReader;
#[doc = "Field `Reserved5` writer - Reserved"]
pub type Reserved5W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::FieldReader;
#[doc = "Field `Reserved6` writer - Reserved"]
pub type Reserved6W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WrProtOfEnblSwModeRstPECIBridge` reader - Write Protection of Enable Software Mode reset PECI bridge"]
pub type WrProtOfEnblSwModeRstPecibridgeR = crate::BitReader;
#[doc = "Field `WrProtOfEnblSwModeRstPECIBridge` writer - Write Protection of Enable Software Mode reset PECI bridge"]
pub type WrProtOfEnblSwModeRstPecibridgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrProtOfEnblSwModeRstPWMCtrl` reader - Write Protection of Enable Software Mode reset PWM controller"]
pub type WrProtOfEnblSwModeRstPwmctrlR = crate::BitReader;
#[doc = "Field `WrProtOfEnblSwModeRstPWMCtrl` writer - Write Protection of Enable Software Mode reset PWM controller"]
pub type WrProtOfEnblSwModeRstPwmctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::FieldReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WrProtOfEnblSwModeRstADCCtrl` reader - Write Protection of Enable Software Mode reset ADC controller"]
pub type WrProtOfEnblSwModeRstAdcctrlR = crate::BitReader;
#[doc = "Field `WrProtOfEnblSwModeRstADCCtrl` writer - Write Protection of Enable Software Mode reset ADC controller"]
pub type WrProtOfEnblSwModeRstAdcctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrProtOfEnblSwModeRstESPICtrl` reader - Write Protection of Enable Software Mode reset eSPI controller"]
pub type WrProtOfEnblSwModeRstEspictrlR = crate::BitReader;
#[doc = "Field `WrProtOfEnblSwModeRstESPICtrl` writer - Write Protection of Enable Software Mode reset eSPI controller"]
pub type WrProtOfEnblSwModeRstEspictrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrProtOfEnblSwModeRstESPICtrl1` reader - Write Protection of Enable Software Mode reset eSPI controller"]
pub type WrProtOfEnblSwModeRstEspictrl1R = crate::BitReader;
#[doc = "Field `WrProtOfEnblSwModeRstESPICtrl1` writer - Write Protection of Enable Software Mode reset eSPI controller"]
pub type WrProtOfEnblSwModeRstEspictrl1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrProtOfEnblSwModeRstJTAGMaster0Ctrl` reader - Write Protection of Enable Software Mode reset JTAG Master #0 controller"]
pub type WrProtOfEnblSwModeRstJtagmaster0ctrlR = crate::BitReader;
#[doc = "Field `WrProtOfEnblSwModeRstJTAGMaster0Ctrl` writer - Write Protection of Enable Software Mode reset JTAG Master #0 controller"]
pub type WrProtOfEnblSwModeRstJtagmaster0ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrProtOfEnblSwModeRstJTAGMasterCtrl` reader - Write Protection of Enable Software Mode reset JTAG Master controller"]
pub type WrProtOfEnblSwModeRstJtagmasterCtrlR = crate::BitReader;
#[doc = "Field `WrProtOfEnblSwModeRstJTAGMasterCtrl` writer - Write Protection of Enable Software Mode reset JTAG Master controller"]
pub type WrProtOfEnblSwModeRstJtagmasterCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrProtOfEnblSwModeRstJTAGMaster1Ctrl` reader - Write Protection of Enable Software Mode reset JTAG Master #1 controller"]
pub type WrProtOfEnblSwModeRstJtagmaster1ctrlR = crate::BitReader;
#[doc = "Field `WrProtOfEnblSwModeRstJTAGMaster1Ctrl` writer - Write Protection of Enable Software Mode reset JTAG Master #1 controller"]
pub type WrProtOfEnblSwModeRstJtagmaster1ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrProtOfEnblSwModeRstSPI0Ctrl` reader - Write Protection of Enable Software Mode reset SPI0 controller"]
pub type WrProtOfEnblSwModeRstSpi0ctrlR = crate::BitReader;
#[doc = "Field `WrProtOfEnblSwModeRstSPI0Ctrl` writer - Write Protection of Enable Software Mode reset SPI0 controller"]
pub type WrProtOfEnblSwModeRstSpi0ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrProtOfEnblSwModeRstSPI1Ctrl` reader - Write Protection of Enable Software Mode reset SPI1 controller"]
pub type WrProtOfEnblSwModeRstSpi1ctrlR = crate::BitReader;
#[doc = "Field `WrProtOfEnblSwModeRstSPI1Ctrl` writer - Write Protection of Enable Software Mode reset SPI1 controller"]
pub type WrProtOfEnblSwModeRstSpi1ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrProtOfEnblSwModeRstI3CBus0Ctrl` reader - Write Protection of Enable Software Mode reset I3C bus0 controller"]
pub type WrProtOfEnblSwModeRstI3cbus0ctrlR = crate::BitReader;
#[doc = "Field `WrProtOfEnblSwModeRstI3CBus0Ctrl` writer - Write Protection of Enable Software Mode reset I3C bus0 controller"]
pub type WrProtOfEnblSwModeRstI3cbus0ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrProtOfEnblSwModeRstI3CBus1Ctrl` reader - Write Protection of Enable Software Mode reset I3C bus1 controller"]
pub type WrProtOfEnblSwModeRstI3cbus1ctrlR = crate::BitReader;
#[doc = "Field `WrProtOfEnblSwModeRstI3CBus1Ctrl` writer - Write Protection of Enable Software Mode reset I3C bus1 controller"]
pub type WrProtOfEnblSwModeRstI3cbus1ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrProtOfEnblSwModeRstI3CBus2Ctrl` reader - Write Protection of Enable Software Mode reset I3C bus2 controller"]
pub type WrProtOfEnblSwModeRstI3cbus2ctrlR = crate::BitReader;
#[doc = "Field `WrProtOfEnblSwModeRstI3CBus2Ctrl` writer - Write Protection of Enable Software Mode reset I3C bus2 controller"]
pub type WrProtOfEnblSwModeRstI3cbus2ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrProtOfEnblSwModeRstI3CBus3Ctrl` reader - Write Protection of Enable Software Mode reset I3C bus3 controller"]
pub type WrProtOfEnblSwModeRstI3cbus3ctrlR = crate::BitReader;
#[doc = "Field `WrProtOfEnblSwModeRstI3CBus3Ctrl` writer - Write Protection of Enable Software Mode reset I3C bus3 controller"]
pub type WrProtOfEnblSwModeRstI3cbus3ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrProtOfEnblSwModeRstI3CBus4Ctrl` reader - Write Protection of Enable Software Mode reset I3C bus4 controller"]
pub type WrProtOfEnblSwModeRstI3cbus4ctrlR = crate::BitReader;
#[doc = "Field `WrProtOfEnblSwModeRstI3CBus4Ctrl` writer - Write Protection of Enable Software Mode reset I3C bus4 controller"]
pub type WrProtOfEnblSwModeRstI3cbus4ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrProtOfEnblSwModeRstI3CBus5Ctrl` reader - Write Protection of Enable Software Mode reset I3C bus5 controller"]
pub type WrProtOfEnblSwModeRstI3cbus5ctrlR = crate::BitReader;
#[doc = "Field `WrProtOfEnblSwModeRstI3CBus5Ctrl` writer - Write Protection of Enable Software Mode reset I3C bus5 controller"]
pub type WrProtOfEnblSwModeRstI3cbus5ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrProtOfEnblSwModeRstI3CBus6Ctrl` reader - Write Protection of Enable Software Mode reset I3C bus6 controller"]
pub type WrProtOfEnblSwModeRstI3cbus6ctrlR = crate::BitReader;
#[doc = "Field `WrProtOfEnblSwModeRstI3CBus6Ctrl` writer - Write Protection of Enable Software Mode reset I3C bus6 controller"]
pub type WrProtOfEnblSwModeRstI3cbus6ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrProtOfEnblSwModeRstI3CBus7Ctrl` reader - Write Protection of Enable Software Mode reset I3C bus7 controller"]
pub type WrProtOfEnblSwModeRstI3cbus7ctrlR = crate::BitReader;
#[doc = "Field `WrProtOfEnblSwModeRstI3CBus7Ctrl` writer - Write Protection of Enable Software Mode reset I3C bus7 controller"]
pub type WrProtOfEnblSwModeRstI3cbus7ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write Protection of Enable Software Mode reset LSC related controllers"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_lscrelated_ctrls(
        &self,
    ) -> WrProtOfEnblSwModeRstLscrelatedCtrlsR {
        WrProtOfEnblSwModeRstLscrelatedCtrlsR::new((self.bits & 1) != 0)
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
    #[doc = "Bit 3 - Write Protection of Enable Software Mode reset PECI bridge"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_pecibridge(&self) -> WrProtOfEnblSwModeRstPecibridgeR {
        WrProtOfEnblSwModeRstPecibridgeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write Protection of Enable Software Mode reset PWM controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_pwmctrl(&self) -> WrProtOfEnblSwModeRstPwmctrlR {
        WrProtOfEnblSwModeRstPwmctrlR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - Write Protection of Enable Software Mode reset ADC controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_adcctrl(&self) -> WrProtOfEnblSwModeRstAdcctrlR {
        WrProtOfEnblSwModeRstAdcctrlR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Write Protection of Enable Software Mode reset eSPI controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_espictrl(&self) -> WrProtOfEnblSwModeRstEspictrlR {
        WrProtOfEnblSwModeRstEspictrlR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 10 - Write Protection of Enable Software Mode reset eSPI controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_espictrl1(&self) -> WrProtOfEnblSwModeRstEspictrl1R {
        WrProtOfEnblSwModeRstEspictrl1R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Write Protection of Enable Software Mode reset JTAG Master #0 controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_jtagmaster0ctrl(
        &self,
    ) -> WrProtOfEnblSwModeRstJtagmaster0ctrlR {
        WrProtOfEnblSwModeRstJtagmaster0ctrlR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 11 - Write Protection of Enable Software Mode reset JTAG Master controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_jtagmaster_ctrl(
        &self,
    ) -> WrProtOfEnblSwModeRstJtagmasterCtrlR {
        WrProtOfEnblSwModeRstJtagmasterCtrlR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Write Protection of Enable Software Mode reset JTAG Master #1 controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_jtagmaster1ctrl(
        &self,
    ) -> WrProtOfEnblSwModeRstJtagmaster1ctrlR {
        WrProtOfEnblSwModeRstJtagmaster1ctrlR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Write Protection of Enable Software Mode reset SPI0 controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_spi0ctrl(&self) -> WrProtOfEnblSwModeRstSpi0ctrlR {
        WrProtOfEnblSwModeRstSpi0ctrlR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Write Protection of Enable Software Mode reset SPI1 controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_spi1ctrl(&self) -> WrProtOfEnblSwModeRstSpi1ctrlR {
        WrProtOfEnblSwModeRstSpi1ctrlR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Write Protection of Enable Software Mode reset I3C bus0 controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_i3cbus0ctrl(&self) -> WrProtOfEnblSwModeRstI3cbus0ctrlR {
        WrProtOfEnblSwModeRstI3cbus0ctrlR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Write Protection of Enable Software Mode reset I3C bus1 controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_i3cbus1ctrl(&self) -> WrProtOfEnblSwModeRstI3cbus1ctrlR {
        WrProtOfEnblSwModeRstI3cbus1ctrlR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Write Protection of Enable Software Mode reset I3C bus2 controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_i3cbus2ctrl(&self) -> WrProtOfEnblSwModeRstI3cbus2ctrlR {
        WrProtOfEnblSwModeRstI3cbus2ctrlR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Write Protection of Enable Software Mode reset I3C bus3 controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_i3cbus3ctrl(&self) -> WrProtOfEnblSwModeRstI3cbus3ctrlR {
        WrProtOfEnblSwModeRstI3cbus3ctrlR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Write Protection of Enable Software Mode reset I3C bus4 controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_i3cbus4ctrl(&self) -> WrProtOfEnblSwModeRstI3cbus4ctrlR {
        WrProtOfEnblSwModeRstI3cbus4ctrlR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Write Protection of Enable Software Mode reset I3C bus5 controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_i3cbus5ctrl(&self) -> WrProtOfEnblSwModeRstI3cbus5ctrlR {
        WrProtOfEnblSwModeRstI3cbus5ctrlR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Write Protection of Enable Software Mode reset I3C bus6 controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_i3cbus6ctrl(&self) -> WrProtOfEnblSwModeRstI3cbus6ctrlR {
        WrProtOfEnblSwModeRstI3cbus6ctrlR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Write Protection of Enable Software Mode reset I3C bus7 controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_i3cbus7ctrl(&self) -> WrProtOfEnblSwModeRstI3cbus7ctrlR {
        WrProtOfEnblSwModeRstI3cbus7ctrlR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write Protection of Enable Software Mode reset LSC related controllers"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_lscrelated_ctrls(
        &mut self,
    ) -> WrProtOfEnblSwModeRstLscrelatedCtrlsW<Wdt06cSpec> {
        WrProtOfEnblSwModeRstLscrelatedCtrlsW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<Wdt06cSpec> {
        Reserved5W::new(self, 1)
    }
    #[doc = "Bits 1:4 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<Wdt06cSpec> {
        Reserved6W::new(self, 1)
    }
    #[doc = "Bit 3 - Write Protection of Enable Software Mode reset PECI bridge"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_pecibridge(
        &mut self,
    ) -> WrProtOfEnblSwModeRstPecibridgeW<Wdt06cSpec> {
        WrProtOfEnblSwModeRstPecibridgeW::new(self, 3)
    }
    #[doc = "Bit 4 - Write Protection of Enable Software Mode reset PWM controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_pwmctrl(
        &mut self,
    ) -> WrProtOfEnblSwModeRstPwmctrlW<Wdt06cSpec> {
        WrProtOfEnblSwModeRstPwmctrlW::new(self, 4)
    }
    #[doc = "Bits 5:7 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<Wdt06cSpec> {
        Reserved4W::new(self, 5)
    }
    #[doc = "Bit 8 - Write Protection of Enable Software Mode reset ADC controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_adcctrl(
        &mut self,
    ) -> WrProtOfEnblSwModeRstAdcctrlW<Wdt06cSpec> {
        WrProtOfEnblSwModeRstAdcctrlW::new(self, 8)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Wdt06cSpec> {
        Reserved3W::new(self, 9)
    }
    #[doc = "Bit 10 - Write Protection of Enable Software Mode reset eSPI controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_espictrl(
        &mut self,
    ) -> WrProtOfEnblSwModeRstEspictrlW<Wdt06cSpec> {
        WrProtOfEnblSwModeRstEspictrlW::new(self, 10)
    }
    #[doc = "Bit 10 - Write Protection of Enable Software Mode reset eSPI controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_espictrl1(
        &mut self,
    ) -> WrProtOfEnblSwModeRstEspictrl1W<Wdt06cSpec> {
        WrProtOfEnblSwModeRstEspictrl1W::new(self, 10)
    }
    #[doc = "Bit 11 - Write Protection of Enable Software Mode reset JTAG Master #0 controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_jtagmaster0ctrl(
        &mut self,
    ) -> WrProtOfEnblSwModeRstJtagmaster0ctrlW<Wdt06cSpec> {
        WrProtOfEnblSwModeRstJtagmaster0ctrlW::new(self, 11)
    }
    #[doc = "Bit 11 - Write Protection of Enable Software Mode reset JTAG Master controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_jtagmaster_ctrl(
        &mut self,
    ) -> WrProtOfEnblSwModeRstJtagmasterCtrlW<Wdt06cSpec> {
        WrProtOfEnblSwModeRstJtagmasterCtrlW::new(self, 11)
    }
    #[doc = "Bit 12 - Write Protection of Enable Software Mode reset JTAG Master #1 controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_jtagmaster1ctrl(
        &mut self,
    ) -> WrProtOfEnblSwModeRstJtagmaster1ctrlW<Wdt06cSpec> {
        WrProtOfEnblSwModeRstJtagmaster1ctrlW::new(self, 12)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Wdt06cSpec> {
        Reserved2W::new(self, 12)
    }
    #[doc = "Bit 13 - Write Protection of Enable Software Mode reset SPI0 controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_spi0ctrl(
        &mut self,
    ) -> WrProtOfEnblSwModeRstSpi0ctrlW<Wdt06cSpec> {
        WrProtOfEnblSwModeRstSpi0ctrlW::new(self, 13)
    }
    #[doc = "Bit 14 - Write Protection of Enable Software Mode reset SPI1 controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_spi1ctrl(
        &mut self,
    ) -> WrProtOfEnblSwModeRstSpi1ctrlW<Wdt06cSpec> {
        WrProtOfEnblSwModeRstSpi1ctrlW::new(self, 14)
    }
    #[doc = "Bit 15 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Wdt06cSpec> {
        Reserved1W::new(self, 15)
    }
    #[doc = "Bit 16 - Write Protection of Enable Software Mode reset I3C bus0 controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_i3cbus0ctrl(
        &mut self,
    ) -> WrProtOfEnblSwModeRstI3cbus0ctrlW<Wdt06cSpec> {
        WrProtOfEnblSwModeRstI3cbus0ctrlW::new(self, 16)
    }
    #[doc = "Bit 17 - Write Protection of Enable Software Mode reset I3C bus1 controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_i3cbus1ctrl(
        &mut self,
    ) -> WrProtOfEnblSwModeRstI3cbus1ctrlW<Wdt06cSpec> {
        WrProtOfEnblSwModeRstI3cbus1ctrlW::new(self, 17)
    }
    #[doc = "Bit 18 - Write Protection of Enable Software Mode reset I3C bus2 controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_i3cbus2ctrl(
        &mut self,
    ) -> WrProtOfEnblSwModeRstI3cbus2ctrlW<Wdt06cSpec> {
        WrProtOfEnblSwModeRstI3cbus2ctrlW::new(self, 18)
    }
    #[doc = "Bit 19 - Write Protection of Enable Software Mode reset I3C bus3 controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_i3cbus3ctrl(
        &mut self,
    ) -> WrProtOfEnblSwModeRstI3cbus3ctrlW<Wdt06cSpec> {
        WrProtOfEnblSwModeRstI3cbus3ctrlW::new(self, 19)
    }
    #[doc = "Bit 20 - Write Protection of Enable Software Mode reset I3C bus4 controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_i3cbus4ctrl(
        &mut self,
    ) -> WrProtOfEnblSwModeRstI3cbus4ctrlW<Wdt06cSpec> {
        WrProtOfEnblSwModeRstI3cbus4ctrlW::new(self, 20)
    }
    #[doc = "Bit 21 - Write Protection of Enable Software Mode reset I3C bus5 controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_i3cbus5ctrl(
        &mut self,
    ) -> WrProtOfEnblSwModeRstI3cbus5ctrlW<Wdt06cSpec> {
        WrProtOfEnblSwModeRstI3cbus5ctrlW::new(self, 21)
    }
    #[doc = "Bit 22 - Write Protection of Enable Software Mode reset I3C bus6 controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_i3cbus6ctrl(
        &mut self,
    ) -> WrProtOfEnblSwModeRstI3cbus6ctrlW<Wdt06cSpec> {
        WrProtOfEnblSwModeRstI3cbus6ctrlW::new(self, 22)
    }
    #[doc = "Bit 23 - Write Protection of Enable Software Mode reset I3C bus7 controller"]
    #[inline(always)]
    pub fn wr_prot_of_enbl_sw_mode_rst_i3cbus7ctrl(
        &mut self,
    ) -> WrProtOfEnblSwModeRstI3cbus7ctrlW<Wdt06cSpec> {
        WrProtOfEnblSwModeRstI3cbus7ctrlW::new(self, 23)
    }
}
#[doc = "WDTn Reset Mask Write Protection Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`wdt06c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt06c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wdt06cSpec;
impl crate::RegisterSpec for Wdt06cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdt06c::R`](R) reader structure"]
impl crate::Readable for Wdt06cSpec {}
#[doc = "`write(|w| ..)` method takes [`wdt06c::W`](W) writer structure"]
impl crate::Writable for Wdt06cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WDT06C to value 0x03ff_fff1"]
impl crate::Resettable for Wdt06cSpec {
    const RESET_VALUE: u32 = 0x03ff_fff1;
}
