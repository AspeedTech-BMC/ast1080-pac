#[doc = "Register `SPROT0E4` reader"]
pub type R = crate::R<Sprot0e4Spec>;
#[doc = "Register `SPROT0E4` writer"]
pub type W = crate::W<Sprot0e4Spec>;
#[doc = "Field `SRNGSADR09` reader - SRNG_SADR09"]
pub type Srngsadr09R = crate::FieldReader<u16>;
#[doc = "Field `SRNGSADR09` writer - SRNG_SADR09"]
pub type Srngsadr09W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SRNGSIZE09` reader - SRNG_SIZE09"]
pub type Srngsize09R = crate::FieldReader<u16>;
#[doc = "Field `SRNGSIZE09` writer - SRNG_SIZE09"]
pub type Srngsize09W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - SRNG_SADR09"]
    #[inline(always)]
    pub fn srngsadr09(&self) -> Srngsadr09R {
        Srngsadr09R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - SRNG_SIZE09"]
    #[inline(always)]
    pub fn srngsize09(&self) -> Srngsize09R {
        Srngsize09R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - SRNG_SADR09"]
    #[inline(always)]
    pub fn srngsadr09(&mut self) -> Srngsadr09W<Sprot0e4Spec> {
        Srngsadr09W::new(self, 0)
    }
    #[doc = "Bits 16:31 - SRNG_SIZE09"]
    #[inline(always)]
    pub fn srngsize09(&mut self) -> Srngsize09W<Sprot0e4Spec> {
        Srngsize09W::new(self, 16)
    }
}
#[doc = "SPROT\\_ADR09\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot0e4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot0e4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sprot0e4Spec;
impl crate::RegisterSpec for Sprot0e4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sprot0e4::R`](R) reader structure"]
impl crate::Readable for Sprot0e4Spec {}
#[doc = "`write(|w| ..)` method takes [`sprot0e4::W`](W) writer structure"]
impl crate::Writable for Sprot0e4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPROT0E4 to value 0"]
impl crate::Resettable for Sprot0e4Spec {}
