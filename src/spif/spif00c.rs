#[doc = "Register `SPIF00C` reader"]
pub type R = crate::R<Spif00cSpec>;
#[doc = "Register `SPIF00C` writer"]
pub type W = crate::W<Spif00cSpec>;
#[doc = "Field `ELOGCNT` reader - ELOG_CNT"]
pub type ElogcntR = crate::FieldReader;
#[doc = "Field `ELOGCNT` writer - ELOG_CNT"]
pub type ElogcntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - ELOG_CNT"]
    #[inline(always)]
    pub fn elogcnt(&self) -> ElogcntR {
        ElogcntR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ELOG_CNT"]
    #[inline(always)]
    pub fn elogcnt(&mut self) -> ElogcntW<Spif00cSpec> {
        ElogcntW::new(self, 0)
    }
}
#[doc = "SPIF\\_ELOG\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`spif00c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif00c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif00cSpec;
impl crate::RegisterSpec for Spif00cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif00c::R`](R) reader structure"]
impl crate::Readable for Spif00cSpec {}
#[doc = "`write(|w| ..)` method takes [`spif00c::W`](W) writer structure"]
impl crate::Writable for Spif00cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF00C to value 0"]
impl crate::Resettable for Spif00cSpec {}
