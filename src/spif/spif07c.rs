#[doc = "Register `SPIF07C` reader"]
pub type R = crate::R<Spif07cSpec>;
#[doc = "Register `SPIF07C` writer"]
pub type W = crate::W<Spif07cSpec>;
#[doc = "Field `WLOCK` reader - WLOCK"]
pub type WlockR = crate::FieldReader<u32>;
#[doc = "Field `WLOCK` writer - WLOCK"]
pub type WlockW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WLOCK"]
    #[inline(always)]
    pub fn wlock(&self) -> WlockR {
        WlockR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WLOCK"]
    #[inline(always)]
    pub fn wlock(&mut self) -> WlockW<Spif07cSpec> {
        WlockW::new(self, 0)
    }
}
#[doc = "SPIF\\_WLOCK\n\nYou can [`read`](crate::Reg::read) this register and get [`spif07c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif07c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif07cSpec;
impl crate::RegisterSpec for Spif07cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif07c::R`](R) reader structure"]
impl crate::Readable for Spif07cSpec {}
#[doc = "`write(|w| ..)` method takes [`spif07c::W`](W) writer structure"]
impl crate::Writable for Spif07cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF07C to value 0"]
impl crate::Resettable for Spif07cSpec {}
