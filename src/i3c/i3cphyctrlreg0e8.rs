#[doc = "Register `I3CPHYCTRLREG0E8` reader"]
pub type R = crate::R<I3cphyctrlreg0e8Spec>;
#[doc = "Register `I3CPHYCTRLREG0E8` writer"]
pub type W = crate::W<I3cphyctrlreg0e8Spec>;
#[doc = "Field `REGCRSDRSDACONTWAIT` reader - REG_CR_SDR_SDA_CONT_WAIT"]
pub type RegcrsdrsdacontwaitR = crate::FieldReader<u16>;
#[doc = "Field `REGCRSDRSDACONTWAIT` writer - REG_CR_SDR_SDA_CONT_WAIT"]
pub type RegcrsdrsdacontwaitW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `REGCRDDRSDACONTWAIT` reader - REG_CR_DDR_SDA_CONT_WAIT"]
pub type RegcrddrsdacontwaitR = crate::FieldReader<u16>;
#[doc = "Field `REGCRDDRSDACONTWAIT` writer - REG_CR_DDR_SDA_CONT_WAIT"]
pub type RegcrddrsdacontwaitW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - REG_CR_SDR_SDA_CONT_WAIT"]
    #[inline(always)]
    pub fn regcrsdrsdacontwait(&self) -> RegcrsdrsdacontwaitR {
        RegcrsdrsdacontwaitR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:15 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:26 - REG_CR_DDR_SDA_CONT_WAIT"]
    #[inline(always)]
    pub fn regcrddrsdacontwait(&self) -> RegcrddrsdacontwaitR {
        RegcrddrsdacontwaitR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - REG_CR_SDR_SDA_CONT_WAIT"]
    #[inline(always)]
    pub fn regcrsdrsdacontwait(&mut self) -> RegcrsdrsdacontwaitW<I3cphyctrlreg0e8Spec> {
        RegcrsdrsdacontwaitW::new(self, 0)
    }
    #[doc = "Bits 16:26 - REG_CR_DDR_SDA_CONT_WAIT"]
    #[inline(always)]
    pub fn regcrddrsdacontwait(&mut self) -> RegcrddrsdacontwaitW<I3cphyctrlreg0e8Spec> {
        RegcrddrsdacontwaitW::new(self, 16)
    }
}
#[doc = "BUS\\_CONTENTION\\_CNT0\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg0e8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg0e8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cphyctrlreg0e8Spec;
impl crate::RegisterSpec for I3cphyctrlreg0e8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cphyctrlreg0e8::R`](R) reader structure"]
impl crate::Readable for I3cphyctrlreg0e8Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cphyctrlreg0e8::W`](W) writer structure"]
impl crate::Writable for I3cphyctrlreg0e8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CPHYCTRLREG0E8 to value 0"]
impl crate::Resettable for I3cphyctrlreg0e8Spec {}
