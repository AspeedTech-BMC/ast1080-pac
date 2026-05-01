#[doc = "Register `SCUC14` reader"]
pub type R = crate::R<Scuc14Spec>;
#[doc = "Register `SCUC14` writer"]
pub type W = crate::W<Scuc14Spec>;
#[doc = "Field `SCUREGSEC12F0` reader - SCU_REG_SEC1_2F0"]
pub type Scuregsec12f0R = crate::BitReader;
#[doc = "Field `SCUREGSEC12F0` writer - SCU_REG_SEC1_2F0"]
pub type Scuregsec12f0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC12F4` reader - SCU_REG_SEC1_2F4"]
pub type Scuregsec12f4R = crate::BitReader;
#[doc = "Field `SCUREGSEC12F4` writer - SCU_REG_SEC1_2F4"]
pub type Scuregsec12f4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC12F8` reader - SCU_REG_SEC1_2F8"]
pub type Scuregsec12f8R = crate::BitReader;
#[doc = "Field `SCUREGSEC12F8` writer - SCU_REG_SEC1_2F8"]
pub type Scuregsec12f8W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 28 - SCU_REG_SEC1_2F0"]
    #[inline(always)]
    pub fn scuregsec12f0(&self) -> Scuregsec12f0R {
        Scuregsec12f0R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - SCU_REG_SEC1_2F4"]
    #[inline(always)]
    pub fn scuregsec12f4(&self) -> Scuregsec12f4R {
        Scuregsec12f4R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - SCU_REG_SEC1_2F8"]
    #[inline(always)]
    pub fn scuregsec12f8(&self) -> Scuregsec12f8R {
        Scuregsec12f8R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 28 - SCU_REG_SEC1_2F0"]
    #[inline(always)]
    pub fn scuregsec12f0(&mut self) -> Scuregsec12f0W<Scuc14Spec> {
        Scuregsec12f0W::new(self, 28)
    }
    #[doc = "Bit 29 - SCU_REG_SEC1_2F4"]
    #[inline(always)]
    pub fn scuregsec12f4(&mut self) -> Scuregsec12f4W<Scuc14Spec> {
        Scuregsec12f4W::new(self, 29)
    }
    #[doc = "Bit 30 - SCU_REG_SEC1_2F8"]
    #[inline(always)]
    pub fn scuregsec12f8(&mut self) -> Scuregsec12f8W<Scuc14Spec> {
        Scuregsec12f8W::new(self, 30)
    }
}
#[doc = "Secure1 Control 6 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuc14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuc14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scuc14Spec;
impl crate::RegisterSpec for Scuc14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scuc14::R`](R) reader structure"]
impl crate::Readable for Scuc14Spec {}
#[doc = "`write(|w| ..)` method takes [`scuc14::W`](W) writer structure"]
impl crate::Writable for Scuc14Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUC14 to value 0"]
impl crate::Resettable for Scuc14Spec {}
