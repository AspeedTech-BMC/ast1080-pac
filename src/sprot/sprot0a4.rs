#[doc = "Register `SPROT0A4` reader"]
pub type R = crate::R<Sprot0a4Spec>;
#[doc = "Register `SPROT0A4` writer"]
pub type W = crate::W<Sprot0a4Spec>;
#[doc = "Field `SRNGWENA09` reader - SRNG_WENA09"]
pub type Srngwena09R = crate::FieldReader;
#[doc = "Field `SRNGWENA09` writer - SRNG_WENA09"]
pub type Srngwena09W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SRNGRENA09` reader - SRNG_RENA09"]
pub type Srngrena09R = crate::FieldReader;
#[doc = "Field `SRNGRENA09` writer - SRNG_RENA09"]
pub type Srngrena09W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - SRNG_WENA09"]
    #[inline(always)]
    pub fn srngwena09(&self) -> Srngwena09R {
        Srngwena09R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - SRNG_RENA09"]
    #[inline(always)]
    pub fn srngrena09(&self) -> Srngrena09R {
        Srngrena09R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SRNG_WENA09"]
    #[inline(always)]
    pub fn srngwena09(&mut self) -> Srngwena09W<Sprot0a4Spec> {
        Srngwena09W::new(self, 0)
    }
    #[doc = "Bits 8:15 - SRNG_RENA09"]
    #[inline(always)]
    pub fn srngrena09(&mut self) -> Srngrena09W<Sprot0a4Spec> {
        Srngrena09W::new(self, 8)
    }
}
#[doc = "SPROT\\_CTL09\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot0a4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot0a4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sprot0a4Spec;
impl crate::RegisterSpec for Sprot0a4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sprot0a4::R`](R) reader structure"]
impl crate::Readable for Sprot0a4Spec {}
#[doc = "`write(|w| ..)` method takes [`sprot0a4::W`](W) writer structure"]
impl crate::Writable for Sprot0a4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPROT0A4 to value 0"]
impl crate::Resettable for Sprot0a4Spec {}
