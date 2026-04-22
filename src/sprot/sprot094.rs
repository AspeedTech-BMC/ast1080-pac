#[doc = "Register `SPROT094` reader"]
pub type R = crate::R<Sprot094Spec>;
#[doc = "Register `SPROT094` writer"]
pub type W = crate::W<Sprot094Spec>;
#[doc = "Field `SRNGWENA05` reader - SRNG_WENA05"]
pub type Srngwena05R = crate::FieldReader;
#[doc = "Field `SRNGWENA05` writer - SRNG_WENA05"]
pub type Srngwena05W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SRNGRENA05` reader - SRNG_RENA05"]
pub type Srngrena05R = crate::FieldReader;
#[doc = "Field `SRNGRENA05` writer - SRNG_RENA05"]
pub type Srngrena05W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - SRNG_WENA05"]
    #[inline(always)]
    pub fn srngwena05(&self) -> Srngwena05R {
        Srngwena05R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - SRNG_RENA05"]
    #[inline(always)]
    pub fn srngrena05(&self) -> Srngrena05R {
        Srngrena05R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SRNG_WENA05"]
    #[inline(always)]
    pub fn srngwena05(&mut self) -> Srngwena05W<Sprot094Spec> {
        Srngwena05W::new(self, 0)
    }
    #[doc = "Bits 8:15 - SRNG_RENA05"]
    #[inline(always)]
    pub fn srngrena05(&mut self) -> Srngrena05W<Sprot094Spec> {
        Srngrena05W::new(self, 8)
    }
}
#[doc = "SPROT\\_CTL05\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot094::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot094::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sprot094Spec;
impl crate::RegisterSpec for Sprot094Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sprot094::R`](R) reader structure"]
impl crate::Readable for Sprot094Spec {}
#[doc = "`write(|w| ..)` method takes [`sprot094::W`](W) writer structure"]
impl crate::Writable for Sprot094Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPROT094 to value 0"]
impl crate::Resettable for Sprot094Spec {}
