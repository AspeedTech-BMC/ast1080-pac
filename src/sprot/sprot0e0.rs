#[doc = "Register `SPROT0E0` reader"]
pub type R = crate::R<Sprot0e0Spec>;
#[doc = "Register `SPROT0E0` writer"]
pub type W = crate::W<Sprot0e0Spec>;
#[doc = "Field `SRNGSADR08` reader - SRNG_SADR08"]
pub type Srngsadr08R = crate::FieldReader<u16>;
#[doc = "Field `SRNGSADR08` writer - SRNG_SADR08"]
pub type Srngsadr08W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SRNGSIZE08` reader - SRNG_SIZE08"]
pub type Srngsize08R = crate::FieldReader<u16>;
#[doc = "Field `SRNGSIZE08` writer - SRNG_SIZE08"]
pub type Srngsize08W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - SRNG_SADR08"]
    #[inline(always)]
    pub fn srngsadr08(&self) -> Srngsadr08R {
        Srngsadr08R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - SRNG_SIZE08"]
    #[inline(always)]
    pub fn srngsize08(&self) -> Srngsize08R {
        Srngsize08R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - SRNG_SADR08"]
    #[inline(always)]
    pub fn srngsadr08(&mut self) -> Srngsadr08W<Sprot0e0Spec> {
        Srngsadr08W::new(self, 0)
    }
    #[doc = "Bits 16:31 - SRNG_SIZE08"]
    #[inline(always)]
    pub fn srngsize08(&mut self) -> Srngsize08W<Sprot0e0Spec> {
        Srngsize08W::new(self, 16)
    }
}
#[doc = "SPROT\\_ADR08\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot0e0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot0e0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sprot0e0Spec;
impl crate::RegisterSpec for Sprot0e0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sprot0e0::R`](R) reader structure"]
impl crate::Readable for Sprot0e0Spec {}
#[doc = "`write(|w| ..)` method takes [`sprot0e0::W`](W) writer structure"]
impl crate::Writable for Sprot0e0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPROT0E0 to value 0"]
impl crate::Resettable for Sprot0e0Spec {}
