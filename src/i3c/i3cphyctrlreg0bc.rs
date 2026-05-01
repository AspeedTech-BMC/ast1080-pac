#[doc = "Register `I3CPHYCTRLREG0BC` reader"]
pub type R = crate::R<I3cphyctrlreg0bcSpec>;
#[doc = "Register `I3CPHYCTRLREG0BC` writer"]
pub type W = crate::W<I3cphyctrlreg0bcSpec>;
#[doc = "Field `REGCNTSDADETECTORTIMEOUT` reader - REG_CNT_SDA_DETECTOR_TIMEOUT"]
pub type RegcntsdadetectortimeoutR = crate::FieldReader<u16>;
#[doc = "Field `REGCNTSDADETECTORTIMEOUT` writer - REG_CNT_SDA_DETECTOR_TIMEOUT"]
pub type RegcntsdadetectortimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `REGCNTCLOCKSTALLTIMEOUT` reader - REG_CNT_CLOCK_STALL_TIMEOUT"]
pub type RegcntclockstalltimeoutR = crate::FieldReader<u16>;
#[doc = "Field `REGCNTCLOCKSTALLTIMEOUT` writer - REG_CNT_CLOCK_STALL_TIMEOUT"]
pub type RegcntclockstalltimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - REG_CNT_SDA_DETECTOR_TIMEOUT"]
    #[inline(always)]
    pub fn regcntsdadetectortimeout(&self) -> RegcntsdadetectortimeoutR {
        RegcntsdadetectortimeoutR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - REG_CNT_CLOCK_STALL_TIMEOUT"]
    #[inline(always)]
    pub fn regcntclockstalltimeout(&self) -> RegcntclockstalltimeoutR {
        RegcntclockstalltimeoutR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - REG_CNT_SDA_DETECTOR_TIMEOUT"]
    #[inline(always)]
    pub fn regcntsdadetectortimeout(&mut self) -> RegcntsdadetectortimeoutW<I3cphyctrlreg0bcSpec> {
        RegcntsdadetectortimeoutW::new(self, 0)
    }
    #[doc = "Bits 16:31 - REG_CNT_CLOCK_STALL_TIMEOUT"]
    #[inline(always)]
    pub fn regcntclockstalltimeout(&mut self) -> RegcntclockstalltimeoutW<I3cphyctrlreg0bcSpec> {
        RegcntclockstalltimeoutW::new(self, 16)
    }
}
#[doc = "SDA\\_DETECTOR\\_CNT2\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg0bc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg0bc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cphyctrlreg0bcSpec;
impl crate::RegisterSpec for I3cphyctrlreg0bcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cphyctrlreg0bc::R`](R) reader structure"]
impl crate::Readable for I3cphyctrlreg0bcSpec {}
#[doc = "`write(|w| ..)` method takes [`i3cphyctrlreg0bc::W`](W) writer structure"]
impl crate::Writable for I3cphyctrlreg0bcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CPHYCTRLREG0BC to value 0x0063_0063"]
impl crate::Resettable for I3cphyctrlreg0bcSpec {
    const RESET_VALUE: u32 = 0x0063_0063;
}
