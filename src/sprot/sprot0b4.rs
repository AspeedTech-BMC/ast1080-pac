#[doc = "Register `SPROT0B4` reader"]
pub type R = crate::R<Sprot0b4Spec>;
#[doc = "Register `SPROT0B4` writer"]
pub type W = crate::W<Sprot0b4Spec>;
#[doc = "Field `SRNGWENA13` reader - SRNG_WENA13"]
pub type Srngwena13R = crate::FieldReader;
#[doc = "Field `SRNGWENA13` writer - SRNG_WENA13"]
pub type Srngwena13W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SRNGRENA13` reader - SRNG_RENA13"]
pub type Srngrena13R = crate::FieldReader;
#[doc = "Field `SRNGRENA13` writer - SRNG_RENA13"]
pub type Srngrena13W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - SRNG_WENA13"]
    #[inline(always)]
    pub fn srngwena13(&self) -> Srngwena13R {
        Srngwena13R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - SRNG_RENA13"]
    #[inline(always)]
    pub fn srngrena13(&self) -> Srngrena13R {
        Srngrena13R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SRNG_WENA13"]
    #[inline(always)]
    pub fn srngwena13(&mut self) -> Srngwena13W<Sprot0b4Spec> {
        Srngwena13W::new(self, 0)
    }
    #[doc = "Bits 8:15 - SRNG_RENA13"]
    #[inline(always)]
    pub fn srngrena13(&mut self) -> Srngrena13W<Sprot0b4Spec> {
        Srngrena13W::new(self, 8)
    }
}
#[doc = "SPROT\\_CTL13\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot0b4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot0b4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sprot0b4Spec;
impl crate::RegisterSpec for Sprot0b4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sprot0b4::R`](R) reader structure"]
impl crate::Readable for Sprot0b4Spec {}
#[doc = "`write(|w| ..)` method takes [`sprot0b4::W`](W) writer structure"]
impl crate::Writable for Sprot0b4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPROT0B4 to value 0"]
impl crate::Resettable for Sprot0b4Spec {}
