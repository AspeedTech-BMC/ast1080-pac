#[doc = "Register `SPROT0BC` reader"]
pub type R = crate::R<Sprot0bcSpec>;
#[doc = "Register `SPROT0BC` writer"]
pub type W = crate::W<Sprot0bcSpec>;
#[doc = "Field `SRNGWENA15` reader - SRNG_WENA15"]
pub type Srngwena15R = crate::FieldReader;
#[doc = "Field `SRNGWENA15` writer - SRNG_WENA15"]
pub type Srngwena15W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SRNGRENA15` reader - SRNG_RENA15"]
pub type Srngrena15R = crate::FieldReader;
#[doc = "Field `SRNGRENA15` writer - SRNG_RENA15"]
pub type Srngrena15W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - SRNG_WENA15"]
    #[inline(always)]
    pub fn srngwena15(&self) -> Srngwena15R {
        Srngwena15R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - SRNG_RENA15"]
    #[inline(always)]
    pub fn srngrena15(&self) -> Srngrena15R {
        Srngrena15R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SRNG_WENA15"]
    #[inline(always)]
    pub fn srngwena15(&mut self) -> Srngwena15W<Sprot0bcSpec> {
        Srngwena15W::new(self, 0)
    }
    #[doc = "Bits 8:15 - SRNG_RENA15"]
    #[inline(always)]
    pub fn srngrena15(&mut self) -> Srngrena15W<Sprot0bcSpec> {
        Srngrena15W::new(self, 8)
    }
}
#[doc = "SPROT\\_CTL15\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot0bc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot0bc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sprot0bcSpec;
impl crate::RegisterSpec for Sprot0bcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sprot0bc::R`](R) reader structure"]
impl crate::Readable for Sprot0bcSpec {}
#[doc = "`write(|w| ..)` method takes [`sprot0bc::W`](W) writer structure"]
impl crate::Writable for Sprot0bcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPROT0BC to value 0"]
impl crate::Resettable for Sprot0bcSpec {}
