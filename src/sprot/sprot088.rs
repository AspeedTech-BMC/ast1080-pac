#[doc = "Register `SPROT088` reader"]
pub type R = crate::R<Sprot088Spec>;
#[doc = "Register `SPROT088` writer"]
pub type W = crate::W<Sprot088Spec>;
#[doc = "Field `SRNGWENA02` reader - SRNG_WENA02"]
pub type Srngwena02R = crate::FieldReader;
#[doc = "Field `SRNGWENA02` writer - SRNG_WENA02"]
pub type Srngwena02W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SRNGRENA02` reader - SRNG_RENA02"]
pub type Srngrena02R = crate::FieldReader;
#[doc = "Field `SRNGRENA02` writer - SRNG_RENA02"]
pub type Srngrena02W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - SRNG_WENA02"]
    #[inline(always)]
    pub fn srngwena02(&self) -> Srngwena02R {
        Srngwena02R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - SRNG_RENA02"]
    #[inline(always)]
    pub fn srngrena02(&self) -> Srngrena02R {
        Srngrena02R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SRNG_WENA02"]
    #[inline(always)]
    pub fn srngwena02(&mut self) -> Srngwena02W<Sprot088Spec> {
        Srngwena02W::new(self, 0)
    }
    #[doc = "Bits 8:15 - SRNG_RENA02"]
    #[inline(always)]
    pub fn srngrena02(&mut self) -> Srngrena02W<Sprot088Spec> {
        Srngrena02W::new(self, 8)
    }
}
#[doc = "SPROT\\_CTL02\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot088::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot088::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sprot088Spec;
impl crate::RegisterSpec for Sprot088Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sprot088::R`](R) reader structure"]
impl crate::Readable for Sprot088Spec {}
#[doc = "`write(|w| ..)` method takes [`sprot088::W`](W) writer structure"]
impl crate::Writable for Sprot088Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPROT088 to value 0"]
impl crate::Resettable for Sprot088Spec {}
