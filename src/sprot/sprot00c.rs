#[doc = "Register `SPROT00C` reader"]
pub type R = crate::R<Sprot00cSpec>;
#[doc = "Register `SPROT00C` writer"]
pub type W = crate::W<Sprot00cSpec>;
#[doc = "Field `WLOCK` reader - WLOCK"]
pub type WlockR = crate::FieldReader;
#[doc = "Field `WLOCK` writer - WLOCK"]
pub type WlockW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - WLOCK"]
    #[inline(always)]
    pub fn wlock(&self) -> WlockR {
        WlockR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - WLOCK"]
    #[inline(always)]
    pub fn wlock(&mut self) -> WlockW<Sprot00cSpec> {
        WlockW::new(self, 0)
    }
}
#[doc = "SPROT\\_WLOCK\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot00c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot00c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sprot00cSpec;
impl crate::RegisterSpec for Sprot00cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sprot00c::R`](R) reader structure"]
impl crate::Readable for Sprot00cSpec {}
#[doc = "`write(|w| ..)` method takes [`sprot00c::W`](W) writer structure"]
impl crate::Writable for Sprot00cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPROT00C to value 0"]
impl crate::Resettable for Sprot00cSpec {}
