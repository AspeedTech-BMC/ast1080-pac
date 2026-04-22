#[doc = "Register `SPROT0B8` reader"]
pub type R = crate::R<Sprot0b8Spec>;
#[doc = "Register `SPROT0B8` writer"]
pub type W = crate::W<Sprot0b8Spec>;
#[doc = "Field `SRNGWENA14` reader - SRNG_WENA14"]
pub type Srngwena14R = crate::FieldReader;
#[doc = "Field `SRNGWENA14` writer - SRNG_WENA14"]
pub type Srngwena14W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SRNGRENA14` reader - SRNG_RENA14"]
pub type Srngrena14R = crate::FieldReader;
#[doc = "Field `SRNGRENA14` writer - SRNG_RENA14"]
pub type Srngrena14W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - SRNG_WENA14"]
    #[inline(always)]
    pub fn srngwena14(&self) -> Srngwena14R {
        Srngwena14R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - SRNG_RENA14"]
    #[inline(always)]
    pub fn srngrena14(&self) -> Srngrena14R {
        Srngrena14R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SRNG_WENA14"]
    #[inline(always)]
    pub fn srngwena14(&mut self) -> Srngwena14W<Sprot0b8Spec> {
        Srngwena14W::new(self, 0)
    }
    #[doc = "Bits 8:15 - SRNG_RENA14"]
    #[inline(always)]
    pub fn srngrena14(&mut self) -> Srngrena14W<Sprot0b8Spec> {
        Srngrena14W::new(self, 8)
    }
}
#[doc = "SPROT\\_CTL14\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot0b8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot0b8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sprot0b8Spec;
impl crate::RegisterSpec for Sprot0b8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sprot0b8::R`](R) reader structure"]
impl crate::Readable for Sprot0b8Spec {}
#[doc = "`write(|w| ..)` method takes [`sprot0b8::W`](W) writer structure"]
impl crate::Writable for Sprot0b8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPROT0B8 to value 0"]
impl crate::Resettable for Sprot0b8Spec {}
