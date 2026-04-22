#[doc = "Register `SPROT0F4` reader"]
pub type R = crate::R<Sprot0f4Spec>;
#[doc = "Register `SPROT0F4` writer"]
pub type W = crate::W<Sprot0f4Spec>;
#[doc = "Field `SRNGSADR13` reader - SRNG_SADR13"]
pub type Srngsadr13R = crate::FieldReader<u16>;
#[doc = "Field `SRNGSADR13` writer - SRNG_SADR13"]
pub type Srngsadr13W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SRNGSIZE13` reader - SRNG_SIZE13"]
pub type Srngsize13R = crate::FieldReader<u16>;
#[doc = "Field `SRNGSIZE13` writer - SRNG_SIZE13"]
pub type Srngsize13W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - SRNG_SADR13"]
    #[inline(always)]
    pub fn srngsadr13(&self) -> Srngsadr13R {
        Srngsadr13R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - SRNG_SIZE13"]
    #[inline(always)]
    pub fn srngsize13(&self) -> Srngsize13R {
        Srngsize13R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - SRNG_SADR13"]
    #[inline(always)]
    pub fn srngsadr13(&mut self) -> Srngsadr13W<Sprot0f4Spec> {
        Srngsadr13W::new(self, 0)
    }
    #[doc = "Bits 16:31 - SRNG_SIZE13"]
    #[inline(always)]
    pub fn srngsize13(&mut self) -> Srngsize13W<Sprot0f4Spec> {
        Srngsize13W::new(self, 16)
    }
}
#[doc = "SPROT\\_ADR13\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot0f4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot0f4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sprot0f4Spec;
impl crate::RegisterSpec for Sprot0f4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sprot0f4::R`](R) reader structure"]
impl crate::Readable for Sprot0f4Spec {}
#[doc = "`write(|w| ..)` method takes [`sprot0f4::W`](W) writer structure"]
impl crate::Writable for Sprot0f4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPROT0F4 to value 0"]
impl crate::Resettable for Sprot0f4Spec {}
