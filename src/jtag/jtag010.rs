#[doc = "Register `JTAG010` reader"]
pub type R = crate::R<Jtag010Spec>;
#[doc = "Register `JTAG010` writer"]
pub type W = crate::W<Jtag010Spec>;
#[doc = "Field `EngIdle` reader - Engine idle."]
pub type EngIdleR = crate::BitReader;
#[doc = "Field `DataTransmissionPause` reader - Data transmission pause."]
pub type DataTransmissionPauseR = crate::BitReader;
#[doc = "Field `InstuctionTransmissionPause` reader - Instuction transmission pause."]
pub type InstuctionTransmissionPauseR = crate::BitReader;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader<u16>;
#[doc = "Field `SwTDIAndTDO` reader - Software TDI and TDO."]
pub type SwTdiandTdoR = crate::BitReader;
#[doc = "Field `SwTDIAndTDO` writer - Software TDI and TDO."]
pub type SwTdiandTdoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SwTMS` reader - Software TMS."]
pub type SwTmsR = crate::BitReader;
#[doc = "Field `SwTMS` writer - Software TMS."]
pub type SwTmsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SwTCK` reader - Software TCK."]
pub type SwTckR = crate::BitReader;
#[doc = "Field `SwTCK` writer - Software TCK."]
pub type SwTckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SwModeEnbl` reader - Software mode enable."]
pub type SwModeEnblR = crate::BitReader;
#[doc = "Field `SwModeEnbl` writer - Software mode enable."]
pub type SwModeEnblW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Engine idle."]
    #[inline(always)]
    pub fn eng_idle(&self) -> EngIdleR {
        EngIdleR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data transmission pause."]
    #[inline(always)]
    pub fn data_transmission_pause(&self) -> DataTransmissionPauseR {
        DataTransmissionPauseR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Instuction transmission pause."]
    #[inline(always)]
    pub fn instuction_transmission_pause(&self) -> InstuctionTransmissionPauseR {
        InstuctionTransmissionPauseR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:15 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 3) & 0x1fff) as u16)
    }
    #[doc = "Bit 16 - Software TDI and TDO."]
    #[inline(always)]
    pub fn sw_tdiand_tdo(&self) -> SwTdiandTdoR {
        SwTdiandTdoR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Software TMS."]
    #[inline(always)]
    pub fn sw_tms(&self) -> SwTmsR {
        SwTmsR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Software TCK."]
    #[inline(always)]
    pub fn sw_tck(&self) -> SwTckR {
        SwTckR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Software mode enable."]
    #[inline(always)]
    pub fn sw_mode_enbl(&self) -> SwModeEnblR {
        SwModeEnblR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Software TDI and TDO."]
    #[inline(always)]
    pub fn sw_tdiand_tdo(&mut self) -> SwTdiandTdoW<Jtag010Spec> {
        SwTdiandTdoW::new(self, 16)
    }
    #[doc = "Bit 17 - Software TMS."]
    #[inline(always)]
    pub fn sw_tms(&mut self) -> SwTmsW<Jtag010Spec> {
        SwTmsW::new(self, 17)
    }
    #[doc = "Bit 18 - Software TCK."]
    #[inline(always)]
    pub fn sw_tck(&mut self) -> SwTckW<Jtag010Spec> {
        SwTckW::new(self, 18)
    }
    #[doc = "Bit 19 - Software mode enable."]
    #[inline(always)]
    pub fn sw_mode_enbl(&mut self) -> SwModeEnblW<Jtag010Spec> {
        SwModeEnblW::new(self, 19)
    }
}
#[doc = "Software mode and status\n\nYou can [`read`](crate::Reg::read) this register and get [`jtag010::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jtag010::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Jtag010Spec;
impl crate::RegisterSpec for Jtag010Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jtag010::R`](R) reader structure"]
impl crate::Readable for Jtag010Spec {}
#[doc = "`write(|w| ..)` method takes [`jtag010::W`](W) writer structure"]
impl crate::Writable for Jtag010Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets JTAG010 to value 0"]
impl crate::Resettable for Jtag010Spec {}
