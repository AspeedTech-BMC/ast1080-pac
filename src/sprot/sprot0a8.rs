#[doc = "Register `SPROT0A8` reader"]
pub type R = crate::R<Sprot0a8Spec>;
#[doc = "Register `SPROT0A8` writer"]
pub type W = crate::W<Sprot0a8Spec>;
#[doc = "Field `SRNGWENA10` reader - SRNG_WENA10"]
pub type Srngwena10R = crate::FieldReader;
#[doc = "Field `SRNGWENA10` writer - SRNG_WENA10"]
pub type Srngwena10W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SRNGRENA10` reader - SRNG_RENA10"]
pub type Srngrena10R = crate::FieldReader;
#[doc = "Field `SRNGRENA10` writer - SRNG_RENA10"]
pub type Srngrena10W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - SRNG_WENA10"]
    #[inline(always)]
    pub fn srngwena10(&self) -> Srngwena10R {
        Srngwena10R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - SRNG_RENA10"]
    #[inline(always)]
    pub fn srngrena10(&self) -> Srngrena10R {
        Srngrena10R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SRNG_WENA10"]
    #[inline(always)]
    pub fn srngwena10(&mut self) -> Srngwena10W<Sprot0a8Spec> {
        Srngwena10W::new(self, 0)
    }
    #[doc = "Bits 8:15 - SRNG_RENA10"]
    #[inline(always)]
    pub fn srngrena10(&mut self) -> Srngrena10W<Sprot0a8Spec> {
        Srngrena10W::new(self, 8)
    }
}
#[doc = "SPROT\\_CTL10\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot0a8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot0a8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sprot0a8Spec;
impl crate::RegisterSpec for Sprot0a8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sprot0a8::R`](R) reader structure"]
impl crate::Readable for Sprot0a8Spec {}
#[doc = "`write(|w| ..)` method takes [`sprot0a8::W`](W) writer structure"]
impl crate::Writable for Sprot0a8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPROT0A8 to value 0"]
impl crate::Resettable for Sprot0a8Spec {}
