#[doc = "Register `SPROT080` reader"]
pub type R = crate::R<Sprot080Spec>;
#[doc = "Register `SPROT080` writer"]
pub type W = crate::W<Sprot080Spec>;
#[doc = "Field `SRNGWENA00` reader - SRNG_WENA00"]
pub type Srngwena00R = crate::FieldReader;
#[doc = "Field `SRNGWENA00` writer - SRNG_WENA00"]
pub type Srngwena00W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SRNGRENA00` reader - SRNG_RENA00"]
pub type Srngrena00R = crate::FieldReader;
#[doc = "Field `SRNGRENA00` writer - SRNG_RENA00"]
pub type Srngrena00W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - SRNG_WENA00"]
    #[inline(always)]
    pub fn srngwena00(&self) -> Srngwena00R {
        Srngwena00R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - SRNG_RENA00"]
    #[inline(always)]
    pub fn srngrena00(&self) -> Srngrena00R {
        Srngrena00R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SRNG_WENA00"]
    #[inline(always)]
    pub fn srngwena00(&mut self) -> Srngwena00W<Sprot080Spec> {
        Srngwena00W::new(self, 0)
    }
    #[doc = "Bits 8:15 - SRNG_RENA00"]
    #[inline(always)]
    pub fn srngrena00(&mut self) -> Srngrena00W<Sprot080Spec> {
        Srngrena00W::new(self, 8)
    }
}
#[doc = "SPROT\\_CTL00\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot080::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot080::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sprot080Spec;
impl crate::RegisterSpec for Sprot080Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sprot080::R`](R) reader structure"]
impl crate::Readable for Sprot080Spec {}
#[doc = "`write(|w| ..)` method takes [`sprot080::W`](W) writer structure"]
impl crate::Writable for Sprot080Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPROT080 to value 0"]
impl crate::Resettable for Sprot080Spec {}
