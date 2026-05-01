#[doc = "Register `SCUC94` reader"]
pub type R = crate::R<Scuc94Spec>;
#[doc = "Register `SCUC94` writer"]
pub type W = crate::W<Scuc94Spec>;
#[doc = "Field `SCUREGSEC22F0` reader - SCU_REG_SEC2_2F0"]
pub type Scuregsec22f0R = crate::BitReader;
#[doc = "Field `SCUREGSEC22F0` writer - SCU_REG_SEC2_2F0"]
pub type Scuregsec22f0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC22F4` reader - SCU_REG_SEC2_2F4"]
pub type Scuregsec22f4R = crate::BitReader;
#[doc = "Field `SCUREGSEC22F4` writer - SCU_REG_SEC2_2F4"]
pub type Scuregsec22f4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC22F8` reader - SCU_REG_SEC2_2F8"]
pub type Scuregsec22f8R = crate::BitReader;
#[doc = "Field `SCUREGSEC22F8` writer - SCU_REG_SEC2_2F8"]
pub type Scuregsec22f8W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 28 - SCU_REG_SEC2_2F0"]
    #[inline(always)]
    pub fn scuregsec22f0(&self) -> Scuregsec22f0R {
        Scuregsec22f0R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - SCU_REG_SEC2_2F4"]
    #[inline(always)]
    pub fn scuregsec22f4(&self) -> Scuregsec22f4R {
        Scuregsec22f4R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - SCU_REG_SEC2_2F8"]
    #[inline(always)]
    pub fn scuregsec22f8(&self) -> Scuregsec22f8R {
        Scuregsec22f8R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 28 - SCU_REG_SEC2_2F0"]
    #[inline(always)]
    pub fn scuregsec22f0(&mut self) -> Scuregsec22f0W<Scuc94Spec> {
        Scuregsec22f0W::new(self, 28)
    }
    #[doc = "Bit 29 - SCU_REG_SEC2_2F4"]
    #[inline(always)]
    pub fn scuregsec22f4(&mut self) -> Scuregsec22f4W<Scuc94Spec> {
        Scuregsec22f4W::new(self, 29)
    }
    #[doc = "Bit 30 - SCU_REG_SEC2_2F8"]
    #[inline(always)]
    pub fn scuregsec22f8(&mut self) -> Scuregsec22f8W<Scuc94Spec> {
        Scuregsec22f8W::new(self, 30)
    }
}
#[doc = "Secure2 Control 6 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuc94::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuc94::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scuc94Spec;
impl crate::RegisterSpec for Scuc94Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scuc94::R`](R) reader structure"]
impl crate::Readable for Scuc94Spec {}
#[doc = "`write(|w| ..)` method takes [`scuc94::W`](W) writer structure"]
impl crate::Writable for Scuc94Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUC94 to value 0"]
impl crate::Resettable for Scuc94Spec {}
