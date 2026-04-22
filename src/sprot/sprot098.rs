#[doc = "Register `SPROT098` reader"]
pub type R = crate::R<Sprot098Spec>;
#[doc = "Register `SPROT098` writer"]
pub type W = crate::W<Sprot098Spec>;
#[doc = "Field `SRNGWENA06` reader - SRNG_WENA06"]
pub type Srngwena06R = crate::FieldReader;
#[doc = "Field `SRNGWENA06` writer - SRNG_WENA06"]
pub type Srngwena06W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SRNGRENA06` reader - SRNG_RENA06"]
pub type Srngrena06R = crate::FieldReader;
#[doc = "Field `SRNGRENA06` writer - SRNG_RENA06"]
pub type Srngrena06W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - SRNG_WENA06"]
    #[inline(always)]
    pub fn srngwena06(&self) -> Srngwena06R {
        Srngwena06R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - SRNG_RENA06"]
    #[inline(always)]
    pub fn srngrena06(&self) -> Srngrena06R {
        Srngrena06R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SRNG_WENA06"]
    #[inline(always)]
    pub fn srngwena06(&mut self) -> Srngwena06W<Sprot098Spec> {
        Srngwena06W::new(self, 0)
    }
    #[doc = "Bits 8:15 - SRNG_RENA06"]
    #[inline(always)]
    pub fn srngrena06(&mut self) -> Srngrena06W<Sprot098Spec> {
        Srngrena06W::new(self, 8)
    }
}
#[doc = "SPROT\\_CTL06\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot098::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot098::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sprot098Spec;
impl crate::RegisterSpec for Sprot098Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sprot098::R`](R) reader structure"]
impl crate::Readable for Sprot098Spec {}
#[doc = "`write(|w| ..)` method takes [`sprot098::W`](W) writer structure"]
impl crate::Writable for Sprot098Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPROT098 to value 0"]
impl crate::Resettable for Sprot098Spec {}
