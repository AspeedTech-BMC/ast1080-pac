#[doc = "Register `SPROT0FC` reader"]
pub type R = crate::R<Sprot0fcSpec>;
#[doc = "Register `SPROT0FC` writer"]
pub type W = crate::W<Sprot0fcSpec>;
#[doc = "Field `SRNGSADR15` reader - SRNG_SADR15"]
pub type Srngsadr15R = crate::FieldReader<u16>;
#[doc = "Field `SRNGSADR15` writer - SRNG_SADR15"]
pub type Srngsadr15W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SRNGSIZE15` reader - SRNG_SIZE15"]
pub type Srngsize15R = crate::FieldReader<u16>;
#[doc = "Field `SRNGSIZE15` writer - SRNG_SIZE15"]
pub type Srngsize15W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - SRNG_SADR15"]
    #[inline(always)]
    pub fn srngsadr15(&self) -> Srngsadr15R {
        Srngsadr15R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - SRNG_SIZE15"]
    #[inline(always)]
    pub fn srngsize15(&self) -> Srngsize15R {
        Srngsize15R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - SRNG_SADR15"]
    #[inline(always)]
    pub fn srngsadr15(&mut self) -> Srngsadr15W<Sprot0fcSpec> {
        Srngsadr15W::new(self, 0)
    }
    #[doc = "Bits 16:31 - SRNG_SIZE15"]
    #[inline(always)]
    pub fn srngsize15(&mut self) -> Srngsize15W<Sprot0fcSpec> {
        Srngsize15W::new(self, 16)
    }
}
#[doc = "SPROT\\_ADR15\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot0fc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot0fc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sprot0fcSpec;
impl crate::RegisterSpec for Sprot0fcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sprot0fc::R`](R) reader structure"]
impl crate::Readable for Sprot0fcSpec {}
#[doc = "`write(|w| ..)` method takes [`sprot0fc::W`](W) writer structure"]
impl crate::Writable for Sprot0fcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPROT0FC to value 0"]
impl crate::Resettable for Sprot0fcSpec {}
