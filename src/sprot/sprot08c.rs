#[doc = "Register `SPROT08C` reader"]
pub type R = crate::R<Sprot08cSpec>;
#[doc = "Register `SPROT08C` writer"]
pub type W = crate::W<Sprot08cSpec>;
#[doc = "Field `SRNGWENA03` reader - SRNG_WENA03"]
pub type Srngwena03R = crate::FieldReader;
#[doc = "Field `SRNGWENA03` writer - SRNG_WENA03"]
pub type Srngwena03W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SRNGRENA03` reader - SRNG_RENA03"]
pub type Srngrena03R = crate::FieldReader;
#[doc = "Field `SRNGRENA03` writer - SRNG_RENA03"]
pub type Srngrena03W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - SRNG_WENA03"]
    #[inline(always)]
    pub fn srngwena03(&self) -> Srngwena03R {
        Srngwena03R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - SRNG_RENA03"]
    #[inline(always)]
    pub fn srngrena03(&self) -> Srngrena03R {
        Srngrena03R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SRNG_WENA03"]
    #[inline(always)]
    pub fn srngwena03(&mut self) -> Srngwena03W<Sprot08cSpec> {
        Srngwena03W::new(self, 0)
    }
    #[doc = "Bits 8:15 - SRNG_RENA03"]
    #[inline(always)]
    pub fn srngrena03(&mut self) -> Srngrena03W<Sprot08cSpec> {
        Srngrena03W::new(self, 8)
    }
}
#[doc = "SPROT\\_CTL03\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot08c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot08c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sprot08cSpec;
impl crate::RegisterSpec for Sprot08cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sprot08c::R`](R) reader structure"]
impl crate::Readable for Sprot08cSpec {}
#[doc = "`write(|w| ..)` method takes [`sprot08c::W`](W) writer structure"]
impl crate::Writable for Sprot08cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPROT08C to value 0"]
impl crate::Resettable for Sprot08cSpec {}
