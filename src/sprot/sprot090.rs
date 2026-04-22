#[doc = "Register `SPROT090` reader"]
pub type R = crate::R<Sprot090Spec>;
#[doc = "Register `SPROT090` writer"]
pub type W = crate::W<Sprot090Spec>;
#[doc = "Field `SRNGWENA04` reader - SRNG_WENA04"]
pub type Srngwena04R = crate::FieldReader;
#[doc = "Field `SRNGWENA04` writer - SRNG_WENA04"]
pub type Srngwena04W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SRNGRENA04` reader - SRNG_RENA04"]
pub type Srngrena04R = crate::FieldReader;
#[doc = "Field `SRNGRENA04` writer - SRNG_RENA04"]
pub type Srngrena04W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - SRNG_WENA04"]
    #[inline(always)]
    pub fn srngwena04(&self) -> Srngwena04R {
        Srngwena04R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - SRNG_RENA04"]
    #[inline(always)]
    pub fn srngrena04(&self) -> Srngrena04R {
        Srngrena04R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SRNG_WENA04"]
    #[inline(always)]
    pub fn srngwena04(&mut self) -> Srngwena04W<Sprot090Spec> {
        Srngwena04W::new(self, 0)
    }
    #[doc = "Bits 8:15 - SRNG_RENA04"]
    #[inline(always)]
    pub fn srngrena04(&mut self) -> Srngrena04W<Sprot090Spec> {
        Srngrena04W::new(self, 8)
    }
}
#[doc = "SPROT\\_CTL04\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot090::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot090::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sprot090Spec;
impl crate::RegisterSpec for Sprot090Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sprot090::R`](R) reader structure"]
impl crate::Readable for Sprot090Spec {}
#[doc = "`write(|w| ..)` method takes [`sprot090::W`](W) writer structure"]
impl crate::Writable for Sprot090Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPROT090 to value 0"]
impl crate::Resettable for Sprot090Spec {}
