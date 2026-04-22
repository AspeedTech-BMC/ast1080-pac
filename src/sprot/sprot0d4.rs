#[doc = "Register `SPROT0D4` reader"]
pub type R = crate::R<Sprot0d4Spec>;
#[doc = "Register `SPROT0D4` writer"]
pub type W = crate::W<Sprot0d4Spec>;
#[doc = "Field `SRNGSADR05` reader - SRNG_SADR05"]
pub type Srngsadr05R = crate::FieldReader<u16>;
#[doc = "Field `SRNGSADR05` writer - SRNG_SADR05"]
pub type Srngsadr05W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SRNGSIZE05` reader - SRNG_SIZE05"]
pub type Srngsize05R = crate::FieldReader<u16>;
#[doc = "Field `SRNGSIZE05` writer - SRNG_SIZE05"]
pub type Srngsize05W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - SRNG_SADR05"]
    #[inline(always)]
    pub fn srngsadr05(&self) -> Srngsadr05R {
        Srngsadr05R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - SRNG_SIZE05"]
    #[inline(always)]
    pub fn srngsize05(&self) -> Srngsize05R {
        Srngsize05R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - SRNG_SADR05"]
    #[inline(always)]
    pub fn srngsadr05(&mut self) -> Srngsadr05W<Sprot0d4Spec> {
        Srngsadr05W::new(self, 0)
    }
    #[doc = "Bits 16:31 - SRNG_SIZE05"]
    #[inline(always)]
    pub fn srngsize05(&mut self) -> Srngsize05W<Sprot0d4Spec> {
        Srngsize05W::new(self, 16)
    }
}
#[doc = "SPROT\\_ADR05\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot0d4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot0d4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sprot0d4Spec;
impl crate::RegisterSpec for Sprot0d4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sprot0d4::R`](R) reader structure"]
impl crate::Readable for Sprot0d4Spec {}
#[doc = "`write(|w| ..)` method takes [`sprot0d4::W`](W) writer structure"]
impl crate::Writable for Sprot0d4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPROT0D4 to value 0"]
impl crate::Resettable for Sprot0d4Spec {}
