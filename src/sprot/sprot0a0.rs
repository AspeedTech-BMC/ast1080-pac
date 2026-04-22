#[doc = "Register `SPROT0A0` reader"]
pub type R = crate::R<Sprot0a0Spec>;
#[doc = "Register `SPROT0A0` writer"]
pub type W = crate::W<Sprot0a0Spec>;
#[doc = "Field `SRNGWENA08` reader - SRNG_WENA08"]
pub type Srngwena08R = crate::FieldReader;
#[doc = "Field `SRNGWENA08` writer - SRNG_WENA08"]
pub type Srngwena08W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SRNGRENA08` reader - SRNG_RENA08"]
pub type Srngrena08R = crate::FieldReader;
#[doc = "Field `SRNGRENA08` writer - SRNG_RENA08"]
pub type Srngrena08W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - SRNG_WENA08"]
    #[inline(always)]
    pub fn srngwena08(&self) -> Srngwena08R {
        Srngwena08R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - SRNG_RENA08"]
    #[inline(always)]
    pub fn srngrena08(&self) -> Srngrena08R {
        Srngrena08R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SRNG_WENA08"]
    #[inline(always)]
    pub fn srngwena08(&mut self) -> Srngwena08W<Sprot0a0Spec> {
        Srngwena08W::new(self, 0)
    }
    #[doc = "Bits 8:15 - SRNG_RENA08"]
    #[inline(always)]
    pub fn srngrena08(&mut self) -> Srngrena08W<Sprot0a0Spec> {
        Srngrena08W::new(self, 8)
    }
}
#[doc = "SPROT\\_CTL08\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot0a0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot0a0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sprot0a0Spec;
impl crate::RegisterSpec for Sprot0a0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sprot0a0::R`](R) reader structure"]
impl crate::Readable for Sprot0a0Spec {}
#[doc = "`write(|w| ..)` method takes [`sprot0a0::W`](W) writer structure"]
impl crate::Writable for Sprot0a0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPROT0A0 to value 0"]
impl crate::Resettable for Sprot0a0Spec {}
