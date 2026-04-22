#[doc = "Register `SPROT0AC` reader"]
pub type R = crate::R<Sprot0acSpec>;
#[doc = "Register `SPROT0AC` writer"]
pub type W = crate::W<Sprot0acSpec>;
#[doc = "Field `SRNGWENA11` reader - SRNG_WENA11"]
pub type Srngwena11R = crate::FieldReader;
#[doc = "Field `SRNGWENA11` writer - SRNG_WENA11"]
pub type Srngwena11W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SRNGRENA11` reader - SRNG_RENA11"]
pub type Srngrena11R = crate::FieldReader;
#[doc = "Field `SRNGRENA11` writer - SRNG_RENA11"]
pub type Srngrena11W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - SRNG_WENA11"]
    #[inline(always)]
    pub fn srngwena11(&self) -> Srngwena11R {
        Srngwena11R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - SRNG_RENA11"]
    #[inline(always)]
    pub fn srngrena11(&self) -> Srngrena11R {
        Srngrena11R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SRNG_WENA11"]
    #[inline(always)]
    pub fn srngwena11(&mut self) -> Srngwena11W<Sprot0acSpec> {
        Srngwena11W::new(self, 0)
    }
    #[doc = "Bits 8:15 - SRNG_RENA11"]
    #[inline(always)]
    pub fn srngrena11(&mut self) -> Srngrena11W<Sprot0acSpec> {
        Srngrena11W::new(self, 8)
    }
}
#[doc = "SPROT\\_CTL11\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot0ac::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot0ac::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sprot0acSpec;
impl crate::RegisterSpec for Sprot0acSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sprot0ac::R`](R) reader structure"]
impl crate::Readable for Sprot0acSpec {}
#[doc = "`write(|w| ..)` method takes [`sprot0ac::W`](W) writer structure"]
impl crate::Writable for Sprot0acSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPROT0AC to value 0"]
impl crate::Resettable for Sprot0acSpec {}
