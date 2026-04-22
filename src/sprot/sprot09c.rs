#[doc = "Register `SPROT09C` reader"]
pub type R = crate::R<Sprot09cSpec>;
#[doc = "Register `SPROT09C` writer"]
pub type W = crate::W<Sprot09cSpec>;
#[doc = "Field `SRNGWENA07` reader - SRNG_WENA07"]
pub type Srngwena07R = crate::FieldReader;
#[doc = "Field `SRNGWENA07` writer - SRNG_WENA07"]
pub type Srngwena07W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SRNGRENA07` reader - SRNG_RENA07"]
pub type Srngrena07R = crate::FieldReader;
#[doc = "Field `SRNGRENA07` writer - SRNG_RENA07"]
pub type Srngrena07W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - SRNG_WENA07"]
    #[inline(always)]
    pub fn srngwena07(&self) -> Srngwena07R {
        Srngwena07R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - SRNG_RENA07"]
    #[inline(always)]
    pub fn srngrena07(&self) -> Srngrena07R {
        Srngrena07R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SRNG_WENA07"]
    #[inline(always)]
    pub fn srngwena07(&mut self) -> Srngwena07W<Sprot09cSpec> {
        Srngwena07W::new(self, 0)
    }
    #[doc = "Bits 8:15 - SRNG_RENA07"]
    #[inline(always)]
    pub fn srngrena07(&mut self) -> Srngrena07W<Sprot09cSpec> {
        Srngrena07W::new(self, 8)
    }
}
#[doc = "SPROT\\_CTL07\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot09c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot09c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sprot09cSpec;
impl crate::RegisterSpec for Sprot09cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sprot09c::R`](R) reader structure"]
impl crate::Readable for Sprot09cSpec {}
#[doc = "`write(|w| ..)` method takes [`sprot09c::W`](W) writer structure"]
impl crate::Writable for Sprot09cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPROT09C to value 0"]
impl crate::Resettable for Sprot09cSpec {}
