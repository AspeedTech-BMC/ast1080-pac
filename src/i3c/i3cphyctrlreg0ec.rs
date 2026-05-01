#[doc = "Register `I3CPHYCTRLREG0EC` reader"]
pub type R = crate::R<I3cphyctrlreg0ecSpec>;
#[doc = "Register `I3CPHYCTRLREG0EC` writer"]
pub type W = crate::W<I3cphyctrlreg0ecSpec>;
#[doc = "Field `REGTGSDRSDACONTWAIT` reader - REG_TG_SDR_SDA_CONT_WAIT"]
pub type RegtgsdrsdacontwaitR = crate::FieldReader<u16>;
#[doc = "Field `REGTGSDRSDACONTWAIT` writer - REG_TG_SDR_SDA_CONT_WAIT"]
pub type RegtgsdrsdacontwaitW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `REGTGDDRSDACONTWAIT` reader - REG_TG_DDR_SDA_CONT_WAIT"]
pub type RegtgddrsdacontwaitR = crate::FieldReader<u16>;
#[doc = "Field `REGTGDDRSDACONTWAIT` writer - REG_TG_DDR_SDA_CONT_WAIT"]
pub type RegtgddrsdacontwaitW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - REG_TG_SDR_SDA_CONT_WAIT"]
    #[inline(always)]
    pub fn regtgsdrsdacontwait(&self) -> RegtgsdrsdacontwaitR {
        RegtgsdrsdacontwaitR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:15 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:26 - REG_TG_DDR_SDA_CONT_WAIT"]
    #[inline(always)]
    pub fn regtgddrsdacontwait(&self) -> RegtgddrsdacontwaitR {
        RegtgddrsdacontwaitR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - REG_TG_SDR_SDA_CONT_WAIT"]
    #[inline(always)]
    pub fn regtgsdrsdacontwait(&mut self) -> RegtgsdrsdacontwaitW<I3cphyctrlreg0ecSpec> {
        RegtgsdrsdacontwaitW::new(self, 0)
    }
    #[doc = "Bits 16:26 - REG_TG_DDR_SDA_CONT_WAIT"]
    #[inline(always)]
    pub fn regtgddrsdacontwait(&mut self) -> RegtgddrsdacontwaitW<I3cphyctrlreg0ecSpec> {
        RegtgddrsdacontwaitW::new(self, 16)
    }
}
#[doc = "BUS\\_CONTENTION\\_CNT1\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg0ec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg0ec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cphyctrlreg0ecSpec;
impl crate::RegisterSpec for I3cphyctrlreg0ecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cphyctrlreg0ec::R`](R) reader structure"]
impl crate::Readable for I3cphyctrlreg0ecSpec {}
#[doc = "`write(|w| ..)` method takes [`i3cphyctrlreg0ec::W`](W) writer structure"]
impl crate::Writable for I3cphyctrlreg0ecSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CPHYCTRLREG0EC to value 0"]
impl crate::Resettable for I3cphyctrlreg0ecSpec {}
