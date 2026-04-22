#[doc = "Register `SPROT0B0` reader"]
pub type R = crate::R<Sprot0b0Spec>;
#[doc = "Register `SPROT0B0` writer"]
pub type W = crate::W<Sprot0b0Spec>;
#[doc = "Field `SRNGWENA12` reader - SRNG_WENA12"]
pub type Srngwena12R = crate::FieldReader;
#[doc = "Field `SRNGWENA12` writer - SRNG_WENA12"]
pub type Srngwena12W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SRNGRENA12` reader - SRNG_RENA12"]
pub type Srngrena12R = crate::FieldReader;
#[doc = "Field `SRNGRENA12` writer - SRNG_RENA12"]
pub type Srngrena12W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - SRNG_WENA12"]
    #[inline(always)]
    pub fn srngwena12(&self) -> Srngwena12R {
        Srngwena12R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - SRNG_RENA12"]
    #[inline(always)]
    pub fn srngrena12(&self) -> Srngrena12R {
        Srngrena12R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SRNG_WENA12"]
    #[inline(always)]
    pub fn srngwena12(&mut self) -> Srngwena12W<Sprot0b0Spec> {
        Srngwena12W::new(self, 0)
    }
    #[doc = "Bits 8:15 - SRNG_RENA12"]
    #[inline(always)]
    pub fn srngrena12(&mut self) -> Srngrena12W<Sprot0b0Spec> {
        Srngrena12W::new(self, 8)
    }
}
#[doc = "SPROT\\_CTL12\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot0b0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot0b0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sprot0b0Spec;
impl crate::RegisterSpec for Sprot0b0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sprot0b0::R`](R) reader structure"]
impl crate::Readable for Sprot0b0Spec {}
#[doc = "`write(|w| ..)` method takes [`sprot0b0::W`](W) writer structure"]
impl crate::Writable for Sprot0b0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPROT0B0 to value 0"]
impl crate::Resettable for Sprot0b0Spec {}
