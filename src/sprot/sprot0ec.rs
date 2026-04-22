#[doc = "Register `SPROT0EC` reader"]
pub type R = crate::R<Sprot0ecSpec>;
#[doc = "Register `SPROT0EC` writer"]
pub type W = crate::W<Sprot0ecSpec>;
#[doc = "Field `SRNGSADR11` reader - SRNG_SADR11"]
pub type Srngsadr11R = crate::FieldReader<u16>;
#[doc = "Field `SRNGSADR11` writer - SRNG_SADR11"]
pub type Srngsadr11W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SRNGSIZE11` reader - SRNG_SIZE11"]
pub type Srngsize11R = crate::FieldReader<u16>;
#[doc = "Field `SRNGSIZE11` writer - SRNG_SIZE11"]
pub type Srngsize11W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - SRNG_SADR11"]
    #[inline(always)]
    pub fn srngsadr11(&self) -> Srngsadr11R {
        Srngsadr11R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - SRNG_SIZE11"]
    #[inline(always)]
    pub fn srngsize11(&self) -> Srngsize11R {
        Srngsize11R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - SRNG_SADR11"]
    #[inline(always)]
    pub fn srngsadr11(&mut self) -> Srngsadr11W<Sprot0ecSpec> {
        Srngsadr11W::new(self, 0)
    }
    #[doc = "Bits 16:31 - SRNG_SIZE11"]
    #[inline(always)]
    pub fn srngsize11(&mut self) -> Srngsize11W<Sprot0ecSpec> {
        Srngsize11W::new(self, 16)
    }
}
#[doc = "SPROT\\_ADR11\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot0ec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot0ec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sprot0ecSpec;
impl crate::RegisterSpec for Sprot0ecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sprot0ec::R`](R) reader structure"]
impl crate::Readable for Sprot0ecSpec {}
#[doc = "`write(|w| ..)` method takes [`sprot0ec::W`](W) writer structure"]
impl crate::Writable for Sprot0ecSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPROT0EC to value 0"]
impl crate::Resettable for Sprot0ecSpec {}
